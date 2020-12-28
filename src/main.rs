use std::{
    collections::{HashMap, HashSet},
    iter, mem,
    ops::RangeInclusive,
};

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

// 11:27
#[test]
fn day08_pt1() {
    let input = std::include_str!("inputs/day08.txt");

    enum Instr {
        Nop,
        Acc(i32),
        Jmp(i32),
    }

    let program: Vec<_> = input
        .lines()
        .map(|line| {
            if line.starts_with("nop") {
                Instr::Nop
            } else if let Some(n) = line.strip_prefix("acc ") {
                Instr::Acc(n.parse().unwrap())
            } else if let Some(n) = line.strip_prefix("jmp ") {
                Instr::Jmp(n.parse().unwrap())
            } else {
                unreachable!()
            }
        })
        .collect();

    let mut pc = 0;
    let mut acc = 0;
    let mut visited = vec![false; program.len()];

    while !visited[pc] {
        visited[pc] = true;
        match program[pc] {
            Instr::Nop => pc += 1,
            Instr::Acc(n) => {
                acc += n;
                pc += 1;
            }
            Instr::Jmp(n) => pc = (pc as i32 + n) as usize,
        }
    }

    println!("{}", acc);
}

// 6:57
#[test]
fn day08_pt2() {
    let input = std::include_str!("inputs/day08.txt");

    enum Instr {
        Nop(i32),
        Acc(i32),
        Jmp(i32),
    }

    let mut program: Vec<_> = input
        .lines()
        .map(|line| {
            if let Some(n) = line.strip_prefix("nop ") {
                Instr::Nop(n.parse().unwrap())
            } else if let Some(n) = line.strip_prefix("acc ") {
                Instr::Acc(n.parse().unwrap())
            } else if let Some(n) = line.strip_prefix("jmp ") {
                Instr::Jmp(n.parse().unwrap())
            } else {
                unreachable!()
            }
        })
        .collect();

    for i in 0..program.len() {
        match program[i] {
            Instr::Acc(_) => continue,
            Instr::Jmp(n) => program[i] = Instr::Nop(n),
            Instr::Nop(n) => program[i] = Instr::Jmp(n),
        }

        let mut pc = 0;
        let mut acc = 0;
        let mut visited = vec![false; program.len()];

        while pc < program.len() && !visited[pc] {
            visited[pc] = true;
            match program[pc] {
                Instr::Nop(_) => pc += 1,
                Instr::Acc(n) => {
                    acc += n;
                    pc += 1;
                }
                Instr::Jmp(n) => pc = (pc as i32 + n) as usize,
            }
        }

        match program[i] {
            Instr::Acc(_) => unreachable!(),
            Instr::Jmp(n) => program[i] = Instr::Nop(n),
            Instr::Nop(n) => program[i] = Instr::Jmp(n),
        }

        if pc == program.len() {
            println!("{}", acc);
            break;
        }
    }
}

// 10:25
#[test]
fn day09_pt1() {
    let input = std::include_str!("inputs/day09.txt");
    let numbers: Vec<i64> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut invalid = None;
    let mut buf = [0; 25];
    for (i, &n) in numbers.iter().enumerate().skip(25) {
        buf.copy_from_slice(&numbers[i - 25..i]);
        buf.sort_unstable();
        let mut valid = false;
        'outer: for (ia, a) in buf.iter().enumerate() {
            for b in buf.iter().skip(ia + 1) {
                if a + b == n {
                    valid = true;
                    break 'outer;
                }
            }
        }
        if !valid {
            invalid = Some(n);
            break;
        }
    }
    println!("{:?}", invalid);
}

// 6:32
#[test]
fn day09_pt2() {
    let input = std::include_str!("inputs/day09.txt");
    let numbers: Vec<i64> = input.lines().map(|l| l.parse().unwrap()).collect();
    let invalid = 177777905;
    let mut res = None;
    'outer: for window_size in 2..numbers.len() {
        for window in numbers.windows(window_size) {
            if window.iter().sum::<i64>() == invalid {
                res = Some(window.iter().min().unwrap() + window.iter().max().unwrap());
                break 'outer;
            }
        }
    }

    println!("{:?}", res);
}

// 7:17
#[test]
fn day10_pt1() {
    let input = std::include_str!("inputs/day10.txt");
    let mut adapters: Vec<i64> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut hist = [0; 4];
    adapters.push(0);
    adapters.push(adapters.iter().max().unwrap() + 3);
    adapters.sort_unstable();
    for pair in adapters.windows(2) {
        hist[(pair[1] - pair[0]) as usize] += 1;
    }

    let res = hist[1] * hist[3];

    println!("{:?}", res);
}

// 12:50
#[test]
fn day10_pt2() {
    let input = std::include_str!("inputs/day10.txt");
    let mut adapters: Vec<i64> = input.lines().map(|l| l.parse().unwrap()).collect();
    adapters.push(0);
    adapters.sort_unstable();
    let mut combs = vec![1usize];

    for i in 1..adapters.len() {
        let current = adapters[i];
        let c = (0..i)
            .rev()
            .take_while(|&j| adapters[j as usize] >= current - 3)
            .map(|j| combs[j as usize])
            .sum();
        combs.push(c);
    }

    println!("{:?}", combs.last());
}

