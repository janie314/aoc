use crate::helpers::str2num;

pub fn a(input: String) -> i64 {
    input
        .split('\n')
        .map(|s| {
            let mut a = s.split(char::is_alphabetic).filter(|s| s.len() > 0);
            let first = str2num(a.nth(0).unwrap_or("0"));
            let last = match a.nth_back(0) {
                Some(k) => str2num(k),
                None => first,
            };
            let n = first.checked_ilog10().unwrap_or_default();
            let first_digit = (first - (first % 10_i64.pow(n))) / 10_i64.pow(n);
            let last_digit = last % 10;
            10 * first_digit + last_digit
        })
        .reduce(|a, b| a + b)
        .unwrap_or_default()
}

const REPL: [(&str, &str); 17] = [
    ("oneight", "o18t"),
    ("twone", "t21e"),
    ("threeight", "t38t"),
    ("fiveight", "f58t"),
    ("sevenine", "s79e"),
    ("eightwo", "e82o"),
    ("eighthree", "e83e"),
    ("nineight", "n98t"),
    ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9"),
];

pub fn b(input: String) -> i64 {
    let newinput = input
        .split('\n')
        .map(|s| {
            REPL.into_iter()
                .fold(s.to_string(), |a, (w, i)| a.replace(w, i))
        })
        .reduce(|a, b| format!("{a}\n{b}"))
        .unwrap_or_default();
    a(newinput)
}
