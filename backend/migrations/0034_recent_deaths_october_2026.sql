-- Notable public figures reported to have died during 3–9 October 2026.
-- This is a curated selection, not an exhaustive list; existing name/date pairs are skipped.
INSERT INTO people (
    id, name, birth_date, death_date, bio, lang, photo_url, occupation,
    birth_place, death_place, birth_precision, death_precision
)
SELECT
    gen_random_uuid(), v.name, v.birth_date, v.death_date,
    v.bio || E'\n\nSource: Wikipedia - https://en.wikipedia.org/wiki/' || v.source_slug
        || CASE WHEN v.death_report IS NULL THEN ''
                ELSE E'\nDeath report: ' || v.death_report END,
    'en', NULL, v.occupation, v.birth_place, v.death_place,
    'day', 'day'
FROM (VALUES
    ('Nana Patekar', DATE '1951-01-01', DATE '2026-10-08',
        'Nana Patekar was an Indian actor and filmmaker known for acclaimed work in Hindi and Marathi cinema, including Parinda, Krantiveer and Natsamrat. He received three National Film Awards and the Padma Shri. He died in Goa on 8 October 2026.',
        'Actor, Filmmaker', 'Murud-Janjira, Maharashtra, India', 'Dona Paula, Goa, India', 'Nana_Patekar',
        'https://www.aljazeera.com/news/2026/10/8/acclaimed-indian-actor-nana-patekar-dies-aged-75'),
    ('Freddie Jackson', DATE '1956-10-02', DATE '2026-10-05',
        'Freddie Jackson was an American R&B singer and songwriter whose recordings helped define 1980s contemporary soul. His best-known songs include Rock Me Tonight, You Are My Lady and Jam Tonight. He died in October 2026, aged 70.',
        'Singer, Songwriter', 'Harlem, New York City, United States', 'New York City, United States', 'Freddie_Jackson',
        'https://www.nytimes.com/2026/10/06/arts/music/freddie-jackson-dead.html'),
    ('Michael Kearns', DATE '1950-01-08', DATE '2026-10-06',
        'Michael Kearns was an American actor, playwright, director and activist. He was one of the first openly gay actors in Hollywood and co-founded Artists Confronting AIDS. He died in Los Angeles on 6 October 2026, aged 76.',
        'Actor, Writer, Director, Activist', 'St. Louis, Missouri, United States', NULL, 'Michael_Kearns_(actor)',
        NULL),
    ('Robert Kelker-Kelly', DATE '1964-04-18', DATE '2026-10-03',
        'Robert Kelker-Kelly was an American actor and professional pilot, known for playing Bo Brady on Days of Our Lives and for roles on Another World and General Hospital. He died at his home in St. Peters, Missouri, on 3 October 2026, aged 62.',
        'Actor, Pilot', 'Wichita, Kansas, United States', 'St. Peters, Missouri, United States', 'Robert_Kelker-Kelly',
        'https://deadline.com/2026/10/robert-kelker-kelly-dead-days-of-our-lives-another-world-1237146925/'),
    ('Jeffrey Archer', DATE '1940-04-15', DATE '2026-10-05',
        'Jeffrey Archer was an English novelist and politician. His bestselling novels included Kane and Abel, and he served as a Member of Parliament before later sitting in the House of Lords. He died in London on 5 October 2026, aged 86.',
        'Novelist, Politician', 'London, England', 'London, England', 'Jeffrey_Archer',
        NULL),
    ('Dennis Hastert', DATE '1942-01-02', DATE '2026-10-03',
        'John Dennis Hastert was an American politician and educator who served as Speaker of the U.S. House of Representatives from 1999 to 2007. In 2016, he was sentenced to prison for financial offenses related to payments made to conceal his sexual abuse of teenage boys, which he admitted in court. He died in Plano, Illinois, on 3 October 2026, aged 84.',
        'Politician, Educator', 'Aurora, Illinois, United States', 'Plano, Illinois, United States', 'Dennis_Hastert',
        'https://www.reuters.com/world/us/former-us-house-speaker-dennis-hastert-dies-84-2026-10-05/')
) AS v(name, birth_date, death_date, bio, occupation, birth_place, death_place, source_slug, death_report)
WHERE NOT EXISTS (
    SELECT 1
    FROM people p
    WHERE p.name = v.name AND p.death_date = v.death_date
);