// 26:16
#[test]
fn day11_pt1() {
    let input = std::include_str!("inputs/day11.txt");

    let width = input.lines().next().unwrap().len() + 2;
    let height = input.lines().count() + 2;

    #[derive(Copy, Clone, PartialEq)]
    #[repr(u8)]
    enum State {
        Floor,
        Empty,
        Occupied,
    }

    let mut grid: Vec<State> = iter::repeat(State::Floor)
        .take(width)
        .chain(input.lines().flat_map(|l| {
            iter::once(State::Floor)
                .chain(l.bytes().map(|c| match c {
                    b'.' => State::Floor,
                    b'L' => State::Empty,
                    _ => State::Occupied,
                }))
                .chain(iter::once(State::Floor))
        }))
        .chain(iter::repeat(State::Floor).take(width))
        .collect();

    let mut grid2 = grid.clone();

    loop {
        for y in 1..height - 1 {
            for x in 1..width - 1 {
                let mut neighbors = 0;
                for &yy in &[-1isize, 0, 1] {
                    for &xx in &[-1isize, 0, 1] {
                        if yy == 0 && xx == 0 {
                            continue;
                        }
                        let id =
                            (x.wrapping_add(xx as usize)) + (y.wrapping_add(yy as usize)) * width;
                        neighbors += (grid[id] == State::Occupied) as u8;
                    }
                }
                let id = x + y * width;
                grid2[id] = match (grid[id], neighbors) {
                    (State::Empty, 0) => State::Occupied,
                    (State::Occupied, 4..=255) => State::Empty,
                    _ => grid[id],
                };
            }
        }
        if grid == grid2 {
            break;
        }
        mem::swap(&mut grid, &mut grid2);
    }

    let res: usize = grid.iter().map(|&s| (s == State::Occupied) as usize).sum();

    println!("{:?}", res);
}

// 6:11
#[test]
fn day11_pt2() {
    let input = std::include_str!("inputs/day11.txt");

    let width = input.lines().next().unwrap().len() + 2;
    let height = input.lines().count() + 2;

    #[derive(Copy, Clone, PartialEq)]
    #[repr(u8)]
    enum State {
        Floor,
        Empty,
        Occupied,
    }

    let mut grid: Vec<State> = iter::repeat(State::Floor)
        .take(width)
        .chain(input.lines().flat_map(|l| {
            iter::once(State::Floor)
                .chain(l.bytes().map(|c| match c {
                    b'.' => State::Floor,
                    b'L' => State::Empty,
                    _ => State::Occupied,
                }))
                .chain(iter::once(State::Floor))
        }))
        .chain(iter::repeat(State::Floor).take(width))
        .collect();

    let mut grid2 = grid.clone();

    loop {
        for y in 1..height - 1 {
            for x in 1..width - 1 {
                let mut neighbors = 0;
                for &yy in &[-1isize, 0, 1] {
                    for &xx in &[-1isize, 0, 1] {
                        if yy == 0 && xx == 0 {
                            continue;
                        }
                        let mut x = x;
                        let mut y = y;
                        loop {
                            x = x.wrapping_add(xx as usize);
                            y = y.wrapping_add(yy as usize);
                            if x == 0 || x >= width || y == 0 || y >= height {
                                break;
                            }
                            let id = x + y * width;
                            match grid[id] {
                                State::Occupied => {
                                    neighbors += 1;
                                    break;
                                }
                                State::Empty => break,
                                _ => (),
                            }
                        }
                    }
                }
                let id = x + y * width;
                grid2[id] = match (grid[id], neighbors) {
                    (State::Empty, 0) => State::Occupied,
                    (State::Occupied, 5..=255) => State::Empty,
                    _ => grid[id],
                };
            }
        }
        if grid == grid2 {
            break;
        }
        mem::swap(&mut grid, &mut grid2);
    }

    let res: usize = grid.iter().map(|&s| (s == State::Occupied) as usize).sum();

    println!("{:?}", res);
}

// 13:53
#[test]
fn day12_pt1() {
    let input = std::include_str!("inputs/day12.txt");

    let mut pos = (0, 0);
    let mut heading = (1, 0);

    for line in input.lines() {
        let cmd = line.as_bytes()[0];
        let mut amt = line[1..].parse::<i32>().unwrap();
        match cmd {
            b'N' => pos.1 += amt,
            b'S' => pos.1 -= amt,
            b'E' => pos.0 += amt,
            b'W' => pos.0 -= amt,
            b'F' => {
                pos.0 += heading.0 * amt;
                pos.1 += heading.1 * amt;
            }
            b'L' => {
                while amt > 0 {
                    let h0 = heading.0;
                    heading.0 = -heading.1;
                    heading.1 = h0;
                    amt -= 90;
                }
            }
            b'R' => {
                while amt > 0 {
                    let h0 = heading.0;
                    heading.0 = heading.1;
                    heading.1 = -h0;
                    amt -= 90;
                }
            }
            _ => unreachable!(),
        }
    }

    let res = pos.0.abs() + pos.1.abs();

    println!("{}", res);
}

// 3:32
#[test]
fn day12_pt2() {
    let input = std::include_str!("inputs/day12.txt");

    let mut pos = (0, 0);
    let mut heading = (10, 1);

    for line in input.lines() {
        let cmd = line.as_bytes()[0];
        let mut amt = line[1..].parse::<i32>().unwrap();
        match cmd {
            b'N' => heading.1 += amt,
            b'S' => heading.1 -= amt,
            b'E' => heading.0 += amt,
            b'W' => heading.0 -= amt,
            b'F' => {
                pos.0 += heading.0 * amt;
                pos.1 += heading.1 * amt;
            }
            b'L' => {
                while amt > 0 {
                    let h0 = heading.0;
                    heading.0 = -heading.1;
                    heading.1 = h0;
                    amt -= 90;
                }
            }
            b'R' => {
                while amt > 0 {
                    let h0 = heading.0;
                    heading.0 = heading.1;
                    heading.1 = -h0;
                    amt -= 90;
                }
            }
            _ => unreachable!(),
        }
    }

    let res = pos.0.abs() + pos.1.abs();

    println!("{}", res);
}

