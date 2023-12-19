use std::cmp;
use std::{collections::BTreeMap, ops::RangeInclusive};
use rayon::prelude::*;

const TEST: &'static str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
const INPUT: &'static str = "L 6 (#3e5440)
U 4 (#2d3353)
R 7 (#1de2c0)
U 4 (#0917d3)
L 7 (#206732)
U 4 (#0d6943)
L 4 (#135080)
U 4 (#37f2e3)
R 6 (#504dc0)
U 4 (#1b0a73)
L 6 (#639e42)
U 4 (#086ea3)
L 2 (#206730)
U 3 (#008013)
L 3 (#2dc280)
U 6 (#0f12e3)
L 4 (#0e0cf0)
U 4 (#18e163)
L 3 (#369520)
D 4 (#4671f3)
L 2 (#1459f0)
D 6 (#052041)
L 5 (#0d6ab0)
D 3 (#3b5c51)
L 7 (#5c6be0)
D 3 (#104dc1)
L 3 (#429df0)
D 2 (#147523)
L 6 (#2ab760)
U 7 (#1f4e83)
L 3 (#3147a0)
U 2 (#1f4e81)
L 9 (#08fa60)
U 3 (#147521)
L 4 (#259d50)
U 3 (#5bfee1)
L 4 (#04f102)
U 3 (#1ff863)
L 4 (#161172)
U 3 (#197101)
R 5 (#53c842)
U 4 (#197103)
R 6 (#504662)
U 7 (#1ff861)
R 2 (#0e2392)
U 3 (#389c91)
R 3 (#331680)
U 4 (#1ea7f1)
R 4 (#4255f0)
U 3 (#09f8e1)
L 5 (#05b710)
U 8 (#340be1)
L 5 (#00acc0)
U 2 (#402723)
L 3 (#371b80)
U 8 (#28d123)
L 4 (#5a5e80)
U 5 (#196393)
L 5 (#0431f0)
D 5 (#0e05e3)
L 5 (#22e560)
D 4 (#4307e1)
L 3 (#013972)
D 4 (#236b21)
L 3 (#013970)
U 9 (#29eeb1)
L 3 (#2db1b0)
U 3 (#550f21)
L 2 (#1ab6e0)
U 8 (#0dcb31)
R 3 (#5dd190)
U 5 (#141861)
R 4 (#3caa00)
D 5 (#5c45a1)
R 3 (#2ac6b0)
U 5 (#4459c1)
R 4 (#2ca2d0)
U 4 (#566b23)
R 3 (#3d6c02)
D 6 (#29b803)
R 2 (#3d6c00)
D 3 (#3494a3)
R 5 (#0829b0)
U 4 (#287351)
R 4 (#0596d0)
U 3 (#0b11d1)
L 4 (#06ab50)
U 3 (#115051)
R 4 (#2ddbb0)
U 4 (#4c2e21)
R 4 (#1931a0)
D 7 (#2ef0b1)
R 2 (#4db8a2)
D 3 (#0aa811)
R 4 (#19ca80)
U 4 (#430251)
R 4 (#4c33d0)
U 3 (#023051)
R 3 (#5e0570)
U 7 (#0f36c1)
R 3 (#612e00)
U 5 (#0d0db1)
R 5 (#053f30)
D 5 (#09b4f1)
R 3 (#331170)
U 3 (#3ce041)
R 2 (#260810)
U 9 (#15b801)
R 3 (#3e6ae0)
D 9 (#39ea51)
R 2 (#20aa00)
D 3 (#21d3e3)
R 3 (#377160)
D 3 (#1ecd33)
R 3 (#19f560)
D 3 (#5f56e3)
R 5 (#2f0a30)
D 5 (#2df443)
R 3 (#08a970)
D 3 (#135db3)
R 3 (#0a53f0)
D 5 (#1229e1)
L 3 (#348fa0)
D 2 (#40f361)
L 7 (#12c8e0)
D 2 (#191f61)
L 2 (#1beb60)
D 5 (#18f4f3)
L 2 (#43e250)
D 5 (#214773)
R 8 (#43e252)
D 5 (#320043)
R 3 (#022260)
D 4 (#04a3b3)
L 6 (#3769b0)
D 4 (#34f4d3)
R 6 (#39d572)
D 4 (#18e7c3)
R 2 (#39d570)
D 5 (#31f863)
R 6 (#1a04b0)
U 4 (#540991)
R 2 (#1860c2)
U 3 (#5ee081)
R 3 (#1860c0)
U 5 (#407691)
R 3 (#0cb690)
U 6 (#2c1571)
R 5 (#09f252)
D 3 (#235171)
R 3 (#09f250)
D 9 (#22fb01)
R 4 (#31fde0)
U 4 (#2a8333)
R 4 (#53d470)
U 2 (#251f23)
L 4 (#0d9010)
U 4 (#3438d1)
R 2 (#071de0)
U 2 (#4955b1)
R 3 (#1af640)
U 8 (#4ceea1)
R 3 (#4d9770)
D 4 (#455311)
R 4 (#216810)
U 7 (#325981)
R 2 (#4adef0)
U 7 (#095d91)
R 3 (#52ed52)
D 3 (#3598c1)
R 2 (#2b47a2)
D 4 (#1f9da1)
R 4 (#1989a0)
D 4 (#5196b1)
L 4 (#3ccc20)
D 3 (#5196b3)
R 3 (#27df30)
D 3 (#333991)
L 7 (#163a30)
D 4 (#1ddbc3)
L 7 (#5604c0)
D 3 (#272033)
R 5 (#028450)
D 6 (#3f6333)
R 7 (#028452)
D 5 (#37d473)
R 4 (#2c0662)
D 4 (#2d1533)
R 2 (#29fe62)
D 2 (#203153)
R 6 (#33fd10)
D 5 (#05fa43)
R 5 (#256080)
D 6 (#46f463)
R 5 (#2b4ae0)
U 6 (#16cb31)
R 7 (#24f310)
D 3 (#4db0e1)
R 5 (#4ba8a0)
U 4 (#314271)
R 6 (#38eac0)
U 6 (#07f121)
L 6 (#145702)
U 3 (#314e81)
R 5 (#309452)
U 5 (#314e83)
R 3 (#3d8142)
U 5 (#41cd61)
R 4 (#0226d2)
U 7 (#1b7661)
R 2 (#24f312)
U 2 (#2987f1)
R 6 (#4352b0)
U 7 (#0f3d11)
R 5 (#205dc0)
D 3 (#1d9ab1)
R 5 (#63b072)
D 4 (#37daf1)
R 5 (#0e3ab2)
U 7 (#5b2791)
R 4 (#04f5e2)
U 4 (#006de1)
R 3 (#02c0e2)
U 3 (#098031)
R 4 (#2bf8a2)
U 7 (#356683)
R 3 (#179b72)
D 5 (#356681)
R 5 (#2e1062)
D 4 (#098033)
R 4 (#001342)
D 3 (#1fa4d1)
R 3 (#55fcd2)
D 5 (#1322c1)
L 8 (#1048b2)
D 2 (#5257a1)
R 8 (#0a0da2)
D 5 (#369e51)
R 4 (#22a6a2)
D 4 (#336ba1)
R 2 (#0629f2)
D 6 (#3b0881)
R 4 (#5c6732)
D 7 (#224bc1)
R 3 (#057b32)
D 3 (#01f7f3)
R 3 (#3911c2)
D 2 (#3e1423)
R 6 (#371330)
D 4 (#25d1e3)
R 4 (#371332)
U 4 (#1e05e3)
R 3 (#3911c0)
U 2 (#437a63)
R 5 (#522302)
U 4 (#17b101)
L 4 (#06bac2)
U 2 (#441d53)
L 4 (#031852)
U 3 (#058aa3)
L 6 (#541d62)
U 7 (#05c853)
R 4 (#3a9d02)
U 4 (#60de73)
R 3 (#36e762)
U 4 (#1deed3)
L 2 (#0facf2)
U 5 (#36f0f3)
L 5 (#2f2602)
U 7 (#28cfd1)
R 3 (#1f27f2)
U 2 (#1f99a1)
R 3 (#382702)
U 3 (#2555b3)
R 5 (#5e54f2)
U 3 (#2555b1)
R 3 (#04e0d2)
D 6 (#39b5b1)
R 6 (#0086e2)
D 5 (#066271)
L 6 (#34e142)
D 4 (#096d81)
R 2 (#236db2)
D 3 (#605ca1)
L 6 (#2a54c2)
D 6 (#605ca3)
R 6 (#226502)
D 4 (#096d83)
R 4 (#0f15b2)
D 5 (#123481)
R 6 (#0dd912)
D 2 (#4d0071)
L 6 (#2bb2f2)
D 4 (#1d77f1)
R 5 (#28a522)
D 4 (#17b103)
R 6 (#181612)
D 3 (#212291)
R 7 (#092cb2)
U 6 (#315121)
R 2 (#0eb5f0)
U 4 (#2e3fe1)
R 5 (#60e3c0)
D 3 (#2e3fe3)
R 2 (#0afa30)
D 4 (#223ec1)
R 3 (#162a80)
U 4 (#314641)
R 4 (#3931a2)
U 3 (#5a9a21)
R 5 (#30ef22)
D 3 (#5a9a23)
R 2 (#269da2)
D 5 (#1f1181)
R 4 (#12b3b2)
D 2 (#0f6331)
R 3 (#16a832)
D 3 (#15bb01)
L 2 (#3d6732)
D 4 (#0b5311)
L 6 (#156040)
D 5 (#52f1d1)
L 3 (#156042)
D 3 (#1d9bd1)
L 2 (#2175c2)
D 5 (#11ed91)
L 4 (#25c902)
U 2 (#64aba1)
L 2 (#1866d2)
U 5 (#194861)
L 3 (#1866d0)
U 6 (#045131)
L 5 (#53a6a2)
D 5 (#3d6b11)
L 3 (#53a6a0)
D 3 (#361881)
L 7 (#25c900)
D 3 (#220121)
R 3 (#20c860)
D 2 (#2224f1)
R 7 (#1e0490)
D 3 (#249c91)
R 4 (#59ed32)
D 4 (#1e2d91)
R 2 (#59ed30)
D 7 (#2b9e41)
L 5 (#0cc030)
D 2 (#0162b1)
L 4 (#45d860)
D 3 (#29af61)
R 6 (#0ae9d2)
D 3 (#5aaee1)
R 2 (#2bbdb2)
D 4 (#324031)
R 4 (#1ceab2)
D 5 (#274d73)
R 2 (#1191c2)
D 4 (#2e8cf3)
R 4 (#1191c0)
U 6 (#0f0513)
R 5 (#315b42)
D 6 (#64df71)
R 3 (#173e92)
D 3 (#324033)
L 10 (#4aacb2)
D 3 (#291021)
L 4 (#05e992)
D 5 (#3f3121)
R 7 (#19c302)
U 3 (#0dbc41)
R 10 (#1327b2)
U 4 (#0ca1f3)
R 3 (#322722)
U 4 (#5c9c13)
R 3 (#171062)
U 4 (#06ab53)
R 9 (#410e52)
D 2 (#2c0573)
R 2 (#478d92)
D 6 (#38b623)
R 2 (#06e792)
D 4 (#373093)
R 4 (#345222)
D 3 (#1d6593)
R 4 (#1aabe2)
D 7 (#07e563)
R 6 (#351f52)
D 2 (#03eb53)
R 5 (#0145c0)
U 4 (#0d6ae3)
R 3 (#38d490)
U 5 (#40ec73)
R 3 (#295df0)
D 3 (#3ca9a3)
R 7 (#116ee2)
D 2 (#120333)
R 4 (#1f54b2)
D 4 (#182353)
L 9 (#0eb5d0)
D 4 (#263fc3)
L 5 (#0eb5d2)
D 6 (#2b9053)
R 5 (#32b4b2)
D 5 (#00f0f3)
R 3 (#1ffd52)
D 3 (#264893)
R 6 (#30d5a2)
D 3 (#48c513)
L 4 (#082c52)
D 6 (#145533)
L 5 (#485bc2)
U 6 (#0ffb23)
L 5 (#4b6ca2)
D 4 (#3b05d3)
L 5 (#0d7152)
U 2 (#3ceda3)
L 4 (#57b082)
U 7 (#07ddd3)
L 5 (#293702)
U 5 (#1cbac3)
L 8 (#2740f2)
U 5 (#3310a3)
L 4 (#2046c2)
U 2 (#639c43)
L 7 (#075ca2)
D 5 (#0b22e3)
R 4 (#38b562)
D 8 (#06fc63)
L 4 (#055e62)
D 5 (#35f373)
L 7 (#3848c2)
D 4 (#187ee1)
L 5 (#3fc222)
U 3 (#2073e1)
L 4 (#095312)
U 5 (#48c021)
L 9 (#119d12)
U 3 (#374fb3)
R 9 (#10f9d0)
U 3 (#1deab3)
L 4 (#10f9d2)
U 3 (#2c7883)
L 7 (#21ca02)
U 5 (#2b38d3)
L 3 (#043332)
D 5 (#4efe03)
L 4 (#1537f2)
D 3 (#343da3)
R 4 (#34ae92)
D 3 (#213d33)
L 4 (#1a2382)
D 5 (#3c06e3)
R 4 (#4c1422)
D 5 (#1f2c33)
L 5 (#0a1a62)
D 7 (#5b3311)
L 2 (#0099e2)
D 5 (#2dc123)
R 4 (#1d5f42)
D 3 (#4b87f3)
R 5 (#1cc880)
D 7 (#2ea0a3)
R 3 (#32bf50)
U 3 (#269713)
R 2 (#12d450)
U 7 (#3dac03)
R 3 (#2b58d0)
D 7 (#0a1983)
R 4 (#25cc30)
D 6 (#4665a3)
L 4 (#37d780)
D 8 (#4b0633)
L 5 (#317020)
D 3 (#56f373)
L 3 (#27b6f0)
U 8 (#08afe3)
L 6 (#028910)
U 3 (#33cbc3)
L 3 (#003020)
D 3 (#2676b3)
L 8 (#43e120)
D 2 (#305503)
L 3 (#1a9560)
D 3 (#56cbb1)
L 3 (#09d0c0)
D 8 (#166ab3)
R 3 (#3d2160)
D 2 (#17c043)
R 4 (#3d46c0)
D 4 (#53f201)
L 2 (#224020)
D 5 (#53f203)
L 5 (#1fe460)
D 3 (#1e79e3)
L 7 (#150ae0)
D 3 (#1538e3)
L 4 (#0403f2)
D 3 (#0defd3)
L 4 (#2c0772)
D 2 (#313663)
L 7 (#46c222)
D 3 (#094813)
L 8 (#11cc00)
D 6 (#119483)
L 4 (#08bf82)
D 7 (#3634b3)
L 6 (#08bf80)
U 8 (#2bd013)
L 4 (#2f7b80)
U 6 (#1b29f1)
L 5 (#2b6372)
U 2 (#468b01)
L 3 (#2b6370)
D 5 (#11e451)
R 4 (#358600)
D 7 (#2b51f3)
L 4 (#12f0b0)
D 4 (#109851)
L 7 (#159610)
D 3 (#5ed9c1)
L 3 (#255d90)
U 8 (#20c101)
L 4 (#454b22)
U 3 (#2bf5c1)
L 7 (#0195c2)
U 6 (#15bf61)
L 8 (#085672)
U 4 (#4847d1)
L 8 (#085670)
U 6 (#137581)
L 3 (#28fc80)
U 2 (#0fb611)
L 2 (#378640)
U 8 (#0f6373)
L 3 (#0d71b0)
U 4 (#3f4c93)
L 4 (#0955c0)
U 2 (#1597d3)
L 3 (#3e4440)
U 8 (#3e7141)
L 6 (#0376c0)
U 5 (#25d691)
L 3 (#479560)
D 8 (#0fb613)
L 3 (#0dbea0)
D 4 (#35d901)
L 3 (#28ae62)
D 5 (#60e581)
L 2 (#150802)
D 2 (#08cea1)
L 5 (#262b22)
U 5 (#17a493)
L 5 (#0e53f2)
U 4 (#4cf2e3)
L 4 (#41af02)
U 3 (#117563)
R 3 (#1b8502)
U 4 (#1e45c1)
R 6 (#0caec0)
U 3 (#42af81)
L 4 (#4c3390)
U 3 (#0812a1)
L 3 (#58e252)
U 5 (#0d04f1)
L 6 (#3eefc2)
U 2 (#3cacc1)
L 5 (#055630)
U 3 (#612d91)
L 8 (#17edc0)
U 3 (#05e171)
L 4 (#299cf0)
U 7 (#004f31)
L 5 (#3af3a2)
U 2 (#263861)
L 3 (#294020)
U 8 (#250843)
L 4 (#2e2a80)
D 7 (#0502d3)
L 4 (#4af290)
D 4 (#0502d1)
R 6 (#0e51a0)
D 3 (#13d263)
L 6 (#1618a0)
D 4 (#39f283)
L 3 (#4ead90)
D 2 (#361563)
L 4 (#117180)
U 8 (#056a73)
R 4 (#2051c0)
U 9 (#0b1a63)
R 2 (#321910)
U 6 (#62e593)
R 7 (#309930)
U 3 (#137623)
L 8 (#54d190)
U 4 (#1a7ed3)
L 3 (#148a12)
U 4 (#622983)
R 4 (#2947b2)
U 2 (#3974d3)
R 7 (#3d3df2)
U 4 (#18d703)
R 6 (#4c7742)
D 7 (#3b6363)
L 4 (#09c060)
D 3 (#2575c1)
R 4 (#152350)
D 7 (#134d71)
R 6 (#36fd20)
U 5 (#134d73)
R 6 (#293cf0)
U 2 (#2575c3)
R 7 (#447860)
U 3 (#1c2d83)
R 4 (#03f0d0)
U 2 (#506ac3)
R 3 (#1a1eb0)
U 9 (#0c4493)
R 4 (#2cc192)
U 4 (#37cbe3)
L 7 (#152ec2)
U 6 (#297713)
L 5 (#312272)
U 3 (#1c3fd3)
L 3 (#11ed22)
U 4 (#07f303)
L 7 (#20d3c2)
U 4 (#398531)
L 2 (#0f9072)
U 7 (#1682e1)
L 2 (#0c4e52)
U 5 (#3236c1)
L 3 (#520ae2)
D 4 (#0336f1)
L 10 (#36e992)
D 2 (#43c1a3)
L 2 (#1faba2)
D 3 (#33f531)
L 2 (#422ff2)
D 4 (#49b721)
R 10 (#211aa2)
D 3 (#1e0323)
R 4 (#2ab462)
D 5 (#5fa933)
L 3 (#1726c2)
D 2 (#5541c3)
L 6 (#468262)
U 5 (#272773)
L 4 (#522bb2)
U 2 (#272771)
L 5 (#357672)
U 4 (#2c7923)";

fn det(a: (i64, i64), b: (i64, i64)) -> i64 {
    a.0 * b.1 - b.0 * a.1
}
fn parse_hex(input: &str) -> i64 {
    input.chars().fold(0, |a, b| a * 16 + b.to_digit(16).unwrap() as i64)
}
fn run(input: &str) {
    let instrs = input
        .lines()
        .map(|l| {
            let mut splitted = l.split_whitespace();
            let dir: (i64, i64) = match splitted.next().unwrap() {
                "U" => (-1, 0),
                "D" => (1, 0),
                "R" => (0, 1),
                "L" => (0, -1),
                _ => unreachable!(),
            };
            let steps = splitted.next().unwrap().parse::<i64>().unwrap();
            let part2 = splitted.next().unwrap();
            let part2 = &part2[2..part2.len() - 1];
            let steps = parse_hex(&part2[0..5]);
            let dir = match &part2[5..] {
                "3" => (-1, 0),
                "1" => (1, 0),
                "0" => (0, 1),
                "2" => (0, -1),
                _ => unreachable!()
            };
            (dir, steps)
        });
        //.collect::<Vec<((i64, i64), i64)>>();
    // The horizontal ranges that exist on each y
    let mut y_ranges: BTreeMap<i64, Vec<RangeInclusive<i64>>> = BTreeMap::new();
    // Vertical ranges on each x
    let mut x_ranges: BTreeMap<i64, Vec<RangeInclusive<i64>>> = BTreeMap::new();
    let mut pos = (0, 0);

    let mut ans = 0;
    let mut vertices = vec![];
    vertices.push(pos);
    for (dir, steps) in instrs {
        ans += steps;
        if dir.1 != 0 {
            let end = pos.1 + dir.1 * steps;
            let range = cmp::min(pos.1, end)..=cmp::max(pos.1, end);
            // y_ranges
            //     .entry(pos.0)
            //     .and_modify(|v| {
            //         v.insert(
            //             v.binary_search_by_key(&range.start(), |r| r.start())
            //                 .unwrap_err(),
            //             range.clone(),
            //         )
            //     })
            //     .or_insert(vec![range]);
        } else {
            let end = pos.0 + dir.0 * steps;
            let range = cmp::min(pos.0, end)..=cmp::max(pos.0, end);
            // x_ranges
            //     .entry(pos.1)
            //     .and_modify(|v| {
            //         v.insert(
            //             v.binary_search_by_key(&range.start(), |r| r.start())
            //                 .unwrap_err(),
            //             range.clone(),
            //         )
            //     })
            //     .or_insert(vec![range]);
        }
        // TODO: insert additional 1-sized ranges
        pos.0 += steps * dir.0;
        pos.1 += steps * dir.1;
        vertices.push(pos);
    }
    //println!("X ranges: {x_ranges:?}");
    //println!("Y ranges: {y_ranges:?}");
    let time = std::time::Instant::now();
    let shoelace = shoelace(&vertices);
    println!("{} in {:?}", shoelace + ans / 2 + 1, std::time::Instant::now() - time);
}

fn shoelace(vertices: &[(i64, i64)]) -> i64 {
    let mut sum = 0;
    //return (0..vertices.len()).into_par_iter().fold(|| 0, |a, b| a+det(vertices[b], vertices[(b + 1) % vertices.len()])).sum::<i64>().abs() / 2;
    for i in 0..vertices.len() {
        sum += det(vertices[i], vertices[(i + 1) % vertices.len()]);
    }
    sum.abs() / 2
}
fn main() {
    run(TEST);
    run(INPUT);
}
