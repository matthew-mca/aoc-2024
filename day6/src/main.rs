use std::fs;
use std::process::exit;

struct Point {
    point_type: PointType,
}

impl Point {
    fn set(&mut self, value: PointType) {
        self.point_type = value;
    }
}

#[derive(PartialEq)]
enum PointType {
    Obstacle,
    Unvisited,
    Visited,
    VisitedLeftRight,
    VisitedUpDown,
    VisitedCorner,
}

enum Orientation {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

struct Guard {
    orientation: Orientation,
    x_position: usize,
    y_position: usize,
}

impl Guard {
    fn new(char: char, x: usize, y:usize) -> Guard {
        let orientation_result = match char {
            '^' => Some(Orientation::NORTH),
            '>' => Some(Orientation::EAST),
            'v' => Some(Orientation::SOUTH),
            '<' => Some(Orientation::WEST),
            _ => None,
        };
        let orientation = orientation_result.unwrap();

        let new_guard = Guard {
            orientation,
            x_position: x,
            y_position: y,
        };

        new_guard
    }

    fn set_orientation(&mut self, new_orientation: Orientation) {
        self.orientation = new_orientation;
    }
}

struct Grid {
    points: Vec<Point>,
    guard: Guard,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(grid_input: String) -> Grid {
        let mut patrol_grid: Vec<Point> = vec![];
        let grid_width = grid_input.find("\n").unwrap();
        // Default values prior to initialisation
        let mut guard = Guard::new('^', 0, 0);
        let mut index = 0;
        for char in grid_input.chars() {
            if char == '\n' {
                continue;
            }

            match char {
                '.' => patrol_grid.push(Point{point_type:PointType::Unvisited }),
                '#' => patrol_grid.push(Point{point_type: PointType::Obstacle }),
                '^'|'>'|'v'|'<' => {
                    patrol_grid.push(Point{point_type:PointType::Visited });
                    let row = (index as f32 / grid_width as f32).floor() as usize;
                    let column = index % grid_width;
                    guard = Guard::new(char, column, row);
                },
                _ => panic!("Invalid grid char"),
            };
            index += 1;
        }
        let grid_height = patrol_grid.len() / grid_width;
        let new_grid = Grid {
            points: patrol_grid,
            guard,
            width: grid_width,
            height: grid_height,
        };

        new_grid
    }

    fn get(&self, row: isize, column: isize) -> Option<&Point> {
        let isize_width = self.width as isize;
        let isize_height = self.height as isize;

        if row < 0 || column < 0 {
            return None;
        }
        if row >= isize_height || column >= isize_width {
            return None;
        }
        Some(&self.points[((isize_width * row) + column) as usize])
    }

    fn mark_visited(&mut self, row: usize, column: usize) {
        let mut target_point = &mut self.points[(self.width * row) + column];
        target_point.set(PointType::Visited);
    }

    fn make_guard_next_move(&mut self) {
        let current_row = self.guard.y_position as isize;
        let current_column = self.guard.x_position as isize;

        let forward_point = match self.guard.orientation {
            Orientation::NORTH => self.get(current_row - 1, current_column),
            Orientation::EAST => self.get(current_row, current_column + 1),
            Orientation::SOUTH => self.get(current_row + 1, current_column),
            Orientation::WEST => self.get(current_row, current_column - 1),
        };

        match forward_point {
            Some(point) => {
                match point.point_type {
                    PointType::Obstacle => self.rotate_guard(),
                    _ => self.move_guard_forward(),
                }
            },
            None => {
                println!("No move made, guard would move outside grid");
                let visited_count = self.points.
                    iter().
                    filter(|x| x.point_type == PointType::Visited)
                    .count();
                println!("Number of visited spaces: {}", visited_count);
                exit(0);
            }
        };
    }

    fn rotate_guard(&mut self) {
        let new_orientation = match self.guard.orientation {
            Orientation::NORTH => Orientation::EAST,
            Orientation::EAST => Orientation::SOUTH,
            Orientation::SOUTH => Orientation::WEST,
            Orientation::WEST => Orientation::NORTH,
        };


        self.guard.set_orientation(new_orientation);
    }

    fn move_guard_forward(&mut self) {
        match self.guard.orientation {
            Orientation::NORTH => self._move_up(),
            Orientation::EAST => self._move_right(),
            Orientation::SOUTH => self._move_down(),
            Orientation::WEST => self._move_left(),
        }
    }

    fn _move_left(&mut self) {
        self.guard.x_position -= 1;
        self.mark_visited(self.guard.y_position, self.guard.x_position);
    }
    fn _move_right(&mut self) {
        self.guard.x_position += 1;
        self.mark_visited(self.guard.y_position, self.guard.x_position);
    }
    fn _move_up(&mut self) {
        self.guard.y_position -= 1;
        self.mark_visited(self.guard.y_position, self.guard.x_position);
    }
    fn _move_down(&mut self) {
        self.guard.y_position += 1;
        self.mark_visited(self.guard.y_position, self.guard.x_position);
    }
}

fn main() {
    part1();
}

fn part1() {
    let challenge_input = fs::read_to_string("input.txt").unwrap();
    let mut patrol_grid: Grid = Grid::new(challenge_input);
    loop {
        patrol_grid.make_guard_next_move();
    }
}