// 13:04
#[test]
fn day13_pt1() {
    let input = std::include_str!("inputs/day13.txt");

    let mut lines = input.lines();

    let now = lines.next().unwrap().parse::<u32>().unwrap();

    let res = lines
        .next()
        .unwrap()
        .split(',')
        .filter_map(|b| b.parse::<u32>().ok())
        .map(|b| {
            let next = ((now - 1) / b + 1) * b;
            (b, next - now)
        })
        .min_by_key(|&(_, wait)| wait)
        .unwrap();

    dbg!(res.0, res.1);

    println!("{}", res.0 * res.1);
}

// 2:42:35 ^-^'
#[test]
fn day13_pt2() {
    let input = std::include_str!("inputs/day13.txt");

    let mut lines = input.lines();

    lines.next().unwrap();

    /// Multiplicative Inverse: a * a⁻¹ ≡ 1 (mod n)
    fn multiplicative_inverse(a: i128, n: i128) -> Option<i128> {
        let (mut t, mut newt) = (0, 1);
        let (mut r, mut newr) = (n, a);
        while newr != 0 {
            t -= r / newr * newt;
            r %= newr;
            mem::swap(&mut t, &mut newt);
            mem::swap(&mut r, &mut newr);
        }
        match (r, t) {
            (r, _) if r > 1 => None,
            (_, t) if t < 0 => Some(t + n),
            (_, t) => Some(t),
        }
    }

    struct Bus {
        period: i128,
        delay: i128,
    };

    let busses: Vec<Bus> = lines
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter_map(|(delay, period)| {
            period
                .parse::<i128>()
                .map(|period| Bus {
                    period,
                    delay: delay as i128,
                })
                .ok()
        })
        .collect();

    /*
        // Common period of all busses (when all busses meet at the port)
        p_lc = p_1 * ... * p_n

        // Congruence relation for a single bus line w/ period `p`:
        //   leaves `d` minutes after the timestamp `x`
        x + d ≡ 0 (mod p)
        x ≡ -d (mod p)

        // Multiply by `(p_lc / p)` to get all relations to modulo `p_lc`
        (p_lc / p) * x ≡ (p_lc / p) * -d (mod (p_lc / p) * p)

        // Sum the relations for all busses together
        (p_lc/p_1) + ... + (p_lc/p_n) * x ≡ ((p_lc/p_1) * -d_1) + ... + ((p_lc/p_n) * -d_n) (mod p_lc)
        ------------- a ------------- * x ≡ ---------------------- b ---------------------- (mod p_lc)
        a * x ≡ b (mod p_lc)

        // Use multiplicative inverse of `a` to get rid of `a` in front of `x`
        (a * a⁻¹) * x ≡ a⁻¹ * b (mod p_lc)
                1 * x ≡ a⁻¹ * b (mod p_lc)

        // Final solution
        x = (a⁻¹ * b) % p_lc
    */

    // Get the least common multiple of all line periods
    let period_lc = busses.iter().map(|bus| bus.period).product();

    let e = busses
        .iter()
        .map(|b| (period_lc / b.period, -b.delay * period_lc / b.period))
        .fold((0, 0), |acc, b| (acc.0 + b.0, acc.1 + b.1));

    let a = (e.0 % period_lc + period_lc) % period_lc;
    let b = (e.1 % period_lc + period_lc) % period_lc;
    let inv = multiplicative_inverse(a, period_lc).unwrap();
    let res = (b * inv) % period_lc;

    println!("{}", res);
}

#[test]
fn day13_pt2b() {
    let input = std::include_str!("inputs/day13.txt");

    let mut lines = input.lines();

    lines.next().unwrap().parse::<u32>().unwrap();

    struct Bus {
        period: i64,
        delay: i64,
    };

    let mut busses: Vec<Bus> = lines
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter_map(|(delay, period)| {
            period
                .parse::<i64>()
                .map(|period| Bus {
                    period,
                    delay: delay as i64,
                })
                .ok()
        })
        .collect();

    busses.sort_by_key(|b| b.period);

    let mut bus = &busses[0];
    let mut i = 0;
    let mut step = 1;
    let mut ts = 100_000_000_000_000i64;
    let res = loop {
        if (ts + bus.delay) % bus.period == 0 {
            step *= bus.period;
            i += 1;
            if i == busses.len() {
                break ts;
            }
            bus = &busses[i];
        }

        ts += step;
    };

    println!("{}", res);
}

// 17:39
#[test]
fn day14_pt1() {
    let input = std::include_str!("inputs/day14.txt");

    let mut or_mask = 0u64;
    let mut and_mask = u64::MAX;
    let mut memory: HashMap<u64, u64> = HashMap::new();

    for line in input.lines() {
        if let Some(mem) = line.strip_prefix("mem[") {
            let addr = mem
                .chars()
                .take_while(|ch| ch.is_ascii_digit())
                .collect::<String>()
                .parse::<u64>()
                .unwrap();
            let mut val = mem.split(" = ").nth(1).unwrap().parse::<u64>().unwrap();
            val |= or_mask;
            val &= and_mask;
            *memory.entry(addr).or_default() = val;
        } else if let Some(mask) = line.strip_prefix("mask = ") {
            or_mask = 0;
            and_mask = u64::MAX;
            for (i, ch) in mask.chars().rev().enumerate() {
                match ch {
                    '0' => and_mask &= !(1 << i),
                    '1' => or_mask |= 1 << i,
                    'X' => (),
                    _ => unreachable!(),
                }
            }
        } else {
            unreachable!()
        }
    }

    let res = memory.values().sum::<u64>();

    println!("{}", res);
}

