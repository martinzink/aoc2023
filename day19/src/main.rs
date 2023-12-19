use std::collections::HashMap;
use std::ptr::hash;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

#[derive(PartialEq, Debug, Clone)]
pub struct CategoryRange {
    pub start: i64,
    pub end: i64,
}

impl CategoryRange {
    pub(crate) fn new(start: i64, length: i64) -> Self {
        Self {
            start,
            end: start + length,
        }
    }

    pub fn split_at(&self, at: i64, duck: char) -> (Option<CategoryRange>, Option<CategoryRange>) {
        if self.start > at {
            return (None, Some(self.clone()))
        }
        if self.end < at {
            return (Some(self.clone()), None)

        }
        let bellow_correcter = if duck == '<' {1} else {0};
        let upper_correcter = if duck == '>' {1} else {0};
        return (
            Some(CategoryRange::new(self.start, at - self.start - bellow_correcter)),
            Some(CategoryRange::new(at+upper_correcter, self.end - at - upper_correcter)),
        );
    }

    pub fn offset_to(&self, dest: i64) -> Self {
        let length = self.end - self.start;
        Self {
            start: dest,
            end: dest + length,
        }
    }
}

#[derive(Debug)]
struct Part {
    values: HashMap<char, i64>
}

impl Part {
    fn new(part_str: &str) -> Self {
        let mut values: HashMap<char, i64> = HashMap::new();
        let mut chars = part_str.chars();
        chars.next();
        chars.next_back();
        chars.as_str().split(',').for_each(|char_equation|{
            let (char_str, val_str) = char_equation.split_once('=').unwrap();
            values.insert(char_str.chars().next().unwrap(), val_str.parse::<i64>().unwrap());
        });
        Self{values}
    }
}

#[derive(Debug)]
struct Rule {
    category: Option<char>,
    duck: Option<char>,
    target_val: Option<i64>,
    target_workflow_id: String
}

fn test(part: &Part, category: char, duck: char, target_val: i64) -> bool {
    let part_val = part.values.get(&category).unwrap();
    match duck {
        '<' => {return *part_val < target_val; }
        '>' => {return *part_val > target_val; }
        _ => {panic!{"Invalid duck"}}
    }
}

impl Rule {
    fn new(rule_str: &str)->Self {
        if !rule_str.contains(':') {
            return Self{category:None, duck:None, target_val:None, target_workflow_id:rule_str.to_string()};
        }

        let (test_str, target_workflow_id) = rule_str.split_once(':').unwrap();
        let mut chars = test_str.chars();
        let category = chars.next().unwrap();
        let duck = chars.next().unwrap();
        let target_val = chars.as_str().parse::<i64>().unwrap();

        return Self{category:Some(category), duck:Some(duck), target_val:Some(target_val), target_workflow_id:target_workflow_id.to_string()};
    }

    fn test_part(&self, part: &Part) -> Option<String> {
        if self.duck.is_none() {
            return Some(self.target_workflow_id.clone());
        }

        if test(part, self.category.unwrap(), self.duck.unwrap(), self.target_val.unwrap()) {
            return Some(self.target_workflow_id.clone());
        }

        return None;
    }

    fn split_at_part(&self, part_range: &PartRange) -> Vec<(PartRange)> {
        if self.duck.is_none() {
            let mut whole_range = part_range.clone();
            whole_range.curr_workflow = Some(self.target_workflow_id.clone());
            return vec!(whole_range);
        }
        let (below_cat_range, above_cat_range) = part_range.ranges.get(&self.category.unwrap()).unwrap().split_at(self.target_val.unwrap(), self.duck.unwrap());
        let below_range = if below_cat_range.is_some() {
            let mut below_range = part_range.clone();
            let mut cat_range = below_range.ranges.get_mut(&self.category.unwrap()).unwrap();
            *cat_range = below_cat_range.unwrap();
            match self.duck.unwrap() {
                '<' => { below_range.curr_workflow = Some(self.target_workflow_id.clone());}
                '>' => { below_range.curr_workflow = None;}
                _ => {panic!{"Invalid duck"}}
            }
            Some(below_range)
        } else {
            None
        };

        let above_range = if above_cat_range.is_some() {
            let mut above_range = part_range.clone();
            let mut cat_range = above_range.ranges.get_mut(&self.category.unwrap()).unwrap();
            *cat_range = above_cat_range.unwrap();
            match self.duck.unwrap() {
                '>' => { above_range.curr_workflow = Some(self.target_workflow_id.clone());}
                '<' => { above_range.curr_workflow = None;}
                _ => {panic!{"Invalid duck"}}
            }
            Some(above_range)
        } else {
            None
        };
        let mut res = vec!();
        if below_range.is_some() {
            res.push(below_range.unwrap());
        }
        if above_range.is_some() {
            res.push(above_range.unwrap());
        }
        return res;
    }
}
#[derive(Debug)]
struct WorkFlow {
    workflow_id: String,
    rules: Vec<Rule>,
}


