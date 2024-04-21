use crate::*;

impl Piece {
    pub fn new(top_col: Color, front_col: Color) -> Piece {
	let rotation = PieceRotation::from_color_pair(top_col, front_col);
	Piece { rotation }
    }

    pub fn to_color_sequence(&self) -> [Color; 6] {
	let (top, front) = self.rotation.to_color_pair();
	let (down, back) = (top.opposite(), front.opposite());
	let (left, right) = self.rotation.cross_product();
	[right, front, top, left, back, down]
    }

    pub fn rotate_wrong(&mut self, Move { side, prime }: &Move) {
	let mut colors = self.to_color_sequence();

	let mut seq = match side {
	    MoveSide::R => COLOR_RIGHT_SEQ,
	    MoveSide::L => COLOR_LEFT_SEQ,
	    MoveSide::U => COLOR_UP_SEQ,
	    MoveSide::D => COLOR_DOWN_SEQ,
	    MoveSide::F => COLOR_FRONT_SEQ,
	    MoveSide::B => COLOR_BACK_SEQ,
	};

	if *prime { seq = reverse_seq(seq) }
	
	cycle_items(&mut colors, seq);

	self.rotation = PieceRotation::from_color_pair(colors[2], colors[1]);
	
    }
    pub fn rotate(&mut self, mov: &Move) {
	let [right, front, top, left, back, down] = self.to_color_sequence();
	let new_colors: [Color; 6] = match mov {
	    &Move { side: MoveSide::R, prime: false }
	    | &Move { side: MoveSide::L, prime: true }
	    => [right, down, front, left, top, back],
	    &Move { side: MoveSide::L, prime: false }
	    | &Move { side: MoveSide::R, prime: true }
	    => [right, top, back, left, down, front],
	    &Move { side: MoveSide::U, prime: false }
	    | &Move { side: MoveSide::D, prime: true }
	    => [back, right, top, front, left, down],
	    &Move { side: MoveSide::D, prime: false }
	    | &Move { side: MoveSide::U, prime: true }
	    => [front, left, top, back, right, down],
	    &Move { side: MoveSide::F, prime: false }
	    | &Move { side: MoveSide::B, prime: true }
	    => [top, front, left, down, back, right],
	    &Move { side: MoveSide::B, prime: false }
	    | &Move { side: MoveSide::F, prime: true }
	    => [down, front, right, top, back, left],
	};

	self.rotation = PieceRotation::from_color_pair(new_colors[SIDE_TOP], new_colors[SIDE_FRONT]);
    }
}

