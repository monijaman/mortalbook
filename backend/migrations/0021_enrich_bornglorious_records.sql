-- Verified enrichment for records imported from BornGlorious.
-- Biographical facts are summarized from the linked Wikipedia/Wikidata records.
UPDATE people
SET birth_date = DATE '1858-11-30',
    birth_place = 'Mymensingh, Bengal Presidency (now Bangladesh)',
    death_place = 'Giridih, Bihar Province (now India)',
    occupation = 'Physicist, biologist, botanist',
    bio = 'Sir Jagadish Chandra Bose was a Bengali physicist, biologist and botanist. He pioneered research in radio and microwave optics, made major contributions to plant physiology, and founded the Bose Institute in Kolkata.'
WHERE lower(name) = lower('Jagadish Chandra Bose');

UPDATE people
SET birth_date = DATE '1899-11-13',
    birth_place = 'Murshidabad, Bengal, British India (now India)',
    death_place = 'London, England',
    occupation = 'Politician, military general',
    bio = 'Iskander Mirza was a Pakistani politician and military general who served as the last Governor-General of Pakistan and the first President of Pakistan. He was overthrown in 1958 and lived in exile in London.'
WHERE lower(name) = lower('Iskander Mirza');

UPDATE people
SET birth_date = DATE '1880-12-12',
    birth_place = 'Sirajganj, Bengal, British India (now Bangladesh)',
    death_place = 'Dhaka, Bangladesh',
    occupation = 'Politician, statesman, Islamic scholar',
    bio = 'Abdul Hamid Khan Bhashani, widely known as Maulana Bhashani, was a Bangladeshi politician and statesman. He was a founding leader of the Awami Muslim League and later the National Awami Party, and was known for his advocacy of the rural poor.'
WHERE lower(name) IN (lower('Maulana Bhashani'), lower('Abdul Hamid Khan Bhashani'));

UPDATE people
SET birth_date = DATE '1782-01-27',
    birth_place = 'Chandpur, Bengal Presidency, British India (now Bangladesh)',
    death_place = 'Narikelbaria, Bengal Presidency, British India (now West Bengal, India)',
    occupation = 'Revolutionary, religious reformer',
    bio = 'Syed Mir Nisar Ali, better known as Titumir, was a Bengali revolutionary who led an agrarian and religious resistance against the East India Company and local zamindars. He was killed during the destruction of his bamboo fort at Narikelbaria in 1831.'
WHERE lower(name) = lower('Titumir');
