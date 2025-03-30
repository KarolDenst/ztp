use nalgebra::DMatrix;

use super::zp::ZpNumber;

// PA = LU
// A = P-1LU
// A-1 = U-1L-1P
#[allow(clippy::type_complexity)]
pub fn factorize_lup(
    a: &DMatrix<ZpNumber>,
) -> Result<(DMatrix<ZpNumber>, DMatrix<ZpNumber>, DMatrix<ZpNumber>), ()> {
    let n = a.nrows();

    let zero = ZpNumber::zero();

    let mut u = a.clone(); // U starts as a copy of A
    let mut l = DMatrix::<ZpNumber>::identity(n, n);
    let mut p = DMatrix::<ZpNumber>::identity(n, n);

    for k in 0..n {
        let mut pivot_row = k;
        let mut found_pivot = false;
        for i in k..n {
            if u[(i, k)] != zero {
                pivot_row = i;
                found_pivot = true;
                break;
            }
        }

        if !found_pivot && u[(k, k)] == zero {
            return Err(());
        }

        if pivot_row != k {
            u.swap_rows(k, pivot_row);
            p.swap_rows(k, pivot_row);

            for j in 0..k {
                let temp = l[(k, j)];
                l[(k, j)] = l[(pivot_row, j)];
                l[(pivot_row, j)] = temp;
            }
        }

        let pivot_val = u[(k, k)];

        if pivot_val == zero {
            return Err(());
        }

        for i in (k + 1)..n {
            let factor = u[(i, k)] / pivot_val;

            l[(i, k)] = factor;

            u[(i, k)] = zero;
            for j in (k + 1)..n {
                let term = factor * u[(k, j)];
                u[(i, j)] -= term;
            }
        }
    }

    Ok((l, u, p))
}

pub fn inverse_upper_triangular(matrix: &DMatrix<ZpNumber>) -> DMatrix<ZpNumber> {
    let n = matrix.nrows();
    let mut inv = DMatrix::zeros(n, n);

    for i in 0..n {
        inv[(i, i)] = matrix[(i, i)].inv();
    }

    for j in 0..n {
        for i in (0..j).rev() {
            let mut sum = ZpNumber::zero();
            for k in (i + 1)..=j {
                sum += matrix[(i, k)] * inv[(k, j)];
            }
            inv[(i, j)] = -sum * inv[(i, i)];
        }
    }

    inv
}
pub fn inverse_lower_triangular(matrix: &DMatrix<ZpNumber>) -> DMatrix<ZpNumber> {
    let n = matrix.nrows();
    let mut inv = DMatrix::zeros(n, n);

    for i in 0..n {
        inv[(i, i)] = matrix[(i, i)].inv();
    }

    for i in 1..n {
        for j in 0..i {
            let mut sum = ZpNumber::zero();
            for k in j..i {
                sum += matrix[(i, k)] * inv[(k, j)];
            }

            inv[(i, j)] = -sum * inv[(i, i)];
        }
    }

    inv
}