// 38:04
#[test]
fn day14_pt2() {
    let input = std::include_str!("inputs/day14.txt");

    let mut or_mask = 0;
    let mut and_mask = (1 << 36) - 1;
    let mut floating = vec![];
    let mut memory: HashMap<u64, u64> = HashMap::new();

    for line in input.lines() {
        if let Some(mem) = line.strip_prefix("mem[") {
            let mut addr = mem
                .chars()
                .take_while(|ch| ch.is_ascii_digit())
                .collect::<String>()
                .parse::<u64>()
                .unwrap();
            let val = mem.split(" = ").nth(1).unwrap().parse::<u64>().unwrap();

            addr |= or_mask;
            addr &= and_mask;

            for i in 0..(1u64 << floating.len()) {
                let addr = addr
                    | floating
                        .iter()
                        .enumerate()
                        .map(|(pos, &m)| ((i >> pos) & 1) * m)
                        .sum::<u64>();
                *memory.entry(addr).or_default() = val;
            }
        } else if let Some(mask) = line.strip_prefix("mask = ") {
            or_mask = 0;
            and_mask = (1 << 36) - 1;
            floating.clear();
            for (i, ch) in mask.chars().rev().enumerate() {
                match ch {
                    '0' => (),
                    '1' => or_mask |= 1 << i,
                    'X' => {
                        and_mask &= !(1 << i);
                        floating.push(1 << i);
                    }
                    _ => unreachable!(),
                }
            }
        } else {
            unreachable!()
        }
    }

    let res = memory.values().sum::<u64>();

    println!("{}", res);
}

// 29:48
#[test]
fn day15_pt1() {
    let input = std::include_str!("inputs/day15.txt");

    let mut history = HashMap::new();

    let mut last = 0;

    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|ch| ch.parse::<usize>().unwrap())
        .enumerate()
        .for_each(|(turn, n)| {
            if turn > 0 {
                history.insert(last, turn);
            }
            last = n;
        });

    for turn in history.len() + 1..2020 {
        let last_turn = *history.get(&last).unwrap_or(&turn);
        history.insert(last, turn);
        last = turn - last_turn;
    }

    println!("{}", last);
}

// 0:40
#[test]
fn day15_pt2() {
    let input = std::include_str!("inputs/day15.txt");

    let mut history = HashMap::new();

    let mut last = 0;

    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|ch| ch.parse::<usize>().unwrap())
        .enumerate()
        .for_each(|(turn, n)| {
            if turn > 0 {
                history.insert(last, turn);
            }
            last = n;
        });

    for turn in history.len() + 1..30000000 {
        let last_turn = *history.get(&last).unwrap_or(&turn);
        history.insert(last, turn);
        last = turn - last_turn;
    }

    println!("{}", last);
}

