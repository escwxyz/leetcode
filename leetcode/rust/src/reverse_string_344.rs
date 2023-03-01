/// Reverse string array in place
///
/// * `s`: Array of chars
///
/// Problem:
/// https://leetcode.cn/problems/reverse-string/
#[allow(dead_code)]
pub fn reverse_string(s: &mut Vec<char>) {
    let mut head = 0;
    let mut tail = s.len() - 1;

    while head < tail {
        s.swap(head, tail);

        head += 1;
        tail -= 1;
    }
}
