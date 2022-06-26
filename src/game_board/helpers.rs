use crate::core::Direction;
use crate::GridPosition;

pub fn move_grid_position(
    mut grid_pos: GridPosition,
    direction: Direction,
    grid_size: (i32, i32)) -> GridPosition {

    match direction {
        Direction::Up => grid_pos.y -= 1,
        Direction::Down => grid_pos.y += 1,
        Direction::Left => grid_pos.x -= 1,
        Direction::Right => grid_pos.x += 1
    }

    // Wrap Around
    grid_pos.x = (grid_pos.x + grid_size.0) % grid_size.0;
    grid_pos.y = (grid_pos.y + grid_size.1) % grid_size.1;
    grid_pos
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_grid_position_normal() {
        assert_eq!(move_grid_position(
            GridPosition{x: 3, y: 2},
            Direction::Up,
            (5,5)
        ), GridPosition{x: 3, y: 1});

        assert_eq!(move_grid_position(
            GridPosition{x: 3, y: 2},
            Direction::Down,
            (5,5)
        ), GridPosition{x: 3, y: 3});

        assert_eq!(move_grid_position(
            GridPosition{x: 3, y: 2},
            Direction::Left,
            (5,5)
        ), GridPosition{x: 2, y: 2});

        assert_eq!(move_grid_position(
            GridPosition{x: 3, y: 2},
            Direction::Right,
            (5,5)
        ), GridPosition{x: 4, y: 2});

    }

    #[test]
    fn move_grid_position_wrap() {
        assert_eq!(move_grid_position(
            GridPosition{x: 3, y: 0},
            Direction::Up,
            (5,5)
        ), GridPosition{x: 3, y: 4});

        assert_eq!(move_grid_position(
            GridPosition{x: 3, y: 4},
            Direction::Down,
            (5,5)
        ), GridPosition{x: 3, y: 0});

        assert_eq!(move_grid_position(
            GridPosition{x: 0, y: 2},
            Direction::Left,
            (5,5)
        ), GridPosition{x: 4, y: 2});

        assert_eq!(move_grid_position(
            GridPosition{x: 4, y: 2},
            Direction::Right,
            (5,5)
        ), GridPosition{x: 0, y: 2});

    }
}