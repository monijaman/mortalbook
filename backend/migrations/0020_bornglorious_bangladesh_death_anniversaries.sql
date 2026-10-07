-- Imported from BornGlorious' Bangladesh death-anniversary page.
-- Source attribution: BornGlorious.com, Freebase (CC BY), and Wikipedia (CC BY-SA).
-- Only add names that are not already present in the catalogue.
DO $migration$
BEGIN
  INSERT INTO people (id, name, birth_date, death_date, bio, lang, photo_url, occupation, birth_place, death_place)
  SELECT gen_random_uuid(), v.name, NULL, v.death_date, v.bio, 'en', NULL, v.occupation, v.birth_place, NULL
  FROM (VALUES
    ('Jagadish Chandra Bose', DATE '1937-11-23', 'Physicist', 'Bikrampur'),
    ('Iskander Mirza', DATE '1969-11-12', 'Politician', 'Murshidabad'),
    ('Maulana Bhashani', DATE '1976-11-17', 'Politician', 'Tangail District'),
    ('Titumir', DATE '1831-11-19', 'Revolutionary', 'North 24 Parganas district'),
    ('Sufia Kamal', DATE '1999-11-20', 'Poet', 'Barisal District'),
    ('Tajuddin Ahmad', DATE '1975-11-03', 'Politician', 'Gazipur District'),
    ('Syed Nazrul Islam', DATE '1975-11-03', 'Politician', 'Kishoreganj District'),
    ('Sanjeeb Choudhury', DATE '2007-11-19', 'Journalist', NULL),
    ('Kalim Sharafi', DATE '2010-11-02', 'Singer', NULL),
    ('Ashwini Kumar Dutta', DATE '1923-11-07', NULL, 'Patuakhali District'),
    ('A. H. M. Qamaruzzaman', DATE '1975-11-03', 'Politician', NULL),
    ('Max Robertson', DATE '2009-11-20', 'Actor', 'Dhaka'),
    ('Khaled Mosharraf', DATE '1975-11-07', NULL, 'Jamalpur District'),
    ('Manabendra Narayan Larma', DATE '1983-11-10', NULL, NULL),
    ('Nur Hossain', DATE '1987-11-10', NULL, 'Dhaka'),
    ('Tibbetibaba', DATE '1930-11-20', NULL, 'Sylhet'),
    ('Haraprasad Shastri', DATE '1931-11-17', NULL, 'Khulna'),
    ('Mohammad Ullah', DATE '1999-11-11', NULL, 'Raipur Upazila'),
    ('Cecil Kershaw', DATE '1972-11-01', NULL, 'Dhaka'),
    ('Gahanananda', DATE '2007-11-04', NULL, NULL)
  ) AS v(name, death_date, occupation, birth_place)
  CROSS JOIN LATERAL (SELECT 'Imported from BornGlorious death-anniversary data. Source: BornGlorious.com; Freebase (CC BY); Wikipedia (CC BY-SA).'::text AS bio)
  WHERE NOT EXISTS (SELECT 1 FROM people p WHERE lower(p.name) = lower(v.name));
END
$migration$;
