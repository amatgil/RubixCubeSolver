
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Color { #[default] White, Red, Blue, Yellow, Orange, Green }

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let o = match self {
            Color::White  => "W",
            Color::Red    => "R",
            Color::Blue   => "B",
            Color::Yellow => "Y",
            Color::Orange => "O",
            Color::Green  => "G",
        };

        write!(f, "{o}")
    }
}

impl Color {
    pub fn from(c: char) -> Option<Color> {
        use Color as C;
        match c.to_ascii_uppercase() {
            'W' => Some(C::White),
            'R' => Some(C::Red),
            'B' => Some(C::Blue),
            'Y' => Some(C::Yellow),
            'O' => Some(C::Orange),
            'G' => Some(C::Green),
            _ => None,
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
