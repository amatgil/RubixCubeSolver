use std::ops::*;

#[derive(Debug, Clone, Copy)] // TODO: Check if Copy is hurting performance
pub struct Matrix<const NF: usize, const NC: usize> (
    [MatRow<NC>; NF]
);

#[derive(Debug, Clone, Copy)] // TODO: Check if Copy is hurting performance
pub struct MatRow<const NROWS: usize>([f64; NROWS]);
impl<const NROWS: usize> Mul<f64> for MatRow<NROWS> {
    type Output = Self;
    fn mul(self, lambda: f64) -> Self { MatRow::<NROWS>(self.0.map(|i| i*lambda)) }
}

impl<const N: usize> From<[f64; N]> for MatRow<N> {
    fn from(v: [f64; N]) -> Self { MatRow::<N>(v) }
}

impl<const N: usize> Index<usize> for MatRow<N> {
    type Output = f64;
    fn index(&self, idx: usize) -> &Self::Output { 
        &self.0[idx]
    }
}
impl<const N: usize> IndexMut<usize> for MatRow<N> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output { 
        &mut self.0[idx]
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

impl<const NROWS: usize> Add for MatRow<NROWS> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut out = [0.0; NROWS];
        for i in 0..NROWS { out[i] = self.0[i] + rhs.0[i]}
        MatRow::<NROWS>(out)
    }
}

// Generic impls
impl<const NF: usize, const NC: usize> Matrix<NF, NC> {
    #[allow(non_snake_case)]
    pub const fn ZERO() -> Matrix<NF, NC> {
        Matrix::<NF, NC>([MatRow::<NC>([0.0; NC]); NF])
    }

}

// Square impls
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

    pub fn inverse(&self) -> Self {
        let mut inverse = Matrix::<N, N>::ID();

        //inverse.0[1] = inverse;

        //inverse
        todo!();
    }
}

impl<const NF: usize, const NC: usize> Add<Matrix<NF, NC>> for Matrix<NF, NC> {
    type Output = Matrix<NF, NC>;
    fn add(mut self, rhs: Self) -> Self::Output {
        for y in 0..NF {
            for x in 0..NC { self.0[y][x] += rhs.0[y][x]; }
        }
        self
    }
}

impl<const NF: usize, const NC: usize> Sub<Matrix<NF, NC>> for Matrix<NF, NC> {
    type Output = Matrix<NF, NC>;
    fn sub(mut self, rhs: Self) -> Self::Output {
        for y in 0..NF {
            for x in 0..NC { self.0[y][x] -= rhs.0[y][x] }
        }
        self
    }
}

impl<const NF: usize, const NC: usize> Mul<f64> for Matrix<NF, NC> {
    type Output = Matrix<NF, NC>;
    fn mul(mut self, rhs: f64) -> Self::Output {
        for y in 0..NF {
            for x in 0..NC { self.0[y][x] = self.0[y][x]*rhs }
        }
        self
    }
}

impl<const NF: usize, const NC: usize> Mul<Matrix<NF, NC>> for f64 {
    type Output = Matrix<NF, NC>;
    fn mul(self, mut rhs: Matrix<NF, NC>) -> Self::Output {
        for y in 0..NF {
            for x in 0..NC { rhs.0[y][x] = rhs.0[y][x]*self }
        }
        rhs
    }
}

impl<
    const N: usize,
    const C: usize, // Unused
    const F: usize  // Unused
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