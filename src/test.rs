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
fn test_regex_search_with_region() {
    let mut region = Region::new();
    let regex = Regex::new("e(l+)").unwrap();
    let r = regex.search_with_region("hello", &mut region, OPTION_NONE).unwrap();
    assert!(region.tree().is_none());
    assert_eq!(r, Some(1));
    assert_eq!(region.len(), 2);
    let pos1 = region.pos(0).unwrap();
    let pos2 = region.pos(1).unwrap();
    assert_eq!(pos1, (1, 4));
    assert_eq!(pos2, (2, 4));
}

#[test]
fn test_regex_match_with_region() {
    let mut region = Region::new();
    let regex = Regex::new("he(l+)").unwrap();
    let r = regex.match_with_region("hello", &mut region, OPTION_NONE).unwrap();
    assert!(region.tree().is_none());
    assert_eq!(r, Some(4));
    assert_eq!(region.len(), 2);
    let pos1 = region.pos(0).unwrap();
    let pos2 = region.pos(1).unwrap();
    assert_eq!(pos1, (0, 4));
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

#[test]
fn test_regex_is_match() {
    let regex = Regex::new("he(l+)o").unwrap();
    assert!(regex.is_match("hello"));
    assert!(!regex.is_match("hello 2.0"));
}

#[test]
fn test_regex_find() {
    let regex = Regex::new("he(l+)o").unwrap();
    assert_eq!(regex.find("hey, hello!"), Some((5, 10)));
    assert_eq!(regex.find("hey, honey!"), None);
}

#[test]
fn test_regex_search_with_region_tree() {
    let mut region = Region::new();
    let mut syntax = SYNTAX_RUBY.clone();
    syntax.enable_operators(SYNTAX_OPERATOR_ATMARK_CAPTURE_HISTORY);
    let regex = Regex::new_with_config("(?@a+(?@b+))|(?@c+(?@d+))", RegexConfig {
        syntax: &syntax,
        options: OPTION_NONE
    }).unwrap();
    let r = regex.search_with_region("- cd aaabbb -", &mut region, OPTION_NONE).unwrap();
    assert_eq!(r, Some(2));
    assert_eq!(region.len(), 5);

    let tree = region.tree().unwrap();

    assert_eq!(tree.len(), 1);
    assert_eq!(tree.group(), 0);
    assert_eq!(tree.pos(), (2, 4));

    assert_eq!(tree[0].len(), 1);
    assert_eq!(tree[0].group(), 3);
    assert_eq!(tree[0].pos(), (2, 4));

    assert_eq!(tree[0][0].len(), 0);
    assert_eq!(tree[0][0].group(), 4);
    assert_eq!(tree[0][0].pos(), (3, 4));
}

#[test]
fn test_regex_lens() {
    let regex = Regex::new("(he)(l+)(o)").unwrap();
    assert_eq!(regex.captures_len(), 3);
    assert_eq!(regex.names_len(), 0);
    assert_eq!(regex.capture_histories_len(), 0);
}
