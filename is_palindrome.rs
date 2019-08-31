pub fn is_palindrome(in_str: &[u8]) -> bool {
    let half = in_str.len() / 2;
    in_str.iter().take(half).eq(in_str.iter().rev().take(half))
}
