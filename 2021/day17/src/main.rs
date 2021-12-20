use std::cmp::Ordering;

extern crate utils;
type Vec2 = utils::Vec2<isize>;

struct Probe {
    pos: Vec2,
    vel: Vec2,
}

impl Probe {
    fn new(v: Vec2) -> Self {
        Self {
            pos: Vec2::default(),
            vel: v,
        }
    }

    fn step(&mut self) {
        self.pos = self.pos + self.vel;
        match self.vel.x.cmp(&0) {
            Ordering::Greater => {
                self.vel.x -= 1;
            }
            Ordering::Less => {
                self.vel.x -= 1;
            }
            _ => {}
        }

        self.vel.y -= 1;
    }
}

fn missed(probe: &Probe, point: Vec2) -> bool {
    probe.pos.x > point.x || probe.pos.y < point.y
}

fn in_area(probe: &Probe, (point_lu, point_rd): (Vec2, Vec2)) -> bool {
    (point_lu.x..=point_rd.x).contains(&probe.pos.x)
        && (point_rd.y..=point_lu.y).contains(&probe.pos.y)
}

fn main() {
    let point_lu = Vec2 { x: 153, y: -75 };
    let point_rd = Vec2 { x: 199, y: -114 };

    let mut cnt = 0;
    let mut highest_vy = isize::MIN;
    for y in -114..114 {
        for x in 1..=199 {
            let mut probe = Probe::new(Vec2 { x, y });
            while !missed(&probe, point_rd) {
                if in_area(&probe, (point_lu, point_rd)) {
                    cnt += 1;
                    highest_vy = y;
                    break;
                }
                probe.step();
            }
        }
    }

    let mut probe = Probe::new(Vec2 {
        x: 0,
        y: highest_vy,
    });
    while probe.vel.y >= 0 {
        probe.step();
    }
    println!("PART1: Max probe Y position: {}", probe.pos.y);
    println!("PART2: {}!", cnt);
}
