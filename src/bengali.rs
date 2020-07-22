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
pub    fn hashmap_bengali(a: i32) -> String {
    let mut text: HashMap<i32, &str> = HashMap::new();
    text.insert(0, "");
    text.insert(1, "এক ");
    text.insert(2, "দুই ");
    text.insert(3, "তিন ");
    text.insert(4, "চার ");
    text.insert(5, "পাঁচ ");
    text.insert(6, "ছয় ");
    text.insert(7, "সাত ");
    text.insert(8, "আট ");
    text.insert(9, "নয় ");
    text.insert(10, "দশ ");
    text.insert(11, "এগারো ");
    text.insert(12, "বারো ");
    text.insert(13, "তেরো ");
    text.insert(14, "চোদ্দো ");
    text.insert(15, "পনেরো ");
    text.insert(16, "ষোলো ");
    text.insert(17, "সতেরো ");
    text.insert(18, "আঠারো ");
    text.insert(19, "উনিশ ");
    text.insert(20, "বিশ ");
    text.insert(21, "একুশ ");
    text.insert(22, "বাইশ ");
    text.insert(23, "তেইশ ");
    text.insert(24, "চব্বিশ ");
    text.insert(25, "পঁচিশ ");
    text.insert(26, "ছাব্বিশ ");
    text.insert(27, "সাতাশ ");
    text.insert(28, "আটাশ");
    text.insert(29, "ঊনত্রিশ");
    text.insert(30, "ত্রিশ");
    text.insert(31, "একত্রিশ ");
    text.insert(32, "বত্রিশ ");
    text.insert(33, "তেত্রিশ ");
    text.insert(34, "চৌত্রিশ ");
    text.insert(35, "পৈন্তীরিশ ");
    text.insert(36, "ছোতিরিশ ");
    text.insert(37, "সাইত্রিশ ");
    text.insert(38, "আটত্রিশ  ");
    text.insert(39, "ঊনচল্লিশ ");
    text.insert(40, "চল্লিশ ");
    text.insert(41, "একচল্লিশ ");
    text.insert(42, "বিয়াল্লিশ ");
    text.insert(43, "তেতাল্লিশ ");
    text.insert(44, "চুয়াল্লিশ ");
    text.insert(45, "পঁয়তাল্লিশ ");
    text.insert(46, "ছিচল্লিশ ");
    text.insert(47, "সাতচল্লিশ ");
    text.insert(48, "আটচল্লিশ ");
    text.insert(49, "ঊনপঞ্চাশ ");
    text.insert(50, "পঞ্চাশ ");
    text.insert(51, "একান্নো ");
    text.insert(52, "বাহান্নো ");
    text.insert(53, "তেপ্পান্নো ");
    text.insert(54, "চুযান্নো ");
    text.insert(55, "পঞ্চান্নো ");
    text.insert(56, "ছাপ্পান্নো ");
    text.insert(57, "সাতান্নো ");
    text.insert(58, "আটান্নো ");
    text.insert(59, "ঊনষাট ");
    text.insert(60, "ষাট ");
    text.insert(61, "একষট্টি ");
    text.insert(62, "বাষট্টি ");
    text.insert(63, "তেষট্টি");
    text.insert(64, "চৌষট্টি");
    text.insert(65, "পৈন্ষট্টি ");
    text.insert(66, "ছেষট্টি ");
    text.insert(67, "সাতষট্টি ");
    text.insert(68, "আটষট্টি ");
    text.insert(69, "উনসত্তর ");
    text.insert(70, "সত্তর ");
    text.insert(71, "একাত্তর ");
    text.insert(72, "বাহাত্তর ");
    text.insert(73, "তেহাত্তর ");
    text.insert(74, "চহত্তর ");
    text.insert(75, "পচাত্তর ");
    text.insert(76, "ছিয়াত্তর ");
    text.insert(77, "সাতাত্তর ");
    text.insert(78, "আটাত্তর ");
    text.insert(79, "উনাশি ");
    text.insert(80, "আশি ");
    text.insert(81, "একাশি ");
    text.insert(82, "বিরাশি ");
    text.insert(83, "তিরাশি ");
    text.insert(84, "চুরাশি ");
    text.insert(85, "পঁচাশি ");
    text.insert(86, "ছিয়াশি ");
    text.insert(87, "সাতাশি ");
    text.insert(88, "আটাশি ");
    text.insert(89, "ঊনোনব্বই ");
    text.insert(90, "নব্বুই ");
    text.insert(91, "একানব্বই ");
    text.insert(92, "বিরানব্বই ");
    text.insert(93, "তিরানব্বই ");
    text.insert(94, "চুরানব্বই ");
    text.insert(95, "পঁচানব্বই ");
    text.insert(96, "ছিয়ানব্বই ");
    text.insert(97, "সাতানব্বই ");
    text.insert(98, "আটানব্বই ");
    text.insert(99, "নিরানব্বই ");
    text.insert(100, "একশো ");
    let mut stri = String::new();
    stri.push_str(text.get(&a).unwrap());
    return stri;
}
pub    fn hashmap_bengali2(a: i32) -> String {
    let mut output = String::new();
    let mut len = lenght(a);
    while len > 0 {
        if len > 9 {
            output.push_str("No out of range ");
            len = 0;
        }
        if len == 8 || len == 9 {
            output.push_str(&hashmap_bengali(a / 10000000));
            if (a % 1000000000) / 100000000 != 0 || (a % 100000000) / 10000000 != 0 {
                output.push_str("কোটি ");
            }
            len = 7;
        }
        if len == 6 || len == 7 {
            if (a % 1_00_00_000) / 10_00_000 != 0 || (a % 10_00_000) / 1_00_000 != 0 {
                output.push_str(&hashmap_bengali((a / 100000) - (a / 10000000) * 100));
                output.push_str("লক্ষ  ");
            }
            len = 5;
        }
        if len == 5 || len == 4 {
            if (a % 100000) / 10000 != 0 || (a % 10000) / 1000 != 0 {
                output.push_str(&hashmap_bengali((a / 1000) - (a / 100000) * 100));

                output.push_str("হাজার ");
            }
            len = 3;
        }
        if len == 3 {
            if (a % 1000) / 100 != 0 {
                output.push_str(&hashmap_bengali((a % 1000) / 100));

                output.push_str("শো ");
            }
            len = 2
        }
        if len == 2 || len==1{
            output.push_str(&hashmap_bengali(a % 100));

            len = 0;
        }
    }
    output
}