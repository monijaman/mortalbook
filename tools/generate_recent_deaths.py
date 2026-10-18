#!/usr/bin/env python3
"""Fetch notable recent deaths from Wikidata and submit them to Mortalbook for review."""

from __future__ import annotations

import json
import os
import re
import time
from datetime import date, datetime, timedelta, timezone
from typing import Any
from urllib.error import HTTPError, URLError
from urllib.parse import urlencode
from urllib.request import Request, urlopen

ENDPOINT = "https://query.wikidata.org/sparql"
USER_AGENT = "MortalbookRecentDeathsBot/1.0 (GitHub Actions)"
SPARQL_TIMEOUT_SECONDS = 120
SPARQL_ATTEMPTS = 3
SPARQL_OCCUPATION_BATCH_SIZE = 4
SUBMIT_ATTEMPTS = 3
MAX_IMAGES = 5

OCCUPATIONS = (
    "Q33999",       # actor
    "Q2405480",     # voice actor
    "Q177220",      # singer
    "Q639669",      # musician
    "Q82955",       # politician
    "Q49757",       # writer
    "Q2526255",     # film director
    "Q36834",       # composer
    "Q2066131",     # athlete
    "Q1930187",     # journalist
    "Q901",         # scientist
    "Q43845",       # businessperson
    "Q947873",      # television presenter
    "Q483501",      # artist
    "Q189290",      # military officer
    "Q1231865",     # religious leader
)

QUERY = """
PREFIX bd: <http://www.bigdata.com/rdf#>
PREFIX schema: <http://schema.org/>
PREFIX wikibase: <http://wikiba.se/ontology#>
PREFIX wdt: <http://www.wikidata.org/prop/direct/>
PREFIX wd: <http://www.wikidata.org/entity/>
PREFIX p: <http://www.wikidata.org/prop/>
PREFIX psv: <http://www.wikidata.org/prop/statement/value/>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
SELECT ?person ?personLabel ?dateOfBirth ?dateOfDeath ?occupationLabel ?image
       ?birthPlaceLabel ?deathPlaceLabel ?article WHERE {
  VALUES ?occupation { __OCCUPATIONS__ }
  ?person wdt:P31 wd:Q5 ;
          wdt:P106 ?occupation ;
          wdt:P570 ?dateOfDeath .
  FILTER(?dateOfDeath >= "__START__"^^xsd:dateTime &&
         ?dateOfDeath <= "__END__"^^xsd:dateTime)
  ?article schema:about ?person ;
           schema:isPartOf <https://en.wikipedia.org/> .
  OPTIONAL {
    ?person p:P569/psv:P569 ?birthValue .
    ?birthValue wikibase:timeValue ?dateOfBirth ;
                wikibase:timePrecision 11 .
  }
  OPTIONAL { ?person wdt:P19 ?birthPlace . }
  OPTIONAL { ?person wdt:P20 ?deathPlace . }
  OPTIONAL { ?person wdt:P18 ?image . }
  SERVICE wikibase:label { bd:serviceParam wikibase:language "en". }
}
"""


def binding_value(row: dict[str, Any], key: str) -> str | None:
    binding = row.get(key)
    return binding.get("value") if binding else None


def parse_date(value: str | None) -> date | None:
    if not value:
        return None
    return datetime.fromisoformat(value.replace("Z", "+00:00")).date()


def build_query(now: datetime, occupations: tuple[str, ...] = OCCUPATIONS) -> str:
    now = now.astimezone(timezone.utc)
    start = (now - timedelta(days=7)).strftime("%Y-%m-%dT%H:%M:%SZ")
    end = now.strftime("%Y-%m-%dT%H:%M:%SZ")
    occupations = " ".join(f"wd:{item}" for item in occupations)
    return (
        QUERY.replace("__OCCUPATIONS__", occupations)
        .replace("__START__", start)
        .replace("__END__", end)
    )


def parse_candidates(bindings: list[dict[str, Any]], now: datetime) -> list[dict[str, Any]]:
    earliest = (now - timedelta(days=7)).date()
    latest = now.date()
    grouped: dict[str, dict[str, Any]] = {}
    for row in bindings:
        person_uri = binding_value(row, "person")
        death_date = parse_date(binding_value(row, "dateOfDeath"))
        name = binding_value(row, "personLabel")
        article = binding_value(row, "article")
        if not person_uri or not name or not death_date or not article:
            continue
        if not earliest <= death_date <= latest:
            continue
        qid = person_uri.rsplit("/", 1)[-1]
        if not re.fullmatch(r"Q[1-9]\d*", qid):
            continue
        if not article.startswith("https://en.wikipedia.org/wiki/"):
            continue

        candidate = grouped.setdefault(
            qid,
            {
                "wikidata_id": qid,
                "name": name,
                "birth_date": None,
                "death_date": death_date,
                "occupations": set(),
                "birth_place": None,
                "death_place": None,
                "wikipedia_url": article,
                "image_urls": [],
            },
        )
        birth_date = parse_date(binding_value(row, "dateOfBirth"))
        if birth_date and birth_date <= death_date:
            candidate["birth_date"] = birth_date
        occupation = binding_value(row, "occupationLabel")
        if occupation:
            candidate["occupations"].add(occupation)
        candidate["birth_place"] = candidate["birth_place"] or binding_value(
            row, "birthPlaceLabel"
        )
        candidate["death_place"] = candidate["death_place"] or binding_value(
            row, "deathPlaceLabel"
        )
        image = binding_value(row, "image")
        if image and image.startswith("http") and image not in candidate["image_urls"]:
            from urllib.parse import quote
            filename = image.rsplit("/", 1)[-1]
            candidate["image_urls"].append(
                "https://commons.wikimedia.org/wiki/Special:FilePath/"
                + quote(filename, safe="") + "?width=1200"
            )
            candidate["image_urls"] = candidate["image_urls"][:MAX_IMAGES]

    candidates = list(grouped.values())
    for candidate in candidates:
        candidate["occupation"] = ", ".join(sorted(candidate.pop("occupations")))
        for key in ("birth_date", "death_date"):
            if candidate[key]:
                candidate[key] = candidate[key].isoformat()
    return sorted(candidates, key=lambda item: (item["death_date"], item["name"]), reverse=True)


