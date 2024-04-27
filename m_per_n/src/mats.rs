use crate::*;

use std::ops::*;

const FLOAT_EPSILON: f64 = 0.0001;

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
	Point { x, y, z }
    }
}

// MatRow impls
impl<const NCOLS: usize> From<[f64; NCOLS]> for MatRow<NCOLS> {
    fn from(v: [f64; NCOLS]) -> Self { MatRow::<NCOLS>(v) }
}

impl<const NCOLS: usize>  MatRow<NCOLS> {
    pub fn pivot_position(&self) -> Option<usize> {
        self.0.iter().position(|&e| e > FLOAT_EPSILON) // Position of first non-zero value
    }
}
// Elementary transformations on rows
// - Swap two rows
impl<const NCOLS: usize>  MatRow<NCOLS> {
    pub fn swap(&mut self, rhs: &mut Self) {
        std::mem::swap(self, rhs);
    }
}
// - Multiply by scalar
impl<const NCOLS: usize> Mul<f64> for MatRow<NCOLS> {
    type Output = Self;
    fn mul(self, lambda: f64) -> Self { MatRow::<NCOLS>(self.0.map(|i| i*lambda)) }
}
impl<const NCOLS: usize> Mul<MatRow<NCOLS>> for f64 {
    type Output = MatRow<NCOLS>;
    fn mul(self, rhs: MatRow<NCOLS>) -> Self::Output { MatRow::<NCOLS>(rhs.0.map(|i| i*self)) }
}

// Add/subtract two rows:
impl<const NCOLS: usize> Add for MatRow<NCOLS> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut out = [0.0; NCOLS];
        for i in 0..NCOLS { out[i] = self.0[i] + rhs.0[i]}
        MatRow::<NCOLS>(out)
    }
}
impl<const NCOLS: usize> Sub for MatRow<NCOLS> { 
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let mut out = [0.0; NCOLS];
        for i in 0..NCOLS { out[i] = self.0[i] - rhs.0[i]}
        MatRow::<NCOLS>(out)
    }
}

impl<const N: usize> Index<usize> for MatRow<N> {
    type Output = f64;
    fn index(&self, idx: usize) -> &Self::Output { &self.0[idx] }
}

impl<const N: usize> IndexMut<usize> for MatRow<N> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output { &mut self.0[idx] }
}

/// Generic (rectangular) impls
impl<const NF: usize, const NC: usize> Matrix<NF, NC> {
    #[allow(non_snake_case)]
    pub const fn ZERO() -> Matrix<NF, NC> {
        Matrix::<NF, NC>([MatRow::<NC>([0.0; NC]); NF])
    }
}

/// Square impls
impl<const N: usize> Matrix<N, N> {
    #[allow(non_snake_case)]
    pub const fn ID() -> Matrix<N, N> {
        let mut out = Matrix::<N, N>([MatRow::<N>([0.0; N]); N]);
        let mut i = 0;
        while i < N {
            out.0[i].0[i] = 1.0;
            i += 1;
        }
        out
    }

    pub fn inverse(&self) -> Option<Self> {
        // Applying these steps to our original matrix will reduce it to the identity, meaning 'inverse' will now be self^(-1)
        let mut original = self.clone();
        let mut inverse = Matrix::<N, N>::ID(); // Starts as ID, will become our result

        // Transform to row echelon form
        //   Align by pivot
        original.0.sort_by(|a, b| a.pivot_position().cmp(&b.pivot_position()));

        if original[0][0] < FLOAT_EPSILON { return None; } // If the first pivot isn't on the first column, there must be a column of zeros. Rank isn't full, no inverse available
        if original[N - 1].0.iter().all(|&e| e < FLOAT_EPSILON) { return None; } // Last row is zero, rank isn't full, no inverse exists

        //   Make all pivots be 1
        for t in 0..N {
            original[t] = (1.0 / original[t][t]) * original[t];
        }

        // Transform to reduced row echelon form (applying both to the original and ID to get our inverse)
        for j in (0..N).into_iter().rev() {
            for i in (0..N).into_iter().rev() {
                if i == j { continue; }
                original[i] = original[i] - original[i][j]*original[j];
                inverse[i] = inverse[i] - inverse[i][j]*inverse[j];
            }
        }





        // inverse is now our solution
        Some(inverse)
    }
}


