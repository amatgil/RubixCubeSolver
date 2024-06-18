use macroquad::prelude::*;
use shared::{Move, Solvable};
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

struct State {
    cube: Cube2,
    kind: StateKind,
}

enum StateKind {
    Manual,
    Solving
}

#[macroquad::main(window_conf)]
async fn main() {
    // Constants / initting
    let dt = 0.05;
    let mut state = State {
        cube: Cube2::default(),
        kind: StateKind::Manual,
    };

    let mut t: f64 = 0.0;
    let mut curr_mov = None;

    // Manual
    let mut manually_moving = false;
    let mut manual_move = Move::R;

    loop {
        clear_background(BACKGROUND_COL);

        match state.kind {
            StateKind::Manual => {
                if manually_moving {
                    t += dt * if manual_move.is_prime() { -1.0 } else { 1.0} ;
                    if t >= 1.0 || t <= -1.0 {
                        manually_moving = false;
                        t = 0.0;
                        state.cube.make_move(manual_move);
                    }

                } else {
                    curr_mov = 
                        if is_key_pressed(KeyCode::R) { Some(Move::R) }
                    else if is_key_pressed(KeyCode::L) { Some(Move::L) }
                    else if is_key_pressed(KeyCode::U) { Some(Move::U) }
                    else if is_key_pressed(KeyCode::D) { Some(Move::D) }
                    else if is_key_pressed(KeyCode::U) { Some(Move::U) }
                    else if is_key_pressed(KeyCode::F) { Some(Move::F) }
                    else if is_key_pressed(KeyCode::B) { Some(Move::B) }
                    else { curr_mov };

                    if let Some(m) = curr_mov {
                        if is_key_pressed(KeyCode::Left) {
                            manually_moving = true;
                            manual_move = m.opposite();
                        }
                        else if is_key_pressed(KeyCode::Right) {
                            manual_move = m;
                            manually_moving = true;
                        }
                    } else {
                        t = 0.0;
                    }
                }
            },
            StateKind::Solving => {
            },
        }
            

        let polys = get_polys(&state.cube,
                      curr_mov.and_then(|m| Some(PartialMove { mov: m, lerp_t: t })),
                      SCREEN_WIDTH, SCREEN_HEIGHT, 7.0);

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
