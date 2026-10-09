from datetime import datetime, timezone
import unittest

from tools.generate_recent_deaths import build_query, parse_candidates, parse_date


def binding(value):
    return {"value": value}


def row(qid, name, death, occupation, article, birth=None):
    result = {
        "person": binding(f"http://www.wikidata.org/entity/{qid}"),
        "personLabel": binding(name),
        "dateOfDeath": binding(death),
        "occupationLabel": binding(occupation),
        "article": binding(article),
    }
    if birth:
        result["dateOfBirth"] = binding(birth)
    return result


class RecentDeathsGenerationTests(unittest.TestCase):
    def test_parse_date_handles_wikidata_timestamp(self):
        self.assertEqual(parse_date("2026-10-08T00:00:00Z").isoformat(), "2026-10-08")
        self.assertIsNone(parse_date(None))

    def test_query_uses_fixed_utc_window_and_exact_death_precision(self):
        query = build_query(datetime(2026, 10, 9, 16, tzinfo=timezone.utc))
        self.assertIn('"2026-10-02T16:00:00Z"^^xsd:dateTime', query)
        self.assertIn('"2026-10-09T16:00:00Z"^^xsd:dateTime', query)
        self.assertIn("wikibase:timePrecision 11", query)
        self.assertIn("wd:Q82955", query)

    def test_candidates_are_recent_deduplicated_and_carry_sources(self):
        now = datetime(2026, 10, 9, 16, tzinfo=timezone.utc)
        results = parse_candidates(
            [
                row(
                    "Q123",
                    "A Person",
                    "2026-10-08T00:00:00Z",
                    "Actor",
                    "https://en.wikipedia.org/wiki/A_Person",
                    "1950-01-02T00:00:00Z",
                ),
                row(
                    "Q123",
                    "A Person",
                    "2026-10-08T00:00:00Z",
                    "Singer",
                    "https://en.wikipedia.org/wiki/A_Person",
                ),
                row(
                    "Q456",
                    "Old Person",
                    "2026-10-01T00:00:00Z",
                    "Politician",
                    "https://en.wikipedia.org/wiki/Old_Person",
                ),
            ],
            now,
        )

        self.assertEqual(len(results), 1)
        self.assertEqual(results[0]["wikidata_id"], "Q123")
        self.assertEqual(results[0]["occupation"], "Actor, Singer")
        self.assertEqual(results[0]["birth_date"], "1950-01-02")
        self.assertEqual(
            results[0]["wikipedia_url"], "https://en.wikipedia.org/wiki/A_Person"
        )

    def test_candidates_without_english_wikipedia_are_excluded(self):
        now = datetime(2026, 10, 9, tzinfo=timezone.utc)
        results = parse_candidates(
            [row("Q123", "A Person", "2026-10-08T00:00:00Z", "Actor", "https://example.com")],
            now,
        )
        self.assertEqual(results, [])


if __name__ == "__main__":
    unittest.main()
