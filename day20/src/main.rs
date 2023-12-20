#![feature(cmp_minmax)]
use std::{
    cmp,
    collections::{HashMap, HashSet, VecDeque},
    ops::Range,
};
// a turns low after 2b + 1 button presses, high every 2b presses
// b turns low every 4b + 3 button presses, high every 4b + 1 presses
// c turns low after every 8b + 7 button presses and high every 8b + 3 presses
// 2b and 4b + 1 = 4b + 2
// A conjunction node will pulse low when all of its inputs are high. If its inputs are low every 2b + 1 and 4b + 3, 8b + 7, then its inputs will be high every 2b and 4b + 1 and 8b + 3, it will turn on at 8b + 6 (???),
// c will only turn on if both a and b are high,
// 2b, 4b + 1
// Inputs: a: 2b, b: 4b + 1, c: 8b + 5, d: 16b + 7, output = 16b + 14 (wtf)
// Inputs: a: 2b, b: 4b + 1, d: 16b + 7, output = 16b + 14 and 16b + 10 (double wtf)
// Inputs: b: 2b, 4b + 1, d: 16b + 7, output = ??
// 2b + (0..1), 4b + (1..3) = 4b + (2..3)
// 4b + (1..3) flip flopped = 8b + (3..7)
// 4b + (0..1) and (2..3) flip flopped =
// 2b + (1..0) and 8b + (3..7) = ?
// 2b + 1 will pulse low at 1, 3, 5, 7 and high at 0, 2, 4, 6, 8 -> 4b + 1 will pulse high at 1, 5, 9 and low at 3, 7, 11, ...
const TEST: &'static str = "broadcaster -> a
%a -> b, d
%b -> c
%c -> d
&d -> rx
";
/*
%c -> d
&d -> e
%e -> rx
";*/
const INPUT: &'static str = include_str!("../input.txt");

#[derive(Debug, Clone, PartialEq, Eq)]
enum NodeType<'a> {
    Broadcaster,
    Conjunction(HashMap<&'a str, bool>),
    FlipFlop(bool),
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Node<'a> {
    node_type: NodeType<'a>,
    outputs: Vec<&'a str>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Wave {
    // time period
    period: u64,
    // phase at which high pulse occurs If a wave is constructed from a conjunction there can be multiple low/high pulses in a single cycle
    high: u64,
    high_pulses: u64,
    // phase at which low occurs
    low: u64,
    low_pulses: u64,
}
impl Wave {
    // Multiply time period by 2 divided by number of low pulses
    fn divide(&self) -> Wave {
        let period = (self.period * 2) / self.low_pulses;
        // The new wave will pulse high after 1 low pulse of source wave
        let high = self.low;
        let high_pulses = 1;
        todo!()
    }
}
// divides a frequency that's low every at + b into 2at + b + a
fn frequency_divide_low(a: u64, b: u64) -> (u64, u64) {
    (2 * a, b + a)
}

fn to_high(a: u64, b: u64) -> (u64, u64) {
    (a, b.checked_sub(a / 2).unwrap_or_else(|| b + a / 2))
}

// Intersects two low ranges based on the interval they are high. Used for conjunction. May produce many ranges so this could go horribly
// Returns ranges of values
fn intersect(a: (u64, u64), b: (u64, u64)) -> Vec<(u64, Range<u64>)> {
    //println!("Intersect called for {a:?} {b:?}");
    let min = std::cmp::min_by_key(a, b, |(a, _)| *a);
    let high_min = to_high(min.0, min.1);
    // The wave with highest time period
    let max = std::cmp::max_by_key(a, b, |(a, _)| *a);
    let high_max = to_high(max.0, max.1);
    // High..Low ranges for both waves
    let mut hl_min_range = cmp::min(high_min.1, min.1)..cmp::max(high_min.1, min.1);
    let mut hl_max_range = cmp::min(high_max.1, max.1)..cmp::max(high_max.1, max.1);
    let [mut hl_min_range, hl_max_range] =
        cmp::minmax_by_key(hl_min_range, hl_max_range, |r| r.start);
    // If we have a wave like 16 b + 15, then it'll be on from 16b + 7..15. Our wave 2b + 1's phase will need to be adjusted to intersect with 7..15
    assert!(
        hl_min_range.start <= hl_max_range.start,
        "Press F to pay respeccs for {:?} {:?}",
        hl_min_range,
        hl_max_range
    );
    if hl_max_range.start > hl_min_range.end {
        hl_min_range.start += min.0 * ((hl_max_range.start - hl_min_range.end + min.0 - 1) / min.0);
        hl_min_range.end += min.0 * ((hl_max_range.start - hl_min_range.end + min.0 - 1) / min.0);
    }
    let mut res = vec![];
    while hl_min_range.start < hl_max_range.end {
        if hl_max_range.start > hl_min_range.end || hl_min_range.start > hl_max_range.end {
            break;
        }
        res.push((
            max.0,
            cmp::max(hl_min_range.start, hl_max_range.start)
                ..cmp::min(hl_min_range.end, hl_max_range.end),
        ));
        if res.last().cloned().unwrap().1.count() == 0 {
            res.pop();
        }
        hl_min_range.start += min.0;
        hl_min_range.end += min.0;
    }
    res
}