// 19:24
#[test]
fn day16_pt1() {
    let input = std::include_str!("inputs/day16.txt");

    struct Field {
        _name: String,
        range1: RangeInclusive<usize>,
        range2: RangeInclusive<usize>,
    }

    let fields = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let mut parts = line.split(": ");
            let name = parts.next().unwrap();
            let mut ranges = parts.next().unwrap().split(" or ");
            let mut range1 = ranges
                .next()
                .unwrap()
                .split('-')
                .map(|num| num.parse::<usize>().unwrap());
            let mut range2 = ranges
                .next()
                .unwrap()
                .split('-')
                .map(|num| num.parse::<usize>().unwrap());
            Field {
                _name: name.to_owned(),
                range1: range1.next().unwrap()..=range1.next().unwrap(),
                range2: range2.next().unwrap()..=range2.next().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    let res = input
        .lines()
        .skip_while(|&line| line != "nearby tickets:")
        .skip(1)
        .map(|line| {
            line.split(',')
                .map(|val| val.parse::<usize>().unwrap())
                .filter(|val| {
                    fields
                        .iter()
                        .all(|f| !f.range1.contains(val) && !f.range2.contains(val))
                })
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("{}", res);
}

// 1:16:56
#[test]
fn day16_pt2() {
    let input = std::include_str!("inputs/day16.txt");

    #[derive(Debug)]
    struct Rule {
        name: String,
        range1: RangeInclusive<usize>,
        range2: RangeInclusive<usize>,
    }

    let rules = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let mut parts = line.split(": ");
            let name = parts.next().unwrap();
            let mut ranges = parts.next().unwrap().split(" or ");
            let mut range1 = ranges
                .next()
                .unwrap()
                .split('-')
                .map(|num| num.parse::<usize>().unwrap());
            let mut range2 = ranges
                .next()
                .unwrap()
                .split('-')
                .map(|num| num.parse::<usize>().unwrap());
            Rule {
                name: name.to_owned(),
                range1: range1.next().unwrap()..=range1.next().unwrap(),
                range2: range2.next().unwrap()..=range2.next().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    let mut tickets = input
        .lines()
        .skip_while(|&line| line != "nearby tickets:")
        .skip(1)
        .map(|line| {
            let mut arr = [0usize; 20];
            line.split(',')
                .map(|val| val.parse::<usize>().unwrap())
                .enumerate()
                .for_each(|(i, val)| arr[i] = val);
            arr
        })
        .filter(|ticket| {
            ticket.iter().all(|val| {
                rules
                    .iter()
                    .any(|rule| rule.range1.contains(val) || rule.range2.contains(val))
            })
        })
        .collect::<Vec<_>>();

    let my_ticket = input
        .lines()
        .skip_while(|&line| line != "your ticket:")
        .skip(1)
        .map(|line| {
            let mut arr = [0usize; 20];
            line.split(',')
                .map(|val| val.parse::<usize>().unwrap())
                .enumerate()
                .for_each(|(i, val)| arr[i] = val);
            arr
        })
        .next()
        .unwrap();

    tickets.push(my_ticket);

    let mut rule_valid_fields: Vec<HashSet<usize>> = vec![(0..20).collect(); 20];

    for ticket in &tickets {
        for (valid_fields, rule) in rule_valid_fields.iter_mut().zip(rules.iter()) {
            if valid_fields.len() > 1 {
                valid_fields.retain(|&field| {
                    let val = ticket[field];
                    rule.range1.contains(&val) || rule.range2.contains(&val)
                });
            }
        }
    }

    let mut rule_field: [Option<usize>; 20] = [None; 20];

    loop {
        for rule in 0..20 {
            if rule_field[rule] == None && rule_valid_fields[rule].len() == 1 {
                let field = *rule_valid_fields[rule].iter().next().unwrap();
                rule_field[rule] = Some(field);
                for valid_fields in &mut rule_valid_fields {
                    valid_fields.remove(&field);
                }
            }
        }
        if rule_field.iter().all(|f| f.is_some()) {
            break;
        }
    }

    let res: usize = rule_field
        .iter()
        .zip(rules)
        .filter(|(_, rule)| rule.name.starts_with("departure"))
        .map(|(field, _)| my_ticket[field.unwrap()])
        .product();

    println!("{}", res);
}

// 18:45
#[test]
fn day17_pt1() {
    let input = std::include_str!("inputs/day17.txt");

    let width0 = input.lines().next().unwrap().len();
    let height0 = input.lines().count();
    let depth0 = 1;

    let step_count = 6;
    let margin = step_count + 1;

    let width = width0 + 2 * margin;
    let height = height0 + 2 * margin;
    let depth = depth0 + 2 * margin;

    let id = |x: usize, y: usize, z: usize| z * width * height + y * width + x;

    let mut grid = vec![false; width * height * depth];
    let mut grid2 = grid.clone();

    for (y, line) in input.lines().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            grid[id(x + margin, y + margin, 0 + margin)] = cell == '#';
        }
    }

    for _ in 0..step_count {
        for z in 1..depth - 1 {
            for y in 1..height - 1 {
                for x in 1..width - 1 {
                    let mut neighbor_count = 0;
                    for zz in z - 1..=z + 1 {
                        for yy in y - 1..=y + 1 {
                            for xx in x - 1..=x + 1 {
                                neighbor_count += grid[id(xx, yy, zz)] as usize;
                            }
                        }
                    }
                    let cell_id = id(x, y, z);
                    let state = grid[cell_id];
                    neighbor_count -= state as usize;
                    grid2[cell_id] = match (state, neighbor_count) {
                        (true, 2..=3) => true,
                        (false, 3) => true,
                        _ => false,
                    };
                }
            }
        }
        mem::swap(&mut grid, &mut grid2);
    }

    let res = grid.iter().filter(|&&state| state).count();

    println!("{}", res);
}

// 4:44
#[test]
fn day17_pt2() {
    let input = std::include_str!("inputs/day17.txt");

    let width0 = input.lines().next().unwrap().len();
    let height0 = input.lines().count();
    let depth0 = 1;
    let wepth0 = 1;

    let step_count = 6;
    let margin = step_count + 1;

    let width = width0 + 2 * margin;
    let height = height0 + 2 * margin;
    let depth = depth0 + 2 * margin;
    let wepth = wepth0 + 2 * margin;

    let id = |x: usize, y: usize, z: usize, w: usize| ((w * depth + z) * height + y) * width + x;

    let mut grid = vec![false; width * height * depth * wepth];
    let mut grid2 = grid.clone();

    for (y, line) in input.lines().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            grid[id(x + margin, y + margin, 0 + margin, 0 + margin)] = cell == '#';
        }
    }

    for _ in 0..step_count {
        for w in 1..wepth - 1 {
            for z in 1..depth - 1 {
                for y in 1..height - 1 {
                    for x in 1..width - 1 {
                        let mut neighbor_count = 0;
                        for ww in w - 1..=w + 1 {
                            for zz in z - 1..=z + 1 {
                                for yy in y - 1..=y + 1 {
                                    for xx in x - 1..=x + 1 {
                                        neighbor_count += grid[id(xx, yy, zz, ww)] as usize;
                                    }
                                }
                            }
                        }
                        let cell_id = id(x, y, z, w);
                        let state = grid[cell_id];
                        neighbor_count -= state as usize;
                        grid2[cell_id] = match (state, neighbor_count) {
                            (true, 2..=3) => true,
                            (false, 3) => true,
                            _ => false,
                        };
                    }
                }
            }
        }
        mem::swap(&mut grid, &mut grid2);
    }

    let res = grid.iter().filter(|&&state| state).count();

    println!("{}", res);
}

// 22:52
#[test]
fn day18_pt1() {
    let input = std::include_str!("inputs/day18.txt");

    #[derive(Debug)]
    enum Op {
        Add,
        Mul,
    }

    fn eval_expr(expr: &str, pos: &mut usize) -> usize {
        let mut acc: usize = 0;
        let mut op: Op = Op::Add;
        while *pos < expr.len() {
            *pos += 1;
            match expr.as_bytes()[*pos - 1] {
                d if d.is_ascii_digit() => match op {
                    Op::Add => acc += (d - b'0') as usize,
                    Op::Mul => acc *= (d - b'0') as usize,
                },
                b'(' => {
                    let d = eval_expr(expr, pos);
                    match op {
                        Op::Add => acc += d,
                        Op::Mul => acc *= d,
                    }
                }
                b'+' => op = Op::Add,
                b'*' => op = Op::Mul,
                b' ' => (),
                b')' => break,
                _ => unreachable!(),
            }
        }
        acc
    }

    let mut sum: usize = 0;

    for line in input.lines() {
        sum += eval_expr(line, &mut 0);
    }

    println!("{}", sum);
}

