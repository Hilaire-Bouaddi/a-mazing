mod constants;
mod maze_generator;
mod model;

use std::{thread::sleep, time::Duration};

use macroquad::prelude::*;

use constants::gui_constants::{
    ACCENT_COLOR, CELL_COLOR, LEP_PATH_COLOR, PATH_COLOR, get_window_config,
};
use model::cell::Position;
use model::grid::Grid;

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

async fn animate_path_creation() {
    let mut grid = model::grid::Grid::new(
        constants::game_constants::NUMBER_OF_CELLS_X,
        constants::game_constants::NUMBER_OF_CELLS_Y,
    );
    let mut initial_pos = grid.get_cells_positions()[0];
    let mut visited = vec![initial_pos];
    let mut path = maze_generator::random_walk(initial_pos, &mut grid, &visited);
    let mut count = 1;
    loop {
        clear_background(BLACK);

        if count >= path.length() {
            count = 1;
            grid = model::grid::Grid::new(
                constants::game_constants::NUMBER_OF_CELLS_X,
                constants::game_constants::NUMBER_OF_CELLS_Y,
            );
            initial_pos = grid.get_cells_positions()[0];
            visited = vec![initial_pos];
            path = maze_generator::random_walk(initial_pos, &mut grid, &visited);
            sleep(Duration::from_millis(500));
        }

        display_grid(
            grid.get_cells_positions(),
            &grid,
            constants::gui_constants::CELL_COLOR,
            false,
        );

        sleep(Duration::from_millis(50));

        display_grid(
            path.get_cells_positions()[0..count].to_vec(),
            &grid,
            constants::gui_constants::PATH_COLOR,
            true,
        );
        count += 1;

        next_frame().await
    }
}

async fn animate_path_loop_erasure(
    path: &maze_generator::Path,
    lep: &maze_generator::Path,
    grid: &model::grid::Grid,
) {
    let mut counter = path.length();

    println!(
        "Displaying path of length: {}, lep length: {}",
        path.length(),
        lep.length()
    );
    // Animate walk
    loop {
        clear_background(BLACK);
        display_grid(grid.get_cells_positions(), &grid, CELL_COLOR, false);

        display_grid(
            path.get_cells_positions()[0..counter].to_vec(),
            &grid,
            PATH_COLOR,
            true,
        );

        display_grid(vec![path.get_cells_positions()[0]], &grid, YELLOW, false);

        if counter == path.length() {
            display_grid(lep.get_cells_positions(), grid, LEP_PATH_COLOR, false);
            next_frame().await;
            sleep(Duration::from_millis(300));
            break;
        }
        counter += 1;
        next_frame().await;
    }
}

async fn animate_maze_creation(grid: &mut model::grid::Grid) {
    let internals = maze_generator::mazify(grid);
    // for paths in internals.get_paths() {
    //     animate_path_loop_erasure(&paths.0, &paths.1, &paths.2).await;
    // }
    loop {
        display_grid(grid.get_cells_positions(), grid, CELL_COLOR, false);
        next_frame().await;
    }
}

#[macroquad::main(get_window_config)]
async fn main() {
    let mut grid = model::grid::Grid::new(
        constants::game_constants::NUMBER_OF_CELLS_X,
        constants::game_constants::NUMBER_OF_CELLS_Y,
    );

    // animate_path_creation().await;
    animate_maze_creation(&mut grid).await;
}
