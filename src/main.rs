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
