use crate::*;

macro_rules! impl_inverse_for_size {
    ($($N:expr, $N_TIMES_TWO:expr);*$(;)?) => {
        $(
            impl Matrix<$N, $N> {
                pub fn inverse(&self) -> Option<Matrix<$N, $N>> {
                    assert!(2 * $N == $N_TIMES_TWO);
                    inverse_helper::<$N, $N_TIMES_TWO>(self)
                }
            }
        )*
    };
}

impl_inverse_for_size! {
    1, 2;
    2, 4;
    3, 6;
    4, 8;
    5, 10;
    6, 12;
}

fn inverse_helper<const N: usize, const N_TIMES_TWO: usize>(m: &Matrix<N, N>) -> Option<Matrix<N, N>> {
    assert!(N_TIMES_TWO == 2*N);  // just in case, even though this function is not public

    // Applying these steps to our original matrix will reduce it to the identity, meaning the right will now be self^(-1)

    let mut aug = Matrix::<N, N_TIMES_TWO>::ZERO();                  // Prepare augmented matrix
    for i in 0..N {
        for j in 0..N {
            aug[i][j] = m[i][j];
            aug[i][j + N] = Matrix::<N, N>::ID()[i][j];
        }
    }

    aug = aug.as_upper_triangle();

    // Check if answer is valid
    for i in 0..N {
        if !are_equal(aug[i][i], 1.0) { return None; }
    }


    // Extract answer
    let mut inverse = Matrix::<N, N>::ZERO();
    for i in 0..N {
        for j in 0..N {
            inverse[i][j] = aug[i][j + N];
        }
    }


    // inverse is now our solution
    Some(inverse)
}
