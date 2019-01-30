use regex::Regex;

pub fn check_new_user(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?m)cphone=[0-9]{1,5}+&phone=[0-9-\-]{6,14}+").unwrap();
    }
    RE.is_match(text)
}