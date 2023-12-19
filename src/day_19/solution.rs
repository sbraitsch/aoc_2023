use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::HashMap;
use std::ops::Range;
use std::str::FromStr;
use crate::utils;
use std::time::Instant;

struct Rule {
    prop: char,
    cmp: Ordering,
    val: usize,
    res: String
}

impl Rule {
    fn reduce_combinations(&self, elem: &mut CombinationElement) -> (String, CombinationElement, CombinationElement) {
        let mut pass_further = elem.clone();
        match self.prop {
            'x' =>
                if self.cmp == Less {
                    elem.x = elem.x.start..self.val.min(elem.x.end);
                    pass_further.x = self.val.max(pass_further.x.start)..pass_further.x.end;
                } else if self.cmp == Greater {
                    elem.x = self.val.max(elem.x.start)+1..elem.x.end;
                    pass_further.x = pass_further.x.start..self.val.min(pass_further.x.end)+1;
                }
            'm' =>
                if self.cmp == Less {
                    elem.m = elem.m.start..self.val.min(elem.m.end);
                    pass_further.m = self.val.max(pass_further.m.start)..pass_further.m.end;
                } else if self.cmp == Greater {
                    elem.m = self.val.max(elem.m.start)+1..elem.m.end;
                    pass_further.m = pass_further.m.start..self.val.min(pass_further.m.end)+1;
                }
            'a' => if self.cmp == Less {
                    elem.a = elem.a.start..self.val.min(elem.a.end);
                    pass_further.a = self.val.max(pass_further.a.start)..pass_further.a.end;
                } else if self.cmp == Greater {
                    elem.a = self.val.max(elem.a.start)+1..elem.a.end;
                    pass_further.a = pass_further.a.start..self.val.min(pass_further.a.end)+1;
                }
            's' => if self.cmp == Less {
                    elem.s = elem.s.start..self.val.min(elem.s.end);
                    pass_further.s = self.val.max(pass_further.s.start)..pass_further.s.end;
                } else if self.cmp == Greater {
                    elem.s = self.val.max(elem.s.start)+1..elem.s.end;
                    pass_further.s = pass_further.s.start..self.val.min(pass_further.s.end)+1;
                }
            _ => {}
        }
        (self.res.clone(), elem.clone(), pass_further)
    }
}

#[derive(Clone, Debug)]
struct CombinationElement {
    x: Range<usize>,
    m: Range<usize>,
    a: Range<usize>,
    s: Range<usize>
}

impl CombinationElement {
    fn count(self) -> usize {
        self.x.count() * self.m.count() * self.a.count() * self.s.count()
    }
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return if s.contains(':') {
            let cmp;
            let (k, r) = if s.contains('>') {
                cmp = Greater;
                s.split_once('>').unwrap()
            } else {
                cmp = Less;
                s.split_once('<').unwrap()
            };
            let (num, to) = r.split_once(':').unwrap();
            Ok(Rule {
                prop: k.chars().next().unwrap(),
                cmp,
                val: num.parse::<usize>().unwrap(),
                res: String::from(to)
            })
        } else { Ok(Rule { prop: ' ', cmp: Equal, val: 0, res: s.to_string() }) }
    }
}
#[derive(Default, Debug)]
struct Element {
    x: usize,
    m: usize,
    a: usize,
    s: usize
}

impl Element {

    fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }

    fn apply_workflow(&self, rules: &Vec<Rule>) -> String {
        for rule in rules {
            let comp = match rule.prop {
                'x' => self.x,
                'm' => self.m,
                'a' => self.a,
                's' => self.s,
                ' ' => return rule.res.clone(),
                _ => return String::new(),
            };
            if comp.cmp(&rule.val) == rule.cmp { return rule.res.clone() }
        };
        String::new()
    }
}

pub fn solve() {
    let lines = utils::file_to_lines("19");
    let mut workflows: HashMap<String, Vec<Rule>> = HashMap::new();
    let split_lines: Vec<&[String]> = lines.split(|l| l.is_empty()).collect();
    split_lines[0].iter().for_each(|line| {
        let (key, rule_string) = line.split_once('{').unwrap();
        let rules = rule_string.strip_suffix('}').unwrap().split(',').collect::<Vec<&str>>();
        workflows.insert(key.to_string(), rules.iter().map(|rule| {
            rule.parse().unwrap()
        }).collect());
    });

    let elements: Vec<Element> = split_lines[1].iter().map(|line| {
        let mut e = Element::default();
        line[1..line.len() - 1].split(',').for_each(|prop| {
            let (c, n) = prop.split_once('=').unwrap();
            let num = n.parse::<usize>().unwrap();
            match c { "x" => e.x = num, "m" => e.m = num, "a" => e.a = num, "s" => e.s = num, _ => {} }
        });
        e
    }).collect::<Vec<Element>>();
    let mut time = Instant::now();
    println!("Part 1: {:?} in {:?}", part_one(&elements, &workflows), time.elapsed());
    time = Instant::now();
    println!("Part 2: {:?} in {:?}", part_two(&workflows), time.elapsed());
}

fn part_one(elements: &Vec<Element>, workflows: &HashMap<String, Vec<Rule>>) -> usize {
    let start = workflows.get("in").unwrap();
    elements.iter().map(|e| {
        let mut next = e.apply_workflow(start);
        while next != "A" && next != "R" {
            let wf = workflows.get(next.as_str()).unwrap();
            next = e.apply_workflow(wf);
        }
        return match next.as_str() { "A" => e.sum(), _ => 0 }
    }).sum()
}

fn part_two(workflows: &HashMap<String, Vec<Rule>>) -> usize {
    let comb = CombinationElement { x: 1..4001, m: 1..4001, a: 1..4001, s: 1..4001};
    let start = workflows.get("in").unwrap();
    reduce_cmb(start, comb, workflows)
}

fn reduce_cmb(workflow: &Vec<Rule>, mut comb: CombinationElement, workflows: &HashMap<String, Vec<Rule>>) -> usize {
    workflow.iter().map(|rule|{
        let (res, rec_comb, pass_comb) = rule.reduce_combinations(&mut comb);
        comb = pass_comb;
        match res.as_str() {
            "R" => 0,
            "A" => rec_comb.count(),
            _ => {
                let wf = workflows.get(&res).unwrap();
                reduce_cmb(wf, rec_comb, workflows)
            }
        }
    }).sum()
}