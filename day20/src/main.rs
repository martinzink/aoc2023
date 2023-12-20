use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use petgraph::dot::{Config, Dot};
use petgraph::Graph;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum SignalState {
    Low,
    High
}

trait Module : Debug {
    fn receive(&mut self, signal_state: SignalState, sender: &String) -> Option<SignalState>;
    fn wire_input(&mut self, input_module: &String);

    fn get_full_name(&self, partial_name: &str) -> String;
}

#[derive(Debug)]
struct FlipFlop {
    state: bool
}

impl FlipFlop {
    fn new() -> Self {
        Self{state: false}
    }
}

impl Module for FlipFlop {
    fn receive(&mut self, signal_state: SignalState, _sender: &String) -> Option<SignalState> {
        match signal_state {
            SignalState::Low => {
                self.state = !self.state;
                match self.state {
                    true => {Some(SignalState::High)}
                    false => {Some(SignalState::Low)}
                }
            }
            SignalState::High => {None}
        }
    }

    fn wire_input(&mut self, _input_module: &String) {}

    fn get_full_name(&self, partial_name: &str) -> String {
        std::format!("%{}", partial_name)
    }
}

#[derive(Debug)]
struct Conjunction {
    states: HashMap<String, SignalState>
}

impl Conjunction {
    fn new() -> Self {Self{states:HashMap::new()}}
}

impl Module for Conjunction {
    fn receive(&mut self, signal_state: SignalState, sender: &String) -> Option<SignalState> {
        self.states.insert(sender.clone(), signal_state);
        if self.states.values().all(|state|{*state==SignalState::High }) {
            Some(SignalState::Low)
        } else {
            Some(SignalState::High)
        }
    }

    fn wire_input(&mut self, _input_module: &String) {
        self.states.insert(_input_module.clone(), SignalState::Low);
    }

    fn get_full_name(&self, partial_name: &str) -> String {
        std::format!("&{}", partial_name)
    }

}

#[derive(Debug)]
struct BroadCaster {

}

impl BroadCaster {
    fn new() -> Self {Self{}}
}

impl Module for BroadCaster {
    fn receive(&mut self, signal_state: SignalState, _sender: &String) -> Option<SignalState> {
        Some(signal_state)
    }

    fn wire_input(&mut self, _input_module: &String) {}
    fn get_full_name(&self, partial_name: &str) -> String {
        std::format!("{}", partial_name)
    }
}

const EXAMPLE: &str = include_str!("example.txt");
const EXAMPLE_1: &str = include_str!("example1.txt");
const INPUT: &str = include_str!("input.txt");

fn export_to_png(graph: &Graph<String, ()>, filename: &str) {
    let dot_data = format!("{:?}", Dot::with_config(graph, &[Config::EdgeNoLabel]));
    let mut file = File::create(std::format!("{}.dot", filename)).expect("Error creating DOT file");
    file.write_all(dot_data.as_bytes()).expect("Error writing to DOT file");
    Command::new("sh")
        .arg("-c")
        .arg(std::format!("dot -Tpng {}.dot -o {}.png", filename, filename))
        .output()
        .expect("failed to execute process");
}

fn get_full_name(stripped_name: &str, modules: &HashMap<String, Box<dyn Module>>) -> String{
    if let Some(output_module) = modules.get(stripped_name) {
        return output_module.get_full_name(stripped_name);
    }
    return stripped_name.to_string();
}

fn main() {
    let mut modules: HashMap<String, Box<dyn Module>> =  HashMap::new();
    let mut module_wiring: HashMap<String, Vec<String>> = HashMap::new();
    let mut graph = Graph::<String, ()>::new();
    let mut node_map = HashMap::new();
    node_map.insert("rx".to_string(), graph.add_node("rx".to_string()));

    for line in INPUT.lines() {
        let (full_name, output_list) = line.split_once(" -> ").unwrap();
        node_map.insert(full_name.to_string(), graph.add_node(full_name.to_string()));
        let outputs = output_list.split(',').map(|o| {o.trim().to_string()}).collect::<Vec<String>>();
        if full_name.starts_with('%') {
            let mut full_name_chars = full_name.chars();
            let _ = full_name_chars.next();
            let name = full_name_chars.collect::<String>();
            modules.insert(name.clone(), Box::new(FlipFlop::new()));
            module_wiring.insert(name, outputs);
        } else if full_name.starts_with('&') {
            let mut full_name_chars = full_name.chars();
            let _ = full_name_chars.next();
            let name = full_name_chars.collect::<String>();
            modules.insert(name.clone(), Box::new(Conjunction::new()));
            module_wiring.insert(name, outputs);
        } else {
            assert_eq!(full_name, "broadcaster");
            modules.insert(full_name.to_string(), Box::new(BroadCaster::new()));
            module_wiring.insert(full_name.to_string(), outputs);
        }
    }


    for (module_name, outputs) in &module_wiring {
        for output_module_name in outputs {
            if let Some(output_module) = modules.get_mut(output_module_name) {
                output_module.wire_input(module_name);
            }
            graph.add_edge(*node_map.get(&get_full_name(module_name, &modules)).unwrap(), *node_map.get(&get_full_name(output_module_name, &modules)).unwrap(), ());
        }
    }

    for (node_name, node_index) in node_map {
        let edges_count = graph.edges(node_index).count() as i64;
        let curr_weight = graph.node_weight_mut(node_index).unwrap();
        *curr_weight = std::format!("{} with {}", curr_weight, edges_count-1);
    }

    let (node_before_rx, _) = module_wiring.iter().find(|(input, outputs)|{outputs.contains(&"rx".to_string())}).unwrap();
    let mut inputs_of_node_before_rx = module_wiring.iter().filter(|(input, outputs)|{outputs.contains(node_before_rx)}).map(|(a, b)|{a}).collect::<Vec<&String>>();

    export_to_png(&graph, "modules");

    let mut high_signals:i64 = 0;
    let mut low_signals:i64 = 0;

    let mut nums: Vec<i64> = vec!{};

    for i in 1..10000 {
        let mut work_queue = vec!{("broadcaster".to_string(), "button".to_string(), SignalState::Low)};
        while !work_queue.is_empty() {
            let mut new_work_queue: Vec<(String, String, SignalState)> = vec!{};
            for (work_item, sender, signal) in &work_queue {
                if let Some(module) = modules.get_mut(work_item) {
                    if let Some(new_input) = module.receive(*signal, sender) {
                        if inputs_of_node_before_rx.contains(&work_item) && *signal == SignalState::Low {
                            nums.push(i);
                            inputs_of_node_before_rx.retain(|a|{*a != work_item});
                        }
                        for output in module_wiring.get(work_item).unwrap() {
                            match new_input {
                                SignalState::Low => {low_signals += 1;}
                                SignalState::High => {high_signals += 1;}
                            }
                            new_work_queue.push((output.clone(), work_item.clone(), new_input.clone()))
                        }
                    }
                }
            }
            work_queue = new_work_queue;
        }
        if (i == 1000) {
            println!("{}", high_signals*(low_signals+1000))
        }
    }

    println!("{}", nums.iter().fold(1, |a, n| a*n));

}
