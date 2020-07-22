use std::collections::HashMap;
use std::i32;
pub fn lenght(mut a2: i32) -> i32 {
    let mut len: i32 = 0;
    while a2 != 0 {
        len = len + 1;
        a2 = a2 / 10;
    }
    len
}
pub fn hashmap_marathi(a: i32) -> String {
    let mut text: HashMap<i32, &str> = HashMap::new();
    text.insert(0, "");
    text.insert(1, "एक ");
    text.insert(2, "दोन ");
    text.insert(3, "तीन ");
    text.insert(4, "चार ");
    text.insert(5, "पाच ");
    text.insert(6, "सहा ");
    text.insert(7, "सात ");
    text.insert(8, "आठ  ");
    text.insert(9, "नऊ ");
    text.insert(10, "दहा ");
    text.insert(11, "अकरा ");
    text.insert(12, "बारा ");
    text.insert(13, "तेरा ");
    text.insert(14, "चौदा ");
    text.insert(15, "पंधरा ");
    text.insert(16, "सोळा ");
    text.insert(17, "सतरा ");
    text.insert(18, "अठरा ");
    text.insert(19, "एकोणीस ");
    text.insert(20, "वीस ");
    text.insert(21, "एकवीस ");
    text.insert(22, "बावीस ");
    text.insert(23, "तेवीस ");
    text.insert(24, "चोवीस ");
    text.insert(25, "पंचवीस ");
    text.insert(26, "सव्वीस ");
    text.insert(27, "सत्तावीस ");
    text.insert(28, "अठ्ठावीस");
    text.insert(29, "एकोणतीस");
    text.insert(30, "तीस");
    text.insert(31, "एकतीस ");
    text.insert(32, "बत्तीस ");
    text.insert(33, "तेहेतीस ");
    text.insert(34, "चौतीस ");
    text.insert(35, "पस्तीस ");
    text.insert(36, "छत्तीस ");
    text.insert(37, "सदतीस ");
    text.insert(38, "अडतीस  ");
    text.insert(39, "एकोणचाळीस ");
    text.insert(40, "चाळीस ");
    text.insert(41, "एक्केचाळीस ");
    text.insert(42, "बेचाळीस ");
    text.insert(43, "त्रेचाळीस ");
    text.insert(44, "चव्वेचाळीस ");
    text.insert(45, "पंचेचाळीस ");
    text.insert(46, "सेहेचाळीस ");
    text.insert(47, "सत्तेचाळीस ");
    text.insert(48, "अठ्ठेचाळीस ");
    text.insert(49, "एकोणपन्नास ");
    text.insert(50, "पन्नास ");
    text.insert(51, "एक्कावन्न ");
    text.insert(52, "बावन्न ");
    text.insert(53, "त्रेपन्न ");
    text.insert(54, "चोपन्न ");
    text.insert(55, "पंचावन्न ");
    text.insert(56, "छप्पन्न ");
    text.insert(57, "सत्तावन्न ");
    text.insert(58, "अठ्ठावन्न ");
    text.insert(59, "एकोणसाठ ");
    text.insert(60, "साठ ");
    text.insert(61, "एकसष्ठ ");
    text.insert(62, "बासष्ठ");
    text.insert(63, "त्रेसष्ठ ");
    text.insert(64, "चौसष्ठ ");
    text.insert(65, "पासष्ठ ");
    text.insert(66, "सहासष्ठ ");
    text.insert(67, "सदुसष्ठ ");
    text.insert(68, "अडुसष्ठ ");
    text.insert(69, "एकोणसत्तर ");
    text.insert(70, "सत्तर ");
    text.insert(71, "एक्काहत्तर ");
    text.insert(72, "बाहत्तर ");
    text.insert(73, "त्र्याहत्तर ");
    text.insert(74, "चौरयाहत्तर ");
    text.insert(75, "पंच्याहत्तर ");
    text.insert(76, "शहात्तर ");
    text.insert(77, "सत्याहत्तर ");
    text.insert(78, "अठ्ठ्याहत्तर ");
    text.insert(79, "एकोणऐंशी ");
    text.insert(80, "ऐंशी ");
    text.insert(81, "एक्क्याऐंशी ");
    text.insert(82, "ब्याऐंशी ");
    text.insert(83, "त्र्याऐंशी ");
    text.insert(84, "चौऱ्याऐंशी ");
    text.insert(85, "पंच्याऐंशी ");
    text.insert(86, "शहाऐंशी ");
    text.insert(87, "सत्त्याऐंशी ");
    text.insert(88, "अठ्ठ्याऐंशी");
    text.insert(89, "एकोणनव्वद ");
    text.insert(90, "नव्वद ");
    text.insert(91, "एक्क्याण्णव");
    text.insert(92, "ब्याण्णव");
    text.insert(93, "त्र्याण्णव");
    text.insert(94, "चौऱ्याण्णव");
    text.insert(95, "पंच्याण्णव");
    text.insert(96, "शहाण्णव");
    text.insert(97, "सत्त्याण्णव");
    text.insert(98, "अठ्ठ्याण्णव");
    text.insert(99, "नव्व्याण्णव");
    text.insert(100, "शंभर");
    let mut stri = String::new();
    stri.push_str(text.get(&a).unwrap());
    return stri;
}
pub fn hashmap_marathi2(a: i32) -> String {
    let mut output = String::new();
    let mut len = lenght(a);
    while len > 0 {
        if len > 9 {
            output.push_str("No out of range ");
            len = 0;
        }
        if len == 8 || len == 9 {
            output.push_str(&hashmap_marathi(a / 10000000));
            if (a % 1000000000) / 100000000 != 0 || (a % 100000000) / 10000000 != 0 {
                output.push_str("कोटी ");
            }
            len = 7;
        }
        if len == 6 || len == 7 {
            if (a % 1_00_00_000) / 10_00_000 != 0 || (a % 10_00_000) / 1_00_000 != 0 {
                output.push_str(&hashmap_marathi((a / 100000) - (a / 10000000) * 100));
                output.push_str("लाख ");
            }
            len = 5;
        }
        if len == 5 || len == 4 {
            if (a % 100000) / 10000 != 0 || (a % 10000) / 1000 != 0 {
                output.push_str(&hashmap_marathi((a / 1000) - (a / 100000) * 100));

                output.push_str("हजार ");
            }
            len = 3;
        }
        if len == 3 {
            if (a % 1000) / 100 != 0 {
                output.push_str(&hashmap_marathi((a % 1000) / 100));

                output.push_str("शे ");
            }
            len = 2
        }
        if len == 2 || len==1{
            output.push_str(&hashmap_marathi(a % 100));

            len = 0;
        }
    }
    output
}