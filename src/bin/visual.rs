use macroquad::prelude::*;
use tubaitu::{get_polys, Cube2};

pub const WHITE_COL : Color     = color_u8![188, 192, 204, 255];
pub const YELLOW_COL: Color     = color_u8![0xCC, 0xCC, 0x00, 255];
pub const RED_COL   : Color     = color_u8![210, 15, 57, 255];
pub const ORANGE_COL: Color     = color_u8![254, 100, 11, 255];
pub const BLUE_COL  : Color     = color_u8![32, 159, 181, 255];
pub const GREEN_COL : Color     = color_u8![64, 160, 43, 255];
pub const BACKGROUND_COL: Color = color_u8![0x24, 0x27, 0x3a, 255];

const SCREEN_WIDTH: usize = 700;
const SCREEN_HEIGHT: usize = 700;

#[macroquad::main("tubaitu")]
async fn main() {
    let mut cube = Cube2::default();

    loop {
        clear_background(BACKGROUND_COL);

        let polys = get_polys(&cube, None, SCREEN_WIDTH, SCREEN_HEIGHT);

        for poly in polys {
            let col = poly.color;
            let col = color_u8![col[0], col[1], col[2], 255];
            for ps in poly.points.windows(2) {
                let p0 = ps[0];
                let p1 = ps[1];

                draw_line(
                    p0.0 as f32,
                    p0.1 as f32,
                    p1.0 as f32,
                    p1.1 as f32,
                    2.0,
                    col
                );
            }
        }

        next_frame().await
    }
}
