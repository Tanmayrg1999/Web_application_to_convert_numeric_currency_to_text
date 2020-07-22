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
pub    fn hashmap_gujrati(digits: i32) -> String {
    let mut text: HashMap<i32, &str> = HashMap::new();
    text.insert(0, " ");
    text.insert(1, "એક ");
    text.insert(2, "બે  ");
    text.insert(3, "ત્રણ ");
    text.insert(4, "ચાર ");
    text.insert(5, "પાંચ ");
    text.insert(6, "છ ");
    text.insert(7, "સાત ");
    text.insert(8, "આઠ  ");
    text.insert(9, "નવ ");
    text.insert(10, "દસ ");
    text.insert(11, "અગિયાર ");
    text.insert(12, "બાર ");
    text.insert(13, "તેર ");
    text.insert(14, "ચૌદ ");
    text.insert(15, "પંદર ");
    text.insert(16, "સોળ ");
    text.insert(17, "સત્તર ");
    text.insert(18, "અઢાર ");
    text.insert(19, "ઓગણિસ ");
    text.insert(20, "વીસ ");
    text.insert(21, "એકવીસ ");
    text.insert(22, "બાવીસ ");
    text.insert(23, "તેવીસ ");
    text.insert(24, "ચોવીસ ");
    text.insert(25, "પચ્ચીસ ");
    text.insert(26, "છવીસ ");
    text.insert(27, "સત્તાવીસ ");
    text.insert(28, "અઠ્ઠાવીસ");
    text.insert(29, "ઓગણત્રીસ");
    text.insert(30, "ત્રીસ");
    text.insert(31, "એકત્રીસ ");
    text.insert(32, "બત્રીસ ");
    text.insert(33, "તેત્રીસ ");
    text.insert(34, "ચોત્રીસ ");
    text.insert(35, "પાંત્રીસ ");
    text.insert(36, "છત્રીસ ");
    text.insert(37, "સડત્રીસ ");
    text.insert(38, "અડત્રીસ  ");
    text.insert(39, "ઓગણચાલીસ ");
    text.insert(40, "ચાલીસ ");
    text.insert(41, "એકતાલીસ ");
    text.insert(42, "બેતાલીસ     ");
    text.insert(43, "ત્રેતાલીસ ");
    text.insert(44, "ચુંમાલીસ ");
    text.insert(45, "પિસ્તાલીસ ");
    text.insert(46, "છેતાલીસ ");
    text.insert(47, "સુડતાલીસ ");
    text.insert(48, "અડતાલીસ ");
    text.insert(49, "ઓગણપચાસ ");
    text.insert(50, "પચાસ    ");
    text.insert(51, "એકાવન ");
    text.insert(52, "બાવન ");
    text.insert(53, "ત્રેપન ");
    text.insert(54, "ચોપન ");
    text.insert(55, "પંચાવન ");
    text.insert(56, "છપ્પન ");
    text.insert(57, "સત્તાવન ");
    text.insert(58, "અઠ્ઠાવન ");
    text.insert(59, "ઓગણસાઠ ");
    text.insert(60, "સાઈઠ ");
    text.insert(61, "એકસઠ ");
    text.insert(62, "બાસઠ");
    text.insert(63, "ત્રેસઠ ");
    text.insert(64, "ચોસઠ  ");
    text.insert(65, "પાંસઠ ");
    text.insert(66, "છાસઠ ");
    text.insert(67, "સડસઠ ");
    text.insert(68, "અડસઠ ");
    text.insert(69, "અગણોસિત્તેર ");
    text.insert(70, "સિત્તેર ");
    text.insert(71, "એકોતેર ");
    text.insert(72, "બોતેર ");
    text.insert(73, "તોતેર ");
    text.insert(74, "ચુમોતેર ");
    text.insert(75, "પંચોતેર ");
    text.insert(76, "છોતેર ");
    text.insert(77, "સિત્યોતેર ");
    text.insert(78, "ઇઠ્યોતેર ");
    text.insert(79, "ઓગણાએંસી ");
    text.insert(80, "એંસી ");
    text.insert(81, "એક્યાસી ");
    text.insert(82, "બ્યાસી ");
    text.insert(83, "ત્યાસી ");
    text.insert(84, "ચોર્યાસી ");
    text.insert(85, "પંચાસી ");
    text.insert(86, "છ્યાસી ");
    text.insert(87, "સિત્યાસી ");
    text.insert(88, "ઈઠ્યાસી ");
    text.insert(89, "નેવ્યાસી ");
    text.insert(90, "નેવું ");
    text.insert(91, "એકાણું ");
    text.insert(92, "બાણું ");
    text.insert(93, "ત્રાણું ");
    text.insert(94, "ચોરાણું ");
    text.insert(95, "પંચાણું ");
    text.insert(96, "છન્નું");
    text.insert(97, "સત્તાણું ");
    text.insert(98, "અઠ્ઠાણું ");
    text.insert(99, "નવ્વાણું ");
    text.insert(100, "સો ");
    let mut wordstr = String::new();
    wordstr.push_str(text.get(&digits).unwrap());
    return wordstr;
}
pub    fn hashmap_gujrati2(a: i32) -> String {
    let mut output = String::new();
    let mut len = lenght(a);
    while len > 0 {
        if len > 9 {
            output.push_str("No out of range ");
            len = 0;
        }
        if len == 8 || len == 9 {
            output.push_str(&hashmap_gujrati(a / 10000000));
            if (a % 1000000000) / 100000000 != 0 || (a % 100000000) / 10000000 != 0 {
                output.push_str("કરોડ઼ ");
            }
            len = 7;
        }
        if len == 6 || len == 7 {
            if (a % 1_00_00_000) / 10_00_000 != 0 || (a % 10_00_000) / 1_00_000 != 0 {
                output.push_str(&hashmap_gujrati((a / 100000) - (a / 10000000) * 100));
                output.push_str("લાખ ");
            }
            len = 5;
        }
        if len == 5 || len == 4 {
            if (a % 100000) / 10000 != 0 || (a % 10000) / 1000 != 0 {
                output.push_str(&hashmap_gujrati((a / 1000) - (a / 100000) * 100));

                output.push_str("હજાર ");
            }
            len = 3;
        }
        if len == 3 {
            if (a % 1000) / 100 != 0 {
                output.push_str(&hashmap_gujrati((a % 1000) / 100));

                output.push_str("સો ");
            }
            len = 2
        }
        if len == 2 || len==1{
            output.push_str(&hashmap_gujrati(a % 100));
            len = 0;
        }
    }
    output
}