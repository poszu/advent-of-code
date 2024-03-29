use std::collections::BTreeSet;

static REQUIRED: &[&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn main() {
    let required_set: BTreeSet<String> = REQUIRED.iter().cloned().map(|x| x.into()).collect();
    let mut passport = BTreeSet::<String>::new();

    let mut valid = 0;

    for line in include_str!("input.txt").lines() {
        match line {
            e if e.is_empty() => {
                if passport == required_set {
                    valid += 1;
                }
                passport.clear();
            }
            l => {
                passport.extend(
                    l.split_whitespace()
                        .map(|kv| Some(kv.split_once(':').map_or(kv, |x| x.0)))
                        .filter_map(|key| match key {
                            Some(k) if k != "cid" => Some(k.into()),
                            _ => None,
                        }),
                );
                // for kv in l.split_whitespace() {
                //     if let Some(key) = kv.splitn(2, ":").next() {
                //         if key != "cid" {
                //             passport.insert(key.into());
                //         }
                //     }
                // }
            }
        }
    }
    if passport == required_set {
        valid += 1;
    }
    print!("Valid: {}", valid);
}
