use crate::GridPosition;

#[derive(PartialEq, Clone, Debug)]
pub enum Direction { Up, Down, Left, Right }

impl Direction {
    pub fn between(from: &GridPosition, to: &GridPosition) -> Option<Direction> {
        let x = (to.x - from.x)/(to.x - from.x).abs().max(1);
        let y = (to.y - from.y)/(to.y - from.y).abs().max(1);
        match (x, y) {
            (1, 0) => Some(Direction::Right),
            (-1, 0) => Some(Direction::Left),
            (0, 1) => Some(Direction::Down),
            (0, -1) => Some(Direction::Up),
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Direction::*;

    #[test]
    fn direction_between() {
        fn gp(x: i32, y: i32) -> GridPosition {
            GridPosition { x, y }
        }

        assert_eq!(Direction::between(&gp(0, 0), &gp(0, -1)), Some(Up));
        assert_eq!(Direction::between(&gp(0, 0), &gp(0, 1)), Some(Down));
        assert_eq!(Direction::between(&gp(0, 0), &gp(-1, 0)), Some(Left));
        assert_eq!(Direction::between(&gp(0, 0), &gp(1, 0)), Some(Right));
        assert_eq!(Direction::between(&gp(0, 0), &gp(10, 0)), Some(Right));

        // same positions
        assert_eq!(Direction::between(&gp(0, 0), &gp(0, 0)), None);
        // diagonal
        assert_eq!(Direction::between(&gp(0, 0), &gp(1, 1)), None);
    }
}
