use std::{default, iter::Peekable, sync::mpsc::{Receiver, Sender}, time::Instant, vec};

use macroquad::{experimental::coroutines::Coroutine, prelude::*};
use shared::{Move, MoveSeq, Solvable, Polygon, PartialMove, Drawable};
use tubaitu::Cube2;
use tribaitri::Cube3;
use std::fmt::Debug;

pub const WHITE_COL : Color     = color_u8![188, 192, 204, 255];
pub const YELLOW_COL: Color     = color_u8![0xCC, 0xCC, 0x00, 255];
pub const RED_COL   : Color     = color_u8![210, 15, 57, 255];
pub const ORANGE_COL: Color     = color_u8![254, 100, 11, 255];
pub const BLUE_COL  : Color     = color_u8![32, 159, 181, 255];
pub const GREEN_COL : Color     = color_u8![64, 160, 43, 255];
pub const BACKGROUND_COL: Color = color_u8![0x24, 0x27, 0x3a, 255];

pub const TEXT_COL: Color = color_u8![128, 135, 162, 255];

const SCREEN_WIDTH: usize = 700;
const SCREEN_HEIGHT: usize = 700;

#[derive(Debug, Default)]
struct Cube {
    tu: Cube2,
    tri: Cube3,
    curr: Curr, 
}

#[derive(Debug, Default)]
enum Curr { #[default] Tu, Tri }

impl Cube {
    fn solve(&self, prints_enabled: bool, outward_comms: Option<Sender<String>>) -> MoveSeq {
        match self.curr {
            Curr::Tu  => self.tu.solve(prints_enabled, outward_comms),
            Curr::Tri => self.tri.solve(prints_enabled, outward_comms),
        }
    }
    fn make_move(&mut self, moviment: Move) {
        match self.curr {
            Curr::Tu  => self.tu.make_move(moviment),
            Curr::Tri => self.tri.make_move(moviment),
        }
    }
    fn reset_current(&mut self) {
        match self.curr {
            Curr::Tu  => self.tu = Cube2::default(),
            Curr::Tri => self.tri = Cube3::default(),
        }
    }
    fn get_polys(&self, part_mov: Option<PartialMove>, width: usize, height: usize, scale: f64) -> Vec<Polygon> {
        match self.curr {
            Curr::Tu  => self.tu.get_polys(part_mov, width, height, scale),
            Curr::Tri => self.tri.get_polys(part_mov, width, height, scale),
        }
    }
}

#[derive(Debug)]
struct State {
    cube: Cube,
    kind: StateKind,
}

#[derive(Debug)]
enum StateKind {
    Manual {
        selected_move: Option<Move>,
        mid_move: Option<(Move, f64)>,
    },
    Scrambling {
        seq: Peekable<vec::IntoIter<Move>>,
        t: f64,
    },
    Solving(SolvingState)
}

#[derive(Debug)]
enum SolvingState {
    _Calculating { // Unused until wasm can deal with threads
        coroutine: Coroutine<MoveSeq>,
        comms: (String, Receiver<String>),
    },
    Ready {
        seq: Peekable<vec::IntoIter<Move>>,
        t: f64,
    },
}

impl State {
    fn curr_t(&self) -> f64 {
        match self.kind {
            StateKind::Manual { mid_move: Some((_, t)), .. }       => t,
            StateKind::Manual { mid_move: None, .. }               => 0.0,
            StateKind::Solving(SolvingState::_Calculating { .. })  => 0.0,
            StateKind::Solving(SolvingState::Ready { t, .. })      => t,
            StateKind::Scrambling { t, .. }                        => t,
        }
    }
    fn curr_mov(&mut self) -> Option<Move> {
        match &mut self.kind {
            StateKind::Manual { mid_move: Some((m, _)), .. }      => Some(*m),
            StateKind::Manual { mid_move: None, .. }              => None,
            StateKind::Solving(SolvingState::_Calculating { .. }) => None,
            StateKind::Solving(SolvingState::Ready { seq, .. })   => seq.peek().copied(),
            StateKind::Scrambling { seq, .. }                     => seq.peek().copied(),
        }
    }

