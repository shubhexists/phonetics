use phonetics::{
    Metaphone,
    metaphone::{metaphone, metaphone_metric},
};

#[test]
fn test_metaphone() {
    let m = Metaphone::new();
    assert_eq!(m.encode("smith").unwrap(), "sm0");
    assert_eq!(m.encode("Schmidt").unwrap(), "skhmtt");
    assert_eq!(m.encode("johnson").unwrap(), "jhnsn");
    assert_eq!(m.encode("Jones").unwrap(), "jns");
    assert_eq!(m.encode("white").unwrap(), "wt");
}

#[test]
fn test_metaphone_compare() {
    let m = Metaphone::new();
    assert!(m.compare("smith", "Schmidt").unwrap() == false);
    assert!(m.compare("johnson", "Johnston").unwrap() == false);
    assert!(m.compare("white", "wight").unwrap() == false);
    assert!(m.compare("knight", "night").unwrap() == true);
}

#[test]
fn test_function_api() {
    assert_eq!(metaphone("phone").unwrap(), "fn");
    assert_eq!(metaphone("knight").unwrap(), "nht");
    assert_eq!(metaphone("psychology").unwrap(), "psxlj");
    assert_eq!(metaphone("beautiful").unwrap(), "btfl");

    assert!(metaphone_metric("white", "wight").unwrap() == false);
    assert!(metaphone_metric("knight", "night").unwrap() == true);
}

#[test]
fn test_de_duplicate() {
    let m = Metaphone::new();
    assert_eq!(m.de_duplicate("aabbbbc"), "abc");
    assert_eq!(m.de_duplicate("hello"), "helo");
    assert_eq!(m.de_duplicate("bookkeeper"), "bokeper");
}

#[test]
fn test_error_handling() {
    let m = Metaphone::new();
    assert!(m.encode("").is_err());
    assert!(m.encode("123").is_err());
    assert!(m.compare("", "test").is_err());
    assert!(m.compare("test", "123").is_err());
}
