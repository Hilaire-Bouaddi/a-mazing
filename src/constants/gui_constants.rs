use macroquad::{
    color::{Color, GRAY, GREEN, PURPLE, RED},
    window::Conf,
};

pub const WINDOW_SIZE_X: usize = 600;
pub const WINDOW_SIZE_Y: usize = 600;

pub const WALL_RATIO: f64 = 0.05;

pub const CELL_COLOR: Color = GRAY;
pub const PATH_COLOR: Color = GREEN;
pub const LEP_PATH_COLOR: Color = PURPLE;
pub const ACCENT_COLOR: Color = RED;

pub fn get_window_config() -> Conf {
    Conf {
        window_title: "Explorer".to_owned(),
        window_width: WINDOW_SIZE_X as i32,
        window_height: WINDOW_SIZE_Y as i32,
        fullscreen: false,
        ..Default::default()
    }
}
