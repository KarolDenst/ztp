use nalgebra::DMatrix;

use super::zp::ZpNumber;

// LAP = U
// A = L-1 U P-1
// A-1 = P U-1 L
pub fn factorize_lup(
    a: &DMatrix<ZpNumber>,
) -> Result<(DMatrix<ZpNumber>, DMatrix<ZpNumber>, DMatrix<ZpNumber>), ()> {
    let mut u = a.clone();
    let size = u.nrows();
    let mut l = DMatrix::identity(size, size);
    let mut p = DMatrix::identity(size, size);
    for i in 0..size {
        for j in i..size {
            if u[(i, j)].val != 0 {
                u.swap_columns(i, j);
                p.swap_columns(i, j);
                break;
            }
        }
        if u[(i, i)].val == 0 {
            return Err(());
        }
        let elimination = get_elimination_matrix(&u, i);

        u = &elimination * u;
        l = &elimination * l;
    }
    Ok((l, u, p))
}

fn get_elimination_matrix(matrix: &DMatrix<ZpNumber>, row: usize) -> DMatrix<ZpNumber> {
    let size = matrix.nrows();
    let mut l = DMatrix::identity(size, size);

    for i in row + 1..size {
        l[(i, row)] = -matrix[(i, row)] / matrix[(row, row)];
    }
    l
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
