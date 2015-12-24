use super::*;

#[test]
fn test_region_create() {
    let region = Region::new();
    assert_eq!(region.len(), 0);
    assert_eq!(region.pos(0), None);
}

#[test]
fn test_region_clear() {
    let mut region = Region::new();
    region.clear();
}

#[test]
fn test_regex_create() {
    Regex::new(".*", OPTION_NONE, SYNTAX_RUBY).unwrap();
}

#[test]
#[should_panic(expected = "Oniguruma error: invalid character property name {foo}")]
fn test_regex_invalid() {
    Regex::new("\\p{foo}", OPTION_NONE, SYNTAX_RUBY).unwrap();
}

#[test]
fn test_regex_search() {
    let mut region = Region::new();
    let regex = Regex::new("e(l+)", OPTION_NONE, SYNTAX_RUBY).unwrap();
    let r = regex.search("hello", &mut region, OPTION_NONE).unwrap();
    assert_eq!(r, Some(1));
    assert_eq!(region.len(), 2);
    let pos1 = region.pos(0).unwrap();
    let pos2 = region.pos(1).unwrap();
    assert_eq!(pos1, (1, 4));
    assert_eq!(pos2, (2, 4));
}
