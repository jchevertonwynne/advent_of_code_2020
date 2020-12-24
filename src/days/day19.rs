use regex::Regex;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;
use std::time::{Duration, Instant};

const INPUT: &str = include_str!("../../files/19.txt");
const INPUT2: &str = include_str!("../../files/19_2.txt");

#[derive(Debug, PartialEq)]
enum Matcher {
    Literal(char),
    Several(Vec<MatcherParam>),
}

#[derive(Debug, PartialEq)]
enum MatcherParam {
    Recurse,
    Other(Rc<Grammar>),
}

#[derive(Debug, PartialEq)]
struct Grammar {
    options: Vec<Matcher>,
}

trait BuildOptions {
    fn regex(&self, anchor: bool, limit: usize) -> String;
}

impl BuildOptions for Grammar {
    fn regex(&self, anchor: bool, limit: usize) -> String {
        let mut res = String::new();

        if anchor {
            res.push('^');
        }
        res.push('(');
        for option in &self.options {
            let mut curr = String::new();
            let mut legal = true;
            match option {
                Matcher::Literal(c) => curr.push(*c),
                Matcher::Several(params) => {
                    for param in params {
                        match param {
                            MatcherParam::Recurse => {
                                if limit > 0 {
                                    curr.push_str(self.regex(false, limit - 1).as_str());
                                } else {
                                    legal = false;
                                    break;
                                }
                            }
                            MatcherParam::Other(o) => {
                                curr.push_str(o.regex(false, limit).as_str());
                            }
                        }
                    }
                }
            }

            if legal {
                res.push_str(curr.as_str());
                res.push('|');
            }
        }

        if let Some('|') = res.chars().last() {
            res.pop();
        }

        if res[1..].chars().all(|c| c == 'a' || c == 'b') {
            res.remove(0);
        } else {
            res.push(')');
        }

        if anchor {
            res.push('$');
        }

        res
    }
}

fn load_input(input: &str) -> (Rc<Grammar>, Vec<&str>) {
    let (samples, ruleset): (Vec<&str>, Vec<&str>) = input
        .lines()
        .filter(|l| !l.is_empty())
        .partition(|line| line.starts_with('a') || line.starts_with('b'));

    let mut grammars = HashMap::new();
    let mut to_do = HashMap::new();

    for line in ruleset {
        let colon = line.find(':').expect("exist");
        let rule_num = line[..colon].parse::<usize>().expect("pls");

        let entry = grammars
            .entry(rule_num)
            .or_insert(Grammar { options: vec![] });
        let to_apply = to_do.entry(rule_num).or_insert(Vec::new());

        let terms = line[colon + 2..].split(' ');

        let mut curr = Vec::new();

        for term in terms {
            if term.starts_with('\"') {
                entry
                    .options
                    .push(Matcher::Literal(term.chars().nth(1).expect("exist")));
            } else if term == "|" {
                to_apply.push(curr.clone());
                curr.clear();
            } else {
                curr.push(term.parse::<usize>().expect("pls"));
            }
        }
        if !curr.is_empty() {
            to_apply.push(curr.clone());
        }
    }

    let mut done: HashMap<usize, Rc<Grammar>> = HashMap::new();

    while !to_do.is_empty() {
        let (&ind, _) = to_do
            .iter()
            .find(|(ind, grammar)| {
                grammar
                    .iter()
                    .all(|g| g.iter().all(|i| done.contains_key(&i) || **ind == *i))
            })
            .expect("should be a thing");

        let opts = to_do.remove(&ind).expect("index should be removeable");
        let mut grammar = grammars.remove(&ind).expect("index should be removeable");
        for opt in opts {
            let item = opt
                .iter()
                .map(|o| match done.get(o) {
                    None => MatcherParam::Recurse,
                    Some(v) => MatcherParam::Other(Rc::clone(v)),
                })
                .collect::<Vec<_>>();
            grammar.options.push(Matcher::Several(item));
        }

        done.insert(ind, Rc::new(grammar));
    }

    (done.get(&0).expect("pls").clone(), samples)
}

fn solver(grammar: &Grammar, to_check: &[&str]) -> usize {
    let r = grammar.regex(true, 4);
    // println!("{}", r);
    let matcher = Regex::new(&r).expect("be valid pls");
    to_check.iter().filter(|t| matcher.is_match(t)).count()
}

