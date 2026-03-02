use super::is_dollar_crate;

#[test]
fn test_is_dollar_crate() {
    assert!(is_dollar_crate("$crate"));
    assert!(!is_dollar_crate(""));
    assert!(!is_dollar_crate("$crate "));
    assert!(!is_dollar_crate(" $crate"));
    assert!(!is_dollar_crate(" $crate "));
    assert!(!is_dollar_crate("$ crate"));

    assert!(is_dollar_crate(format_args!("{}", "$crate")));
    assert!(is_dollar_crate(format_args!("{}{}", "$", "crate")));
    assert!(is_dollar_crate(format_args!("{}{}", "$crate", "")));
    assert!(is_dollar_crate(format_args!("{}{}", "", "$crate")));
    assert!(!is_dollar_crate(format_args!("{} {}", "$", "crate")));
    assert!(!is_dollar_crate(format_args!("{}{} ", "$", "crate")));
    assert!(!is_dollar_crate(format_args!(" {}{}", "$", "crate")));
    assert!(!is_dollar_crate(format_args!("{}{}", "$crate", " ")));
}
