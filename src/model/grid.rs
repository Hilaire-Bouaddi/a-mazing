type GridDimensionsIntegerType = u16;

use crate::model::cell::Cell;
use crate::model::cell::Position;
use std::collections::HashMap;

use std::clone::Clone;

pub struct Grid {
    number_of_cells_x: GridDimensionsIntegerType,
    number_of_cells_y: GridDimensionsIntegerType,

    cells_per_pos: HashMap<Position, Cell>,
    neighbors: HashMap<Position, Vec<Position>>,
}

impl Grid {
    pub fn new(
        number_of_cells_x: GridDimensionsIntegerType,
        number_of_cells_y: GridDimensionsIntegerType,
    ) -> Grid {
        let mut cells_per_pos = HashMap::new();
        for y in 0..number_of_cells_y as usize {
            for x in 0..number_of_cells_x as usize {
                let pos = Position { x: x, y: y };
                cells_per_pos.insert(pos, Cell::new(x, y));
            }
        }

        Self {
            number_of_cells_x: number_of_cells_x,
            number_of_cells_y: number_of_cells_y,
            cells_per_pos: cells_per_pos,
            neighbors: HashMap::new(),
        }
    }

    pub fn get_cells_positions(&self) -> Vec<Position> {
        self.cells_per_pos.keys().cloned().collect()
    }

    pub fn get_number_of_cells_x_y(
        &self,
    ) -> (GridDimensionsIntegerType, GridDimensionsIntegerType) {
        (self.number_of_cells_x, self.number_of_cells_y)
    }

    pub fn add_neighbor(&mut self, pos1: Position, pos2: Position) {
        if !self.neighbors.contains_key(&pos1) {
            self.neighbors.insert(pos1, Vec::new());
        }

        if !self.neighbors.contains_key(&pos2) {
            self.neighbors.insert(pos2, Vec::new());
        }

        match self.neighbors.get_mut(&pos1) {
            Some(neighbors) => {
                neighbors.push(pos2);
            }
            None => {
                self.neighbors.insert(pos1, vec![pos2]);
            }
        };
        match self.neighbors.get_mut(&pos2) {
            Some(neighbors) => {
                neighbors.push(pos1);
            }
            None => {
                self.neighbors.insert(pos2, vec![pos1]);
            }
        };
    }

    pub fn are_neighbors(&self, pos1: Position, pos2: Position) -> bool {
        match self.neighbors.get(&pos1) {
            Some(neighbors) => neighbors.contains(&pos2),
            None => false,
        }
    }

    pub fn get_neighbors(&self) -> &HashMap<Position, Vec<Position>> {
        &self.neighbors
    }
}

impl Clone for Grid {
    fn clone(&self) -> Grid {
        Self {
            number_of_cells_x: self.number_of_cells_x,
            number_of_cells_y: self.number_of_cells_y,
            cells_per_pos: self.cells_per_pos.clone(),
            neighbors: self.neighbors.clone(),
        }
    }
}