pub fn run() -> (String, String, Duration) {
    let start = Instant::now();
    let (rules, to_check) = load_input(INPUT);
    let p1 = solver(&*rules, &to_check);
    let (rules, to_check) = load_input(INPUT2);
    let p2 = solver(&*rules, &to_check);

    (p1.to_string(), p2.to_string(), start.elapsed())
}

#[cfg(test)]
mod tests {
    use crate::days::day19::{load_input, BuildOptions, Grammar, Matcher, MatcherParam};
    use std::rc::Rc;

    #[test]
    fn test_parse() {
        let s = "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb";
        let (grammar, _) = load_input(s);

        let r5 = Rc::new(Grammar {
            options: vec![Matcher::Literal('b')],
        });
        let r4 = Rc::new(Grammar {
            options: vec![Matcher::Literal('a')],
        });
        let r3 = Rc::new(Grammar {
            options: vec![
                Matcher::Several(vec![
                    MatcherParam::Other(Rc::clone(&r4)),
                    MatcherParam::Other(Rc::clone(&r5)),
                ]),
                Matcher::Several(vec![
                    MatcherParam::Other(Rc::clone(&r5)),
                    MatcherParam::Other(Rc::clone(&r4)),
                ]),
            ],
        });
        let r2 = Rc::new(Grammar {
            options: vec![
                Matcher::Several(vec![
                    MatcherParam::Other(Rc::clone(&r4)),
                    MatcherParam::Other(Rc::clone(&r4)),
                ]),
                Matcher::Several(vec![
                    MatcherParam::Other(Rc::clone(&r5)),
                    MatcherParam::Other(Rc::clone(&r5)),
                ]),
            ],
        });
        let r1 = Rc::new(Grammar {
            options: vec![
                Matcher::Several(vec![
                    MatcherParam::Other(Rc::clone(&r2)),
                    MatcherParam::Other(Rc::clone(&r3)),
                ]),
                Matcher::Several(vec![
                    MatcherParam::Other(Rc::clone(&r3)),
                    MatcherParam::Other(Rc::clone(&r2)),
                ]),
            ],
        });
        let expected = Grammar {
            options: vec![Matcher::Several(vec![
                MatcherParam::Other(Rc::clone(&r4)),
                MatcherParam::Other(Rc::clone(&r1)),
                MatcherParam::Other(Rc::clone(&r5)),
            ])],
        };

        assert_eq!(Rc::new(expected), grammar);
    }

    #[test]
    fn test_execution() {
        let r5 = Rc::new(Grammar {
            options: vec![Matcher::Literal('b')],
        });
        let r4 = Rc::new(Grammar {
            options: vec![Matcher::Literal('a')],
        });
        let r3 = Rc::new(Grammar {
            options: vec![
                Matcher::Several(vec![
                    MatcherParam::Other(Rc::clone(&r5)),
                    MatcherParam::Other(Rc::clone(&r4)),
                ]),
                Matcher::Several(vec![
                    MatcherParam::Other(Rc::clone(&r4)),
                    MatcherParam::Other(Rc::clone(&r5)),
                ]),
            ],
        });
        let r2 = Rc::new(Grammar {
            options: vec![
                Matcher::Several(vec![
                    MatcherParam::Other(Rc::clone(&r4)),
                    MatcherParam::Other(Rc::clone(&r4)),
                ]),
                Matcher::Several(vec![
                    MatcherParam::Other(Rc::clone(&r5)),
                    MatcherParam::Other(Rc::clone(&r5)),
                ]),
            ],
        });
        let r1 = Rc::new(Grammar {
            options: vec![
                Matcher::Several(vec![
                    MatcherParam::Other(Rc::clone(&r2)),
                    MatcherParam::Other(Rc::clone(&r3)),
                ]),
                Matcher::Several(vec![
                    MatcherParam::Other(Rc::clone(&r3)),
                    MatcherParam::Other(Rc::clone(&r2)),
                ]),
            ],
        });
        let g = Grammar {
            options: vec![Matcher::Several(vec![
                MatcherParam::Other(Rc::clone(&r4)),
                MatcherParam::Other(Rc::clone(&r1)),
                MatcherParam::Other(Rc::clone(&r5)),
            ])],
        };
        println!("{}", g.regex(true, 5));
    }

    #[test]
    fn test_recurse() {
        let s = "0: 1 0 2 | 1 2
1: \"a\"
2: \"b\"";
        let (grammar, _) = load_input(s);
        println!("{}", grammar.regex(true, 0));
        println!("{}", grammar.regex(true, 1));
        println!("{}", grammar.regex(true, 10));
    }
}
