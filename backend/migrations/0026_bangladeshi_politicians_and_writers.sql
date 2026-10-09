-- Notable Bangladeshi politicians, writers and public figures who have passed away.
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
    ('Abul Hasnat Muhammad Qamaruzzaman', DATE '1926-06-26', DATE '1975-11-03',
        'Abul Hasnat Muhammad Qamaruzzaman was a Bangladeshi politician and one of the leading organisers of the Awami League. He served in the provisional government formed during the Bangladesh Liberation War and held senior cabinet positions after independence. He was killed in Dhaka Central Jail on 3 November 1975, alongside three other national leaders.',
        'Politician, Freedom fighter', 'Rajshahi, Bengal Presidency', 'Dhaka, Bangladesh', 'day', 'A._H._M._Qamaruzzaman'),
    ('Justice Shahabuddin Ahmed', DATE '1930-02-01', DATE '2022-03-19',
        'Shahabuddin Ahmed was a Bangladeshi jurist and statesman who served as president of Bangladesh and later as chief justice. As chief adviser of the caretaker government, he oversaw the transition to parliamentary democracy after the 1990 mass uprising. He was widely known for his judicial career and role in the country''s democratic transition.',
        'Politician, Jurist', 'Paban, Bengal Presidency', 'Dhaka, Bangladesh', 'day', 'Shahabuddin_Ahmed'),
    ('Abdur Rashid Tarkabagish', DATE '1900-11-27', DATE '1986-08-20',
        'Abdur Rashid Tarkabagish was a Bangladeshi politician, Islamic scholar and language-movement organiser. He served as president of the East Pakistan Awami Muslim League and later as a member of parliament in independent Bangladesh. He was an advocate for Bengali language rights and political representation in East Bengal.',
        'Politician, Islamic scholar', 'Tarkabagish, Bengal Presidency', 'Dhaka, Bangladesh', 'day', 'Abdur_Rashid_Tarkabagish'),
    ('Mohammad Toaha', DATE '1922-01-01', DATE '1987-11-29',
        'Mohammad Toaha was a Bangladeshi communist politician and language-movement activist. He participated in the 1952 Bengali Language Movement and was involved in left-wing politics in East Pakistan and Bangladesh. He served as a member of parliament after independence.',
        'Politician, Language activist', 'Lakshmipur, Bengal Presidency', 'Dhaka, Bangladesh', 'year', 'Mohammad_Toaha'),
    ('Pankaj Bhattacharya', DATE '1939-08-06', DATE '2023-04-23',
        'Pankaj Bhattacharya was a Bangladeshi left-wing politician and organiser. He was a founding member of the National Awami Party and later led the United Communist League of Bangladesh. Across decades of political work, he advocated secularism, democracy and social justice.',
        'Politician', 'Rangamati, Bengal Presidency', 'Dhaka, Bangladesh', 'day', 'Pankaj_Bhattacharya'),
    ('Abdul Matin', DATE '1924-12-03', DATE '2014-10-08',
        'Abdul Matin was a Bangladeshi language activist known as Bhasha Matin. He was a student leader during the Bengali Language Movement and helped organise protests demanding recognition of Bangla as a state language. He later remained active in left-wing politics and public life.',
        'Language activist, Politician', 'Chouhali, Bengal Presidency', 'Dhaka, Bangladesh', 'day', 'Abdul_Matin_(language_activist)'),
    ('Haji Mohammad Danesh', DATE '1900-06-16', DATE '1986-06-28',
        'Haji Mohammad Danesh was a Bangladeshi politician and peasant leader. He helped organise the Tebhaga movement in northern Bengal, which called for a larger share of harvested crops for tenant farmers. He later took part in left-wing politics and served in public office in independent Bangladesh.',
        'Politician, Peasant leader', 'Sultanpur, Bengal Presidency', 'Dhaka, Bangladesh', 'year', 'Haji_Mohammad_Danesh'),
    ('Abul Gaffar Choudhury', DATE '1934-12-12', DATE '2022-05-19',
        'Abdul Gaffar Choudhury was a Bangladeshi-British journalist, columnist, writer and lyricist. He wrote the lyrics of “Amar Bhaier Rokte Rangano”, the song associated with the Bengali Language Movement and International Mother Language Day. He worked for newspapers in Bangladesh and, after settling in the United Kingdom, continued writing about Bangladeshi politics and culture.',
        'Writer, Journalist, Lyricist', 'Ulania, Bengal Presidency', 'London, England', 'day', 'Abdul_Gaffar_Choudhury'),
    ('Syed Shamsul Haq', DATE '1935-12-27', DATE '2016-09-27',
        'Syed Shamsul Haq was a Bangladeshi poet, novelist, playwright and translator who wrote in Bengali. His work ranged across poetry, fiction, drama and essays, and he received the Bangla Academy Literary Award and the Ekushey Padak. He was often described as a versatile or “sabyasachi” writer.',
        'Writer, Poet, Playwright', 'Kurigram, Bengal Presidency', 'Dhaka, Bangladesh', 'day', 'Syed_Shamsul_Haq'),
    ('Sayeed Ahmad', DATE '1931-01-01', DATE '2010-01-21',
        'Sayeed Ahmad was a Bangladeshi playwright, writer and cultural organiser. He wrote plays in Bengali and English and contributed to the development of modern theatre in Bangladesh. His dramatic works were staged in Bangladesh and abroad.',
        'Writer, Playwright', 'Bengal Presidency', 'Dhaka, Bangladesh', 'year', 'Sayeed_Ahmad_(playwright)'),
    ('Abul Mansur Ahmed', DATE '1898-09-03', DATE '1979-03-18',
        'Abul Mansur Ahmed was a Bangladeshi writer, journalist and politician. His satirical writing, including the collection Ayna, made him an important figure in Bengali literature, while his political career included service as a minister in Pakistan. He supported Bengali language rights and the political autonomy of East Bengal.',
        'Writer, Journalist, Politician', 'Dhanikhola, Bengal Presidency', 'Dhaka, Bangladesh', 'day', 'Abul_Mansur_Ahmed'),
    ('Muhammad Enamul Haque', DATE '1902-05-20', DATE '1982-02-16',
        'Muhammad Enamul Haque was a Bangladeshi linguist, writer and academic whose research focused on Bengali language, literature and Sufism. He taught at universities and held leadership roles at the Bangla Academy. His scholarship contributed to the study and documentation of Bengali literary history.',
        'Writer, Linguist, Academic', 'Feni, Bengal Presidency', 'Dhaka, Bangladesh', 'day', 'Muhammad_Enamul_Haque'),
    ('Ghulam Azam', DATE '1922-11-07', DATE '2014-10-23',
        'Ghulam Azam was a Bangladeshi politician who led Jamaat-e-Islami Bangladesh for many years. He opposed the independence of Bangladesh in 1971 and remained a deeply controversial figure in the country''s politics. In 2013, the International Crimes Tribunal sentenced him for crimes against humanity committed during the Liberation War; he died in hospital while serving his sentence.',
        'Politician', 'Birgaon, Bengal Presidency', 'Dhaka, Bangladesh', 'day', 'Ghulam_Azam'),
    ('Salahuddin Quader Chowdhury', DATE '1949-03-13', DATE '2015-11-22',
        'Salahuddin Quader Chowdhury was a Bangladeshi politician who served multiple terms as a member of parliament and held senior roles in the Bangladesh Nationalist Party. In 2013, the International Crimes Tribunal convicted him of crimes against humanity during the 1971 Liberation War; the conviction was upheld on appeal and he was executed in 2015.',
        'Politician', 'Raozan, East Bengal', 'Dhaka, Bangladesh', 'day', 'Salahuddin_Quader_Chowdhury'),
    ('Mohammad Farhad', DATE '1938-07-05', DATE '1987-10-09',
        'Mohammad Farhad was a Bangladeshi communist politician and organiser who led the Communist Party of Bangladesh. He took part in the Language Movement and the Liberation War and served as a member of parliament after independence. He was known for his work in labour and student movements.',
        'Politician, Organiser', 'Panchagarh, Bengal Presidency', 'Dhaka, Bangladesh', 'day', 'Mohammad_Farhad'),
    ('A. K. M. Shamsul Haque', DATE '1920-01-01', DATE '1971-12-14',
        'A. K. M. Shamsul Haque was a Bangladeshi educationist and intellectual who was abducted and killed during the 1971 Bangladesh Liberation War. He served as a professor and took part in public and cultural life in Dhaka. He is remembered among the Bengali intellectuals killed near the end of the war.',
        'Academic, Writer', 'Bengal Presidency', 'Dhaka, Bangladesh', 'year', 'A._K._M._Shamsul_Haque_(academic)')
) AS v(name, birth_date, death_date, bio, occupation, birth_place, death_place, birth_precision, source_slug)
WHERE NOT EXISTS (
    SELECT 1
    FROM people p
    WHERE p.name = v.name AND p.death_date = v.death_date
);