// 21:42
#[test]
fn day18_pt2() {
    let input = std::include_str!("inputs/day18.txt");

    fn eval_expr(expr: &str, pos: &mut usize) -> usize {
        let mut mul_acc: usize = 1;
        let mut add_acc: usize = 0;
        while *pos < expr.len() {
            *pos += 1;
            match expr.as_bytes()[*pos - 1] {
                d if d.is_ascii_digit() => {
                    add_acc += (d - b'0') as usize;
                }
                b'(' => {
                    add_acc += eval_expr(expr, pos);
                }
                b'+' => (),
                b'*' => {
                    // Multiplication ends the previous chain of additions
                    // Submit it and clear add_acc to get ready for the next chain
                    mul_acc *= add_acc;
                    add_acc = 0;
                }
                b' ' => (),
                b')' => break,
                _ => unreachable!(),
            }
        }
        mul_acc *= add_acc;
        mul_acc
    }

    let mut sum: usize = 0;

    for line in input.lines() {
        sum += eval_expr(line, &mut 0);
    }

    println!("{}", sum);
}

// 1:30:48
#[test]
fn day19_pt1() {
    let input = std::include_str!("inputs/day19.txt");

    let mut literals: HashMap<u8, usize> = HashMap::new();
    let mut rules: HashMap<usize, Rule> = HashMap::new();

    struct Rule(Vec<Vec<usize>>);

    fn parse_seq(s: &str) -> Vec<usize> {
        let parts = s.split(' ');
        let seq: Vec<usize> = parts
            .map(|p| p.trim())
            .map(|p| p.parse::<usize>().unwrap())
            .collect();
        seq
    }

    fn parse_rule(s: &str) -> Vec<Vec<usize>> {
        let parts = s.split(" | ");
        parts.map(|p| parse_seq(p)).collect::<Vec<_>>()
    }

    for line in input.lines() {
        if line.is_empty() {
            break;
        }
        let mut parts = line.split(": ");
        let num = parts.next().unwrap().parse::<usize>().unwrap();
        let rule = parts.next().unwrap();
        let bytes = rule.as_bytes();
        if bytes[0] == b'\"' {
            literals.insert(bytes[1], num);
        } else {
            rules.insert(num, Rule(parse_rule(rule)));
        }
    }

    let mut sum = 0;

    fn match_seq(
        target: &[usize],
        stack: &mut Vec<usize>,
        lits: &[usize],
        rules: &HashMap<usize, Rule>,
    ) -> bool {
        if target.is_empty() && stack.is_empty() {
            true
        } else if target.is_empty() != stack.is_empty() {
            false
        } else {
            let next = stack.pop().unwrap();
            if next == target[0] {
                if match_seq(&target[1..], stack, lits, rules) {
                    return true;
                }
            } else if !lits.contains(&next) {
                let rule = rules.get(&next).unwrap();
                for opt in &rule.0 {
                    for &sym in opt.iter().rev() {
                        stack.push(sym);
                    }
                    if match_seq(target, stack, lits, rules) {
                        return true;
                    }
                    for _ in opt.iter() {
                        stack.pop();
                    }
                }
            }
            stack.push(next);
            false
        }
    };

    let lit_a = *literals.get(&b'a').unwrap();
    let lit_b = *literals.get(&b'b').unwrap();
    let lits = [lit_a, lit_b];
    let mut stack = Vec::new();

    for line in input.lines().skip_while(|l| !l.is_empty()).skip(1) {
        let bytes = line.as_bytes();
        let seq: Vec<usize> = bytes.iter().map(|b| *literals.get(b).unwrap()).collect();
        for &num in rules.keys() {
            stack.clear();
            stack.push(num);
            if match_seq(&seq, &mut stack, &lits, &rules) {
                sum += 1;
                break;
            }
        }
    }
    println!("{}", sum);
}

// 10:18
#[test]
fn day19_pt2() {
    let input = std::include_str!("inputs/day19.txt");

    let mut literals: HashMap<u8, usize> = HashMap::new();
    let mut rules: HashMap<usize, Rule> = HashMap::new();

    struct Rule(Vec<Vec<usize>>);

    fn parse_seq(s: &str) -> Vec<usize> {
        let parts = s.split(' ');
        let seq: Vec<usize> = parts
            .map(|p| p.trim())
            .map(|p| p.parse::<usize>().unwrap())
            .collect();
        seq
    }

    fn parse_rule(s: &str) -> Vec<Vec<usize>> {
        let parts = s.split(" | ");
        parts.map(|p| parse_seq(p)).collect::<Vec<_>>()
    }

    for line in input.lines() {
        if line.is_empty() {
            break;
        }
        let mut parts = line.split(": ");
        let num = parts.next().unwrap().parse::<usize>().unwrap();
        let rule = parts.next().unwrap();
        let bytes = rule.as_bytes();
        if bytes[0] == b'\"' {
            literals.insert(bytes[1], num);
        } else {
            rules.insert(num, Rule(parse_rule(rule)));
        }
    }

    rules.insert(8, Rule(vec![vec![42], vec![42, 8]]));
    rules.insert(11, Rule(vec![vec![42, 31], vec![42, 11, 31]]));

    let mut sum = 0;

    fn match_seq(
        target: &[usize],
        stack: &mut Vec<usize>,
        lits: &[usize],
        rules: &HashMap<usize, Rule>,
    ) -> bool {
        if target.is_empty() && stack.is_empty() {
            true
        } else if target.is_empty() != stack.is_empty() {
            false
        } else {
            let next = stack.pop().unwrap();
            if next == target[0] {
                if match_seq(&target[1..], stack, lits, rules) {
                    return true;
                }
            } else if !lits.contains(&next) {
                let rule = rules.get(&next).unwrap();
                for opt in &rule.0 {
                    for &sym in opt.iter().rev() {
                        stack.push(sym);
                    }
                    if match_seq(target, stack, lits, rules) {
                        return true;
                    }
                    for _ in opt.iter() {
                        stack.pop();
                    }
                }
            }
            stack.push(next);
            false
        }
    };

    let lit_a = *literals.get(&b'a').unwrap();
    let lit_b = *literals.get(&b'b').unwrap();
    let lits = [lit_a, lit_b];
    let mut stack = Vec::new();

    for line in input.lines().skip_while(|l| !l.is_empty()).skip(1) {
        let bytes = line.as_bytes();
        let seq: Vec<usize> = bytes.iter().map(|b| *literals.get(b).unwrap()).collect();
        stack.clear();
        stack.push(0);
        if match_seq(&seq, &mut stack, &lits, &rules) {
            sum += 1;
        }
    }
    println!("{}", sum);
}

