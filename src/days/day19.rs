use fxhash::FxBuildHasher;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;
use std::time::{Duration, Instant};

const INPUT: &str = include_str!("../../files/19.txt");
const INPUT2: &str = include_str!("../../files/19_2.txt");

#[derive(Debug, PartialEq)]
enum Matcher {
    Literal(char),
    Sequence(Vec<MatcherParam>),
}

#[derive(Debug, PartialEq)]
enum MatcherParam {
    Recurse,
    Grammar(Rc<Grammar>),
}

#[derive(Debug, PartialEq)]
struct Grammar {
    matchers: Vec<Matcher>,
}

impl Grammar {
    fn matches(&self, s: &str) -> bool {
        self.part_match(s).iter().any(|s| s.is_empty())
    }

    fn part_match<'a, 'b>(&'a self, s: &'b str) -> Vec<&'b str> {
        let mut res = Vec::new();

        if s.is_empty() {
            return res;
        }

        'matcher_loop: for matcher in &self.matchers {
            match matcher {
                Matcher::Literal(c) => {
                    if s.chars().next().unwrap() == *c {
                        res.push(&s[1..]);
                    }
                }
                Matcher::Sequence(params) => {
                    let mut potential = vec![s];
                    for param in params {
                        let mut new_pot = Vec::new();
                        match param {
                            MatcherParam::Recurse => {
                                new_pot.extend(potential.iter().flat_map(|p| self.part_match(p)));
                            }
                            MatcherParam::Grammar(g) => {
                                new_pot.extend(potential.iter().flat_map(|p| g.part_match(p)));
                            }
                        };
                        if potential.is_empty() {
                            continue 'matcher_loop;
                        }
                        potential = new_pot;
                    }

                    res.extend(potential);
                }
            }
        }

        res
    }
}

fn load_input(input: &str) -> (Rc<Grammar>, Vec<&str>) {
    let (samples, ruleset): (Vec<&str>, Vec<&str>) = input
        .lines()
        .filter(|l| !l.is_empty())
        .partition(|line| line.starts_with('a') || line.starts_with('b'));

    let mut grammars = HashMap::with_capacity_and_hasher(ruleset.len(), FxBuildHasher::default());
    let mut to_do = HashMap::with_capacity_and_hasher(ruleset.len(), FxBuildHasher::default());

    for line in ruleset {
        let colon = line.find(':').expect("exist");
        let rule_num = line[..colon].parse::<usize>().expect("pls");

        let entry = grammars
            .entry(rule_num)
            .or_insert(Grammar { matchers: vec![] });
        let to_apply = to_do.entry(rule_num).or_insert_with(Vec::new);

        let terms = line[colon + 2..].split(' ');

        let mut curr = Vec::new();

        for term in terms {
            if term.starts_with('\"') {
                entry
                    .matchers
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

    let mut done: HashMap<usize, Rc<Grammar>, FxBuildHasher> =
        HashMap::with_capacity_and_hasher(to_do.len(), FxBuildHasher::default());

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
                    Some(v) => MatcherParam::Grammar(Rc::clone(v)),
                })
                .collect::<Vec<_>>();
            grammar.matchers.push(Matcher::Sequence(item));
        }

        done.insert(ind, Rc::new(grammar));
    }

    (done.get(&0).expect("pls").clone(), samples)
}

fn solver(rules: Rc<Grammar>, to_check: &[&str]) -> usize {
    to_check.iter().filter(|t| rules.matches(t)).count()
}

pub fn run() -> (String, String, Duration) {
    let start = Instant::now();
    let (rules, to_check) = load_input(INPUT);
    let p1 = solver(Rc::clone(&rules), &to_check);
    let (rules, to_check) = load_input(INPUT2);
    let p2 = solver(Rc::clone(&rules), &to_check);

    (p1.to_string(), p2.to_string(), start.elapsed())
}

#[cfg(test)]
mod tests {
    use crate::days::day19::{load_input, Grammar, Matcher, MatcherParam};
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
        let expected = make_grammar();
        assert_eq!(expected, grammar);
    }

    #[test]
    fn test_valid_check() {
        let g = make_grammar();
        assert_eq!(g.matches("hello"), false);
        assert_eq!(g.matches("aaaabb"), true);
        assert_eq!(g.matches("aaabab"), true);
        assert_eq!(g.matches("abbabb"), true);
        assert_eq!(g.matches("abbbab"), true);
        assert_eq!(g.matches("aabaab"), true);
        assert_eq!(g.matches("aabbbb"), true);
        assert_eq!(g.matches("abaaab"), true);
        assert_eq!(g.matches("ababbb"), true);
    }

    fn make_grammar() -> Rc<Grammar> {
        let r5 = Rc::new(Grammar {
            matchers: vec![Matcher::Literal('b')],
        });
        let r4 = Rc::new(Grammar {
            matchers: vec![Matcher::Literal('a')],
        });
        let r3 = Rc::new(Grammar {
            matchers: vec![
                Matcher::Sequence(vec![
                    MatcherParam::Grammar(Rc::clone(&r4)),
                    MatcherParam::Grammar(Rc::clone(&r5)),
                ]),
                Matcher::Sequence(vec![
                    MatcherParam::Grammar(Rc::clone(&r5)),
                    MatcherParam::Grammar(Rc::clone(&r4)),
                ]),
            ],
        });
        let r2 = Rc::new(Grammar {
            matchers: vec![
                Matcher::Sequence(vec![
                    MatcherParam::Grammar(Rc::clone(&r4)),
                    MatcherParam::Grammar(Rc::clone(&r4)),
                ]),
                Matcher::Sequence(vec![
                    MatcherParam::Grammar(Rc::clone(&r5)),
                    MatcherParam::Grammar(Rc::clone(&r5)),
                ]),
            ],
        });
        let r1 = Rc::new(Grammar {
            matchers: vec![
                Matcher::Sequence(vec![
                    MatcherParam::Grammar(Rc::clone(&r2)),
                    MatcherParam::Grammar(Rc::clone(&r3)),
                ]),
                Matcher::Sequence(vec![
                    MatcherParam::Grammar(Rc::clone(&r3)),
                    MatcherParam::Grammar(Rc::clone(&r2)),
                ]),
            ],
        });
        Rc::new(Grammar {
            matchers: vec![Matcher::Sequence(vec![
                MatcherParam::Grammar(Rc::clone(&r4)),
                MatcherParam::Grammar(Rc::clone(&r1)),
                MatcherParam::Grammar(Rc::clone(&r5)),
            ])],
        })
    }
}
