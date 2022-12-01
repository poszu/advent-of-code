fn main() {
    let mut max = 0;
    let mut current = 0;
    for line in include_str!("input.txt").lines() {
        if line.is_empty() {
            max = std::cmp::max(max, current);
            current = 0;
        } else {
            current += line.parse::<usize>().unwrap();
        }
    }

    println!("{}",max)
}
