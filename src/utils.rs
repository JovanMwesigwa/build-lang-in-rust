use core::panic;

pub(crate) fn extract_expression(s: &str) -> (&str, &str) {
    // This finds the index of the operation sign and returns it (index)
    // If not found it returns the whole length of the string so that we can just return it...

    let mut op_index = s
        .char_indices()
        .find_map(|(idx, curr_char)| {
            if curr_char.is_ascii_digit() {
                None
            } else {
                Some(idx)
            }
        })
        .unwrap_or_else(|| s.len());

    // 5 + 6
    // 0,1,2
    let digits = &s[op_index..];
    let remainder = &s[..op_index];

    (digits, remainder)
}

pub(crate) fn extract_op(s: &str) -> (&str, &str) {
    match &s[0..1] {
        "+" | "-" | "*" | "/" => {}
        _ => panic!("bad operator"),
    }

    (&s[1..], &s[0..1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_op() {
        assert_eq!(extract_op("+9"), ("9", "+"));
    }

    #[test]
    fn test_extract_expression_add() {
        assert_eq!(extract_expression("5+6"), ("+6", "5"));
    }
}
