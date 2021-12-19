
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
    fn _format(&self) -> String {
        let mut output = String::from(format!("--- scanner {} ---\n", self.id));
        self.probes.iter().for_each(|x| output.push_str(&format!("{},{},{}\n", x.0, x.1, x.2)));
        output.trim().to_string()
    }

    fn rotate(&self, spec: &str) -> Scanner {
        let mut ret = self.clone();
        for i in spec.chars(){
            if i == 'X' { ret.probes = self.probes.iter().map(|c| (c.0, -c.2, c.1)).collect() }
            if i == 'Y' { ret.probes = self.probes.iter().map(|c| (c.2, c.1, -c.0)).collect(); }
            if i == 'Z' { ret.probes = self.probes.iter().map(|c| (-c.1, c.0, c.2)).collect(); }
        }
        ret
    }
}

pub fn main(){    
    let mut input = include_str!("../data/day_19.txt").trim().lines();
    input.next();
    let mut scanners: Vec<Scanner> = Vec::new();

    let mut cur_scanner = Scanner{ id: 0, probes: HashSet::new(), located: false, position: (0,0,0) };
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
    let rotations = ["","X","Y","Z","XX","XY","XZ","YX","YY","ZY","ZZ","XXX","XY","XXZ","XYX","XYY","XZZ","YXX","YYY","ZZ","XXY","XXYX","XYXX","XYYY"];
    
    'outer: while let Some(_) = scanners.iter().find(|x| !x.located && x.id != 0 ) {
        for s in &scanners {
            if s.located { continue; }
            if s.id == 0 { continue; }

            for spec in rotations {
                cur_scanner = s.clone().rotate(spec);

                println!("{}\n", cur_scanner._format());

                let mut distances = HashMap::new();
                'outer2: for i in &all_probes {
                    for j in &cur_scanner.probes {
                        let d = (i.0 - j.0, i.1 - j.1, i.2 - j.2);
                        if let Some(x) = distances.get_mut(&d) {
                            *x += 1;
                            if *x == 12 {break 'outer2;}
                        } else {
                            distances.insert(d, 1);
                        }
                    }
                }

                if let Some((p,_)) = distances.iter().find(|x| *x.1 >= 12){
                    for cs in &cur_scanner.probes {
                        all_probes.insert((cs.0 + p.0, cs.1 + p.1, cs.2 + p.2));
                    }
                    scanners.iter_mut().for_each(|x| if x.id == cur_scanner.id {x.located = true; x.position = *p;});
                    continue 'outer;
                }

            }

        }
    }

    println!("part 1: {}", all_probes.len());

    let b = (0..scanners.len()).permutations(2).fold(0, |a, x| cmp::max(a, i32::abs(scanners[x[0]].position.0 - scanners[x[1]].position.0) + i32::abs(scanners[x[0]].position.1 - scanners[x[1]].position.1) + i32::abs(scanners[x[0]].position.2 - scanners[x[1]].position.2)));
    println!("Part 2: {}", b);
}