impl<'a> Node<'a> {
    // For each output, output the state
    fn handle(&mut self, input_src: &'a str, input: bool) -> Vec<bool> {
        match &mut self.node_type {
            NodeType::Broadcaster => vec![input; self.outputs.len()],
            NodeType::FlipFlop(state) => {
                if !input {
                    *state = !*state;
                    if *state {
                        vec![true; self.outputs.len()]
                    } else {
                        vec![false; self.outputs.len()]
                    }
                } else {
                    vec![]
                }
            }

            NodeType::Conjunction(hm) => {
                hm.insert(input_src, input);
                if hm.values().all(|&v| v) {
                    vec![false; self.outputs.len()]
                } else {
                    vec![true; self.outputs.len()]
                }
            }
            _ => todo!(),
        }
    }
}

fn parse(input: &str) -> HashMap<&str, Node> {
    let mut parsed = input
        .lines()
        .map(|l| {
            let (name, outputs) = l.split_once(" -> ").unwrap();
            let outputs = outputs.split(", ").collect::<Vec<&str>>();
            let node = if name == "broadcaster" {
                Node {
                    node_type: NodeType::Broadcaster,
                    outputs,
                }
            } else {
                if &name[0..1] == "%" {
                    Node {
                        node_type: NodeType::FlipFlop(false),
                        outputs,
                    }
                } else if &name[0..1] == "&" {
                    Node {
                        node_type: NodeType::Conjunction(HashMap::new()),
                        outputs,
                    }
                } else {
                    unreachable!()
                }
            };
            let name = if &name[0..1] != "b" { &name[1..] } else { name };
            (name, node)
        })
        .collect::<HashMap<&str, Node>>();
    for node in parsed.clone() {
        for output in node.1.outputs {
            if let Some(NodeType::Conjunction(hm)) =
                &mut parsed.get_mut(output).map(|node| &mut node.node_type)
            {
                hm.insert(node.0, false);
            }
        }
    }
    parsed
}

fn simulate1<'a>(nodes: &mut HashMap<&'a str, Node<'a>>) -> Option<(u64, u64)> {
    let mut queue = VecDeque::new();
    queue.push_back(("", false, "broadcaster"));
    let mut low_pulses = 1;
    let mut high_pulses = 0;
    let mut rx_pulse_count = 0;
    while queue.len() != 0 {
        let (src, pulse, node_name) = queue.pop_front().unwrap();
        //println!("Input of {node_name}: {pulse} from {src}");
        if node_name == "rx" && pulse == false {
            println!("RX received low pulse!");
            rx_pulse_count += 1;
        }

        if !nodes.contains_key(&node_name) {
            continue;
        }
        let output = nodes.get_mut(node_name).unwrap().handle(src, pulse);
        //println!("Output of {node_name}: {output:?}");
        for &pulse in &output {
            if pulse {
                high_pulses += 1;
            } else {
                low_pulses += 1;
            }
        }
        output
            .iter()
            .zip(nodes[node_name].outputs.clone().into_iter())
            .for_each(|(&pulse, node)| queue.push_back((node_name, pulse, node)));
    }
    //println!("{low_pulses} {high_pulses}");
    if rx_pulse_count != 1 {
        Some((low_pulses, high_pulses))
    } else {
        None
    }
}
fn simulate3ver3finalforsurewillworkthistime<'a>(
    nodes: &mut HashMap<&'a str, Node<'a>>,
) -> HashMap<&'a str, bool> {
    let mut queue = VecDeque::new();
    queue.push_back(("", false, "broadcaster"));
    let mut low_pulses = 1;
    let mut high_pulses = 0;
    let mut pulsed = HashMap::new();
    while queue.len() != 0 {
        let (src, pulse, node_name) = queue.pop_front().unwrap();
        //println!("Input of {node_name}: {pulse} from {src}");
        if node_name == "rx" && pulse == false {
            println!("RX received low pulse!");
        }

        if !nodes.contains_key(&node_name) {
            continue;
        }
        let output = nodes.get_mut(node_name).unwrap().handle(src, pulse);
        //println!("Output of {node_name}: {output:?}");
        for &pulse in &output {
            if pulse {
                high_pulses += 1;
            } else {
                low_pulses += 1;
            }
        }
        if output.first() == Some(&true) {
            match node_name {
                name @ ("vt" | "sk" | "xc" | "kk") => {pulsed.insert(name, true); },
                _ => {}
            };
        }
        output
            .iter()
            .zip(nodes[node_name].outputs.clone().into_iter())
            .for_each(|(&pulse, node)| queue.push_back((node_name, pulse, node)));
    }
    pulsed
}

