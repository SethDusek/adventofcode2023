#![feature(cmp_minmax)]
use std::collections::*;
use std::str::FromStr;
const TEST: &'static str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr
";
const INPUT: &'static str = include_str!("../input.txt");

type Parsed<'a> = HashMap<&'a str, Vec<&'a str>>;
fn parse<'a>(input: &'a str) -> Parsed<'a> {
    let mut hm: Parsed<'a> = HashMap::new();
    input
        .lines()
        .map(|l| l.split_once(": "))
        .map(Option::unwrap)
        .flat_map(|(parent, edges)| edges.split_whitespace().map(move |edge| (parent, edge)))
        .for_each(|(a, b)| {
            hm.entry(a).and_modify(|v| v.push(b)).or_insert(vec![b]);
            hm.entry(b).and_modify(|v| v.push(a)).or_insert(vec![a]);
        });
    hm
}

fn run<'a>(input: &Parsed) {
    let mut parents = HashMap::new();
    let mut ranks = HashMap::new();
    for &node in input.keys() {
        make_set(&mut parents, &mut ranks, node);
    }
    println!("{parents:?} {ranks:?}");
    let mut forests = input.len();
    'outer: for (&node, edges) in input {
        for edge in edges {
            let node_root = root(&parents, node);
            let edge_root = root(&parents, edge);
            if node_root != edge_root {
                println!("{} {}", node, edge);
                union(&mut parents, &mut ranks, node, edge);
                forests -= 1;
                if forests == 2 {
                    break 'outer;
                }
            }
        }
    }
    dbg!(forests);
    let mut sorted = ranks.values().copied().collect::<Vec<usize>>();
    sorted.sort();
    println!("{:?}", sorted);
    println!("{}", sorted[sorted.len() - 1] * sorted[sorted.len() - 2]);
    for node in input.keys() {
        //println!("{} {}", node, parents[node]);
    }
}

fn make_set<'a>(
    parents: &mut HashMap<&'a str, &'a str>,
    ranks: &mut HashMap<&'a str, usize>,
    node: &'a str,
) {
    parents.insert(node, node);
    ranks.insert(node, 1);
}

fn root<'a>(parents: &HashMap<&'a str, &'a str>, node: &'a str) -> &'a str {
    let parent = parents[node];
    if parent == node {
        return parent;
    }
    root(parents, parent)
}

fn union<'a>(
    parents: &mut HashMap<&'a str, &'a str>,
    ranks: &mut HashMap<&'a str, usize>,
    a: &'a str,
    b: &'a str,
) {
    let mut parent_a = root(&*parents, a);
    let mut parent_b = root(&*parents, b);
    if parent_a != parent_b {
        if ranks[parent_a] < ranks[parent_b] {
            std::mem::swap(&mut parent_a, &mut parent_b);
        }
        parents.insert(parent_b, parent_a);
        *ranks.get_mut(parent_a).unwrap() += ranks[parent_b];
    }
}

fn simple_path<'a>(
    graph: &Parsed<'a>,
    flows: &HashMap<(&'a str, &'a str), i64>,
    cur: &'a str,
    dest: &'a str,
    prev: &mut HashMap<&'a str, &'a str>,
    visited: &mut HashSet<&'a str>,
) -> bool {
    if cur == dest {
        return true;
    }
    visited.insert(cur);

    for adj in &graph[&cur] {
        if 1 - flows.get(&(cur, adj)).copied().unwrap_or(0) > 0 && !visited.contains(adj) {
            prev.insert(adj, cur);
            if simple_path(graph, flows, adj, dest, prev, visited) {
                return true;
            }
        }
    }
    false
}

fn ford_fulkerson<'a>(graph: &Parsed<'a>, source: &'a str, dest: &'a str) -> HashSet<[&'a str; 2]> {
    let mut flows: HashMap<(&'a str, &'a str), i64> = HashMap::new();

    //let mut paths: Vec<Vec<&str>> = vec![];
    loop {
        let mut prev = HashMap::new();
        let mut visited = HashSet::new();
        if simple_path(graph, &flows, source, dest, &mut prev, &mut visited) {
            let mut cur = dest;
            let mut min_residual_capacity = i64::MAX;
            while let Some(prev) = prev.get(&cur).copied() {
                min_residual_capacity = std::cmp::min(
                    min_residual_capacity,
                    1 - flows.get(&(prev, cur)).copied().unwrap_or(0),
                );
                cur = prev;
            }
            assert_eq!(cur, source);
            let mut cur = dest;
            //let mut path_vec = vec![];
            //path_vec.push(cur);
            while let Some(prev) = prev.get(&cur).copied() {
                flows
                    .entry((prev, cur))
                    .and_modify(|flow| *flow += min_residual_capacity)
                    .or_insert(min_residual_capacity);
                flows
                    .entry((cur, prev))
                    .and_modify(|flow| *flow -= min_residual_capacity)
                    .or_insert(-min_residual_capacity);
                //path_vec.push(prev);
                cur = prev;
            }
            //path_vec.reverse();
            //paths.push(path_vec);
            //paths+=1;
        } else {
            break;
        }
    }
    let mut edges_to_cut = HashSet::new();
    let mut visited = HashSet::new();
    let mut prev = HashMap::new();
    simple_path(graph, &flows, source, "", &mut prev, &mut visited);
    for (&node, edges) in graph {
        for &edge in edges {
            if visited.contains(node) && !visited.contains(edge) {
                edges_to_cut.insert(std::cmp::minmax(node, edge));
            }
        }
    }

    edges_to_cut
}
fn main() {
    let mut parsed = parse(INPUT);
    'outer: for src in parsed.clone().keys() {
        for dst in parsed.clone().keys() {
            if src == dst {
                continue;
            }
            let res = ford_fulkerson(&parsed, src, dst);
            if res.len() != 3 {
                continue;
            }
            dbg!(res.len());
            for [a, b] in res {
                let (idx, _) = parsed
                    .get_mut(a)
                    .unwrap()
                    .iter()
                    .enumerate()
                    .find(|(_, node)| **node == b).unwrap();
                parsed.get_mut(a).unwrap().remove(idx);
                println!("Deleting {a} {b}");
                let (idx, _) = parsed
                    .get_mut(b)
                    .unwrap()
                    .iter()
                    .enumerate()
                    .find(|(_, node)| **node == a).unwrap();
                parsed.get_mut(b).unwrap().remove(idx);
            }
            break 'outer;
        }
    }

    //println!("{:?}", ford_fulkerson(&parsed, "cmg", "xhk"));
    run(&parsed);
    //println!("{:?}", parsed);
}