def fetch_candidates(now: datetime | None = None) -> list[dict[str, Any]]:
    now = now or datetime.now(timezone.utc)
    bindings: list[dict[str, Any]] = []
    for start in range(0, len(OCCUPATIONS), SPARQL_OCCUPATION_BATCH_SIZE):
        occupation_batch = OCCUPATIONS[start : start + SPARQL_OCCUPATION_BATCH_SIZE]
        payload = urlencode({"query": build_query(now, occupation_batch), "format": "json"}).encode()
        request = Request(
            ENDPOINT,
            data=payload,
            method="POST",
            headers={
                "Accept": "application/sparql-results+json",
                "Content-Type": "application/x-www-form-urlencoded",
                "User-Agent": USER_AGENT,
            },
        )
        last_error: Exception | None = None
        for attempt in range(SPARQL_ATTEMPTS):
            try:
                with urlopen(request, timeout=SPARQL_TIMEOUT_SECONDS) as response:
                    result = json.load(response)
                bindings.extend(result["results"]["bindings"])
                break
            except (HTTPError, TimeoutError, URLError) as error:
                last_error = error
                if isinstance(error, HTTPError):
                    details = error.read().decode("utf-8", errors="replace").strip()
                    last_error = RuntimeError(
                        f"HTTP {error.code} from Wikidata SPARQL endpoint"
                        + (f": {details[:500]}" if details else "")
                    )
                if attempt < SPARQL_ATTEMPTS - 1:
                    time.sleep(2**attempt)
        else:
            raise RuntimeError(
                f"Wikidata SPARQL request failed after {SPARQL_ATTEMPTS} attempts"
            ) from last_error
    return parse_candidates(bindings, now)


def submit_candidates(
    candidates: list[dict[str, Any]], api_url: str, token: str
) -> dict[str, Any]:
    queued = 0
    # Story drafts are generated with OpenAI inside the API request. Keep each
    # request small so the scheduled importer does not time out while processing
    # a large batch sequentially.
    batches = [candidates[index : index + 1] for index in range(0, len(candidates), 1)]
    for batch in batches or [[]]:
        payload = json.dumps({"candidates": batch}).encode("utf-8")
        request = Request(
            f"{api_url.rstrip('/')}/api/admin/recent-deaths",
            data=payload,
            method="POST",
            headers={
                "Authorization": f"Bearer {token}",
                "Content-Type": "application/json",
                "User-Agent": USER_AGENT,
            },
        )
        last_error: Exception | None = None
        for attempt in range(SUBMIT_ATTEMPTS):
            try:
                with urlopen(request, timeout=300) as response:
                    result = json.load(response)
                queued += result.get("queued", 0)
                break
            except (HTTPError, TimeoutError, URLError) as error:
                last_error = error
                if isinstance(error, HTTPError):
                    details = error.read().decode("utf-8", errors="replace").strip()
                    if "spend limit" in details.lower() or "insufficient_quota" in details.lower():
                        raise RuntimeError(
                            "OpenAI project spend limit reached; increase the project limit "
                            "before rerunning the recent-deaths importer. "
                            + (details[:500] if details else "")
                        ) from error
                    last_error = RuntimeError(
                        f"HTTP {error.code} from Mortalbook API"
                        + (f": {details[:500]}" if details else "")
                    )
                if attempt < SUBMIT_ATTEMPTS - 1:
                    time.sleep(2**attempt)
        else:
            raise RuntimeError(
                f"Mortalbook API request failed after {SUBMIT_ATTEMPTS} attempts"
            ) from last_error
    return {"queued": queued}


def main() -> None:
    token = os.environ.get("RECENT_DEATHS_INGEST_TOKEN", "")
    if not token:
        raise RuntimeError("RECENT_DEATHS_INGEST_TOKEN is required")
    api_url = os.environ.get("MORTALBOOK_API_URL", "https://mortalbook.com")
    candidates = fetch_candidates()
    result = submit_candidates(candidates, api_url, token)
    print(
        f"Sent {len(candidates)} candidate(s) to admin review; "
        f"{result.get('queued', 0)} newly queued."
    )


if __name__ == "__main__":
    main()