fn part1(input: &'static str) {
    let mut parsed = parse(input);
    println!("{:?}", parsed);
    let mut low_pulses = 0;
    let mut high_pulses = 0;
    for i in 0..1000 {
        let (low, high) = simulate1(&mut parsed).unwrap();
        println!("{i}: Low: {low}, High: {high}");
        low_pulses += low;
        high_pulses += high;
    }
    println!("{}", low_pulses * high_pulses);
}
fn simulate2<'a>(nodes: &mut HashMap<&'a str, Node<'a>>) -> Vec<(u64, u64)> {
    let mut queue = VecDeque::new();
    queue.push_back((vec![(1, 0)], "broadcaster"));
    let mut node_input_waves: HashMap<&'a str, Vec<(u64, u64)>> = HashMap::new();
    while queue.len() != 0 {
        let (waves, node_name) = queue.pop_front().unwrap();
        //println!("Input of {node_name}: {pulse} from {src}");

        if !nodes.contains_key(&node_name) {
            continue;
        }
        let mut output_waves = vec![];
        println!("{node_name}");
        match nodes[node_name].node_type {
            NodeType::FlipFlop(_) => output_waves.extend(
                node_input_waves[node_name]
                    .iter()
                    .map(|wave| frequency_divide_low(wave.0, wave.1)),
            ),
            NodeType::Conjunction(_) => {
                let input_waves = node_input_waves[node_name].clone();
                if input_waves.len() == 1 {
                    output_waves.push(to_high(input_waves[0].0, input_waves[0].1));
                } else {
                    let mut interescting_ranges = intersect(input_waves[0], input_waves[1]);
                    for i in 2..input_waves.len() {
                        interescting_ranges = interescting_ranges
                            .iter()
                            .flat_map(|(frequency, range)| {
                                range.clone().map(move |r| (frequency, r))
                            })
                            .flat_map(|(a, b)| intersect((*a, b), input_waves[i]))
                            .collect();
                    }
                    output_waves = interescting_ranges
                        .iter()
                        .flat_map(|(frequency, range)| range.clone().map(move |r| (*frequency, r)))
                        .collect();
                    output_waves.sort();
                }
            }
            NodeType::Broadcaster => output_waves = vec![waves[0]],
        }
        nodes[node_name].outputs.iter().for_each(|&node| {
            println!(
                "{node} input = {output_waves:?} from {:?}",
                nodes[node_name]
            );
            node_input_waves
                .entry(node)
                .and_modify(|o| o.extend_from_slice(&output_waves))
                .or_insert(output_waves.clone());
            queue.push_back((output_waves.clone(), node))
        });
        // let output = nodes.get_mut(node_name).unwrap().handle(src, pulse);
        // //println!("Output of {node_name}: {output:?}");
        // output.iter().zip(nodes[node_name].outputs.clone().into_iter()).for_each(|(&pulse, node)| queue.push_back((node_name, pulse, node)));
    }
    //println!("{low_pulses} {high_pulses}");
    let rx_waves = node_input_waves["rx"].clone();
    let mut output_waves = vec![];
    if rx_waves.len() == 1 {
        output_waves.push(to_high(rx_waves[0].0, rx_waves[0].1));
    } else {
        dbg!(&rx_waves);
        let mut intersecting_ranges = intersect(rx_waves[0], rx_waves[1]);
        dbg!(&intersecting_ranges);
        for i in 2..rx_waves.len() {
            intersecting_ranges = intersecting_ranges
                .iter()
                .flat_map(|(frequency, range)| range.clone().map(move |r| (frequency, r)))
                .flat_map(|(a, b)| intersect((*a, b), rx_waves[i]))
                .collect();
        }
        output_waves = intersecting_ranges
            .iter()
            .flat_map(|(frequency, range)| range.clone().map(move |r| (*frequency, r)))
            .collect()
    }
    output_waves
}

fn part2(input: &'static str) {
    let mut parsed = parse(input);
    //let last_module = &input.lines().last().unwrap().split_once(" -> ").unwrap().0[1..];
    //println!("Last module: {last_module}");
    //parsed.get_mut(last_module).unwrap().outputs.push("rx");
    println!("{:?}", parsed);
    let mut i = 0;
    loop {
        let res = simulate1(&mut parsed);
        if res.is_none() {
            println!("ans - 1 = {i}");
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
        i += 1;
        //println!("{i}: Low: {low}, High: {high}");
    }
}
fn part2_actual(input: &'static str) {
    let mut parsed = parse(input);
    //let last_module = &input.lines().last().unwrap().split_once(" -> ").unwrap().0[1..];
    //println!("Last module: {last_module}");
    //parsed.get_mut(last_module).unwrap().outputs.push("rx");
    println!("{:?}", parsed);
    let mut hm = HashMap::new();
    let mut i = 1;
    loop {
        let res = simulate3ver3finalforsurewillworkthistime(&mut parsed);
        res.iter().filter(|(_, &v)| v).for_each(|(k, _)| { hm.entry(*dbg!(k)).or_insert(i); });
        i+=1;
        if hm.len() == 4 {
            break;
        }
    }
    println!("{:?}", hm);
}
fn main() {
    //part1(TEST);
    //part1(INPUT);
    println!("Intersections: {:?}", intersect((8, 3), (8, 2)));
    //part2_actual(INPUT);
    part2_actual(INPUT);
}