impl WorkFlow{
    fn new(work_flow_str: &str) -> Self {
        let (workflow_id, rest) = work_flow_str.split_once('{').unwrap();
        let mut rest_chars = rest.chars();
        rest_chars.next_back();
        let rules = rest_chars.as_str().split(',').map(|rule_str| {Rule::new(rule_str)}).collect::<Vec<Rule>>();
        Self {workflow_id: workflow_id.to_string(), rules}
    }

    fn route(&self, part: &Part) -> String {
        for rule in &self.rules {
            if let Some(target) =  rule.test_part(part) {
                return target;
            }
        }
        panic!("Should be done by now...");
    }
}


fn part1() {
    let (workflows_str, parts_str) = INPUT.split_once("\n\n").unwrap();
    let mut workflows = HashMap::new();
    workflows_str.lines().map(|workflow_line| {WorkFlow::new(workflow_line)}).for_each(|work_flow| {
        workflows.insert(work_flow.workflow_id.clone(), work_flow);
    });
    let parts = parts_str.lines().map(|part_line| {Part::new(part_line)}).collect::<Vec<Part>>();
    let mut accepted = 0;
    let mut rejected = 0;

    let mut res = 0;
    'outer: for part in &parts {
        let mut next_workflow = workflows.get("in").unwrap();
        'inner: loop {
            match next_workflow.route(part).as_str() {
                "A" => {
                    part.values.values().for_each(|v| res+= v);
                    accepted += 1;
                    break 'inner;
                },
                "R" => {
                    rejected += 1;
                    break 'inner;
                },
                x => {
                    next_workflow = workflows.get(&x.to_string()).unwrap();
                }
            }
        }
    }

    println!("{:?}", res);

}
#[derive(Debug, Clone)]

struct PartRange {
    ranges: HashMap<char, CategoryRange>,
    curr_workflow: Option<String>
}

impl PartRange {
    fn split_based_on_workflow(&self, work_flows: &HashMap<String, WorkFlow>) -> Vec<PartRange> {
        let workflow = work_flows.get(&self.curr_workflow.clone().unwrap()).unwrap();
        let mut res = vec!();
        let mut work_queue = vec!(self.clone());

        let mut rule_iter = &mut workflow.rules.iter();
        while !work_queue.is_empty() {
            let rule = rule_iter.next().unwrap();
            let mut new_work_parts = vec!();
            for work_part in &work_queue {
                let new_parts = rule.split_at_part(work_part);
                for new_part in &new_parts {
                    if new_part.curr_workflow.is_some() {
                        res.push(new_part.clone());
                    } else {
                        new_work_parts.push(new_part.clone());
                    }
                }
            }
            work_queue = new_work_parts;
        }
        return res;
    }
}

fn main() {
    let (workflows_str, parts_str) = INPUT.split_once("\n\n").unwrap();
    let mut workflows = HashMap::new();
    workflows_str.lines().map(|workflow_line| {WorkFlow::new(workflow_line)}).for_each(|work_flow| {
        workflows.insert(work_flow.workflow_id.clone(), work_flow);
    });
    let mut hashmap = HashMap::new();
    hashmap.insert('x', CategoryRange{start:1, end:4000});
    hashmap.insert('m', CategoryRange{start:1, end:4000});
    hashmap.insert('a', CategoryRange{start:1, end:4000});
    hashmap.insert('s', CategoryRange{start:1, end:4000});
    let mut part_ranges = vec!(PartRange{ranges:hashmap, curr_workflow:Some("in".to_string())});
    let mut accepteds = vec!();
    while !part_ranges.is_empty() {
        let part_range= part_ranges.pop().unwrap();
        let curr_workflow = part_range.curr_workflow.clone().unwrap();;
        if curr_workflow.as_str() == "A" {
            accepteds.push(part_range);
        } else if curr_workflow.as_str() == "R" {

        } else {
            part_ranges.extend(part_range.split_based_on_workflow(&workflows));
        }
    }
    let mut res = 0;
    for accepted in accepteds {
        let mut accepted_res = 1;
        accepted.ranges.values().for_each(|a| {accepted_res *= (a.end-a.start+1)});
        res += accepted_res;
    }
    println!("{:?}",res);
}
