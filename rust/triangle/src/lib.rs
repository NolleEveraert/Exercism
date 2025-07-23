pub struct Triangle<T> {
    sides: [T; 3],
}

use std::ops::Add;

impl<T> Triangle<T>
where
    T: Copy + PartialOrd + Add<Output = T> + PartialEq + Default,
{
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        let zero = T::default();

        if sides[0] <= zero || sides[1] <= zero || sides[2] <= zero {
            return None;
        }

        if sides[0] + sides[1] >= sides[2] && sides[1] + sides[2] >= sides[0] && sides[0] + sides[2] >= sides[1] {
            Some(Triangle { sides })
        } else {
            None
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides[0] == self.sides[1] && self.sides[1] == self.sides[2]
    }

    pub fn is_scalene(&self) -> bool {
        self.sides[0] != self.sides[1] && self.sides[1] != self.sides[2] && self.sides[0] != self.sides[2]
    }

    pub fn is_isosceles(&self) -> bool {
        self.sides[0] == self.sides[1] || self.sides[1] == self.sides[2] || self.sides[0] == self.sides[2]
    }
}
