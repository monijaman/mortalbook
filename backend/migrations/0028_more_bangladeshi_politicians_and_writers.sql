-- Additional notable Bangladeshi politicians and writers who have passed away.
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
    ('Suranjit Sengupta', DATE '1945-05-05', DATE '2017-02-05',
        'Suranjit Sengupta was a Bangladeshi politician and lawyer who served for many years as a member of parliament. He was a senior leader of the Awami League and held the position of minister of railways. He was also known for his speeches and participation in parliamentary debates.',
        'Politician, Lawyer', 'Derai, Bengal Presidency', 'Dhaka, Bangladesh', 'day', 'Suranjit_Sengupta'),
    ('Syed Ashraful Islam', DATE '1952-11-01', DATE '2019-01-03',
        'Syed Ashraful Islam was a Bangladeshi politician who served as a member of parliament and as a minister in the government of Bangladesh. A senior leader of the Awami League, he served as its general secretary and held cabinet portfolios including local government and public administration. His father, Syed Nazrul Islam, was a leader of the wartime provisional government.',
        'Politician', 'Mymensingh, East Bengal', 'Bangkok, Thailand', 'day', 'Syed_Ashraful_Islam'),
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
    ('Alauddin Al Azad', DATE '1932-05-06', DATE '2009-07-03',
        'Alauddin Al Azad was a Bangladeshi writer, novelist, poet and academic. His work included novels, short stories, poetry and plays, and he wrote about social change and the experiences of Bengali people. He received the Bangla Academy Literary Award and the Ekushey Padak.',
        'Writer, Novelist, Poet, Academic', 'Raipura, Bengal Presidency', 'Dhaka, Bangladesh', 'day', 'Alauddin_Al_Azad'),
    ('Mahbubul Alam Choudhury', DATE '1927-11-07', DATE '2007-12-23',
        'Mahbubul Alam Choudhury was a Bangladeshi poet and language activist. His poem Kandte Ashini, Phanshir Dabi Niye Eshechi, written in 1952, is associated with the Bengali Language Movement. He received the Ekushey Padak for his contribution to Bengali literature.',
        'Poet, Language activist', 'Gahira, Bengal Presidency', 'Chittagong, Bangladesh', 'day', 'Mahbubul_Alam_Choudhury'),
    ('Mohammad Abdul Hai', DATE '1919-07-26', DATE '1969-06-11',
        'Mohammad Abdul Hai was a Bangladeshi linguist, academic and writer who made important contributions to the study of Bengali phonetics and linguistics. He taught at the University of Dhaka and wrote influential works on the Bengali language. His scholarship helped establish modern linguistics as an academic discipline in Bangladesh.',
        'Linguist, Academic, Writer', 'Murshidabad, Bengal Presidency', 'Dhaka, East Pakistan', 'day', 'Mohammad_Abdul_Hai')
) AS v(name, birth_date, death_date, bio, occupation, birth_place, death_place, birth_precision, source_slug)
WHERE NOT EXISTS (
    SELECT 1
    FROM people p
    WHERE p.name = v.name AND p.death_date = v.death_date
);
