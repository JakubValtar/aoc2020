use std::collections::{HashMap, HashSet};

fn main() {
    println!("Hello, world!");
}

// 16:44
#[test]
fn day01_pt1() {
    let input = std::include_str!("inputs/day01.txt");
    let mut numbers: Vec<i32> = input.lines().map(|l| l.parse().unwrap()).collect();
    numbers.sort_unstable();
    let mut res = None;
    for (ia, &a) in numbers.iter().enumerate() {
        if let Ok(pos) = numbers[ia + 1..].binary_search_by(|b| (a + b).cmp(&2020)) {
            res = Some(a * numbers[ia + 1 + pos]);
            break;
        }
    }
    println!("{:?}", res);
}

// 8:46
#[test]
fn day01_pt2() {
    let input = std::include_str!("inputs/day01.txt");
    let mut numbers: Vec<i32> = input.lines().map(|l| l.parse().unwrap()).collect();
    numbers.sort_unstable();
    let mut res = None;
    for (ia, &a) in numbers.iter().enumerate() {
        for (ib, b) in numbers
            .iter()
            .enumerate()
            .skip(ia + 1)
            .skip_while(|(_, b)| a + *b + numbers[numbers.len() - 1] < 2020)
            .take_while(|(_, b)| a + *b + *b <= 2020)
        {
            let ab = a + b;
            if let Ok(pos) = numbers[ib + 1..].binary_search_by(|c| (ab + c).cmp(&2020)) {
                res = Some(a * b * numbers[ib + 1 + pos]);
                break;
            }
        }
    }
    println!("{:?}", res);
}

// 16:30
#[test]
fn day02_pt1() {
    let input = std::include_str!("inputs/day02.txt");

    let re = regex::Regex::new("([0-9]+)-([0-9]+) ([a-zA-Z]): ([a-zA-Z]+)").unwrap();

    let count = input
        .lines()
        .filter(|line| {
            if let Some(caps) = re.captures(line) {
                let min: u32 = caps[1].parse().unwrap();
                let max: u32 = caps[2].parse().unwrap();
                let letter: u8 = caps[3].as_bytes()[0];
                let password: &str = &caps[4];
                let count = password.bytes().filter(|&ch| ch == letter).count() as u32;
                count >= min && count <= max
            } else {
                false
            }
        })
        .count();
    println!("{}", count);
}

// 3:05
#[test]
fn day02_pt2() {
    let input = std::include_str!("inputs/day02.txt");

    let re = regex::Regex::new("([0-9]+)-([0-9]+) ([a-zA-Z]): ([a-zA-Z]+)").unwrap();

    let count = input
        .lines()
        .filter(|line| {
            if let Some(caps) = re.captures(line) {
                let pos1: usize = caps[1].parse().unwrap();
                let pos2: usize = caps[2].parse().unwrap();
                let letter: u8 = caps[3].as_bytes()[0];
                let password: &str = &caps[4];
                let bytes = password.as_bytes();
                (bytes.get(pos1 - 1) == Some(&letter)) ^ (bytes.get(pos2 - 1) == Some(&letter))
            } else {
                false
            }
        })
        .count();
    println!("{}", count);
}

// 7:01
#[test]
fn day03_pt1() {
    let input = std::include_str!("inputs/day03.txt");

    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let grid: Vec<bool> = input
        .lines()
        .flat_map(|line| line.bytes().map(|b| b == b'#'))
        .collect();

    let slope = (3, 1);

    let mut pos = (0, 0);

    let mut count = 0;

    while pos.1 < height {
        if grid[pos.0 + width * pos.1] {
            count += 1;
        }
        pos.0 += slope.0;
        pos.0 %= width;
        pos.1 += slope.1;
    }

    println!("{}", count);
}

