
use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use std::cmp;

#[derive(Clone)]
struct Scanner {
    id: i32,
    probes: HashSet<(i32, i32, i32)>,
    located: bool,
    position: (i32, i32, i32)
}

impl Scanner {
    fn rotate(&mut self, i: char) {
        if i == 'Y' { self.probes = self.probes.iter().map(|c| (c.0, -c.2, c.1)).collect() }
        else if i == 'L' { self.probes = self.probes.iter().map(|c| (-c.1, c.0, c.2)).collect(); }
        else if i == 'R' { self.probes = self.probes.iter().map(|c| (c.1, -c.0, c.2)).collect(); }
    }
}

pub fn main(){    
    let mut input = include_str!("../data/day_19.txt").trim().lines();
    input.next();
    let mut scanners: Vec<Scanner> = Vec::new();

    let mut cur_scanner = Scanner{ id: 0, probes: HashSet::new(), located: true, position: (0,0,0) };
    while let Some(line) = input.next() {

        if line == "" { 
            scanners.push(cur_scanner.clone());
            input.next();
            cur_scanner = Scanner{ id: cur_scanner.id + 1, probes: HashSet::new(), located: false, position: (0,0,0) };
            continue;
        }

        let cords: Vec<i32> = line.split(",").map(|x| x.parse().unwrap()).collect();
        cur_scanner.probes.insert((cords[0], cords[1], cords[2]));
    }
    scanners.push(cur_scanner.clone());

    let mut all_probes = scanners[0].probes.clone();
    let rotations = [' ', 'Y', 'R', 'R', 'R', 'Y', 'L', 'L', 'L', 'Y', 'R', 'R', 'R', 'Y', 'L', 'L', 'L', 'Y', 'R', 'R', 'R', 'Y', 'L', 'L', 'L'];
    
    while let Some(_) = scanners.iter().find(|x| !x.located) {
        'outer: for s in scanners.clone().iter().filter(|x| !x.located) {

            cur_scanner = s.clone();

            for spec in rotations {
                cur_scanner.rotate(spec);

                let mut distances = HashMap::new();
                for i in &all_probes {
                    for j in &cur_scanner.probes {
                        let d = (i.0 - j.0, i.1 - j.1, i.2 - j.2);

                        if let Some(x) = distances.get_mut(&d) {
                            *x += 1;

                            if *x == 12 {
                                for cs in &cur_scanner.probes {
                                    all_probes.insert((cs.0 + d.0, cs.1 + d.1, cs.2 + d.2));
                                }
                                scanners.iter_mut().for_each(|x| if x.id == cur_scanner.id {x.located = true; x.position = d;});
                                continue 'outer;
                            }

                        } else {
                            distances.insert(d, 1);
                        }
                    }
                }

            }

        }
    }

    println!("part 1: {}", all_probes.len());
    let b = (0..scanners.len()).permutations(2).fold(0, |a, x| cmp::max(a, i32::abs(scanners[x[0]].position.0 - scanners[x[1]].position.0) + i32::abs(scanners[x[0]].position.1 - scanners[x[1]].position.1) + i32::abs(scanners[x[0]].position.2 - scanners[x[1]].position.2)));
    println!("Part 2: {}", b);
}