// 39:22
#[test]
fn day20_pt1() {
    let input = std::include_str!("inputs/day20.txt");

    struct Tile {
        id: usize,
        edges: [[u16; 4]; 8],
    }

    let mut tiles = Vec::new();

    for tile in input.split("\n\n") {
        let id = tile
            .lines()
            .next()
            .and_then(|l| l.strip_prefix("Tile "))
            .and_then(|l| l.strip_suffix(":"))
            .and_then(|l| l.parse::<usize>().ok())
            .unwrap();

        let mut top = 0;
        let mut bottom = 0;
        let mut left = 0;
        let mut right = 0;

        for (y, line) in tile.lines().skip(1).enumerate() {
            if y == 0 {
                for b in line.bytes() {
                    top *= 2;
                    top += (b == b'#') as u16;
                }
            }
            if y == 9 {
                for b in line.bytes() {
                    bottom *= 2;
                    bottom += (b == b'#') as u16;
                }
            }
            left *= 2;
            left += (line.bytes().next().unwrap() == b'#') as u16;

            right *= 2;
            right += (line.bytes().last().unwrap() == b'#') as u16;
        }

        bottom = bottom.reverse_bits() >> 6;
        left = left.reverse_bits() >> 6;

        let mut tile = Tile {
            id,
            edges: Default::default(),
        };
        let mut edge = [top, right, bottom, left];
        for j in 0..2 {
            for i in 0..4 {
                tile.edges[i + 4 * j] = edge;
                edge.rotate_left(1);
            }
            edge.reverse();
            for e in &mut edge {
                *e = e.reverse_bits() >> 6;
            }
        }
        tiles.push(tile);
    }

    let mut map: HashMap<u16, Vec<usize>> = HashMap::new();
    for (i, tile) in tiles.iter().enumerate() {
        for edge in &[&tile.edges[0], &tile.edges[4]] {
            for e in *edge {
                let is = map.entry(*e).or_default();
                if !is.contains(&i) {
                    is.push(i);
                }
            }
        }
    }

    let mut unique: Vec<usize> = map
        .iter()
        .filter(|(&e, tiles)| {
            tiles.len() == 1 && map.get(&(e.reverse_bits() >> 6)).unwrap().len() == 1
        })
        .map(|(_, tiles)| tiles[0])
        .collect();
    unique.sort_unstable();

    let mut hist = vec![0; tiles.len()];
    for &i in &unique {
        hist[i] += 1;
    }

    let mut res = 1;
    let mut count = 0;

    for (i, _) in hist.iter().enumerate().filter(|(_, e)| **e == 4) {
        res *= tiles[i].id;
        count += 1;
    }

    println!("{}, count: {}", res, count);
}

