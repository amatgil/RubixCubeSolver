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

fn inverse_helper<const N: usize, const N_TIMES_TWO: usize>(m: &Matrix<N, N>) -> Option<Matrix<N, N>> {
    assert!(N_TIMES_TWO == 2*N);  // just in case, even though this function is not public
    // Applying these steps to our original matrix will reduce it to the identity, meaning 'inverse' will now be self^(-1)
    let identity = Matrix::<N, N>::ID(); 

    let mut aug = Matrix::<N, N_TIMES_TWO>::ZERO();
    for i in 0..N {
        for j in 0..N {
            aug[i][j] = m[i][j];
            aug[i][j + N] = identity[i][j];
        }
    }

    // Transform to row echelon form
    //   Align by pivot
    aug = aug.sort_by_pivot_position();

    // If the first pivot isn't on the first column or there's a row full of zeros, there must be a column of zeros. Rank isn't full, no inverse available
    if are_equal(aug[0][0], 0.0) || aug[N - 1].0.iter().take(N).all(|&e| are_equal(e, 0.0)) {
        return dbg!(None);
    } 

    for j in 0..N {
        println!("Pre\n{aug}");
        aug = aug.sort_by_pivot_position();
        println!("Post\n{aug}");
        dbg!(j);
        if are_equal(aug[j][j], 0.0) { return dbg!(None); }  // Row full of zeros, no inverse! (aug sorted-by-pivot rows)
        aug[j] = (1.0 / aug[j][j]) * aug[j]; // Set pivot to 1

        for i in (j+1)..N { // Set all numbers below first pivot to 1
            println!("{aug}");
            aug[i] = aug[i] - (aug[i][j]*aug[j]);
        }
    }

    println!("Reducing time:");
    println!("{aug}");

    // Transform to reduced row echelon form (applying both to the original and ID to get our inverse)
    for j in (0..N).into_iter().rev() {
        for i in (0..N).into_iter().rev() {
            if i == j { continue; }
            aug[i] = aug[i] - aug[i][j]*aug[j];
        }
        println!("{aug}");
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

impl_inverse_for_size! {
    0, 0;
    1, 2;
    2, 4;
    3, 6;
    4, 8;
    5, 10;
    6, 12;
}
