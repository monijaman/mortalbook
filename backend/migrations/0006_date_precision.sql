-- How precise each date is. Dates known only to the month/year/decade/century are stored as the
-- first day of that period (BC dates as e.g. DATE '0044-03-15 BC'), and only 'day' deaths
-- show up in the "died on this day" views.
ALTER TABLE people
    ADD COLUMN birth_precision TEXT NOT NULL DEFAULT 'day'
        CHECK (birth_precision IN ('day', 'month', 'year', 'decade', 'century')),
    ADD COLUMN death_precision TEXT NOT NULL DEFAULT 'day'
        CHECK (death_precision IN ('day', 'month', 'year', 'decade', 'century'));
