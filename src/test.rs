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
    Regex::new(".*").unwrap();
}

#[test]
#[should_panic(expected = "Error(-223, invalid character property name {foo})")]
fn test_regex_invalid() {
    Regex::new("\\p{foo}").unwrap();
}

#[test]
fn test_error_code() {
    match Regex::new("\\p{foo}") {
        Ok(..) => panic!("should fail"),
        Err(error) => assert_eq!(error.code(), -223)
    }
}


#[test]
fn test_regex_search() {
    let mut region = Region::new();
    let regex = Regex::new("e(l+)").unwrap();
    let r = regex.search("hello", &mut region, OPTION_NONE).unwrap();
    assert_eq!(r, Some(1));
    assert_eq!(region.len(), 2);
    let pos1 = region.pos(0).unwrap();
    let pos2 = region.pos(1).unwrap();
    assert_eq!(pos1, (1, 4));
    assert_eq!(pos2, (2, 4));
}

#[test]
fn test_regex_captures() {
    let regex = Regex::new("e(l+)|(r+)").unwrap();
    let captures = regex.captures("hello").unwrap();
    assert_eq!(captures.len(), 3);
    assert_eq!(captures.is_empty(), false);
    let pos1 = captures.pos(0).unwrap();
    let pos2 = captures.pos(1).unwrap();
    let pos3 = captures.pos(2);
    assert_eq!(pos1, (1, 4));
    assert_eq!(pos2, (2, 4));
    assert_eq!(pos3, None);
    let str1 = captures.at(0).unwrap();
    let str2 = captures.at(1).unwrap();
    let str3 = captures.at(2);
    assert_eq!(str1, "ell");
    assert_eq!(str2, "ll");
    assert_eq!(str3, None);

}

#[test]
fn test_regex_subcaptures() {
    let regex = Regex::new("e(l+)").unwrap();
    let captures = regex.captures("hello").unwrap();
    let caps = captures.iter().collect::<Vec<_>>();
    assert_eq!(caps[0], Some("ell"));
    assert_eq!(caps[1], Some("ll"));
    assert_eq!(caps.len(), 2);

}

#[test]
fn test_regex_subcapturespos() {
    let regex = Regex::new("e(l+)").unwrap();
    let captures = regex.captures("hello").unwrap();
    let caps = captures.iter_pos().collect::<Vec<_>>();
    assert_eq!(caps[0], Some((1, 4)));
    assert_eq!(caps[1], Some((2, 4)));
    assert_eq!(caps.len(), 2);

}