    fn set_back_to_manual(&mut self) {
        self.kind = StateKind::Manual { selected_move: None, mid_move: None };
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Constants / initting
    let dt = 0.05;
    let mut state = State {
        cube: Cube::default(),
        kind: StateKind::Manual {
            selected_move: None,
            mid_move: None
        },
    };
    let solve_bind    = KeyCode::Q;
    let scramble_bind = KeyCode::S;
    let reset_bind    = KeyCode::P;

    let l_bind        = KeyCode::L;
    let r_bind        = KeyCode::R;
    let f_bind        = KeyCode::F;
    let b_bind        = KeyCode::B;
    let u_bind        = KeyCode::U;
    let d_bind        = KeyCode::D;


    loop {
        clear_background(BACKGROUND_COL);

        if is_key_pressed(scramble_bind) {
            let (_, scramble) = Cube2::random_scramble(rand::gen_range(10, 20));
            state.kind = StateKind::Scrambling { seq: scramble.into_iter().peekable(), t: 0.0 };
        }
        else if is_key_pressed(solve_bind) { 
            //let (tx, rx) = mpsc::channel();
            //let coroutine = start_coroutine(async move {
            //    state.cube.clone().solve(true, Some(tx))
            //});
            let solving_start = Instant::now();
            let seq = state.cube.solve(true, None).0.into_iter().peekable();
            let time_taken = solving_start.elapsed();

            println!("Found solution in: {}s", time_taken.as_secs_f32());

            state.kind = StateKind::Solving(SolvingState::Ready { seq, t: 0.0 });
        }
        else if is_key_pressed(reset_bind) { 
            println!("[INFO]: Resetting cube");
            state.cube.reset_current();
            state.set_back_to_manual();
        }

        match state.kind {
            StateKind::Manual { ref selected_move, mid_move: Some((mid_move, ref mut t))  } => {
                draw_selected_move(selected_move);

                *t += dt;
                if *t >= 1.0 {
                    state.set_back_to_manual();
                    state.cube.make_move(mid_move);
                }

            },
            StateKind::Manual { ref mut selected_move, mid_move: None } => {
                *selected_move = 
                     if is_key_pressed(r_bind) { Some(Move::R) }
                else if is_key_pressed(l_bind) { Some(Move::L) }
                else if is_key_pressed(u_bind) { Some(Move::U) }
                else if is_key_pressed(d_bind) { Some(Move::D) }
                else if is_key_pressed(f_bind) { Some(Move::F) }
                else if is_key_pressed(b_bind) { Some(Move::B) }
                else { *selected_move };

                draw_selected_move(selected_move);

                if let &mut Some(m) = selected_move {
                    if is_key_pressed(KeyCode::Left)       { state.kind = StateKind::Manual { selected_move: *selected_move, mid_move: Some((m.opposite(), 0.0)) }}
                    else if is_key_pressed(KeyCode::Right) { state.kind = StateKind::Manual { selected_move: *selected_move, mid_move: Some((m,            0.0)) }}
                } 

            },
            StateKind::Scrambling { ref mut seq, ref mut t } => {
                let scrambling_dt = 0.05;

                draw_current_move_seq("Scrambling: ", seq);
                if let Some(scramble_move) = &mut seq.peek() { // Advance and check while we're at it
                    *t += scrambling_dt;
                    if *t >= 1.0 {
                        state.cube.make_move(scramble_move.clone());
                        seq.next();
                        state.kind = StateKind::Scrambling { seq: seq.clone() , t: 0.0 };
                    }
                } else {
                    state.set_back_to_manual();
                }

            },
            StateKind::Solving(SolvingState::_Calculating { coroutine, comms: (mut acc_comms, rx_comms) })  => {
                let mut text = "Finding solution: ".to_string();
                text.push_str(&acc_comms);
                draw_simple_text(&text);

                if coroutine.is_done() {
                    state.kind = StateKind::Solving(SolvingState::Ready {
                        seq: coroutine.retrieve().expect("We just checked that it's done").0.into_iter().peekable(),
                        t: 0.0 });
                }
                else {
                    while let Ok(new_acc_str) = rx_comms.try_recv() {
                        acc_comms = new_acc_str;
                    }
                    state.kind = StateKind::Solving(SolvingState::_Calculating { coroutine, comms: (acc_comms, rx_comms) });
                }
            },
            StateKind::Solving(SolvingState::Ready { ref mut seq, ref mut t }) => {
                let solving_dt = 0.05;

                draw_current_move_seq("Solve: ", seq);
                if let Some(solve_move) = &mut seq.peek() {
                    *t += solving_dt;
                    if *t >= 1.0 {
                        state.cube.make_move(solve_move.clone());
                        seq.next();
                        state.kind = StateKind::Solving(SolvingState::Ready { seq: seq.clone(), t: 0.0 });
                    }
                } else {
                    state.set_back_to_manual();
                }

            }
        }
            

        let curr_move = state.curr_mov().and_then(|m| Some(PartialMove { mov: m, lerp_t: state.curr_t() })); 
        let polys = state.cube.get_polys(curr_move, SCREEN_WIDTH, SCREEN_HEIGHT, 7.0);

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

fn draw_simple_text(text: &str) {
    let font_size = 40.0;
    draw_text(&text, 10.0, font_size*1.2, font_size, TEXT_COL);
}

fn draw_current_move_seq(pre: &str, seq: &Peekable<vec::IntoIter<Move>>) {
    let font_size = 50.0;

    let seq: MoveSeq = seq.clone().collect::<Vec<Move>>().into();
    let seq_text = seq.to_string();

    let mut text = pre.to_string();
    text.push_str(&seq_text);

    draw_text(&text, 10.0, font_size*1.2, font_size, TEXT_COL);
    
}

fn draw_selected_move(m: &Option<Move>) {
    let val: String = m.and_then(|o| Some(o.to_string())).or(Some("None".into())).expect("Cannot panic");

    let mut text = String::from("Selected: ");
    text.push_str(&val);

    let font_size = 50.0;
    draw_text(&text, 10.0, font_size*1.2, font_size, TEXT_COL);
}


fn draw_quad(p0: Vec2, p1: Vec2, p2: Vec2, p3: Vec2, col: Color) {
    draw_triangle(p0, p1, p2, col);
    draw_triangle(p0, p2, p3, col);
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
