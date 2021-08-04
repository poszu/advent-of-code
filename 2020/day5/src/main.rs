use std::io;
use std::io::prelude::*;


fn reduce(s: &str, take_low_c: char, take_high_c: char) -> i32 {
    let mut low = 0;
    let mut high = (1 << s.len()) - 1;
    for ch in s.chars() {
        match ch {
            v if v == take_high_c => {
                low = low + (high - low) / 2 + 1;
            },
            v if v == take_low_c => {
                high = low + (high - low) / 2;
            },
            _ => panic!("Wrong char!")
        }
    }
    low
}

fn get_row(pass: &str) -> i32 {
    reduce(&pass[..7], 'F', 'B')
}

fn get_col(pass: &str) -> i32 {
    reduce(&pass[7..], 'L', 'R')
}

fn get_seat_id(pass: &str) -> i32 {
    get_row(pass) * 8 + get_col(pass)
}

fn main() {
    let boarding_passes = io::stdin().lock().lines().collect::<Result<Vec<String>, _>>().unwrap();

    let highest = boarding_passes.iter().map(|pass| get_seat_id(pass)).max().unwrap();
    print!("Highest seat ID: {}", highest);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_row() {
        assert_eq!(get_row("FBFBBFFRLR"), 44);
        assert_eq!(get_row("BFFFBBFRRR"), 70);
        assert_eq!(get_row("FFFBBBFRRR"), 14);
        assert_eq!(get_row("BBFFBBFRLL"), 102);
    }
    #[test]
    fn test_get_col() {
        assert_eq!(get_col("FBFBBFFRLR"), 5);
        assert_eq!(get_col("BFFFBBFRRR"), 7);
        assert_eq!(get_col("FFFBBBFRRR"), 7);
        assert_eq!(get_col("BBFFBBFRLL"), 4);
    }
    #[test]
    fn test_get_seat_id() {
        assert_eq!(get_seat_id("FBFBBFFRLR"), 357);
        assert_eq!(get_seat_id("BFFFBBFRRR"), 567);
        assert_eq!(get_seat_id("FFFBBBFRRR"), 119);
        assert_eq!(get_seat_id("BBFFBBFRLL"), 820);
    }
}
