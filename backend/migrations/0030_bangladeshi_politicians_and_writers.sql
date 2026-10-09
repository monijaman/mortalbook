-- More notable Bangladeshi politicians and writers who have passed away.
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
        'Abdur Rab Serniabat was a Bangladeshi politician and leader of the Awami League. He served as minister of agriculture after Bangladesh became independent and was active in public life in Barisal. He was killed with members of his family during the military coup in Dhaka on 15 August 1975.',
        'Politician, Freedom fighter', 'Barisal, Bengal Presidency', 'Dhaka, Bangladesh', 'day', 'Abdur_Rab_Serniabat'),
    ('Shamsul Alam Khan Milon', DATE '1957-08-21', DATE '1990-11-27',
        'Shamsul Alam Khan Milon was a Bangladeshi physician and political activist. While studying medicine, he became involved in the movement against the military rule of Hussain Muhammad Ershad. He was shot and killed in Dhaka during the 1990 pro-democracy uprising; his death became a rallying point for the movement.',
        'Physician, Political activist', 'Dhaka, East Pakistan', 'Dhaka, Bangladesh', 'day', 'Shamsul_Alam_Khan_Milon'),
    ('Shah Moazzem Hossain', DATE '1939-01-10', DATE '2022-09-14',
        'Shah Moazzem Hossain was a Bangladeshi politician who served as deputy prime minister and held several senior posts in national politics. He was active in the Awami League during the independence period and later joined the Bangladesh Nationalist Party and the Jatiya Party. He also served as a member of parliament.',
        'Politician', 'Louhajang, Bengal Presidency', 'Dhaka, Bangladesh', 'day', 'Shah_Moazzem_Hossain'),
    ('M. K. Anwar', DATE '1933-01-01', DATE '2017-10-24',
        'M. K. Anwar was a Bangladeshi politician and senior civil servant. After a career in public administration, he served as a member of parliament and as a cabinet minister in Bangladesh Nationalist Party governments. His ministerial roles included agriculture and shipping.',
        'Politician, Civil servant', 'Comilla, Bengal Presidency', 'Dhaka, Bangladesh', 'year', 'M._K._Anwar'),
    ('Oli Ahad', DATE '1928-06-02', DATE '2012-10-20',
        'Oli Ahad was a Bangladeshi politician and language activist who took part in the Bengali Language Movement. He was a founding organiser of the Awami Muslim League and later founded the Democratic League. His political memoirs document the language movement and the struggle for autonomy in East Bengal.',
        'Politician, Language activist, Writer', 'Brahmanbaria, Bengal Presidency', 'Dhaka, Bangladesh', 'day', 'Oli_Ahad'),
    ('Hannan Shah', DATE '1941-10-11', DATE '2016-09-27',
        'Hannan Shah was a Bangladeshi politician and retired army officer who served as a member of parliament and a minister. A senior leader of the Bangladesh Nationalist Party, he was active in opposition politics and held leadership roles in the party. He died while receiving treatment in Singapore.',
        'Politician, Military officer', 'Kapasia, Bengal Presidency', 'Singapore', 'day', 'Hannan_Shah'),
    ('Abdul Jalil', DATE '1939-01-21', DATE '2013-03-06',
        'Abdul Jalil was a Bangladeshi politician and businessman who served as a member of parliament and as minister of commerce. He held senior positions in the Awami League, including general secretary, and took part in Bangladesh''s national political life for several decades.',
        'Politician, Businessperson', 'Naogaon, Bengal Presidency', 'Singapore', 'day', 'Abdul_Jalil_(Bangladeshi_politician)'),
    ('Qazi Anwar Hossain', DATE '1936-07-19', DATE '2022-01-19',
        'Qazi Anwar Hossain was a Bangladeshi writer, translator and publisher, best known for creating the popular adventure series Masud Rana. He founded Sheba Prokashoni, a publishing house that translated and adapted international genre fiction for Bengali readers. His books introduced generations of readers to detective, spy and adventure stories.',
        'Writer, Translator, Publisher', 'Dhaka, Bengal Presidency', 'Dhaka, Bangladesh', 'day', 'Qazi_Anwar_Hossain'),
    ('Abul Hossain', DATE '1922-08-15', DATE '2014-06-29',
        'Abul Hossain was a Bangladeshi poet whose modernist Bengali poetry helped shape literature in the region. His collections include Nabanna and Dushswapna. He received the Bangla Academy Literary Award and the Ekushey Padak for his contribution to Bengali literature.',
        'Poet, Writer', 'Khulna, Bengal Presidency', 'Dhaka, Bangladesh', 'day', 'Abul_Hossain_(poet)'),
    ('Foyez Ahmed', DATE '1928-05-02', DATE '2012-02-20',
        'Foyez Ahmed was a Bangladeshi journalist, writer and television personality. He worked for newspapers and broadcasting organisations and became known for commentary and programmes on public affairs and culture. He received the Ekushey Padak in recognition of his contribution to journalism.',
        'Journalist, Writer, Television presenter', 'Munshiganj, Bengal Presidency', 'Dhaka, Bangladesh', 'day', 'Foyez_Ahmed')
) AS v(name, birth_date, death_date, bio, occupation, birth_place, death_place, birth_precision, source_slug)
WHERE NOT EXISTS (
    SELECT 1
    FROM people p
    WHERE p.name = v.name AND p.death_date = v.death_date
);
