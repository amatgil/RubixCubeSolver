use macroquad::prelude::*;
use shared::Move;
use tubaitu::{get_polys, Cube2, PartialMove};

pub const WHITE_COL : Color     = color_u8![188, 192, 204, 255];
pub const YELLOW_COL: Color     = color_u8![0xCC, 0xCC, 0x00, 255];
pub const RED_COL   : Color     = color_u8![210, 15, 57, 255];
pub const ORANGE_COL: Color     = color_u8![254, 100, 11, 255];
pub const BLUE_COL  : Color     = color_u8![32, 159, 181, 255];
pub const GREEN_COL : Color     = color_u8![64, 160, 43, 255];
pub const BACKGROUND_COL: Color = color_u8![0x24, 0x27, 0x3a, 255];

const SCREEN_WIDTH: usize = 700;
const SCREEN_HEIGHT: usize = 700;

#[macroquad::main(window_conf)]
async fn main() {
    let mut cube = Cube2::default();

    let mut t = 0.0;
    loop {
        clear_background(BACKGROUND_COL);

        let polys = get_polys(&cube, Some(PartialMove { mov: Move::U, lerp_t: t }), SCREEN_WIDTH, SCREEN_HEIGHT, 7.0);

        for poly in polys {
            let col = poly.color;

            draw_quad(
                Vec2::new(poly.points[0].0 as f32, poly.points[0].1 as f32),
                Vec2::new(poly.points[1].0 as f32, poly.points[1].1 as f32),
                Vec2::new(poly.points[2].0 as f32, poly.points[2].1 as f32),
                Vec2::new(poly.points[3].0 as f32, poly.points[3].1 as f32),
                color_u8![col[0], col[1], col[2], 255]
            );
            
        }

        t += 0.01;
        next_frame().await
    }
}

fn draw_quad(p0: Vec2, p1: Vec2, p2: Vec2, p3: Vec2, col: Color) {
    draw_triangle(
        p0,
        p1,
        p2,
        col
    );
    draw_triangle(
        p0,
        p2,
        p3,
        col
    );
}

fn window_conf() -> Conf {
    Conf {
        window_title: "tubaitu".to_owned(),
        window_width: SCREEN_WIDTH as i32,
        window_height: SCREEN_HEIGHT as i32,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}
