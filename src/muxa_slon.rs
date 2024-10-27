fn find_solutions() -> Vec<(i32, i32)> {
    let mut solutions = Vec::new();
    
    for m in 1..=8 {
        for u in 1..=8 {
            if u == m { continue; }
            for x in 1..=8 {
                if x == m || x == u { continue; }
                for a in 1..=8 {
                    if a == m || a == u || a == x { continue; }
                    let muxa = m * 1000 + u * 100 + x * 10 + a;
                    for s in 1..=8 {
                        if s == m || s == u || s == x || s == a { continue; }
                        for l in 1..=8 {
                            if l == m || l == u || l == x || l == a || l == s { continue; }
                            for o in 1..=8 {
                                if o == m || o == u || o == x || o == a || o == s || o == l { continue; }
                                for n in 1..=8 {
                                    if n == m || n == u || n == x || n == a || n == s || n == l || n == o { continue; }
                                    let slon = s * 1000 + l * 100 + o * 10 + n;
                                    if muxa * a == slon {
                                        solutions.push((muxa, slon));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    solutions
}

pub fn main() {
    let solutions = find_solutions();
    for (muxa, slon) in &solutions {
        println!("{} x {} = {}", muxa, muxa % 10, slon);
    }
    println!("Кількість рішень: {}", solutions.len());
}
