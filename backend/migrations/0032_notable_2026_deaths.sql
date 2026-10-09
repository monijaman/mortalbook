-- Notable public-figure deaths verified in reporting published by 9 October 2026.
-- This is a curated set, not an exhaustive list of all deaths during the year.
-- Dalia Nausheen is already in the catalogue and is therefore not repeated here.
INSERT INTO people (
    id, name, birth_date, death_date, bio, lang, photo_url, occupation,
    birth_place, death_place, birth_precision, death_precision
)
SELECT
    gen_random_uuid(), v.name, v.birth_date, v.death_date,
    v.bio || E'\n\nSource: Wikipedia — https://en.wikipedia.org/wiki/' || v.source_slug
        || E'\nDeath report: ' || v.death_report,
    'en', NULL, v.occupation, v.birth_place, v.death_place,
    v.birth_precision, 'day'
FROM (VALUES
    ('Asha Bhosle', DATE '1933-09-08', DATE '2026-04-12',
        'Asha Bhosle was an Indian playback singer whose career spanned more than eight decades. She recorded thousands of songs in many Indian languages, across film music, pop, ghazal and other styles, and received major Indian civilian and music honours. She died in Mumbai on 12 April 2026.',
        'Singer', 'Goar, Sangli district, Bombay Presidency', 'Mumbai, Maharashtra', 'day', 'Asha_Bhosle',
        'https://www.deccanherald.com/india/maharashtra/legendary-singer-asha-bhosle-passes-away-at-92-3965138'),
    ('Rusmir Mahmutćehajić', DATE '1948-06-29', DATE '2026-04-05',
        'Rusmir Mahmutćehajić was a Bosnian author, academic and politician. He served as deputy prime minister and minister of energy, mining and industry in Bosnia and Herzegovina and wrote on the country''s history, culture and civic life. He died in Sarajevo on 5 April 2026.',
        'Politician, Writer, Academic', 'Stolac, Bosnia and Herzegovina', 'Sarajevo, Bosnia and Herzegovina', 'day', 'Rusmir_Mahmut%C4%87ehaji%C4%87',
        'https://n1info.ba/english/news/former-bosnian-deputy-prime-minister-and-academic-rusmir-mahmutcehajic-passes-away-in-sarajevo/'),
    ('Mohsina Kidwai', DATE '1932-01-01', DATE '2026-04-08',
        'Mohsina Kidwai was an Indian politician and senior leader of the Indian National Congress. During a public career spanning decades, she served in Parliament and held Union government portfolios including health and family welfare, urban development and rural development. She died in Noida on 8 April 2026.',
        'Politician', 'Barabanki district, United Provinces', 'Noida, Uttar Pradesh', 'year', 'Mohsina_Kidwai',
        'https://www.thehindu.com/news/national/veteran-congress-leader-mohsina-kidwai-passes-away-at-94/article70837031.ece'),
    ('C. D. Gopinath', DATE '1930-03-01', DATE '2026-04-09',
        'C. D. Gopinath was an Indian cricketer who played as a right-handed batter and represented India in Test cricket. He was a member of India''s first Test-winning team, which defeated England in Madras in 1952, and later captained Madras. He died in Adyar, Chennai, on 9 April 2026.',
        'Cricketer', 'Madras, Madras Presidency', 'Adyar, Chennai, Tamil Nadu', 'day', 'C._D._Gopinath',
        'https://sportstar.thehindu.com/cricket/cd-gopinath-passes-away-adyar-india-first-ever-test-winning-team-rip-tribute/article70842372.ece'),
    ('Singeetham Srinivasa Rao', DATE '1931-09-21', DATE '2026-10-03',
        'Singeetham Srinivasa Rao was an Indian filmmaker, screenwriter, composer and actor whose work spanned Telugu, Kannada, Tamil, Hindi and other cinema. His films included Pushpaka Vimana, Aditya 369 and Bhairava Dweepam, and his career lasted more than seven decades. He died in Chennai on 3 October 2026.',
        'Film director, Screenwriter, Composer, Actor', 'Udayagiri, Madras Presidency', 'Chennai, Tamil Nadu', 'day', 'Singeetham_Srinivasa_Rao',
        'https://www.newindianexpress.com/states/tamil-nadu/2026/Oct/04/singeetham-srinivasa-rao-dies-at-95')
) AS v(name, birth_date, death_date, bio, occupation, birth_place, death_place, birth_precision, source_slug, death_report)
WHERE NOT EXISTS (
    SELECT 1
    FROM people p
    WHERE p.name = v.name AND p.death_date = v.death_date
);