/// Matrix addition (must have the same dimensions, enforced by type-system)
impl<const NF: usize, const NC: usize> Add<Matrix<NF, NC>> for Matrix<NF, NC> {
    type Output = Matrix<NF, NC>;
    fn add(mut self, rhs: Self) -> Self::Output {
        for y in 0..NF {
            for x in 0..NC { self.0[y][x] += rhs.0[y][x]; }
        }
        self
    }
}

/// Matrix subtraction (must have the same dimensions, enforced by type-system)
impl<const NF: usize, const NC: usize> Sub<Matrix<NF, NC>> for Matrix<NF, NC> {
    type Output = Matrix<NF, NC>;
    fn sub(mut self, rhs: Self) -> Self::Output {
        for y in 0..NF {
            for x in 0..NC { self.0[y][x] -= rhs.0[y][x] }
        }
        self
    }
}

/// Scalar times matrix, `l * A`
impl<const NF: usize, const NC: usize> Mul<f64> for Matrix<NF, NC> {
    type Output = Matrix<NF, NC>;
    fn mul(mut self, rhs: f64) -> Self::Output {
        for y in 0..NF {
            for x in 0..NC { self.0[y][x] = self.0[y][x]*rhs }
        }
        self
    }
}

/// Matrix times scalar, `A * l`
impl<const NF: usize, const NC: usize> Mul<Matrix<NF, NC>> for f64 {
    type Output = Matrix<NF, NC>;
    fn mul(self, mut rhs: Matrix<NF, NC>) -> Self::Output {
        for y in 0..NF {
            for x in 0..NC { rhs.0[y][x] = rhs.0[y][x]*self }
        }
        rhs
    }
}

/// Matrix multiplication, validity enforced by the type-system
impl<
    const N: usize,
    const C: usize, // Unused, NC of right
    const F: usize  // Unused, NF of left
> Mul<Matrix<N, C>> for Matrix<F, N> {
    type Output = Matrix<F, C>;
    fn mul(self, rhs: Matrix<N, C>) -> Self::Output {
        let mut out: Self::Output = Matrix::ZERO();
        for y in 0..F {
            for x in 0..C { out.0[y][x] = (0..N).map(|i| self.0[y][i]*rhs.0[i][x]).sum() }
        }
        out
    }
}

impl<const NF: usize, const NC: usize> Index<usize> for Matrix<NF, NC> {
    type Output = MatRow<NC>;
    fn index(&self, idx: usize) -> &Self::Output { 
        &self.0[idx]
    }
}
impl<const NF: usize, const NC: usize> IndexMut<usize> for Matrix<NF, NC> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output { 
        &mut self.0[idx]
    }
}

impl<const NF: usize, const NC: usize> Matrix <NF,NC>{
    pub fn transpose(&self) -> Matrix<NC,NF>{
        let mut result = Matrix::<NC,NF>::ZERO();
        for y in 0..NC {
            for x in 0..NF {
                result[y][x] = self[x][y];
            }
        }
        result
    }
}
/// For tests: panics if they're unequal
fn compare_mats<const NF: usize, const NC: usize>(a: [MatRow<NC>; NF], b: [MatRow<NC>; NF]) {
    for y in 0..NF {
        for x in 0..NC {
            if a[y][x] != b[y][x] {
                panic!("{a:?} is unequal from {b:?} at index ({x}, {y})");
            }
        }
    }
}

#[test]
fn mat_addition() {
    let a = Matrix::<2, 2>(
        [[1.0, 2.0].into(),
         [3.0, 4.0].into()]
    );
    let b = Matrix::<2, 2>(
        [[5.0, 6.0].into(),
         [-3.0, 1.5].into()]
    );

    let c = Matrix::<2, 2>(
        [[6.0, 8.0].into(),
         [0.0, 5.5].into()]
    );
    compare_mats((a+b).0, c.0);
}

#[test]
fn mat_subtraction() {
    let a = Matrix::<2, 2>(
        [[1.0, 2.0].into(),
         [3.0, 4.0].into()]
    );
    let b = Matrix::<2, 2>(
        [[5.0, 6.0].into(),
         [-3.0, 1.5].into()]
    );

    let c = Matrix::<2, 2>(
        [[-4.0, -4.0].into(),
         [6.0, 2.5].into()]
    );

    compare_mats((a-b).0, c.0);
}


