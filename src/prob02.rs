use crate::helpers::{replace_if_1, str2num};
use std::cmp::max;

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
                        .map(|v| match replace_if_1(v.to_string(), "green", "") {
                            Some(w) => str2num(&w.trim()) <= 13,
                            None => match replace_if_1(v.to_string(), "red", "") {
                                Some(w) => str2num(&w.trim()) <= 12,
                                None => match replace_if_1(v.to_string(), "blue", "") {
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

#[derive(Debug)]
struct Game {
    red: i64,
    blue: i64,
    green: i64,
}

impl Game {
    fn power(&self) -> i64 {
        self.red * self.blue * self.green
    }
    fn swallow(&self, x: Game) -> Game {
        Game {
            red: max(x.red, self.red),
            green: max(x.green, self.green),
            blue: max(x.blue, self.blue),
        }
    }
}

pub fn b(input: String) -> i64 {
    input
        .split('\n')
        .map(|s| {
            s.split(":")
                .nth(1)
                .unwrap_or_default()
                .split(";")
                .map(|u| {
                    u.split(",")
                        .map(|v| match replace_if_1(v.to_string(), "green", "") {
                            Some(w) => Game {
                                red: 0,
                                blue: 0,
                                green: str2num(&w.trim()),
                            },
                            None => match replace_if_1(v.to_string(), "red", "") {
                                Some(w) => Game {
                                    red: str2num(&w.trim()),
                                    blue: 0,
                                    green: 0,
                                },
                                None => match replace_if_1(v.to_string(), "blue", "") {
                                    Some(w) => Game {
                                        red: 0,
                                        blue: str2num(&w.trim()),
                                        green: 0,
                                    },
                                    None => Game {
                                        red: 0,
                                        blue: 0,
                                        green: 0,
                                    },
                                },
                            },
                        })
                        .reduce(|a, b| a.swallow(b))
                        .unwrap_or(Game {
                            red: 0,
                            blue: 0,
                            green: 0,
                        })
                })
                .reduce(|a, b| a.swallow(b))
                .unwrap_or(Game {
                    red: 0,
                    blue: 0,
                    green: 0,
                })
                .power()
        })
        .reduce(|a, b| a + b)
        .unwrap_or(0)
}
