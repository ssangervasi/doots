pub fn pad_end(unpadded: &str, fill: &str, width: usize) -> String {
    let char_len = unpadded.chars().count();
    if width <= char_len {
        return unpadded.to_string();
    }
    let mut right = String::new();
    let mut fill_chars: Vec<char> = fill.chars().collect();
    if fill_chars.len() < 1 {
        fill_chars.push(' ');
    }
    for i in 0..(width - char_len) {
        let fill_index = i % fill_chars.len();
        right.push(fill_chars[fill_index]);
    }
    format!("{}{}", unpadded, right)
}

pub fn pad_out(unpadded: &str, fill: &str, width: usize) -> String {
    let char_len = unpadded.chars().count();
    if width <= char_len {
        return unpadded.to_string();
    }
    let mut left = String::new();
    let mut right = String::new();
    let mut fill_chars: Vec<char> = fill.chars().collect();
    if fill_chars.len() < 1 {
        fill_chars.push(' ');
    }
    for i in 0..(width - char_len) {
        let fill_index = i % fill_chars.len();
        if i % 2 == 0 {
            left.push(fill_chars[fill_index]);
        } else {
            right.push(fill_chars[fill_index]);
        }
    }
    format!("{}{}{}", left, unpadded, right)
}

#[test]
fn test_pad_end() {
    assert_eq!("012", pad_end("012", " ", 3));
    assert_eq!("012  ", pad_end("012", " ", 5));
    assert_eq!("012  ", pad_end("012", "", 5));
    assert_eq!("012--", pad_end("012", "-", 5));
    assert_eq!("01234", pad_end("012", "34", 5));
    assert_eq!("012343", pad_end("012", "34", 6));
}

#[test]
fn test_pad_out() {
    assert_eq!("012", pad_out("012", " ", 0));
    assert_eq!("012", pad_out("012", " ", 3));
    assert_eq!(" 012 ", pad_out("012", " ", 5));
    assert_eq!(" 012 ", pad_out("012", "", 5));
    assert_eq!("-012-", pad_out("012", "-", 5));
    assert_eq!("30124", pad_out("012", "34", 5));
    assert_eq!("330124", pad_out("012", "34", 6));
}
