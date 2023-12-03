fn replace1andtell(a: String, to_find: &str, to_replace: &str) -> Option<String> {
    match a.find(to_find) {
        Some(_) => Some(a.replacen(to_find, to_replace, 1)),
        None => None,
    }
}

fn str2num(s: &str) -> i64 {
    i64::from_str_radix(s, 10).unwrap_or_default()
}

pub fn a(input: String) -> i64 {
    input
        .split('\n')
        .map(|s| {
            let game_ind = str2num(&s.split(":").nth(0).unwrap_or("0").replace("Game ", ""));
            if s.split(":")
                .nth(1)
                .unwrap_or_default()
                .split(";")
                .map(|u| {
                    u.split(",")
                        .map(|v| match replace1andtell(v.to_string(), "green", "") {
                            Some(w) => str2num(&w.trim()) <= 13,
                            None => match replace1andtell(v.to_string(), "red", "") {
                                Some(w) => str2num(&w.trim()) <= 12,
                                None => match replace1andtell(v.to_string(), "blue", "") {
                                    Some(w) => str2num(&w.trim()) <= 14,
                                    None => true,
                                },
                            },
                        })
                        .reduce(|a, b| a && b)
                        .unwrap_or(true)
                })
                .reduce(|a, b| a && b)
                .unwrap_or(true)
            {
                game_ind
            } else {
                0
            }
        })
        .reduce(|a, b| a + b)
        .unwrap_or_default()
}
