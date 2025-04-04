mod constants;
mod maze_generator;
mod model;
mod mouse;

use std::{thread::sleep, time::Duration};

use macroquad::prelude::*;

use constants::gui_constants::{
    ACCENT_COLOR, CELL_COLOR, LEP_PATH_COLOR, PATH_COLOR, get_window_config,
};
use maze_generator::MazeGenerationInternals;
use model::cell::Position;
use model::grid::Grid;
use mouse::Algo;
use mouse::BFSMouse;

fn game_coord_to_window_coord(x: usize, y: usize, grid_size: (u16, u16)) -> (usize, usize) {
    let window_x: usize = x * constants::gui_constants::WINDOW_SIZE_X / grid_size.0 as usize;
    let window_y: usize = y * constants::gui_constants::WINDOW_SIZE_Y / grid_size.1 as usize;

    (window_x, window_y)
}

fn display_grid(
    positions: Vec<Position>,
    grid: &Grid,
    color: macroquad::color::Color,
    accent_on_last: bool,
) {
    fn draw_cell(pos: Position, grid: &Grid, color: macroquad::color::Color) {
        let (x_px, y_px) = game_coord_to_window_coord(pos.x, pos.y, grid.get_number_of_cells_x_y());
        let cell_width = constants::gui_constants::WINDOW_SIZE_X
            / constants::game_constants::NUMBER_OF_CELLS_X as usize;
        let cell_height = constants::gui_constants::WINDOW_SIZE_Y
            / constants::game_constants::NUMBER_OF_CELLS_Y as usize;

        let wall_thickness_width =
            (constants::gui_constants::WALL_RATIO * cell_width as f64) as usize;
        let wall_thickness_height =
            (constants::gui_constants::WALL_RATIO * cell_height as f64) as usize;

        // scaling
        let mut x: isize = pos.x as isize;
        let mut y: isize = pos.y as isize;

        let mut offset_left: usize = 0;
        let mut offset_top: usize = 0;

        let mut rect_width = cell_width;
        let mut rect_height = cell_height;

        if !grid.are_neighbors(
            pos,
            Position {
                x: (x - 1) as usize,
                y: y as usize,
            },
        ) {
            offset_left += wall_thickness_width;
            rect_width -= wall_thickness_width;
        }
        if !grid.are_neighbors(
            pos,
            Position {
                x: (x + 1) as usize,
                y: y as usize,
            },
        ) {
            rect_width -= wall_thickness_width;
        }
        if !grid.are_neighbors(
            pos,
            Position {
                x: x as usize,
                y: (y + 1) as usize,
            },
        ) {
            rect_height -= wall_thickness_height;
        }
        if !grid.are_neighbors(
            pos,
            Position {
                x: x as usize,
                y: (y - 1) as usize,
            },
        ) {
            offset_top += wall_thickness_height;
            rect_height -= wall_thickness_height;
        }

        draw_rectangle(
            (x_px + offset_left) as f32,
            (y_px + offset_top) as f32,
            rect_width as f32,
            rect_height as f32,
            color,
        );
    }

    for position in positions.iter() {
        draw_cell(position.clone(), grid, color);
    }

    if accent_on_last {
        if let Some(last) = positions.last() {
            draw_cell(last.to_owned(), grid, ACCENT_COLOR);
        }
    }
}

async fn animate_path(path: Vec<Position>, grid: &Grid) {
    let mut count = 0;
    loop {
        clear_background(BLACK);

        display_grid(
            grid.get_cells_positions(),
            &grid,
            constants::gui_constants::CELL_COLOR,
            false,
        );
        display_grid(
            path[0..count].to_vec(),
            &grid,
            constants::gui_constants::PATH_COLOR,
            true,
        );
        // sleep(Duration::from_millis(50));

        if count < path.len() {
            count += 1;
        }

        next_frame().await
    }
}

async fn display_path_loop_erasure(
    path: &maze_generator::Path,
    lep: &maze_generator::Path,
    grid: &model::grid::Grid,
) {
    println!(
        "Displaying path of length: {}, lep length: {}",
        path.length(),
        lep.length()
    );
    // Animate walk
    clear_background(BLACK);
    display_grid(grid.get_cells_positions(), &grid, CELL_COLOR, false);
    display_grid(path.get_cells_positions(), &grid, PATH_COLOR, true);
    display_grid(vec![path.get_cells_positions()[0]], &grid, YELLOW, false);

    display_grid(lep.get_cells_positions(), grid, LEP_PATH_COLOR, false);
    next_frame().await;
    sleep(Duration::from_millis(500));
}

async fn animate_maze_creation(grid: &model::grid::Grid, internals: &MazeGenerationInternals) {
    for paths in internals.get_paths() {
        display_path_loop_erasure(&paths.0, &paths.1, &paths.2).await;
    }

    // loop {
    //     display_grid(grid.get_cells_positions(), grid, CELL_COLOR, false);
    //     next_frame().await;
    // }
}

#[macroquad::main(get_window_config)]
async fn main() {
    let mut grid = model::grid::Grid::new(
        constants::game_constants::NUMBER_OF_CELLS_X,
        constants::game_constants::NUMBER_OF_CELLS_Y,
    );

    let internals = maze_generator::mazify(&mut grid);
    // animate_maze_creation(&grid, &internals).await;
    let mouse: BFSMouse = BFSMouse::new(Position { x: 0, y: 0 });
    let mouse_path = mouse.solve(
        Position {
            x: grid.get_number_of_cells_x_y().0 as usize - 1,
            y: grid.get_number_of_cells_x_y().1 as usize - 1,
        },
        &grid,
    );

    // println!("Mouse path: ");
    // println!("{:?}", mouse_path);

    animate_path(mouse_path, &grid).await;
}
