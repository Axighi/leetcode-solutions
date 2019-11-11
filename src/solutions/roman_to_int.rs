/**
 * [13] Roman to Integer
 *
 * Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.
 *
 *
 * Symbol       Value
 * I             1
 * V             5
 * X             10
 * L             50
 * C             100
 * D             500
 * M             1000
 *
 * For example, two is written as II in Roman numeral, just two one's added together. Twelve is written as, XII, which is simply X + II. The number twenty seven is written as XXVII, which is XX + V + II.
 *
 * Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not IIII. Instead, the number four is written as IV. Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as IX. There are six instances where subtraction is used:
 *
 *
 * 	I can be placed before V (5) and X (10) to make 4 and 9.
 * 	X can be placed before L (50) and C (100) to make 40 and 90.
 * 	C can be placed before D (500) and M (1000) to make 400 and 900.
 *
 *
 * Given a roman numeral, convert it to an integer. Input is guaranteed to be within the range from 1 to 3999.
 *
 * Example 1:
 *
 *
 * Input: "III"
 * Output: 3
 *
 * Example 2:
 *
 *
 * Input: "IV"
 * Output: 4
 *
 * Example 3:
 *
 *
 * Input: "IX"
 * Output: 9
 *
 * Example 4:
 *
 *
 * Input: "LVIII"
 * Output: 58
 * Explanation: L = 50, V= 5, III = 3.
 *
 *
 * Example 5:
 *
 *
 * Input: "MCMXCIV"
 * Output: 1994
 * Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.
 *
 */
use std::collections::HashMap;

#[allow(dead_code)]
pub fn roman_to_int(s: String) -> i32 {
    let mut roman_values = HashMap::new();
    roman_values.insert("I".as_bytes()[0], 1);
    roman_values.insert("V".as_bytes()[0], 5);
    roman_values.insert("X".as_bytes()[0], 10);
    roman_values.insert("L".as_bytes()[0], 50);
    roman_values.insert("C".as_bytes()[0], 100);
    roman_values.insert("D".as_bytes()[0], 500);
    roman_values.insert("M".as_bytes()[0], 1000);

    let s_vec = s.as_bytes();
    let mut ret = 0;
    let mut index = 0;

    while index < s_vec.len() {
        if index == s_vec.len() - 1 {
            ret += roman_values.get(&s_vec[s_vec.len() - 1]).unwrap();
            index += 1;
        } else {
            if s_vec[index] == "I".as_bytes()[0] {
                if s_vec[index + 1] == "V".as_bytes()[0] {
                    ret += 4;
                    index += 2;
                } else if s_vec[index + 1] == "X".as_bytes()[0] {
                    ret += 9;
                    index += 2;
                } else {
                    ret += 1;
                    index += 1;
                }
            } else if s_vec[index] == "X".as_bytes()[0] {
                if s_vec[index + 1] == "L".as_bytes()[0] {
                    ret += 40;
                    index += 2;
                } else if s_vec[index + 1] == "C".as_bytes()[0] {
                    ret += 90;
                    index += 2;
                } else {
                    ret += 10;
                    index += 1;
                }
            } else if s_vec[index] == "C".as_bytes()[0] {
                if s_vec[index + 1] == "D".as_bytes()[0] {
                    ret += 400;
                    index += 2;
                } else if s_vec[index + 1] == "M".as_bytes()[0] {
                    ret += 900;
                    index += 2;
                } else {
                    ret += 100;
                    index += 1;
                }
            } else if s_vec[index] == "V".as_bytes()[0] {
                ret += 5;
                index += 1;
            } else if s_vec[index] == "L".as_bytes()[0] {
                ret += 50;
                index += 1;
            } else if s_vec[index] == "D".as_bytes()[0] {
                ret += 500;
                index += 1;
            } else {
                ret += 1000;
                index += 1;
            }
        }
    }

    ret
}

#[test]
fn test() {
    assert_eq!(roman_to_int("III".to_string()), 3);
    assert_eq!(roman_to_int("IV".to_string()), 4);
    assert_eq!(roman_to_int("IX".to_string()), 9);
    assert_eq!(roman_to_int("MCMXCIV".to_string()), 1994);
    assert_eq!(roman_to_int("DCXXI".to_string()), 621);
}
