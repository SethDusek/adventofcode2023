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
            while let Some(prev) = prev.get(&cur).copied() {
                flows
                    .entry((prev, cur))
                    .and_modify(|flow| *flow += min_residual_capacity)
                    .or_insert(min_residual_capacity);
                flows
                    .entry((cur, prev))
                    .and_modify(|flow| *flow -= min_residual_capacity)
                    .or_insert(-min_residual_capacity);
                cur = prev;
            }
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

    let mut visited = HashSet::new();
    let mut stack = Vec::new();
    stack.push(parsed.keys().next().unwrap());
    while let Some(cur) = stack.pop().copied() {
        visited.insert(cur);
        for edge in parsed[&cur].iter().filter(|&&edge| !visited.contains(edge)) {
            stack.push(edge);
        }
    }
    println!("{}", visited.len() * (parsed.len() - visited.len()));

}
