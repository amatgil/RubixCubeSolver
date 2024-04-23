use std::ops::*;

pub struct Matrix<const NF: usize, const NC: usize> (
    [[f64; NC]; NF]
);


impl<const NF: usize, const NC: usize> Add<Matrix<NF, NC>> for Matrix<NF, NC> {
    type Output = Matrix<NF, NC>;
    fn add(self, rhs: Self) -> Self::Output {
        let mut out = Matrix::<NF, NC>([[0.0; NC]; NF]);
        for y in 0..NF {
            for x in 0..NC {
                out.0[y][x] = self.0[y][x] + rhs.0[y][x];
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
         [-3.0, 1.0]]
    );

    let c = Matrix::<2, 2>(
        [[6.0, 8.0],
         [0.0, 5.0]]
    );

    assert_eq!((a + b).0, c.0);
}