// 2:38
#[test]
fn day03_pt2() {
    let input = std::include_str!("inputs/day03.txt");

    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let grid: Vec<bool> = input
        .lines()
        .flat_map(|line| line.bytes().map(|b| b == b'#'))
        .collect();

    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let res: usize = slopes
        .iter()
        .map(|slope| {
            let mut pos = (0, 0);
            let mut count = 0;
            while pos.1 < height {
                if grid[pos.0 + width * pos.1] {
                    count += 1;
                }
                pos.0 += slope.0;
                pos.0 %= width;
                pos.1 += slope.1;
            }
            count
        })
        .product();

    println!("{}", res);
}

// 16:58
#[test]
fn day04_pt1() {
    let input = std::include_str!("inputs/day04.txt");

    let req_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let count = input
        .split("\n\n")
        .map(|passport| {
            passport
                .lines()
                .flat_map(|line| line.split(' '))
                .map(|kv| {
                    let mut parts = kv.split(':');
                    (
                        parts.next().unwrap().to_string(),
                        parts.next().unwrap().to_string(),
                    )
                })
                .collect::<HashMap<String, String>>()
        })
        .filter(|passport| req_fields.iter().all(|f| passport.contains_key(*f)))
        .count();

    println!("{}", count);
}

// 15:42
#[test]
fn day04_pt2() {
    let input = std::include_str!("inputs/day04.txt");

    let valid_ecl = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

    let count = input
        .split("\n\n")
        .map(|passport| {
            passport
                .lines()
                .flat_map(|line| line.split(' '))
                .filter(|line| !line.trim().is_empty())
                .map(|kv| {
                    let mut parts = kv.split(':');
                    (
                        parts.next().unwrap().to_string(),
                        parts.next().unwrap().to_string(),
                    )
                })
                .collect::<HashMap<String, String>>()
        })
        .filter(|p| {
            p.get("byr")
                .and_then(|v| v.parse::<i32>().ok())
                .filter(|&v| v >= 1920 && v <= 2002)
                //
                .and_then(|_| p.get("iyr"))
                .and_then(|v| v.parse::<i32>().ok())
                .filter(|&v| v >= 2010 && v <= 2020)
                //
                .and_then(|_| p.get("eyr"))
                .and_then(|v| v.parse::<i32>().ok())
                .filter(|&v| v >= 2020 && v <= 2030)
                //
                .and_then(|_| p.get("hgt"))
                .and_then(|v| {
                    if let Some(cm) = v.strip_suffix("cm") {
                        cm.parse::<i32>().ok().filter(|&v| v >= 150 && v <= 193)
                    } else if let Some(inch) = v.strip_suffix("in") {
                        inch.parse::<i32>().ok().filter(|&v| v >= 59 && v <= 76)
                    } else {
                        None
                    }
                })
                //
                .and_then(|_| p.get("hcl"))
                .and_then(|v| v.strip_prefix('#'))
                .filter(|v| {
                    v.chars().all(|ch| {
                        ch.is_ascii_digit()
                            || (ch.is_ascii_lowercase() && ch as u8 >= b'a' && ch as u8 <= b'f')
                    })
                })
                //
                .and_then(|_| p.get("ecl"))
                .filter(|v| valid_ecl.contains(&v.as_str()))
                //
                .and_then(|_| p.get("pid"))
                .filter(|v| v.len() == 9 && v.chars().all(|ch| ch.is_ascii_digit()))
                //
                .is_some()
        })
        .count();

    println!("{}", count);
}

// 6:32
#[test]
fn day05_pt1() {
    let input = std::include_str!("inputs/day05.txt");

    let max = input
        .lines()
        .map(|line| {
            let mut v = 0;
            for (i, b) in line.bytes().enumerate() {
                let x = (b == b'B' || b == b'R') as u16;
                v |= x << (9 - i);
            }
            v
        })
        .max();

    println!("{:?}", max);
}

