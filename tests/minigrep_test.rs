use tmrustplayground::minigrep;

const CONTEXT: &str = "\
Rust:
safe, fas, productive.
Pick three.
Duct tape.";

#[test]
fn case_sensitive() {
    let query = "duct";

    assert_eq!(
        vec!["safe, fas, productive."],
        minigrep::search_case_sensitive(query, CONTEXT)
    );
}

#[test]
fn case_insensitive() {
    let query = "DuCt";

    assert_eq!(
        vec!["safe, fas, productive.", "Duct tape."],
        minigrep::search_case_insensitive(query, CONTEXT)
    );
}