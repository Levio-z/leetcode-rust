/// [385] Mini Parser
///
/// Given a string s represents the serialization of a nested list, implement a
/// parser to deserialize it and return the deserialized NestedInteger.
/// Each element is either an integer or a list whose elements may also be
/// integers or other lists.  
/// <strong class="example">Example 1:
///
/// Input: s = "324"
/// Output: 324
/// Explanation: You should return a NestedInteger object which contains a
/// single integer 324.
///
/// <strong class="example">Example 2:
///
/// Input: s = "[123,[456,[789]]]"
/// Output: [123,[456,[789]]]
/// Explanation: Return a NestedInteger object containing a nested list with 2
/// elements:
/// 1. An integer containing value 123.
/// 2. A nested list containing two elements: i.  An integer containing value
///    456. ii. A nested list with one element: a. An integer containing value
///    789
///
///  
/// Constraints:
///
///     1 <= s.length <= 5 * 10^4
///     s consists of digits, square brackets "[]", negative sign '-', and
/// commas ','.     s is the serialization of valid NestedInteger.
///     All the values in the input are in the range [-10^6, 10^6].
pub struct Solution {}

// problem: https://leetcode.com/problems/mini-parser/
// discuss: https://leetcode.com/problems/mini-parser/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}
impl Solution {
    pub fn deserialize(s: String) -> NestedInteger {
        if !s.starts_with('[') {
            return NestedInteger::Int(s.parse().unwrap());
        }

        let mut p = Parser {
            stack: Vec::new(),
            num: 0,
            sign: 1,
            in_number: false,
        };

        for ch in s.bytes() {
            match ch {
                b'-' => p.sign = -1,
                b'0'..=b'9' => {
                    p.num = p.num * 10 + (ch - b'0') as i32;
                    p.in_number = true;
                }
                b'[' => {
                    p.stack.push(NestedInteger::List(vec![]));
                }
                b',' | b']' => {
                    p.flush_number();
                    if ch == b']' {
                        p.close_list();
                    }
                }
                _ => {}
            }
        }

        p.stack.pop().unwrap()
    }
}

struct Parser {
    stack: Vec<NestedInteger>,
    num: i32,
    sign: i32,
    in_number: bool,
}

impl Parser {
    fn flush_number(&mut self) {
        if self.in_number {
            if let Some(NestedInteger::List(top)) = self.stack.last_mut() {
                top.push(NestedInteger::Int(self.sign * self.num));
            }
            self.num = 0;
            self.sign = 1;
            self.in_number = false;
        }
    }

    fn close_list(&mut self) {
        if self.stack.len() > 1 {
            let ni = self.stack.pop().unwrap();
            if let Some(NestedInteger::List(top)) = self.stack.last_mut() {
                top.push(ni);
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_385() {
        assert_eq!(
            Solution::deserialize("324".to_string()),
            NestedInteger::Int(324)
        );
        // 新增测试
        assert_eq!(
            Solution::deserialize("[123,[456,[789]]]".to_string()),
            NestedInteger::List(vec![
                NestedInteger::Int(123),
                NestedInteger::List(vec![
                    NestedInteger::Int(456),
                    NestedInteger::List(vec![NestedInteger::Int(789)])
                ])
            ])
        );
    }
    #[test]
    fn test_385_2() {
        let s = "123".to_string();
        println!("{}", s);
        if !s.starts_with('[') {
            println!("{}", s);
            let x = s.parse::<i32>().unwrap();
        }
        println!("{}", s);
    }
}
