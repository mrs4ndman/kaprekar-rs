const WIDTH: usize = 4;
const TARGET: u32 = 6174;

enum Outcome {
    Reached(u32),
    Loop,
}

fn main() {
    for n in 0..=9999 {
        match kaprekar_steps(n) {
            Outcome::Reached(steps) => println!("{:04}: {}", n, steps),
            Outcome::Loop => println!("{:04}: loop", n),
        }
    }
}

fn kaprekar_steps(start: u32) -> Outcome {
    if start == TARGET {
        return Outcome::Reached(0);
    }

    let mut current = start;
    let mut seen = [false; 10000];
    seen[current as usize] = true;

    // For 4-digit Kaprekar, all paths stabilize quickly; this guards against bugs.
    for steps in 1..=20 {
        current = kaprekar_step(current);

        if current == TARGET {
            return Outcome::Reached(steps);
        }

        if seen[current as usize] {
            return Outcome::Loop;
        }

        seen[current as usize] = true;
    }

    Outcome::Loop
}

fn kaprekar_step(number: u32) -> u32 {
    let digits = format!("{:0width$}", number, width = WIDTH);
    let mut chars: Vec<char> = digits.chars().collect();

    chars.sort_unstable();
    let asc: String = chars.iter().collect();
    chars.reverse();
    let desc: String = chars.iter().collect();

    let asc_num = asc.parse::<u32>().unwrap();
    let desc_num = desc.parse::<u32>().unwrap();
    desc_num - asc_num
}
