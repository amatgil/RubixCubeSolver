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

    for j in 0..N {
        aug = aug.sort_by_pivot_position();                           // Make sure the pivots are aligned
        if are_equal(aug[j][j], 0.0) { return None; }                 // Row full of zeros, no inverse! 
        aug[j] = (1.0 / aug[j][j]) * aug[j];                          // Set pivot to 1
        for i in (j+1)..N { aug[i] = aug[i] - (aug[i][j]*aug[j]) }    // Set all numbers below first pivot to 0
    }


    // Transform to reduced row echelon form, propagating down
    for j in 0..N {
        for i in 0..N {
            if i == j { continue; } // Don't touch the pivots
            aug[i] = aug[i] - aug[i][j]*aug[j];
        }
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
