use std::ops::*;

#[derive(Debug, Clone, Copy)] // TODO: Check if Copy is hurting performance
pub struct Matrix<const NF: usize, const NC: usize> (
    [[f64; NC]; NF]
);


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
    const NF: usize,
    const NC: usize,
    const C: usize, // Unused
    const F: usize  // Unused
> Mul<Matrix<F, NC>> for Matrix<NF, C> {
    type Output = Matrix<NF, NC>;
    fn mul(self, mut rhs: Matrix<F, NC>) -> Self::Output {
        todo!("Matrix multiplication unimplemented")
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
        todo!()
    );

    assert_eq!((a * b).0, c.0);
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
        [[ ],
         [ ],
         [ ]]
    );

    assert_eq!((a * b).0, c.0);
}