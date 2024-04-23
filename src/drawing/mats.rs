use std::ops::*;

#[derive(Debug, Clone, Copy)] // TODO: Check if Copy is hurting performance
pub struct Matrix<const NF: usize, const NC: usize> (
    [[f64; NC]; NF]
);

// Generic impls
impl<const NF: usize, const NC: usize> Matrix<NF, NC> {
    #[allow(non_snake_case)]
    pub const fn ZERO() -> Matrix<NF, NC> {
        Matrix::<NF, NC>([[0.0; NC]; NF])
    }

}

// Square impls
impl<const N: usize> Matrix<N, N> {
    #[allow(non_snake_case)]
    pub const fn ID() -> Matrix<N, N> {
        let mut out = Matrix::<N, N>([[0.0; N]; N]);
        let mut i = 0;
        while i < N {
            out.0[i][i] = 1.0;
            i += 1;
        }
        out
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
            for x in 0..C {
                let mut val = 0.0;
                for i in 0..N {
                    val += self.0[y][i]*rhs.0[i][x];
                }
                out.0[y][x] = val;
            }
        }
        out
    }
}


#[test]
fn mat_addition() {
    let a = Matrix::<2, 2>(
        [[1.0, 2.0],
         [3.0, 4.0]]
    );
    let b = Matrix::<2, 2>(
        [[5.0, 6.0],
         [-3.0, 1.5]]
    );

    let c = Matrix::<2, 2>(
        [[6.0, 8.0],
         [0.0, 5.5]]
    );

    assert_eq!((a + b).0, c.0);
}

#[test]
fn mat_subtraction() {
    let a = Matrix::<2, 2>(
        [[1.0, 2.0],
         [3.0, 4.0]]
    );
    let b = Matrix::<2, 2>(
        [[5.0, 6.0],
         [-3.0, 1.5]]
    );

    let c = Matrix::<2, 2>(
        [[-4.0, -4.0],
         [6.0, 2.5]]
    );

    assert_eq!((a - b).0, c.0);
}


#[test]
fn mat_mult_by_scalar() {
    let a = Matrix::<2, 2>(
        [[1.0, 2.0],
         [0.0, 4.0]]
    );
    let k = -7.4;

    let c = Matrix::<2, 2>(
        [[-7.4, -14.8],
         [-0.0, -29.6]]
    );

    assert_eq!((a * k).0, c.0); // I hate float equality lmao
    assert_eq!((k * a).0, c.0); // I hate float equality lmao
}

#[test]
fn mat_mult_square() {
    let a = Matrix::<2, 2>(
        [[1.0, 2.0],
         [0.0, 4.0]]
    );

    let b = Matrix::<2, 2>(
        [[5.0, 6.0],
         [-3.0, 1.5]]
    );

    let c = Matrix::<2, 2>(
        [[-1.0, 9.0],
        [-12.0, 6.0]]
    ); 

    let d = Matrix::<2, 2>(
        [[5.0, 34.0],
        [-3.0, 0.0]]
    );


    assert_eq!((a * b).0, c.0);
    assert_eq!((b * a).0, d.0);

}

#[test]
fn mat_mult_rectangle() {
    let a = Matrix::<3, 2>(
        [[5.0,  6.0],
         [-1.0, 1.0],
         [-3.0, 1.5]]
    );
    
    let b = Matrix::<2, 3>(
        [[1.0, 2.0, 3.0],
         [0.0, 4.0, 5.0]]
    );


    let c = Matrix::<3, 3>(
        [[5.0, 234.0, 295.0],
         [-1.0, 2.0, 2.0 ],
         [-3.0, 0.0, -1.5]],
    );

    assert_eq!((a * b).0, c.0);
}


#[test]
fn mat_mult_rectangle_other() {
    let a = Matrix::<3, 2>(
        [[5.0,  6.0],
         [-1.0, 1.0],
         [-3.0, 1.5]]
    );
    
    let b = Matrix::<2, 3>(
        [[1.0, 2.0, 3.0],
         [0.0, 4.0, 5.0]]
    );


    let c = Matrix::<2, 2>(
        [[-6.0, 62.5],
         [-19.0, 11.5 ]]
    );

    assert_eq!((b * a).0, c.0);
}

#[test]
fn zeros() {
    let one = Matrix::<1, 1>::ZERO();
    let two = Matrix::<2, 2>::ZERO();
    let three = Matrix::<3, 3>::ZERO();

    assert_eq!(one.0, Matrix::<1, 1>([[0.0; 1]; 1]).0);
    assert_eq!(two.0, Matrix::<2, 2>([[0.0; 2]; 2]).0);
    assert_eq!(three.0, Matrix::<3, 3>([[0.0; 3]; 3]).0);

}
#[test]
fn matrix_identity() {
    let id = Matrix::<3, 3>::ID();
    let correct_id = Matrix::<3, 3>(
        [[1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0]]
    );
    assert_eq!(id.0, correct_id.0);
}