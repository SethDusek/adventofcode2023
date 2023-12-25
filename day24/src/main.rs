use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};
const TEST: &'static str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
const INPUT: &'static str = include_str!("../input.txt");

fn parse(input: &str) -> Vec<([f64; 3], [f64; 3])> {
    input
        .lines()
        .map(|l| l.split_once(" @ "))
        .map(Option::unwrap)
        .map(|(pos, vel)| {
            (
                pos.split(", ")
                    .map(str::trim_start)
                    .map(str::trim_end)
                    .map(f64::from_str)
                    .map(Result::unwrap)
                    .collect::<Vec<f64>>()
                    .try_into()
                    .unwrap(),
                vel.split(", ")
                    .map(str::trim_start)
                    .map(str::trim_end)
                    .map(f64::from_str)
                    .map(Result::unwrap)
                    .collect::<Vec<f64>>()
                    .try_into()
                    .unwrap(),
            )
        })
        .collect()
}

fn det2(a: f64, b: f64, c: f64, d: f64) -> f64 {
    a * d - b * c
}
fn solve2(posa: &[f64], vela: &[f64], posb: &[f64], velb: &[f64]) -> Option<(f64, f64, [f64; 2])> {
    let mut pos = posb.to_vec();
    for (i, compa) in posa.iter().copied().enumerate() {
        pos[i] -= compa;
    }
    // let mut vel = velb.to_vec();
    // for (i, compb) in vela.iter().copied().enumerate() {
    //     vel[i]-=compb;
    // }

    // det(A)
    let deta = det2(vela[0], -velb[0], vela[1], -velb[1]);
    let detb = det2(pos[0], -velb[0], pos[1], -velb[1]);
    let detc = det2(vela[0], pos[0], vela[1], pos[1]);

    if deta == 0.0 {
        return None;
    } else {
        let t = detb / deta;
        let d = detc / deta;
        if t < 0.0 || d < 0.0 {
            return None;
        }
        //println!("{}", t);
        Some((t, d, [posa[0] + vela[0] * t, posa[1] + vela[1] * t]))
    }
}

fn to_system(parsed: &[([f64; 3], [f64; 3])]) -> Vec<Vec<f64>> {
    let mut system = Vec::new();
    for (i, (pos, vel)) in parsed.iter().enumerate() {

    }
    system
}
fn main() {
    let mut intersections = 0;
    let parsed = parse(&INPUT);
    for l in &parsed {
        for (i, f) in l.0.iter().chain(l.1.iter()).enumerate() {
            print!("{f}");
            if i != 5 {
                print!(",")
            }
        }
        print!("\n");
    }
    let boundary = (200000000000000.0, 400000000000000.0);
    //let parsed = parse(&TEST);
    //let boundary = (7.0, 27.0);
    let mut crossed: HashMap<usize, Vec<(f64, usize)>> = HashMap::new();
    for i in 0..parsed.len() {
        for j in i + 1..parsed.len() {
            if let Some((t, d, sol)) =
                solve2(&parsed[i].0, &parsed[i].1, &parsed[j].0, &parsed[j].1)
            {
                if sol[0] >= boundary.0
                    && sol[0] <= boundary.1
                    && sol[1] >= boundary.0
                    && sol[1] <= boundary.1
                {
                    intersections += 1;
                }
            }
        }
    }
    //println!("{:?}", intersections);
    return;
}
