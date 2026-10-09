-- Additional notable Bangladeshi politicians, writers and activists who have passed away.
-- Biographies are concise summaries; source links point to English Wikipedia.
-- Existing name/date pairs are skipped so earlier imports are not duplicated.
INSERT INTO people (
    id, name, birth_date, death_date, bio, lang, photo_url, occupation,
    birth_place, death_place, birth_precision, death_precision
)
SELECT
    gen_random_uuid(), v.name, v.birth_date, v.death_date,
    v.bio || E'\n\nSource: Wikipedia — https://en.wikipedia.org/wiki/' || v.source_slug,
    'en', NULL, v.occupation, v.birth_place, v.death_place,
    v.birth_precision, 'day'
FROM (VALUES
    ('Abdur Rab Serniabat', DATE '1921-03-01', DATE '1975-08-15',
        'Abdur Rab Serniabat was a Bangladeshi politician and a leader of the Awami League. He served as minister of agriculture in independent Bangladesh and represented Barisal in national politics. He was killed with members of his family during the military coup in Dhaka on 15 August 1975.',
        'Politician', 'Barisal, Bengal Presidency', 'Dhaka, Bangladesh', 'day', 'Abdur_Rab_Serniabat'),
    ('Shamsul Alam Khan Milon', DATE '1957-08-21', DATE '1990-11-27',
        'Shamsul Alam Khan Milon was a Bangladeshi physician and political activist. While studying medicine, he became involved in the movement against the military rule of Hussain Muhammad Ershad. He was shot and killed in Dhaka during the 1990 pro-democracy uprising; his death became a rallying point for the movement.',
        'Physician, Political activist', 'Dhaka, East Pakistan', 'Dhaka, Bangladesh', 'day', 'Shamsul_Alam_Khan_Milon'),
    ('Qazi Anwar Hossain', DATE '1936-07-19', DATE '2022-01-19',
        'Qazi Anwar Hossain was a Bangladeshi writer, translator and publisher, best known for creating the popular adventure series Masud Rana. He founded Sheba Prokashoni, a publishing house that translated and adapted international genre fiction for Bengali readers. His books introduced generations of readers to detective, spy and adventure stories.',
        'Writer, Translator, Publisher', 'Dhaka, Bengal Presidency', 'Dhaka, Bangladesh', 'day', 'Qazi_Anwar_Hossain'),
    ('Abul Hossain', DATE '1922-08-15', DATE '2014-06-29',
        'Abul Hossain was a Bangladeshi poet whose modernist Bengali poetry helped shape literature in the region. His collections include Nabanna and Dushswapna. He received the Bangla Academy Literary Award and the Ekushey Padak for his contribution to Bengali literature.',
        'Poet, Writer', 'Khulna, Bengal Presidency', 'Dhaka, Bangladesh', 'day', 'Abul_Hossain_(poet)'),
    ('Foyez Ahmed', DATE '1928-05-02', DATE '2012-02-20',
        'Foyez Ahmed was a Bangladeshi journalist, writer and television personality. He worked for newspapers and broadcasting organisations and became known for commentary and programmes on public affairs and culture. He received the Ekushey Padak in recognition of his contribution to journalism.',
        'Journalist, Writer, Television presenter', 'Munshiganj, Bengal Presidency', 'Dhaka, Bangladesh', 'day', 'Foyez_Ahmed'),
    ('Begum Mushtari Shafi', DATE '1938-01-01', DATE '2021-12-20',
        'Begum Mushtari Shafi was a Bangladeshi writer, publisher and women''s rights activist. During the 1971 Liberation War, her home in Chittagong was used to support members of the resistance; her husband and brother were killed by the Pakistani army. She later wrote about the war and received the Begum Rokeya Padak for her social and cultural work.',
        'Writer, Publisher, Social activist', 'Chittagong, Bengal Presidency', 'Chittagong, Bangladesh', 'year', 'Begum_Mushtari_Shafi')
) AS v(name, birth_date, death_date, bio, occupation, birth_place, death_place, birth_precision, source_slug)
WHERE NOT EXISTS (
    SELECT 1
    FROM people p
    WHERE p.name = v.name AND p.death_date = v.death_date
);