impl PieceRotation {
    pub fn to_color_pair(&self) -> (Color, Color) {
	use PieceRotation as PR;
	match self {
	    PR::WB => (Color::White, Color::Blue),
	    PR::WR => (Color::White, Color::Red),
	    PR::WO => (Color::White, Color::Orange),
	    PR::WG => (Color::White, Color::Green),
	    PR::RW => (Color::Red, Color::White),
	    PR::BW => (Color::Blue, Color::White),
	    PR::OW => (Color::Orange, Color::White),
	    PR::GW => (Color::Green, Color::White),
	    PR::YR => (Color::Yellow, Color::Red),
	    PR::YB => (Color::Yellow, Color::Blue),
	    PR::YO => (Color::Yellow, Color::Orange),
	    PR::YG => (Color::Yellow, Color::Green),
	    PR::RY => (Color::Red, Color::Yellow),
	    PR::BY => (Color::Blue, Color::Yellow),
	    PR::OY => (Color::Orange, Color::Yellow),
	    PR::GY => (Color::Green, Color::Yellow),
	    PR::OG => (Color::Orange, Color::Green),
	    PR::GO => (Color::Green, Color::Orange),
	    PR::OB => (Color::Orange, Color::Blue),
	    PR::BO => (Color::Blue, Color::Orange),
	    PR::RG => (Color::Red, Color::Green),
	    PR::GR => (Color::Green, Color::Red),
	    PR::RB => (Color::Red, Color::Blue),
	    PR::BR => (Color::Blue, Color::Red),
	}
    }
    pub fn from_color_pair(top: Color, front: Color) -> Self {
	use PieceRotation as PR;
	use Color as C;
	match (top, front) {
	    (C::White, C::White) 
		| (C::Red, C::Red)
		| (C::Blue, C::Blue)
		| (C::Orange, C::Orange)
		| (C::Green, C::Green)
		| (C::Yellow, C::Yellow)
		=> panic!("Cannot have repeated colors in a cube"),
	    (C::White, C::Yellow)
		| (C::Yellow, C::White)
		| (C::Blue, C::Green)
		| (C::Green, C::Blue)
		| (C::Orange, C::Red)
		| (C::Red, C::Orange)
		=> panic!("Front and top may not be opposite eachother"),
	    (C::White,  C::Red)    => PR::WR,
	    (C::White,  C::Blue)   => PR::WB,
	    (C::White,  C::Orange) => PR::WO,
	    (C::White,  C::Green)  => PR::WG,
	    (C::Red,    C::White)  => PR::RW,
	    (C::Red,    C::Blue)   => PR::RB,
	    (C::Red,    C::Yellow) => PR::RY,
	    (C::Red,    C::Green)  => PR::RG,
	    (C::Blue,   C::White)  => PR::BW,
	    (C::Blue,   C::Red)    => PR::BR,
	    (C::Blue,   C::Yellow) => PR::BY,
	    (C::Blue,   C::Orange) => PR::BO,
	    (C::Yellow, C::Red)    => PR::YR,
	    (C::Yellow, C::Blue)   => PR::YB,
	    (C::Yellow, C::Orange) => PR::YO,
	    (C::Yellow, C::Green)  => PR::YG,
	    (C::Orange, C::White)  => PR::OW,
	    (C::Orange, C::Blue)   => PR::OB,
	    (C::Orange, C::Yellow) => PR::OY,
	    (C::Orange, C::Green)  => PR::OG,
	    (C::Green,  C::White)  => PR::GW,
	    (C::Green,  C::Red)    => PR::GR,
	    (C::Green,  C::Yellow) => PR::GY,
	    (C::Green,  C::Orange) => PR::GO,
	}
    }
    /// Returns (left, right)
    pub fn cross_product(&self) -> (Color, Color) {
	use Color as C;
	use PieceRotation as PR;
	match self {
	    PR::WR => (C::Green,  C::Blue),
	    PR::WB => (C::Red,    C::Orange),
	    PR::WO => (C::Blue,   C::Green),
	    PR::WG => (C::Orange, C::Red),
	    PR::RW => (C::Blue,   C::Green),
	    PR::BW => (C::Orange, C::Red),
	    PR::OW => (C::Green,  C::Blue),
	    PR::GW => (C::Red,    C::Orange),
	    PR::YR => (C::Blue,   C::Green),
	    PR::YB => (C::Orange, C::Red),
	    PR::YO => (C::Green,  C::Blue),
	    PR::YG => (C::Red,    C::Orange),
	    PR::RY => (C::Green,  C::Blue),
	    PR::BY => (C::Red,    C::Orange),
	    PR::OY => (C::Blue,   C::Green),
	    PR::GY => (C::Orange, C::Red),
	    PR::OG => (C::Yellow, C::White),
	    PR::GO => (C::White,  C::Yellow),
	    PR::OB => (C::White,  C::Yellow),
	    PR::BO => (C::Yellow, C::White),
	    PR::RG => (C::White,  C::Yellow),
	    PR::GR => (C::Yellow, C::White),
	    PR::RB => (C::Yellow, C::White),
	    PR::BR => (C::White,  C::Yellow),
	}
    }
}


impl Color {
    pub fn new(c: char) -> Color {
        use Color as C;
        match c.to_ascii_uppercase() {
            'W' => C::White,
            'R' => C::Red,
            'B' => C::Blue,
            'Y' => C::Yellow,
            'O' => C::Orange,
            'G' => C::Green,
            _ => panic!("Char {c} is not a valid color"),
        }
    }
    pub fn opposite(&self) -> Self {
	use Color as C;
	match self {
	    C::White  => C::Yellow,
	    C::Yellow => C::White,
	    C::Red    => C::Orange,
	    C::Orange => C::Red,
	    C::Blue   => C::Green,
	    C::Green  => C::Blue,
	}
    }
}
