#[derive(Default)]
struct FishColony {
    // Keeps number of fishes with given 'time-to-birth'
    // denoted by its position in the array.
    fishes: [usize; 9],
}

impl FromIterator<usize> for FishColony {
    fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
        let mut res = Self::default();
        for fish in iter {
            res.fishes[fish] += 1;
        }
        res
    }
}

impl FishColony {
    fn day_passed(&mut self) {
        let fish_babies = self.fishes[0];
        for i in 0..8 {
            self.fishes[i] = self.fishes[i + 1]
        }
        self.fishes[8] = fish_babies;
        self.fishes[6] += fish_babies;
    }

    fn fish_count(&self) -> usize {
        self.fishes.iter().sum()
    }
}

fn main() {
    let input = include_str!("input.txt");
    let mut colony = FishColony::from_iter(
        input
            .lines()
            .next()
            .unwrap()
            .split(',')
            .map(|val| val.parse().unwrap()),
    );

    for _ in 0..80 {
        colony.day_passed();
    }
    println!("PART1: There are {} fishes.", colony.fish_count());

    for _ in 80..256 {
        colony.day_passed();
    }
    println!("PART2: There are {} fishes.", colony.fish_count());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut colony = FishColony::from_iter([3, 4, 3, 1, 2]);
        for _ in 0..18 {
            colony.day_passed();
        }
        assert_eq!(26, colony.fish_count());

        for _ in 18..80 {
            colony.day_passed();
        }
        assert_eq!(5934, colony.fish_count());

        for _ in 80..256 {
            colony.day_passed();
        }
        assert_eq!(26984457539, colony.fish_count());
    }
}
