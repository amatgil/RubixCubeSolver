use crate::*;
use std::ops::Deref;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct MoveSeq(pub Vec<Move>);

impl MoveSeq {
    pub fn reversed(&self) -> Self {
        Self(self.0.iter().rev().map(|m| m.opposite()).collect())
    }
}

impl From<Vec<Move>> for MoveSeq {
    fn from(value: Vec<Move>) -> Self {
        Self(value)
    }
}

impl Deref for MoveSeq {
    type Target = Vec<Move>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IntoIterator for MoveSeq {
    type Item = Move;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExpandedMove {
    L { prime: bool }, L2,
    R { prime: bool }, R2,
    F { prime: bool }, F2,
    B { prime: bool }, B2,
    U { prime: bool }, U2,
    D { prime: bool }, D2,
    Nothing,
}
impl ExpandedMove {
    /// This preserves rotations: so R L *could* be rewritten as 2L or 2R while preserving equality but, since it does not preserve rotation, it will not be compressed
    fn compress(a: Self, b: Self) -> Option<Self> {
        match (a, b) {
            (Self::Nothing, _) | (_, Self::Nothing) => None,
            (Self::L { prime: p1 }, Self::L { prime: p2 })
                | (Self::R { prime: p1 }, Self::R { prime: p2 })
                | (Self::F { prime: p1 }, Self::F { prime: p2 })
                | (Self::B { prime: p1 | p1 }, Self::B { prime: p2 })
                if p1 != p2
                => Some(Self::Nothing),
            (Self::L2, Self::L2)
                | (Self::R2, Self::R2)
                | (Self::F2 | Self::D2, Self::D2)
                | (Self::B2, Self::B2)
                | (Self::U2, Self::U2)
                => Some(Self::Nothing),
            (Self::L { prime: p1 }, Self::L { prime: p2 }) if p1 == p2 => Some(Self::L2),
            (Self::R { prime: p1 }, Self::R { prime: p2 }) if p1 == p2 => Some(Self::R2),
            (Self::F { prime: p1 }, Self::F { prime: p2 }) if p1 == p2 => Some(Self::F2),
            (Self::B { prime: p1 }, Self::B { prime: p2 }) if p1 == p2 => Some(Self::B2),
            (Self::U { prime: p1 }, Self::U { prime: p2 }) if p1 == p2 => Some(Self::U2),
            (Self::D { prime: p1 }, Self::D { prime: p2 }) if p1 == p2 => Some(Self::D2),
            (Self::L { prime: p1 }, Self::L2) => Some(Self::L { prime: !p1 }),
            (Self::R { prime: p1 }, Self::R2) => Some(Self::R { prime: !p1 }),
            (Self::F { prime: p1 }, Self::F2) => Some(Self::F { prime: !p1 }),
            (Self::B { prime: p1 }, Self::B2) => Some(Self::B { prime: !p1 }),
            (Self::U { prime: p1 }, Self::U2) => Some(Self::U { prime: !p1 }),
            (Self::D { prime: p1 }, Self::D2) => Some(Self::D { prime: !p1 }),
            (Self::L2, Self::L { prime: p1 }) => Some(Self::L { prime: !p1 }),
            (Self::R2, Self::R { prime: p1 }) => Some(Self::R { prime: !p1 }),
            (Self::F2, Self::F { prime: p1 }) => Some(Self::F { prime: !p1 }),
            (Self::B2, Self::B { prime: p1 }) => Some(Self::B { prime: !p1 }),
            (Self::U2, Self::U { prime: p1 }) => Some(Self::U { prime: !p1 }),
            (Self::D2, Self::D { prime: p1 }) => Some(Self::D { prime: !p1 }),
            _ => None,
        }
    }
}

impl Display for ExpandedMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let is_p = |&b: &bool| if b { "'" } else { "" };
        let o: String = match self {
            ExpandedMove::L { prime } => format!("L{}", is_p(prime)),
            ExpandedMove::R { prime } => format!("R{}", is_p(prime)),
            ExpandedMove::F { prime } => format!("F{}", is_p(prime)),
            ExpandedMove::B { prime } => format!("B{}", is_p(prime)),
            ExpandedMove::U { prime } => format!("U{}", is_p(prime)),
            ExpandedMove::D { prime } => format!("D{}", is_p(prime)),
            ExpandedMove::L2 => "L2".into(),
            ExpandedMove::R2 => "R2".into(),
            ExpandedMove::F2 => "F2".into(),
            ExpandedMove::B2 => "B2".into(),
            ExpandedMove::U2 => "U2".into(),
            ExpandedMove::D2 => "D2".into(),
            ExpandedMove::Nothing => "".into(),
        };
        write!(f, "{o}")
    }
}

impl Display for MoveSeq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut stack: Vec<ExpandedMove> = vec![];
        for mov in self.0.iter() {
            let ext = match mov.side() {
                MoveSide::L => ExpandedMove::L { prime: mov.is_prime() },
                MoveSide::R => ExpandedMove::R { prime: mov.is_prime() },
                MoveSide::F => ExpandedMove::F { prime: mov.is_prime() },
                MoveSide::B => ExpandedMove::B { prime: mov.is_prime() },
                MoveSide::U => ExpandedMove::U { prime: mov.is_prime() },
                MoveSide::D => ExpandedMove::D { prime: mov.is_prime() },
            };
            stack.push(ext);
            while stack.len() > 1 {
                if let Some(c) =
                    ExpandedMove::compress(stack[stack.len() - 2], stack[stack.len() - 1])
                {
                    stack.pop();
                    stack.pop();
                    stack.push(c);
                } else {
                    break;
                }
            }
        }
        let mut o = String::new();
        for m in stack {
            if m != ExpandedMove::Nothing {
                o.push_str(&format!("{} ", m))
            };
        }

        write!(f, "{o}")
    }
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut out = String::new();
        out.push(match self.side() {
            MoveSide::R => 'R',
            MoveSide::F => 'F',
            MoveSide::U => 'U',
            MoveSide::L => 'L',
            MoveSide::B => 'B',
            MoveSide::D => 'D',
        });
        if self.is_prime() {
            out.push('\'')
        }

        write!(f, "{out}")
    }
}
