use std::{collections::HashMap, thread::sleep, time::Duration};

use crate::model::{cell::Position, grid::Grid};

pub trait Algo {
    fn solve(&self, end_pos: Position, grid: &Grid) -> Vec<Position>;
}

pub struct BFSMouse {
    pos: Position,
}

impl BFSMouse {
    fn set_new_position(&mut self, position: Position) {
        self.pos = position;
    }

    pub fn new(pos: Position) -> BFSMouse {
        Self { pos: pos }
    }
}

fn construct_path_to_pos(
    pos: &Position,
    previouses: &HashMap<Position, Position>,
) -> Vec<Position> {
    let mut p = pos.clone();
    let mut v: Vec<Position> = Vec::new();
    loop {
        if !previouses.contains_key(&p) {
            break;
        }
        v.push(previouses[&p]);
        p = previouses[&p];
    }
    v.reverse();
    v
}

fn backtrack_to_current_pos(
    current_path: &mut Vec<Position>,
    absolute_path_to_next_pos: &Vec<Position>,
    absolute_path_to_current_pos: &Vec<Position>,
) {
    let mut index_common_pos_in_current_path: usize = 0;
    let mut index_common_pos_in_next_pos_path: usize = 0;
    for i in (0..absolute_path_to_next_pos.len()).rev() {
        if let Some(v) = absolute_path_to_current_pos
            .iter()
            .position(|pos| *pos == absolute_path_to_next_pos[i])
        {
            index_common_pos_in_current_path = v;
            index_common_pos_in_next_pos_path = i;
            break;
        }
    }
    // if absolute_path_to_current_pos.len() > 0 {
    //     println!(
    //         "absolute path to {:?} pos is: {:?}",
    //         current_path.last().unwrap(),
    //         absolute_path_to_current_pos
    //     );
    //     println!(
    //         "Last common pos is: {:?}",
    //         absolute_path_to_current_pos[index_common_pos_in_current_path],
    //     );
    // }

    let mut return_to_common_root_path: Vec<Position> = Vec::new();
    let mut to_current_pos: Vec<Position> = Vec::new();

    if (absolute_path_to_current_pos.len() > 0) {
        return_to_common_root_path = absolute_path_to_current_pos
            [index_common_pos_in_current_path..absolute_path_to_current_pos.len() - 1]
            .to_vec();
        return_to_common_root_path.reverse();
    }
    // println!("return to root: {:?}", return_to_common_root_path);
    for i in index_common_pos_in_next_pos_path + 1..absolute_path_to_next_pos.len() {
        to_current_pos.push(absolute_path_to_next_pos[i]);
    }
    // println!("to_current_pos: {:?}", to_current_pos);

    current_path.append(&mut return_to_common_root_path);
    current_path.append(&mut to_current_pos);
}

impl Algo for BFSMouse {
    fn solve(&self, end_pos: Position, grid: &Grid) -> Vec<Position> {
        let mut path: Vec<Position> = Vec::new();

        let mut to_visit: Vec<Position> = vec![self.pos];
        let mut visited: Vec<Position> = Vec::new();
        let mut path_to_visited: HashMap<Position, Position> = HashMap::new();
        while !to_visit.is_empty() {
            let next_pos: Position = to_visit.remove(0);
            if visited.contains(&next_pos) {
                continue;
            }
            // we want to backtrack to current pos. We saved the path to the cell before the current pos
            let absolute_path_to_next_pos: Vec<Position> =
                construct_path_to_pos(&next_pos, &path_to_visited);

            let mut absolute_path_to_current_pos: Vec<Position> = Vec::new();
            if path.len() > 0 {
                absolute_path_to_current_pos =
                    construct_path_to_pos(path.last().unwrap(), &path_to_visited);
                absolute_path_to_current_pos.push(path.last().unwrap().to_owned());
            }

            backtrack_to_current_pos(
                &mut path,
                &absolute_path_to_next_pos,
                &absolute_path_to_current_pos,
            );
            path.push(next_pos);
            // println!("path: {:?}", path);

            if (next_pos == end_pos) {
                break;
            }

            let mut neighbors: Vec<Position> = grid.get_neighbors_of_pos(next_pos);
            neighbors.retain(|neighbor| {
                !visited.contains(&neighbor) && !path_to_visited.contains_key(&neighbor)
            });
            for n in neighbors.iter() {
                path_to_visited.insert(*n, next_pos);
            }
            to_visit.append(&mut neighbors);
            visited.push(next_pos);
            println!("still {} to visit", to_visit.len());
            println!("path size {}", path.len());
            // sleep(Duration::from_secs(1));
        }

        path
    }
}
