use crate::model::cell::Cell;
use crate::model::cell::Position;
use crate::model::grid::Grid;
use rand::Rng;

use std::collections::HashMap;
enum VisitingStatus {
    NotVisited,
    Visited,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

#[derive(Debug, Copy, Clone)]
struct CellPositionWrapperForWilson {
    pub cell_position: Position,
    pub move_direction: Direction,
}

#[derive(Debug)]
pub struct Path {
    cell_positions_wrapped: Vec<CellPositionWrapperForWilson>,
}

impl Path {
    fn get_cell_wrappers(&self) -> &Vec<CellPositionWrapperForWilson> {
        return &self.cell_positions_wrapped;
    }

    pub fn length(&self) -> usize {
        self.cell_positions_wrapped.len()
    }

    pub fn get_cells_positions(&self) -> Vec<Position> {
        self.cell_positions_wrapped
            .iter()
            .map(|wrapper| wrapper.cell_position.clone())
            .collect()
    }
}

impl Clone for Path {
    fn clone(&self) -> Path {
        Path {
            cell_positions_wrapped: self.cell_positions_wrapped.clone(),
        }
    }
}

// Making this function recursive will actually make it crash
pub fn random_walk(
    starting_position: Position,
    grid: &mut Grid,
    visited_positions: &Vec<Position>,
) -> Path {
    let mut path = Path {
        cell_positions_wrapped: Vec::new(),
    };

    let mut rng = rand::rng();

    let mut position = starting_position;
    loop {
        let mut dir: Direction;
        let new_pos: Position;
        loop {
            let diru8: u8 = rng.random_range(0..4);
            if diru8 == 0 {
                dir = Direction::Left;
            } else if diru8 == 1 {
                dir = Direction::Up;
            } else if diru8 == 2 {
                dir = Direction::Right;
            } else {
                dir = Direction::Down;
            }

            let mut x: isize = position.x as isize;
            let mut y: isize = position.y as isize;
            match dir {
                Direction::Left => x -= 1,
                Direction::Right => x += 1,
                Direction::Down => y += 1,
                Direction::Up => y -= 1,
            }

            let dir_ok = x >= 0
                && x < grid.get_number_of_cells_x_y().0 as isize
                && y >= 0
                && y < grid.get_number_of_cells_x_y().1 as isize;

            if dir_ok {
                new_pos = Position {
                    x: x as usize,
                    y: y as usize,
                };
                break;
            }
        }

        let wrapper: CellPositionWrapperForWilson = CellPositionWrapperForWilson {
            cell_position: position.clone(),
            move_direction: dir,
        };
        path.cell_positions_wrapped.push(wrapper);

        if visited_positions.contains(&new_pos) {
            path.cell_positions_wrapped
                .push(CellPositionWrapperForWilson {
                    cell_position: new_pos,
                    move_direction: Direction::Up,
                });
            break;
        }
        position = new_pos;
    }
    path
}

fn loop_erased_path(path: &Path, grid: &Grid) -> Path {
    let mut reversed_path: Path = Path {
        cell_positions_wrapped: Vec::new(),
    };

    let mut i = path.get_cell_wrappers().len() - 1;
    let mut pos_wrapper = &path.get_cell_wrappers()[i];
    while i != 0 {
        for j in 0..i {
            //for prev in path.get_cell_wrappers().iter() {
            let prev: &CellPositionWrapperForWilson = &path.get_cell_wrappers()[j];
            let dir = prev.move_direction;
            let mut x = prev.cell_position.x as isize;
            let mut y = prev.cell_position.y as isize;
            match dir {
                Direction::Left => x -= 1,
                Direction::Right => x += 1,
                Direction::Down => y += 1,
                Direction::Up => y -= 1,
            };

            if x as usize != pos_wrapper.cell_position.x
                || y as usize != pos_wrapper.cell_position.y
            {
                // println!(
                //     "Trying to reach {};{} but reached {};{}",
                //     pos_wrapper.cell_position.x, pos_wrapper.cell_position.y, x, y,
                // );
                continue;
            }
            // grid.add_neighbor(
            //    (x as usize, y as usize),
            //    (prev.cell.get_x(), prev.cell.get_y()),
            // );

            reversed_path
                .cell_positions_wrapped
                .push(pos_wrapper.clone());

            pos_wrapper = prev;
            i = j;
            break;
        }
        // panic!("alll {} {}", i, path.length())
    }
    // add the first cell in the path
    if pos_wrapper.cell_position.x == path.cell_positions_wrapped[0].cell_position.x
        && pos_wrapper.cell_position.y == path.cell_positions_wrapped[0].cell_position.y
    {
        reversed_path
            .cell_positions_wrapped
            .push(pos_wrapper.clone());
    }

    // our algorithm works in reverse, put in in the right order
    reversed_path.cell_positions_wrapped.reverse();
    reversed_path
}

pub struct MazeGenerationInternals {
    paths: Vec<(Path, Path, Grid)>, // Association of random walks with their loop erased counterparts
}

impl MazeGenerationInternals {
    pub fn get_paths(&self) -> &Vec<(Path, Path, Grid)> {
        &self.paths
    }
}

// Wilson's algorithm
pub fn mazify(grid: &mut Grid) -> MazeGenerationInternals {
    let mut to_visit: Vec<Position> = Vec::new();
    for position in grid.get_cells_positions() {
        to_visit.push(position);
    }

    let mut rng = rand::rng();
    let mut visited: Vec<Position> = Vec::new();
    let mut paths_from_wilson: Vec<(Path, Path, Grid)> = Vec::new();
    let mut counter = 0;

    while !to_visit.is_empty() {
        let index = rng.random_range(0..to_visit.len());

        let cell_to_visit_pos: Position = to_visit.swap_remove(index);

        // println!(
        //     "Start walk with {};{}",
        //     cell_to_visit_pos.x, cell_to_visit_pos.y
        // );
        if visited.is_empty() {
            visited.push(cell_to_visit_pos);
        }

        // the magic happens
        let path = random_walk(cell_to_visit_pos, grid, &visited);
        let lep: Path = loop_erased_path(&path, grid);
        // println!("{:?}", path);
        let cell_positions = lep.get_cells_positions();
        for i in 0..lep.length() - 1 {
            let j = i + 1;
            // println!(
            //     "Neighbors ({};{}) (){};{})",
            //     cell_positions[i].x, cell_positions[i].y, cell_positions[j].x, cell_positions[j].y
            // );
            grid.add_neighbor(cell_positions[i], cell_positions[j]);

            visited.push(cell_positions[i]);
            if let Some(index) = to_visit.iter().position(|pos| *pos == cell_positions[i]) {
                to_visit.remove(index);
            }
        }

        // print!("random walk");
        // println!("{:?}", path);
        // println!("Now reversed:");
        // println!("{:?}", lep);
        //
        // path.debug();
        // println!("pth size {}", path.cells.len());
        // paths_from_wilson.push(path);
        paths_from_wilson.push((path, lep, grid.clone()));

        counter += 1;
    }
    MazeGenerationInternals {
        paths: paths_from_wilson,
    }
}
