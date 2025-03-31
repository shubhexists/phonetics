use phonetics::Nysiis;

#[test]
fn test_empty_string() {
    let nysiis = Nysiis::new();
    assert_eq!(nysiis.encode(""), "");
}

#[test]
fn test_single_letter() {
    let nysiis = Nysiis::new();
    assert_eq!(nysiis.encode("A"), "A");
}

#[test]
fn test_basic_names() {
    let nysiis = Nysiis::new();
    assert_eq!(nysiis.encode("Smith"), "SNAT");
    assert_eq!(nysiis.encode("Johnson"), "JAONSA");
    assert_eq!(nysiis.encode("Williams"), "WALAN");
    assert_eq!(nysiis.encode("Jones"), "JAN");
    assert_eq!(nysiis.encode("Brown"), "BRAON");
    assert_eq!(nysiis.encode("Miller"), "MALAR");
}

#[test]
fn test_first_character_translations() {
    let nysiis = Nysiis::new();
    assert_eq!(nysiis.encode("MacDowell"), "MCDAOA");
    assert_eq!(nysiis.encode("Knight"), "NAGT");
    assert_eq!(nysiis.encode("King"), "CANG");
    assert_eq!(nysiis.encode("Phillip"), "FALAP");
    assert_eq!(nysiis.encode("Pfister"), "FASTAR");
    assert_eq!(nysiis.encode("Schultz"), "SALT");
}

#[test]
fn test_last_character_translations() {
    let nysiis = Nysiis::new();
    assert_eq!(nysiis.encode("Bailey"), "BALY");
    assert_eq!(nysiis.encode("Bailie"), "BALY");
    assert_eq!(nysiis.encode("Bent"), "BAD");
    assert_eq!(nysiis.encode("Bernard"), "BARNAD");
}

#[test]
fn test_special_cases() {
    let nysiis = Nysiis::new();
    assert_eq!(nysiis.encode("Gbemisola"), "JANASA");
    assert_eq!(nysiis.encode("Kpabom"), "CPABAN");
    assert_eq!(nysiis.encode("Nwachukwu"), "WACACW");
    assert_eq!(nysiis.encode("Tsonga"), "SANG");
    assert_eq!(nysiis.encode("Sharma"), "SARN");
    assert_eq!(nysiis.encode("Bhatt"), "BAT");
    assert_eq!(nysiis.encode("Dhoni"), "DAN");
    assert_eq!(nysiis.encode("Ghosh"), "G");
    assert_eq!(nysiis.encode("Jharkhand"), "JARCKA");
    assert_eq!(nysiis.encode("Khan"), "CAN");
}
