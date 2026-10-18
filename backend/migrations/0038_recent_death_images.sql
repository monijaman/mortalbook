ALTER TABLE pending_people
    ADD COLUMN image_urls TEXT[] NOT NULL DEFAULT '{}';
