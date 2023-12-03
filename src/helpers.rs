pub fn replace1andtell(a: String, to_find: &str, to_replace: &str) -> Option<String> {
    match a.find(to_find) {
        Some(_) => Some(a.replacen(to_find, to_replace, 1)),
        None => None,
    }
}

pub fn str2num(s: &str) -> i64 {
    i64::from_str_radix(s, 10).unwrap_or_default()
}
