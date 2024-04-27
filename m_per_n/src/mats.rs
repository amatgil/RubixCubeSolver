use crate::*;

use std::{fmt::Display, ops::*};

const FLOAT_EPSILON: f64 = 0.0001;

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
	Point { x, y, z }
    }
}


impl<const NF: usize, const NC: usize> Display for Matrix<NF, NC> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::new();
        for i in 0..NF {
            buffer.push_str("|");
            for j in 0..NC {
                buffer.push_str(&format!("{:>4}", &self[i][j].to_string()));
            }
            buffer.push_str("|\n");
        }

        write!(f, "{buffer}")
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
    //pub fn inverse(&self) -> Option<Self> {
    //    // Applying these steps to our original matrix will reduce it to the identity, meaning 'inverse' will now be self^(-1)
    //    let mut m = self.clone();
    //    let mut inverse = Matrix::<N, N>::ID(); // Starts as ID, will become our result

    //    // Transform to row echelon form
    //    //   Align by pivot
    //    m.0.sort_by(|a, b| a.pivot_position().cmp(&b.pivot_position()));
    //    if m[0][0] < FLOAT_EPSILON { return None; } // If the first pivot isn't on the first column, there must be a column of zeros. Rank isn't full, no inverse available
    //    if m[N - 1].0.iter().all(|&e| e < FLOAT_EPSILON) { return None; } // Last row is zero, rank isn't full, no inverse exists

    //    for j in 0..N {
    //        for i in 0..N {
    //            if i <= j { continue; }
    //            m[i] = m[i][j] * m[j] - m[j][i]*m[i];
    //            inverse[i] = inverse[i][j] * inverse[j] - inverse[j][i]*inverse[i];
    //        }
    //    }

    //    //   Make all pivots be 1
    //    //for t in 0..N {
    //    //    m[t] = (1.0 / m[t][t]) * m[t];
    //    //    inverse[t] = (1.0 / inverse[t][t]) * inverse[t];
    //    //}

    //    for j in 0..N {
    //        for i in 0..N {
    //            if i > j { continue; }
    //            m[i] = m[i][j] * m[j] - m[j][i]*m[i];
    //            inverse[i] = inverse[i][j] * inverse[j] - inverse[j][i]*inverse[i];
    //        }
    //    }
    //    println!("{m}");

    //    // Transform to reduced row echelon form (applying both to the original and ID to get our inverse)
    //    //for j in (0..N).into_iter().rev() {
    //    //    for i in (0..N).into_iter().rev() {
    //    //        if i == j { continue; }
    //    //        original[i] = original[i] - original[i][j]*original[j];
    //    //        inverse[i] = inverse[i] - inverse[i][j]*inverse[j];
    //    //    }
    //    //}


    //    // inverse is now our solution
    //    Some(inverse)
    //}

    /// Uses the Gauss-Jordan method to find an inverse: applying the sequence of elementary transformations that would turn the original into the identity onto the identity, we end up with the resulting inverse.
    pub fn inverse(&self) -> Option<Self> {
        assert!(N == 3, "Inverses not yet implemented for non 3x3 matricies");
        let mut matrix = Matrix::<3, 6>([
            MatRow([self[0][0], self[0][1], self[0][2], 1.0, 0.0, 0.0]),
            MatRow([self[1][0], self[1][1], self[1][2], 0.0, 1.0, 0.0]),
            MatRow([self[2][0], self[2][1], self[2][2], 0.0, 0.0, 1.0]),
        ]);

        matrix[1] = matrix[0][0] * matrix[1] - matrix[1][0] * matrix[0];
        matrix[2] = matrix[0][0] * matrix[2] - matrix[2][0] * matrix[0];

        matrix[0] = matrix[1][1] * matrix[0] - matrix[0][1] * matrix[1];
        matrix[2] = matrix[1][1] * matrix[2] - matrix[2][1] * matrix[1];

        matrix[0] = matrix[2][2] * matrix[0] - matrix[0][2] * matrix[2];
        matrix[1] = matrix[2][2] * matrix[1] - matrix[1][2] * matrix[2];


        if matrix[0][0] == 0.0 || matrix[1][1] == 0.0 || matrix[2][2] == 0.0 {
            return None;
        }
        matrix[0] = 1.0/matrix[0][0] * matrix[0];
        matrix[1] = 1.0/matrix[1][1] * matrix[1];
        matrix[2] = 1.0/matrix[2][2] * matrix[2];

        let result = Matrix::<3,3>([
            MatRow([matrix[0][0+3], matrix[0][1+3], matrix[0][2+3]]),
            MatRow([matrix[1][0+3], matrix[1][1+3], matrix[1][2+3]]),
            MatRow([matrix[2][0+3], matrix[2][1+3], matrix[2][2+3]]),
        ]);

        let mut temporary_holder = Matrix::<N, N>::ZERO(); // We've only got 3x3's inverses done, we'll fix this bodge later
        for i in 0..3 {
            for j in 0..3 {
                temporary_holder[i][j] = result[i][j];
            }
        }


        Some(temporary_holder)
    }

    /// Uses Sarrus' rule to compute the determinant
    pub fn determinant(&self) -> f64 {
        todo!("Not yet fully thought through");
        let mut result = 0.0;
        for x in 0..N {
            let mut diag_acc = 0.0;
            for y in 0..N {
                diag_acc *= self.0[y][x];
            }
            diag_acc += result;
        }

        result
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
/// 
/// This relies of float equality, so it's not public. It should only be used in controlled ways, when
/// you already know the result
#[allow(unused)] // It's for testing only, of course it's "unused"
fn compare_mats<const NF: usize, const NC: usize>(a: Matrix<NF, NC>, b: Matrix<NF, NC>) {
    for y in 0..NF {
        for x in 0..NC {
            if a.0[y][x] != b.0[y][x] {
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
    compare_mats(a+b, c);
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

    compare_mats(a-b, c);
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
    compare_mats(a * k, c);
    compare_mats(k * a, c);
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


    compare_mats(a * b, c);
    compare_mats(b * a, d);

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

    compare_mats(a * b, c);
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

    compare_mats(b * a, c);
}

#[test]
fn zeros() {
    let one = Matrix::<1, 1>::ZERO();
    let two = Matrix::<2, 2>::ZERO();
    let three = Matrix::<3, 3>::ZERO();

    compare_mats(one, Matrix::<1, 1>([[0.0; 1].into(); 1]));
    compare_mats(two, Matrix::<2, 2>([[0.0; 2].into(); 2]));
    compare_mats(three, Matrix::<3, 3>([[0.0; 3].into(); 3]));

}
#[test]
fn matrix_identity() {
    let id = Matrix::<3, 3>::ID();
    let correct_id = Matrix::<3, 3>(
        [[1.0, 0.0, 0.0].into(),
         [0.0, 1.0, 0.0].into(),
         [0.0, 0.0, 1.0].into()]
    );
    compare_mats(id, correct_id);
}

#[test]
fn inverse_of_id() {
    compare_mats(Matrix::<1, 1>::ID().inverse().unwrap(), Matrix::<1, 1>::ID());
    compare_mats(Matrix::<2, 2>::ID().inverse().unwrap(), Matrix::<2, 2>::ID());
    compare_mats(Matrix::<3, 3>::ID().inverse().unwrap(), Matrix::<3, 3>::ID());
    compare_mats(Matrix::<4, 4>::ID().inverse().unwrap(), Matrix::<4, 4>::ID());
    compare_mats(Matrix::<5, 5>::ID().inverse().unwrap(), Matrix::<5, 5>::ID());
}

#[test]
fn three_by_three_first() {
    println!("Weird");
    let a = Matrix::<3, 3>(
        [[3.0, 1.0, 5.0].into(),
         [7.0, 5.0, 1.0].into(),
         [2.0, 6.0, 8.0].into()]
    );
    let b: Matrix<3,3> = a.inverse().unwrap();
    for row in b.0 {
        println!("{}, {}, {}", row[0], row[1], row[2]);
    }
    compare_mats(a.inverse().unwrap(), b);
}

#[test]
fn inverse_zero() {
    assert!(Matrix::<1, 1>::ZERO().inverse().is_none());
    assert!(Matrix::<2, 2>::ZERO().inverse().is_none());
    assert!(Matrix::<3, 3>::ZERO().inverse().is_none());
    assert!(Matrix::<4, 4>::ZERO().inverse().is_none());
    assert!(Matrix::<5, 5>::ZERO().inverse().is_none());
}

#[test]
fn two_by_two_normal_inverse() {
    let a = Matrix::<2, 2>(
        [[1.0, 2.0].into(),
         [3.0, 4.0].into()]
    );
    let a_prime = Matrix::<2, 2>(
        [[-2.0, 1.0].into(),
         [1.5, -0.5].into()]
    );
    compare_mats(a.inverse().unwrap(), a_prime);
}

#[test]
fn two_by_two_no_inverse() {
    let a = Matrix::<2, 2>(
        [[1.0, 2.0].into(),
         [2.0, 4.0].into()]
    );
    assert!(a.inverse().is_none())
}

#[test]
fn three_by_three_normal_inverse() {
    let a = Matrix::<3, 3>(
        [[1.0, 2.0, 3.0].into(),
         [1.0, 2.0, 3.0].into(),
         [3.0, 4.0, 2.0].into()]
    );
    let a_prime = Matrix::<3, 3>(
        [[4.0, -6.0, 1.0].into(),
         [-7.0/2.0, 5.0, -0.5].into(),
         [1.0, -1.0, 0.0].into()]
    );
    compare_mats(a.inverse().unwrap(), a_prime);
}

#[test]
fn three_by_three_no_inverse() {
    let a = Matrix::<3, 3>(
        [[1.0, 2.0, 2.0].into(),
         [1.0, 2.0, 2.0].into(),
         [3.0, 4.0, 6.0].into()]
    );

    assert!(a.inverse().is_none())
}

#[test]
fn identity_determinant() {
    assert_eq!(Matrix::<1, 1>::ID().determinant(), 1.0);
    assert_eq!(Matrix::<2, 2>::ID().determinant(), 1.0);
    assert_eq!(Matrix::<3, 3>::ID().determinant(), 1.0);
    assert_eq!(Matrix::<4, 4>::ID().determinant(), 1.0);
    assert_eq!(Matrix::<5, 5>::ID().determinant(), 1.0);
}

#[test]
fn two_by_two_inverse() {
    let a = Matrix::<2, 2>([
        [1.0, 2.0].into(),
        [1.0, -2.0].into()
    ]);
    let inv_a = Matrix::<2, 2>([
        [0.5, 0.5].into(),
        [0.25, -0.25].into()
    ]);
    compare_mats(a.inverse().unwrap(), inv_a);
}

#[test]
fn two_by_two_inverse_again() {
    let a = Matrix::<2, 2>([
        [1.0, -4.0].into(),
        [5.0, -4.0].into()
    ]);
    let inv_a = Matrix::<2, 2>([
        [-0.25, 0.25].into(),
        [-0.3125, 0.0625].into()
    ]);
    compare_mats(a.inverse().unwrap(), inv_a);
}

#[test]
fn three_by_three_inverse() {
    let a = Matrix::<3, 3>([
        [1.0, 2.0, 4.0].into(),
        [1.0, 2.0, -4.0].into(),
        [1.0, -2.0, 0.0].into()
    ]);
    let inv_a = Matrix::<3, 3>([
        [0.25, 0.25, 0.5].into(),
        [0.125, 0.125, -0.25].into(),
        [0.125, -0.125, 0.0].into()
    ]);
    compare_mats(a.inverse().unwrap(), inv_a);
}

#[test]
fn three_by_three_inverse_again() {
    let a = Matrix::<3, 3>([
        [2.0, 1.0, 0.0].into(),
        [3.0, -4.0, -4.0].into(),
        [4.0, 6.0, 0.0].into()
    ]);
    let inv_a = Matrix::<3, 3>([
        [0.75, 0.0, -0.125].into(),
        [-0.5, 0.0, 0.25].into(),
        [1.0625, -0.25, -0.34375].into()
    ]);
    compare_mats(a.inverse().unwrap(), inv_a);
}