// 2:32:55
#[test]
fn day20_pt2() {
    let input = std::include_str!("inputs/day20.txt");

    #[derive(Copy, Clone)]
    struct Tile {
        edges: [[u16; 4]; 8],
        content: u64,
    }

    fn other_edge(e: u16) -> u16 {
        e.reverse_bits() >> 6
    }

    let mut tiles = Vec::new();

    for tile in input.split("\n\n") {
        let mut top = 0;
        let mut bottom = 0;
        let mut left = 0;
        let mut right = 0;
        let mut content: u64 = 0;

        for (y, line) in tile.lines().skip(1).enumerate() {
            if y == 0 {
                for b in line.bytes() {
                    top *= 2;
                    top += (b == b'#') as u16;
                }
            } else if y == 9 {
                for b in line.bytes() {
                    bottom *= 2;
                    bottom += (b == b'#') as u16;
                }
            } else {
                for b in line.bytes().skip(1).take(8) {
                    content *= 2;
                    content += (b == b'#') as u64;
                }
            }
            left *= 2;
            left += (line.bytes().next().unwrap() == b'#') as u16;

            right *= 2;
            right += (line.bytes().last().unwrap() == b'#') as u16;
        }

        bottom = other_edge(bottom);
        left = other_edge(left);

        let mut tile = Tile {
            edges: Default::default(),
            content,
        };
        let mut edge = [top, right, bottom, left];
        for j in 0..2 {
            for i in 0..4 {
                tile.edges[i + 4 * j] = edge;
                edge.rotate_right(1);
            }
            edge.reverse();
            for e in &mut edge {
                *e = other_edge(*e);
            }
        }
        for edge in tile.edges.iter() {
            assert_eq!((edge[0] >> 9) & 1, edge[3] & 1);
            assert_eq!((edge[1] >> 9) & 1, edge[0] & 1);
            assert_eq!((edge[2] >> 9) & 1, edge[1] & 1);
            assert_eq!((edge[3] >> 9) & 1, edge[2] & 1);
        }
        tiles.push(tile);
    }

    let mut map: HashMap<u16, Vec<usize>> = HashMap::new();
    for (i, tile) in tiles.iter().enumerate() {
        for edge in &[&tile.edges[0], &tile.edges[4]] {
            for e in *edge {
                let is = map.entry(*e).or_default();
                if !is.contains(&i) {
                    is.push(i);
                }
            }
        }
    }

    let mut unique: Vec<usize> = map
        .iter()
        .filter(|(&e, tiles)| tiles.len() == 1 && map.get(&other_edge(e)).unwrap().len() == 1)
        .map(|(_, tiles)| tiles[0])
        .collect();
    unique.sort_unstable();

    let mut hist = vec![0; tiles.len()];
    for &i in &unique {
        hist[i] += 1;
    }

    let corner_i = hist
        .iter()
        .enumerate()
        .filter(|(_, e)| **e == 4)
        .map(|(i, _)| i)
        .next()
        .unwrap();

    let mut grid = vec![(0usize, 0usize); tiles.len()];

    {
        let mut ori: usize = 0;
        let corner = tiles[corner_i];
        for _ in 0..4 {
            let edges = corner.edges[ori];
            let top_edge = edges[0];
            let left_edge = edges[3];
            if map[&top_edge].len() != 1 || map[&left_edge].len() != 1 {
                ori += 1
            }
        }
        assert!(ori < 4);
        grid[0] = (corner_i, ori);
    }

    for i in 1..tiles.len() {
        let last_i;
        let edge_ori;
        let other_edge_ori;
        if i / 12 == 0 {
            last_i = i - 1;
            edge_ori = 1; // right
            other_edge_ori = 3; // left
        } else {
            last_i = i - 12;
            edge_ori = 2; // bottom
            other_edge_ori = 0; // top
        }
        let last = grid[last_i];
        let tile = tiles[last.0];
        let edges = tile.edges[last.1];
        let matching_edge = other_edge(edges[edge_ori]);
        let mut edges = map.get(&matching_edge).unwrap().clone();
        edges.remove(edges.iter().position(|e| *e == last.0).unwrap());
        assert_eq!(edges.len(), 1);
        let ori = tiles[edges[0]]
            .edges
            .iter()
            .enumerate()
            .find(|(_, edges)| edges[other_edge_ori] == matching_edge)
            .unwrap();
        grid[i] = (edges[0], ori.0);
    }

    let mut is: Vec<_> = grid.iter().map(|(i, _)| i).copied().collect();
    is.sort_unstable();
    is.dedup();
    assert_eq!(is.len(), grid.len());

    for y in 0..12 {
        for x in 0..11 {
            let t1 = grid[y * 12 + x];
            let t2 = grid[y * 12 + x + 1];
            assert_eq!(
                tiles[t1.0].edges[t1.1][1],
                other_edge(tiles[t2.0].edges[t2.1][3])
            );
        }
    }
    for y in 0..11 {
        for x in 0..12 {
            let t1 = grid[y * 12 + x];
            let t2 = grid[(y + 1) * 12 + x];
            assert_eq!(
                tiles[t1.0].edges[t1.1][2],
                other_edge(tiles[t2.0].edges[t2.1][0])
            );
        }
    }

    let pic_w = 12 * 8;
    let pic_h = 12 * 8;
    let mut picture = vec![0u8; pic_w * pic_h];
    for (i, &(tile_i, ori)) in grid.iter().enumerate() {
        let tile_x = i % 12;
        let tile_y = i / 12;
        let tile = tiles[tile_i];
        let content: u64 = tile.content;
        for y in 0..8usize {
            for x in 0..8usize {
                let rows = content.to_be_bytes();
                let val = match ori {
                    0 => (rows[y] >> (7 - x)) & 1,
                    1 => (rows[7 - x] >> (7 - y)) & 1,
                    2 => (rows[7 - y] >> x) & 1,
                    3 => (rows[x] >> y) & 1,
                    4 => (rows[x] >> (7 - y)) & 1,
                    5 => (rows[y] >> x) & 1,
                    6 => (rows[7 - x] >> y) & 1,
                    7 => (rows[7 - y] >> (7 - x)) & 1,
                    _ => unreachable!(),
                };
                let pic_x = tile_x * 8 + x;
                let pic_y = tile_y * 8 + y;
                picture[pic_y * 12 * 8 + pic_x] = if val == 1 { b'#' } else { b'.' };
            }
        }
    }

    let mon_w = 20;
    let mon_h = 3;
    let monster = [
        b"                  # ",
        b"#    ##    ##    ###",
        b" #  #  #  #  #  #   ",
    ];

    for ori in 0..8 {
        let id_fn = |x: usize, y: usize| -> usize {
            match ori {
                0 => y * pic_w + x,
                1 => (pic_h - 1 - x) * pic_w + y,
                2 => (pic_h - 1 - y) * pic_w + (pic_w - 1 - x),
                3 => x * pic_w + (pic_w - 1 - y),
                4 => x * pic_w + y,
                5 => y * pic_w + (pic_w - 1 - x),
                6 => (pic_h - 1 - x) * pic_w + (pic_w - 1 - y),
                7 => (pic_h - 1 - y) * pic_w + x,
                _ => unreachable!(),
            }
        };
        for y in 0..pic_h - mon_h + 1 {
            'pos_loop: for x in 0..pic_w - mon_w + 1 {
                for yy in 0..mon_h {
                    for xx in 0..mon_w {
                        if monster[yy][xx] == b'#' && picture[id_fn(x + xx, y + yy)] == b'.' {
                            continue 'pos_loop;
                        }
                    }
                }
                for yy in 0..mon_h {
                    for xx in 0..mon_w {
                        if monster[yy][xx] == b'#' {
                            let prev = picture[id_fn(x + xx, y + yy)];
                            assert!(prev == b'#' || prev == b'O');
                            picture[id_fn(x + xx, y + yy)] = b'O';
                        }
                    }
                }
            }
        }
    }

    // for row_block in picture.chunks_exact(pic_w * 8) {
    //     for row in row_block.chunks_exact(pic_w) {
    //         for block in row.chunks_exact(8) {
    //             print!("{} ", std::str::from_utf8(block).unwrap());
    //         }
    //         println!();
    //     }
    //     println!();
    // }

    let res = picture.iter().filter(|&&c| c == b'#').count();

    println!("{}", res);
}
