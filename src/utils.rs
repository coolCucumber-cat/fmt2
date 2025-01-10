#[inline]
#[must_use]
pub fn has_newlines(s: &str) -> bool {
    let v = s.contains('\n');
    #[cfg(debug_assertions)]
    {
        let v2 = s.chars().any(|c| c == '\n');
        debug_assert_eq!(v, v2);
        let count = count_newlines(s);
        debug_assert_eq!(v, count != 0);
    }
    v
}

#[inline]
#[must_use]
pub fn count_newlines(s: &str) -> usize {
    s.chars().filter(|&c| c == '\n').count()
}

#[inline]
#[must_use]
pub fn first_line(s: &str) -> &str {
    let s = first_line_no_debug_assertion(s);
    #[cfg(debug_assertions)]
    {
        debug_assert!(!has_newlines(s));
        debug_assert_eq!(count_newlines(s), 0);
    }
    s
}

#[inline]
#[must_use]
pub fn first_line_no_debug_assertion(s: &str) -> &str {
    let s = match s.find('\n') {
        Some(i) => unsafe { s.get_unchecked(..i) },
        None => s,
    };
    s
}

#[allow(unused)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_line() {
        assert_eq!(first_line_no_debug_assertion("123"), "123");
        assert_eq!(first_line_no_debug_assertion("123\nabc"), "123");
        assert_eq!(first_line_no_debug_assertion("123\n"), "123");
        assert_eq!(first_line_no_debug_assertion("123\nabc\n456"), "123");
        assert_eq!(first_line_no_debug_assertion("123\nabc\n"), "123");
        assert_eq!(first_line_no_debug_assertion("\nabc"), "");
        assert_eq!(first_line_no_debug_assertion("\n"), "");
        assert_eq!(first_line_no_debug_assertion(""), "");
    }
}
