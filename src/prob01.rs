pub fn a(input: String) -> i64 {
    input
        .split('\n')
        .map(|s| {
            let mut a = s.split(char::is_alphabetic).filter(|s| s.len() > 0);
            let first = i64::from_str_radix(a.nth(0).unwrap_or("0"), 10).unwrap_or_default();
            let last = i64::from_str_radix(a.nth_back(0).unwrap_or_default(), 10).unwrap_or(first);
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
            let mut res = String::from(s);
            for (w, i) in REPL {
                res = res.replace(w, i);
            }
            return res;
        })
        .reduce(|a, b| format!("{a}\n{b}"))
        .unwrap_or_default();
    a(newinput)
}
