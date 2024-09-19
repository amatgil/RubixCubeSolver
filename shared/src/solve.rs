/// Default solving algorithm (only viable for 2x2)

use crate::*;

use std::{
    collections::{HashSet, VecDeque}, fmt::Display, hash::Hash, ops::Deref, rc::Rc, sync::mpsc::Sender
};


#[derive(Clone)]
struct State<C> {
    past_state: Option<(Rc<State<C>>, Move)>, 
    length_of_path: usize,
    cube: C,
}

impl<C: Solvable> Hash for State<C> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.cube.hash(state);
    }
}
impl<C: Solvable> PartialEq for State<C> {
    fn eq(&self, rhs: &Self) -> bool {
        self.cube == rhs.cube
    }
}
impl<C: Solvable> Eq for State<C> {}

fn advance_bfs<C: Solvable>(
    visited: &mut HashSet<Rc<State<C>>>,
    queue: &mut VecDeque<Rc<State<C>>>,
) {
    let current_depth = queue.back().unwrap().length_of_path;
    while let Some(state) = queue.pop_front() {
        if state.length_of_path > current_depth { return; }
        for (m, y) in find_adjacents(&state.cube) {
            let new_state = Rc::new(State {
                past_state: Some((Rc::clone(&state), m)),
                length_of_path: state.length_of_path + 1,
                cube: y,
            });
            if !have_we_seen_this_state_before(visited, new_state.clone()) {
                visited.insert(new_state.clone());
                queue.push_back(new_state);
            }
        }
    }
}

fn have_we_seen_this_state_before<C: Solvable>(
    seen: &HashSet<Rc<State<C>>>,
    new: Rc<State<C>>,
) -> bool {
    seen.contains(&new) // Equality only depends on the cube
}

fn find_adjacents<C: Solvable>(x: &C) -> Vec<(Move, C)> {
    let moviments = C::moves_of_adjacency();

    let mut t = Vec::new();
    for mov in moviments {
        let mut alt = x.clone();
        alt.make_move(mov);
        t.push((mov, alt))
    }
    t
}

pub fn cycle_items<T: Clone, const N: usize>(v: &mut [T; N], idxs: [usize; 4]) {
    v.swap(idxs[0], idxs[1]);
    v.swap(idxs[0], idxs[2]);
    v.swap(idxs[0], idxs[3]);
}

pub fn reverse_seq([a, b, c, d]: [usize; 4]) -> [usize; 4] {
    [d, c, b, a]
}

pub trait Solvable: Display + Eq + Sized + Default + Clone + Hash {
    fn moves_of_adjacency() -> Vec<Move>;
    fn make_move(&mut self, movimement: Move);

    fn solve(&self, prints_enabled: bool, mut outward_comms: Option<Sender<String>>) -> MoveSeq {
        let first_state_unsolved = Rc::new(State {
            cube: self.clone(),
            past_state: None,
            length_of_path: 0,
        });
        let mut w_from_unsolved: HashSet<Rc<State<Self>>> = HashSet::from([first_state_unsolved.clone()]);
        let mut queue_from_unsolved: VecDeque<Rc<State<Self>>> = VecDeque::from([first_state_unsolved]);

        let first_state_solved = Rc::new(State {
            past_state: None,
            length_of_path: 0,
            cube: Self::default(),
        });
        let mut w_from_solved = HashSet::from([first_state_solved.clone()]);
        let mut queue_from_solved = VecDeque::from([first_state_solved]);

        while w_from_solved.is_disjoint(&w_from_unsolved) {
            advance_bfs(&mut w_from_unsolved, &mut queue_from_unsolved);
            advance_bfs(&mut w_from_solved, &mut queue_from_solved);
            let comms = format!("\r {} visitats", w_from_unsolved.len() + w_from_solved.len());
            print_inline(&comms);
            if let Some(ref mut sender) = outward_comms { sender.send(comms).expect("Should not have hung up"); }
        }
        if prints_enabled {
            println!(); 
            println!("[INFO]: Found solution after exploring: {} states from unsolved and {} states from solved",
                     w_from_unsolved.len(),
                     w_from_solved.len(),
            );
            // TODO: This only prints one solution, even though we've likely found many. This should be iterated through and the "best" one picked out.
            //       The definition of "best" should include something like length and the ratio of 'nice' moves (U, F, R) to 'weird' moves (the rest, like B')
            println!(
                "[INFO]: Number of intersecting states found is: {}",
                w_from_solved.intersection(&w_from_unsolved).count()
            );
        }
        
        let schrodinger_state: &State<_> =
            (*w_from_solved.intersection(&w_from_unsolved).next().unwrap()).deref();
        let mut path_from_unsolved: Vec<Move> =
            extract_path_from_first_state(w_from_unsolved.get(schrodinger_state).cloned());
        let path_from_solved: Vec<Move> =
            extract_path_from_first_state(w_from_solved.get(schrodinger_state).cloned());

        if prints_enabled {
            println!("[INFO]: Found halves of the math: merging...");

            for m in path_from_solved.clone().into_iter().rev() {
                path_from_unsolved.push(m.opposite());
            }
            println!("[INFO]: Verifying solution...");
        }

        let mut test_cube = self.clone();
        for m in &path_from_unsolved {
            test_cube.make_move(*m);
        }
        if prints_enabled {
            if test_cube == Self::default() { println!("[INFO]: Verification succeeded") }
            else { println!("[ERROR]: Verification incorrect, missing moves for linking rotation") }
        }

        path_from_unsolved.into()
    }

    fn cycle_elements<const N: usize>(
        pieces: &mut [Piece; N],
        mut seq: [usize; 4],
        mov: Move,
    ) {
        if mov.is_prime() { seq = reverse_seq(seq); }
        cycle_items(pieces, seq); // Move the pieces
        for i in seq { pieces[i].rotate(mov) } // Rotate the pieces we just cycled around
    }

    fn scramble(moves: &MoveSeq) -> Self {
        let mut c = Self::default();
        for m in moves.iter() { c.make_move(*m) }
        c
    }
    fn random_scramble(length: usize) -> (Self, MoveSeq) {
        fn get_move_from_n(n: usize) -> Move { Move(n as u8) }
        use rand::{RngCore, SeedableRng};
        use rand::rngs::SmallRng;

        #[cfg(target_family = "wasm")]
        let mut rng = SmallRng::from_seed([42; 16]);

        #[cfg(not(target_family = "wasm"))]
        let mut rng = SmallRng::from_seed([42; 32]);

        let mut scramble = vec![];
        for _ in 0..length {
            scramble.push(get_move_from_n(rng.next_u32() as usize % 12));
        }

        let c = Self::scramble(&scramble.clone().into());
        (c, scramble.into())
    }
}

/// inefficient, but only called twice so eh
fn extract_path_from_first_state<C>(s: Option<Rc<State<C>>>) -> Vec<Move> {
    let mut v = VecDeque::new();
    if s.is_none() { return v.into(); }
    let mut s: Rc<State<C>> = s.unwrap();
    while let Some((past_state, m)) = &s.past_state {
        v.push_front(*m);
        if past_state.past_state.is_none() { break; }
        s = (s.past_state).clone().unwrap().0;
    }

    v.into()
}

fn print_inline(text: &str) {
    use std::io::Write;
    print!("{text}");
    std::io::stdout().flush().unwrap();
}
