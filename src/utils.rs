pub(crate) fn extract_digits(s: &str) -> (&str, &str) {
    // 1. Find the index of the operation sign and return ir, otherwise, just return the length of the whole thing
    let operation_index = s
        .char_indices()
        .find_map(|(idx, current_char)| {
            if current_char.is_ascii_digit() {
                None
            } else {
                Some(idx)
            }
        })
        .unwrap_or_else(|| s.len());

    // 5+6
    // (+6, 5)
    let digits = &s[..operation_index];
    let remainder = &s[operation_index..];

    (remainder, digits)
}

pub(crate) fn extract_op(s: &str) -> (&str, &str) {
    // +6
    // (6, +)
    // + 6
    // 0,1
    match &s[0..1] {
        "+" | "-" | "*" | "/" => {}
        _ => panic!("Bad operation"),
    }

    (&s[1..], &s[0..1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_digits_add() {
        assert_eq!(extract_digits("5+6"), ("+6", "5"));
    }
}
