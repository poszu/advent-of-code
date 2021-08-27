use std::collections::{HashMap, HashSet};
use std::io;
use std::io::prelude::*;

fn count_anyone<S: AsRef<str>>(list: impl IntoIterator<Item = S>) -> usize {
    let mut cnt = 0;
    let mut hist = HashSet::<char>::new();

    for line in list {
        if line.as_ref().is_empty() {
            cnt += hist.len();
            hist.clear();
        } else {
            for c in line.as_ref().chars() {
                hist.insert(c);
            }
        }
    }
    cnt + hist.len()
}

fn count_everyone<S: AsRef<str>>(list: impl IntoIterator<Item = S>) -> usize {
    let mut cnt = 0;
    let mut people_in_group = 0;
    let mut hist = HashMap::<char, usize>::new();

    for line in list {
        if line.as_ref().is_empty() {
            cnt += hist.iter().filter(|(&_, &v)| v == people_in_group).count();
            hist.clear();
            people_in_group = 0;
        } else {
            people_in_group += 1;
            for c in line.as_ref().chars() {
                if let Some(v) = hist.get_mut(&c) {
                    *v += 1;
                } else {
                    hist.insert(c, 1);
                }
            }
        }
    }

    cnt + hist.iter().filter(|(&_, &v)| v == people_in_group).count()
}

fn main() {
    let lines = io::stdin()
        .lock()
        .lines()
        .collect::<Result<Vec<String>, _>>()
        .unwrap();

    println!("Part1, The sum is {}!", count_anyone(&lines));
    println!("Part2, The sum is {}!", count_everyone(lines));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_anyone() {
        assert_eq!(count_anyone(["abc"]), 3);
    }

    #[test]
    fn test_count_anyone2() {
        assert_eq!(
            count_anyone([
                "abc", "", "a", "b", "c", "", "ab", "ac", "", "a", "a", "a", "a", "", "b"
            ]),
            11
        );
    }

    #[test]
    fn test_count_everyone() {
        assert_eq!(
            count_everyone([
                "abc", "", "a", "b", "c", "", "ab", "ac", "", "a", "a", "a", "a", "", "b"
            ]),
            6
        );
    }
}
