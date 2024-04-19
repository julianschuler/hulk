use serde::{Deserialize, Serialize};

use linear_algebra::{vector, Vector2};
use serialize_hierarchy::SerializeHierarchy;

#[derive(
    Clone, Copy, Default, Debug, Deserialize, Eq, PartialEq, Serialize, SerializeHierarchy,
)]
pub enum Direction {
    Clockwise,
    Counterclockwise,
    #[default]
    Colinear,
}

impl Direction {
    pub fn rotate_vector_90_degrees<Frame>(&self, subject: Vector2<Frame>) -> Vector2<Frame> {
        match self {
            Direction::Clockwise => vector![subject.y(), -subject.x()],
            Direction::Counterclockwise => vector![-subject.y(), subject.x()],
            Direction::Colinear => subject,
        }
    }
}
