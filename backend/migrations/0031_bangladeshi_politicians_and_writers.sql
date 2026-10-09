-- Additional notable Bangladeshi politicians, writers and public figures who have passed away.
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
    ('Abul Hashim', DATE '1905-01-27', DATE '1974-10-05',
        'Abul Hashim was a Bengali politician and writer who served as general secretary of the Bengal Provincial Muslim League in the 1940s. He advocated a more inclusive and socially reformist political programme and took part in debates over Bengal''s future during the final years of British rule. His writings addressed politics, society and religion.',
        'Politician, Writer', 'Kashiara, Bengal Presidency', 'Calcutta, West Bengal', 'day', 'Abul_Hashim'),
    ('Syed Mahbub Murshed', DATE '1911-01-11', DATE '1979-04-03',
        'Syed Mahbub Murshed was a Bangladeshi jurist, judge and intellectual who served as chief justice of the Dhaka High Court. He became known for defending constitutional rights and opposing authoritarian rule in East Pakistan. His public essays and speeches also addressed democracy, Bengali identity and the rule of law.',
        'Jurist, Judge, Writer', 'Dhaka, Bengal Presidency', 'Dhaka, Bangladesh', 'day', 'Syed_Mahbub_Murshed'),
    ('Kamruddin Ahmad', DATE '1913-09-01', DATE '1983-02-06',
        'Kamruddin Ahmad was a Bangladeshi politician, writer and historian who took part in the Language Movement and the struggle for political rights in East Bengal. He wrote accounts of the political history of Bengal and the emergence of Bangladesh, drawing on his experience in public life. He was also a lawyer and member of the Pakistan Constituent Assembly.',
        'Politician, Writer, Historian, Lawyer', 'Dhaka, Bengal Presidency', 'Dhaka, Bangladesh', 'day', 'Kamruddin_Ahmad'),
    ('Shahjahan Siraj', DATE '1943-03-01', DATE '2020-07-14',
        'Shahjahan Siraj was a Bangladeshi politician and student leader who took part in the movement for independence. He was one of the leaders of the Sarbadaliya Chhatra Sangram Parishad during the 1969 uprising and later helped found the Jatiya Samajtantrik Dal. He served as a member of parliament and as minister of environment and forests.',
        'Politician, Student leader', 'Tangail, Bengal Presidency', 'Dhaka, Bangladesh', 'day', 'Shahjahan_Siraj'),
    ('Nurul Islam Manzur', DATE '1939-03-01', DATE '2020-05-26',
        'Nurul Islam Manzur was a Bangladeshi politician who served as a member of parliament and minister of communications. He was active in the Jatiya Party and represented constituencies in Barisal. His political career included service in national government and party leadership.',
        'Politician', 'Barisal, Bengal Presidency', 'Dhaka, Bangladesh', 'day', 'Nurul_Islam_Manzur'),
    ('Abdul Wahab Khan', DATE '1905-01-01', DATE '1987-04-14',
        'Abdul Wahab Khan was a politician from East Bengal who served as speaker of Pakistan''s National Assembly. A lawyer by profession, he held senior legislative office during the period when Bangladesh was East Pakistan and took part in the region''s public and political life.',
        'Politician, Lawyer', 'Bengal Presidency', 'Dhaka, Bangladesh', 'year', 'Abdul_Wahab_Khan')
) AS v(name, birth_date, death_date, bio, occupation, birth_place, death_place, birth_precision, source_slug)
WHERE NOT EXISTS (
    SELECT 1
    FROM people p
    WHERE p.name = v.name AND p.death_date = v.death_date
);
