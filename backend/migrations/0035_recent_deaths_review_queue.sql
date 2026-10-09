CREATE TABLE pending_people (
    id UUID PRIMARY KEY,
    wikidata_id TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL CHECK (length(name) BETWEEN 1 AND 200),
    birth_date DATE,
    death_date DATE NOT NULL,
    bio TEXT NOT NULL DEFAULT '',
    occupation TEXT,
    birth_place TEXT,
    death_place TEXT,
    wikidata_url TEXT NOT NULL,
    wikipedia_url TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending'
        CHECK (status IN ('pending', 'approved', 'rejected')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    reviewed_at TIMESTAMPTZ,
    CHECK (birth_date IS NULL OR birth_date <= death_date)
);

CREATE INDEX pending_people_review_idx
    ON pending_people (created_at DESC)
    WHERE status = 'pending';
