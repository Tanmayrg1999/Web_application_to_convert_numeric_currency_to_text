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
 pub fn hashmap_hindi(digits: i32) -> String {
    let mut text: HashMap<i32, &str> = HashMap::new();
    text.insert(0, " ");
    text.insert(1, "एक ");
    text.insert(2, "दो ");
    text.insert(3, "तीन ");
    text.insert(4, "चार ");
    text.insert(5, "पांच ");
    text.insert(6, "छह ");
    text.insert(7, "सात ");
    text.insert(8, "आठ  ");
    text.insert(9, "नौ ");
    text.insert(10, "दस ");
    text.insert(11, "ग्यारह ");
    text.insert(12, "बारह ");
    text.insert(13, "तेरह ");
    text.insert(14, "चौदह ");
    text.insert(15, "पंद्रह ");
    text.insert(16, "सोलह ");
    text.insert(17, "सत्रह ");
    text.insert(18, "अठारह ");
    text.insert(19, "उन्नीस ");
    text.insert(20, "बीस ");
    text.insert(21, "इकीस ");
    text.insert(22, "बाईस ");
    text.insert(23, "तेइस ");
    text.insert(24, "चौबीस ");
    text.insert(25, "पच्चीस ");
    text.insert(26, "छब्बीस ");
    text.insert(27, "सताइस ");
    text.insert(28, "अट्ठाइस");
    text.insert(29, "उनतीस");
    text.insert(30, "तीस");
    text.insert(31, "इकतीस ");
    text.insert(32, "बतीस ");
    text.insert(33, "तैंतीस ");
    text.insert(34, "चौंतीस ");
    text.insert(35, "पैंतीस ");
    text.insert(36, "छतीस ");
    text.insert(37, "सैंतीस ");
    text.insert(38, "अड़तीस  ");
    text.insert(39, "उनतालीस ");
    text.insert(40, "चालीस ");
    text.insert(41, "इकतालीस ");
    text.insert(42, "बयालीस  ");
    text.insert(43, "तैतालीस ");
    text.insert(44, "चवालीस ");
    text.insert(45, "पैंतालीस ");
    text.insert(46, "छयालिस ");
    text.insert(47, "सैंतालीस ");
    text.insert(48, "अड़तालीस ");
    text.insert(49, "उनचास ");
    text.insert(50, "पचास ");
    text.insert(51, "इक्यावन ");
    text.insert(52, "बावन ");
    text.insert(53, "तिरपन ");
    text.insert(54, "चौवन ");
    text.insert(55, "पचपन ");
    text.insert(56, "छप्पन ");
    text.insert(57, "सतावन ");
    text.insert(58, "अठावन ");
    text.insert(59, "उनसठ ");
    text.insert(60, "साठ ");
    text.insert(61, "इकसठ ");
    text.insert(62, "बासठ");
    text.insert(63, "तिरसठ ");
    text.insert(64, "चौंसठ  ");
    text.insert(65, "पैंसठ ");
    text.insert(66, "छियासठ ");
    text.insert(67, "सड़सठ ");
    text.insert(68, "अड़सठ ");
    text.insert(69, "उनहतर ");
    text.insert(70, "सत्तर ");
    text.insert(71, "इकहतर ");
    text.insert(72, "बहतर ");
    text.insert(73, "तिहतर ");
    text.insert(74, "चौहतर ");
    text.insert(75, "पचहतर ");
    text.insert(76, "छिहतर ");
    text.insert(77, "सतहतर ");
    text.insert(78, "अठहतर ");
    text.insert(79, "उन्नासी ");
    text.insert(80, "अस्सी   ");
    text.insert(81, "इक्यासी ");
    text.insert(82, "बयासी ");
    text.insert(83, "तिरासी ");
    text.insert(84, "चौरासी ");
    text.insert(85, "पचासी ");
    text.insert(86, "छियासी ");
    text.insert(87, "सतासी ");
    text.insert(88, "अट्ठासी");
    text.insert(89, "नवासी ");
    text.insert(90, "नब्बे ");
    text.insert(91, "इक्यानवे");
    text.insert(92, "बानवे");
    text.insert(93, "तिरानवे");
    text.insert(94, "चौरानवे");
    text.insert(95, "पचानवे");
    text.insert(96, "छियानवे");
    text.insert(97, "सतानवे");
    text.insert(98, "अट्ठानवे");
    text.insert(99, "निन्यानवे");
    text.insert(100, "एकसौ");
    let mut wordstr = String::new();
    wordstr.push_str(text.get(&digits).unwrap());
    return wordstr;
}
pub    fn hashmap_hindi2(a: i32) -> String {
    let mut output = String::new();
    let mut len = lenght(a);
    while len > 0 {
        if len > 9 {
            output.push_str("No out of range ");
            len = 0;
        }
        if len == 8 || len == 9 {
            output.push_str(&hashmap_hindi(a / 10000000));
            if (a % 1000000000) / 100000000 != 0 || (a % 100000000) / 10000000 != 0 {
                output.push_str("करोड़ ");
            }
            len = 7;
        }
        if len == 6 || len == 7 {
            if (a % 1_00_00_000) / 10_00_000 != 0 || (a % 10_00_000) / 1_00_000 != 0 {
                output.push_str(&hashmap_hindi((a / 100000) - (a / 10000000) * 100));
                output.push_str("लाख ");
            }
            len = 5;
        }
        if len == 5 || len == 4 {
            if (a % 100000) / 10000 != 0 || (a % 10000) / 1000 != 0 {
                output.push_str(&hashmap_hindi((a / 1000) - (a / 100000) * 100));

                output.push_str("हजार ");
            }
            len = 3;
        }
        if len == 3 {
            if (a % 1000) / 100 != 0 {
                output.push_str(&hashmap_hindi((a % 1000) / 100));

                output.push_str("सौ ");
            }
            len = 2
        }
        if len == 2 || len==1{
            output.push_str(&hashmap_hindi(a % 100));

            len = 0;
        }
    }
    output
}