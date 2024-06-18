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

#[derive(Debug, Clone, Copy)]
struct State {
    cube: Cube2,
    kind: StateKind,
}

#[derive(Debug, Clone, Copy)]
enum StateKind {
    Manual {
        selected_move: Option<Move>,
        mid_move: Option<(Move, f64)>,
    },
    Solving
}

impl State {
    fn curr_t(&self) -> f64 {
        match self.kind {
            StateKind::Manual { mid_move: Some((_, t)), .. } => t,
            StateKind::Manual { mid_move: None, .. }         => 0.0,
            StateKind::Solving                               => todo!("Solving not yet implemented"),
        }
    }
    fn curr_mov(&self) -> Option<Move> {
        match self.kind {
            StateKind::Manual { mid_move: Some((m, _)), .. } => Some(m),
            StateKind::Manual { mid_move: None, .. }         => None,
            StateKind::Solving                               => todo!("Solving not yet implemented"),
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Constants / initting
    let dt = 0.05;
    let mut state = State {
        cube: Cube2::default(),
        kind: StateKind::Manual {
            selected_move: None,
            mid_move: None
        },
    };

    loop {
        clear_background(BACKGROUND_COL);

        dbg!(&state.kind);
        match state.kind {
            StateKind::Manual { mid_move: Some((mid_move, ref mut t)), .. } => {
                *t += dt;
                if *t >= 1.0 || *t <= -1.0 {
                    state.kind = StateKind::Manual { selected_move: None, mid_move: None };
                    state.cube.make_move(mid_move);
                }
            },
            StateKind::Manual { ref mut selected_move, mid_move: None } => {
                *selected_move = 
                    if is_key_pressed(KeyCode::R) { Some(Move::R) }
                else if is_key_pressed(KeyCode::L) { Some(Move::L) }
                else if is_key_pressed(KeyCode::U) { Some(Move::U) }
                else if is_key_pressed(KeyCode::D) { Some(Move::D) }
                else if is_key_pressed(KeyCode::U) { Some(Move::U) }
                else if is_key_pressed(KeyCode::F) { Some(Move::F) }
                else if is_key_pressed(KeyCode::B) { Some(Move::B) }
                else { *selected_move };

                if let &mut Some(m) = selected_move {
                    if is_key_pressed(KeyCode::Left)       { state.kind = StateKind::Manual { selected_move: *selected_move, mid_move: Some((m.opposite(), 0.0)) }}
                    else if is_key_pressed(KeyCode::Right) { state.kind = StateKind::Manual { selected_move: *selected_move, mid_move: Some((m, 0.0)) }}
                } 
            },
            StateKind::Solving => {
                todo!("No solving capabilities yet!");
            },
        }
            

        let polys = get_polys(&state.cube,
                      state.curr_mov().and_then(|m| Some(PartialMove { mov: m, lerp_t: state.curr_t() })),
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
