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
pub fn hashmap_english(a: i32) -> String {
    let mut text: HashMap<i32, &str> = HashMap::new();
    text.insert(0, "");
    text.insert(1, "One ");
    text.insert(2, "Two ");
    text.insert(3, "Three ");
    text.insert(4, "Four ");
    text.insert(5, "Five ");
    text.insert(6, "Six ");
    text.insert(7, "Seven ");
    text.insert(8, "Eight ");
    text.insert(9, "Nine ");
    text.insert(10, "Ten ");
    text.insert(11, "Eleven ");
    text.insert(12, "Twelve ");
    text.insert(13, "Thirteen ");
    text.insert(14, "Fourteen ");
    text.insert(15, "Fifteen ");
    text.insert(16, "Sixteen ");
    text.insert(17, "Seventeen ");
    text.insert(18, "Eighteen ");
    text.insert(19, "Nineteen ");
    text.insert(20, "Twenty ");
    text.insert(30, "Thirty ");
    text.insert(40, "Fourty ");
    text.insert(50, "Fifty ");
    text.insert(60, "Sixty ");
    text.insert(70, "Seventy ");
    text.insert(80, "Eighty ");
    text.insert(90, "Ninety ");
    let mut stri = String::new();
    stri.push_str(text.get(&a).unwrap());
    return stri;
}
pub fn hashmap_english2(a: i32) -> String {
    let mut output = String::new();
    let x: i32 = a % 100;
    if x > 0 && x <= 20 {
        output.push_str(&hashmap_english(x));
    } else {
        let y: i32 = x - (x % 10);
        let m: i32 = x % 10;
        output.push_str(&hashmap_english(y));
        output.push_str(&hashmap_english(m));
    }
    output
}
pub fn hashmap_english3(a: i32) -> String {
    let mut output = String::new();
    let mut len = lenght(a);

    while len >= 3 {
        if len > 9 {
            output.push_str("No out of range ");
            len = 0;
        }
        if len == 8 || len == 9 {
            output.push_str(&hashmap_english2(a / 10000000));
            if (a % 1000000000) / 100000000 != 0 || (a % 100000000) / 10000000 != 0 {
                output.push_str("Crore ");
            }
            len = 7;
        }
        if len == 6 || len == 7 {
            output.push_str(&hashmap_english2(a / 100000));
            if (a % 10000000) / 1000000 != 0 || (a % 1000000) / 100000 != 0 {
                output.push_str("Lakh ");
            }
            len = 5;
        }
        if len == 5 || len == 4 {
            output.push_str(&hashmap_english2(a / 1000));
            if (a % 100000) / 10000 != 0 || (a % 10000) / 1000 != 0 {
                output.push_str("Thousand ");
            }
            len = 3;
        }
        if len == 3 {
            output.push_str(&hashmap_english2((a % 1000) / 100));
            if (a % 1000) / 100 != 0 {
                output.push_str("Hundred ");
            }
            output.push_str(&hashmap_english2(a % 100));
            len = len - 1;
        }
    }
    output
}