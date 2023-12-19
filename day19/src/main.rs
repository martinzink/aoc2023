use std::collections::HashMap;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

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
    rule_string: String,
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
        return Self{rule_string: rule_str.to_string()};

    }

    fn test_part(&self, part: &Part) -> Option<String> {
        if !self.rule_string.contains(':') {
            return Some(self.rule_string.clone());
        }

        let (test_str, target_workflow_id) = self.rule_string.split_once(':').unwrap();
        let mut chars = test_str.chars();
        let category = chars.next().unwrap();
        let duck = chars.next().unwrap();
        let target_val = chars.as_str().parse::<i64>().unwrap();

        if test(part, category, duck, target_val) {
            return Some(target_workflow_id.to_string());
        }

        return None;
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


fn main() {
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