#[test]
fn mat_mult_by_scalar() {
    let a = Matrix::<2, 2>(
        [[1.0, 2.0].into(),
         [0.0, 4.0].into()]
    );
    let k = -7.4;

    let c = Matrix::<2, 2>(
        [[-7.4, -14.8].into(),
         [-0.0, -29.6].into()]
    );
    // I hate float equality lmao
    compare_mats((a * k).0, c.0);
    compare_mats((k * a).0, c.0);
}

#[test]
fn mat_mult_square() {
    let a = Matrix::<2, 2>(
        [[1.0, 2.0].into(),
         [0.0, 4.0].into()]
    );

    let b = Matrix::<2, 2>(
        [[5.0, 6.0].into() ,
         [-3.0, 1.5].into()]
    );

    let c = Matrix::<2, 2>(
        [[-1.0, 9.0].into(),
        [-12.0, 6.0].into()]
    ); 

    let d = Matrix::<2, 2>(
        [[5.0, 34.0].into(),
        [-3.0, 0.0].into()]
    );


    compare_mats((a * b).0, c.0);

}

#[test]
fn mat_mult_rectangle() {
    let a = Matrix::<3, 2>(
        [[5.0,  6.0].into(),
         [-1.0, 1.0].into(),
         [-3.0, 1.5].into()]
    );
    
    let b = Matrix::<2, 3>(
        [[1.0, 2.0, 3.0].into(),
         [0.0, 4.0, 5.0].into()]
    );


    let c = Matrix::<3, 3>(
        [[5.0, 34.0, 45.0].into(),
         [-1.0, 2.0, 2.0 ].into(),
         [-3.0, 0.0, -1.5].into()],
    );

    compare_mats((a * b).0, c.0);
}


#[test]
fn mat_mult_rectangle_other() {
    let a = Matrix::<3, 2>(
        [[5.0,  6.0].into(),
         [-1.0, 1.0].into(),
         [-3.0, 1.5].into()]
    );
    
    let b = Matrix::<2, 3>(
        [[1.0, 2.0, 3.0].into(),
         [0.0, 4.0, 5.0].into()]
    );


    let c = Matrix::<2, 2>(
        [[-6.0, 12.5].into(),
         [-19.0, 11.5 ].into()]
    );

    compare_mats((b * a).0, c.0);
}

#[test]
fn zeros() {
    let one = Matrix::<1, 1>::ZERO();
    let two = Matrix::<2, 2>::ZERO();
    let three = Matrix::<3, 3>::ZERO();

    compare_mats(one.0, Matrix::<1, 1>([[0.0; 1].into(); 1]).0);
    compare_mats(two.0, Matrix::<2, 2>([[0.0; 2].into(); 2]).0);
    compare_mats(three.0, Matrix::<3, 3>([[0.0; 3].into(); 3]).0);

}
#[test]
fn matrix_identity() {
    let id = Matrix::<3, 3>::ID();
    let correct_id = Matrix::<3, 3>(
        [[1.0, 0.0, 0.0].into(),
            [0.0, 1.0, 0.0].into(),
            [0.0, 0.0, 1.0].into()]
    );
    compare_mats(id.0, correct_id.0);
}

#[test]
fn inverse_id() {
    let id = Matrix::<3, 3>::ID();
    let correct_id = Matrix::<3, 3>(
        [[1.0, 0.0, 0.0].into(),
            [0.0, 1.0, 0.0].into(),
            [0.0, 0.0, 1.0].into()]
    );
    compare_mats(id.inverse().unwrap().0, correct_id.0);
}

#[test]
fn inverse_zero() {
    assert!(Matrix::<1, 1>::ZERO().inverse().is_none());
    assert!(Matrix::<2, 2>::ZERO().inverse().is_none());
    assert!(Matrix::<3, 3>::ZERO().inverse().is_none());
    assert!(Matrix::<4, 4>::ZERO().inverse().is_none());
    assert!(Matrix::<5, 5>::ZERO().inverse().is_none());
}