// 4:39
#[test]
fn day05_pt2() {
    let input = std::include_str!("inputs/day05.txt");

    let mut seats: Vec<_> = input
        .lines()
        .map(|line| {
            let mut v = 0;
            for (i, b) in line.bytes().enumerate() {
                let x = (b == b'B' || b == b'R') as u16;
                v |= x << (9 - i);
            }
            v
        })
        .collect();

    seats.sort_unstable();

    let mut res = None;
    for w in seats.windows(2) {
        if w[0] + 2 == w[1] {
            res = Some(w[0] + 1);
            break;
        }
    }

    println!("{:?}", res);
}

// 4:23
#[test]
fn day06_pt1() {
    let input = std::include_str!("inputs/day06.txt");

    let count: usize = input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .flat_map(|person| person.bytes())
                .collect::<HashSet<u8>>()
        })
        .map(|unique| unique.len())
        .sum();

    println!("{}", count);
}

// 3:29
#[test]
fn day06_pt2() {
    let input = std::include_str!("inputs/day06.txt");

    let count: usize = input
        .split("\n\n")
        .map(|group| {
            let mut set: HashSet<_> = (b'a'..=b'z').collect();
            group
                .lines()
                .for_each(|person| set.retain(|a| person.as_bytes().contains(a)));
            set
        })
        .map(|unique| unique.len())
        .sum();

    println!("{}", count);
}

// 44:49
#[test]
fn day07_pt1() {
    let input = std::include_str!("inputs/day07.txt");

    let rules = input
        .lines()
        .flat_map(|line| {
            let mut words = line.split(' ');
            let mut from = String::new();
            from.push_str(words.next().unwrap());
            from.push(' ');
            from.push_str(words.next().unwrap());

            words.next(); // bags
            words.next(); // contain

            let mut to = Vec::new();

            while let Some(count) = words.next() {
                let count = match count.parse::<u32>() {
                    Ok(count) => count,
                    _ => break,
                };
                let mut color = String::new();
                color.push_str(words.next().unwrap());
                color.push(' ');
                color.push_str(words.next().unwrap());
                words.next(); // bags

                to.push((count, color));
            }

            to.into_iter().map(move |to| (from.clone(), to.1))
        })
        .collect::<Vec<(String, String)>>();

    let mut parents: HashMap<String, Vec<String>> = HashMap::new();
    for (from, to) in rules {
        parents.entry(to).or_default().push(from);
    }

    let mut visited = HashSet::new();
    let mut to_visit_stack = vec![String::from("shiny gold")];
    while let Some(to_visit) = to_visit_stack.pop() {
        for parent in parents.get(&to_visit).map(|v| v.as_slice()).unwrap_or(&[]) {
            if visited.insert(parent.clone()) {
                to_visit_stack.push(parent.clone());
            }
        }
    }

    println!("{}", visited.len());
}

// 9:38
#[test]
fn day07_pt2() {
    let input = std::include_str!("inputs/day07.txt");

    let rules = input
        .lines()
        .map(|line| {
            let mut words = line.split(' ');
            let mut from = String::new();
            from.push_str(words.next().unwrap());
            from.push(' ');
            from.push_str(words.next().unwrap());

            words.next(); // bags
            words.next(); // contain

            let mut to = Vec::new();

            while let Some(count) = words.next() {
                let count = match count.parse::<u32>() {
                    Ok(count) => count,
                    _ => break, // no other bags
                };
                let mut color = String::new();
                color.push_str(words.next().unwrap());
                color.push(' ');
                color.push_str(words.next().unwrap());
                words.next(); // bags

                to.push((count, color));
            }

            (from, to)
        })
        .collect::<HashMap<String, Vec<(u32, String)>>>();

    fn count_inner_bags(rules: &HashMap<String, Vec<(u32, String)>>, bag: &str) -> u32 {
        let mut sum = 0;
        if let Some(inner_bags) = rules.get(bag) {
            for (cnt, inner_bag) in inner_bags {
                sum += cnt * (1 + count_inner_bags(rules, inner_bag));
            }
        }
        sum
    }

    let count = count_inner_bags(&rules, "shiny gold");

    println!("{}", count);
}
