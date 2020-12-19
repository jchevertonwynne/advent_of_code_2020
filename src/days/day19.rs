use regex::Regex;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::{Duration, Instant};

const INPUT: &str = include_str!("../../files/19.txt");
const INPUT2: &str = include_str!("../../files/19_2.txt");

#[derive(Debug)]
enum Matcher {
    Literal(char),
    Several(Vec<Rc<Grammar>>),
}

#[derive(Debug)]
struct Grammar {
    options: Vec<Matcher>,
}

trait BuildOptions {
    fn build(&self) -> Vec<String>;
    fn regex(&self, anchor: bool) -> String;
}

impl BuildOptions for Grammar {
    fn build(&self) -> Vec<String> {
        let mut res = Vec::new();

        for m in &self.options {
            res.extend(m.build());
        }

        res
    }

    fn regex(&self, anchor: bool) -> String {
        let mut res = String::new();

        if anchor {
            res.push('^');
        }
        res.push('(');
        for option in &self.options {
            res.push_str(option.regex(false).as_str());
            res.push('|');
        }
        res.pop();
        res.push(')');
        if anchor {
            res.push('$');
        }

        res
    }
}

impl BuildOptions for Matcher {
    fn build(&self) -> Vec<String> {
        match self {
            Matcher::Literal(c) => vec![c.to_string()],
            Matcher::Several(opts) => {
                let parts = opts.iter().map(|o| o.build()).collect::<Vec<_>>();
                let mut buildable = vec![String::new()];
                for part in parts {
                    buildable = buildable
                        .into_iter()
                        .flat_map(|b| (0..part.len()).map(move |_| b.clone()))
                        .collect();
                    for (b, next) in buildable.iter_mut().zip(part.iter().cycle()) {
                        b.push_str(&next);
                    }
                }
                buildable
            }
        }
    }

    fn regex(&self, anchor: bool) -> String {
        match self {
            Matcher::Literal(c) => c.to_string(),
            Matcher::Several(opts) => {
                let mut res = String::new();
                for opt in opts {
                    res.push_str(opt.regex(anchor).as_str());
                }
                res
            }
        }
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
            .find(|(_, grammar)| {
                grammar
                    .iter()
                    .all(|g| g.iter().all(|i| done.contains_key(&i)))
            })
            .expect("should be a thing");

        let opts = to_do.remove(&ind).expect("index should be removeable");
        let mut grammar = grammars.remove(&ind).expect("index should be removeable");
        for opt in opts {
            let item = opt
                .iter()
                .map(|o| done.get(o).expect("must exist").clone())
                .collect::<Vec<_>>();
            grammar.options.push(Matcher::Several(item));
        }

        done.insert(ind, Rc::new(grammar));
    }

    (done.get(&0).expect("pls").clone(), samples)
}

fn solver(grammar: &Grammar, to_check: &[&str]) -> usize {
    let r = grammar.regex(true);
    let matcher = Regex::new(&r).expect("be valid pls");
    to_check.iter().filter(|t| matcher.is_match(t)).count()
}

pub fn run() -> (usize, usize, Duration) {
    let start = Instant::now();
    let (rules, to_check) = load_input(INPUT);
    let p1 = solver(&*rules, &to_check);
    let (rules, to_check) = load_input(INPUT2);
    let p2 = solver(&*rules, &to_check);

    (p1, p2, start.elapsed())
}

#[cfg(test)]
mod tests {
    use crate::days::day19::{load_input, BuildOptions, Grammar, Matcher};
    use std::collections::HashSet;
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
                Matcher::Several(vec![Rc::clone(&r5), Rc::clone(&r4)]),
                Matcher::Several(vec![Rc::clone(&r4), Rc::clone(&r5)]),
            ],
        });
        let r2 = Rc::new(Grammar {
            options: vec![
                Matcher::Several(vec![Rc::clone(&r4), Rc::clone(&r4)]),
                Matcher::Several(vec![Rc::clone(&r5), Rc::clone(&r5)]),
            ],
        });
        let r1 = Rc::new(Grammar {
            options: vec![
                Matcher::Several(vec![Rc::clone(&r2), Rc::clone(&r3)]),
                Matcher::Several(vec![Rc::clone(&r3), Rc::clone(&r2)]),
            ],
        });
        let expected = Grammar {
            options: vec![Matcher::Several(vec![
                Rc::clone(&r4),
                Rc::clone(&r1),
                Rc::clone(&r5),
            ])],
        };

        assert_eq!(
            expected.build().iter().collect::<HashSet<_>>(),
            grammar.build().iter().collect::<HashSet<_>>()
        );
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
                Matcher::Several(vec![Rc::clone(&r5), Rc::clone(&r4)]),
                Matcher::Several(vec![Rc::clone(&r4), Rc::clone(&r5)]),
            ],
        });
        let r2 = Rc::new(Grammar {
            options: vec![
                Matcher::Several(vec![Rc::clone(&r4), Rc::clone(&r4)]),
                Matcher::Several(vec![Rc::clone(&r5), Rc::clone(&r5)]),
            ],
        });
        let r1 = Rc::new(Grammar {
            options: vec![
                Matcher::Several(vec![Rc::clone(&r2), Rc::clone(&r3)]),
                Matcher::Several(vec![Rc::clone(&r3), Rc::clone(&r2)]),
            ],
        });
        let g = Grammar {
            options: vec![Matcher::Several(vec![
                Rc::clone(&r4),
                Rc::clone(&r1),
                Rc::clone(&r5),
            ])],
        };
        let opts = g.build();
        println!("{:?}", opts);
        println!("{}", g.regex());
    }
}
