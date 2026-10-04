CREATE TABLE people (
    id          UUID PRIMARY KEY,
    name        TEXT NOT NULL,
    birth_date  DATE,
    death_date  DATE NOT NULL,
    bio         TEXT NOT NULL DEFAULT '',
    lang        TEXT,                 -- detected language of `bio`
    photo_url   TEXT,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- "who died on this day" lookups
CREATE INDEX people_death_md_idx ON people (
    (EXTRACT(MONTH FROM death_date)), (EXTRACT(DAY FROM death_date))
);
CREATE INDEX people_created_idx ON people (created_at DESC);

CREATE TABLE media (
    id         UUID PRIMARY KEY,
    person_id  UUID NOT NULL REFERENCES people(id) ON DELETE CASCADE,
    kind       TEXT NOT NULL CHECK (kind IN ('image', 'video')),
    url        TEXT NOT NULL,         -- /uploads/... or external video link
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX media_person_idx ON media (person_id);

-- cache so each string is machine-translated once per target language
CREATE TABLE translations (
    src_hash TEXT NOT NULL,
    target   TEXT NOT NULL,
    output   TEXT NOT NULL,
    PRIMARY KEY (src_hash, target)
);
