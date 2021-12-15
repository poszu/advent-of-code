#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> std::ops::Add<Vec2<T>> for Vec2<T>
where
    T: std::ops::Add<Output = T>,
{
    type Output = Vec2<T>;
    fn add(self, rhs: Vec2<T>) -> Vec2<T> {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

/// Iterate over vectors of all 8 directions (north, north-east, east and so on)
pub fn all_directions() -> impl Iterator<Item = Vec2<isize>> {
    (-1..=1)
        .map(|x| (-1..=1).map(move |y| Vec2 { x, y }))
        .flatten()
        .filter(|v| *v != Vec2 { x: 0, y: 0 })
}
