use nalgebra::DMatrix;

use super::zp::Zp;

// LAP = U
// A = L-1 U P-1
// A-1 = P U-1 L
pub fn factorize_lup(
    a: &DMatrix<u32>,
    zp: &Zp,
) -> Result<(DMatrix<u32>, DMatrix<u32>, DMatrix<u32>), ()> {
    let mut u = a.clone();
    let size = u.nrows();
    let mut l = DMatrix::identity(size, size);
    let mut p = DMatrix::identity(size, size);
    for i in 0..size {
        for j in i..size {
            if u[(i, j)] != 0 {
                u.swap_columns(i, j);
                p.swap_columns(i, j);
                break;
            }
        }
        if u[(i, i)] == 0 {
            return Err(());
        }
        let elimination = get_elimination_matrix(&u, i, zp);
        println!("{}", l);
        u = &elimination * u;
        u.apply(|x| zp.fix(x));
        l = &elimination * l;
        l.apply(|x| zp.fix(x));
    }
    Ok((l, u, p))
}

fn get_elimination_matrix(matrix: &DMatrix<u32>, row: usize, zp: &Zp) -> DMatrix<u32> {
    let size = matrix.nrows();
    let mut l = DMatrix::identity(size, size);

    for i in row + 1..size {
        // l[(i, row)] = -[matrix[(i, row)] / matrix[(row, row)];
        l[(i, row)] = zp.div(zp.sub(0, matrix[(i, row)]), matrix[(row, row)]);
    }
    l
}

pub fn inverse_upper_triangular(matrix: &DMatrix<u32>, zp: &Zp) -> DMatrix<u32> {
    let n = matrix.nrows();
    let mut inv = DMatrix::zeros(n, n);

    for i in 0..n {
        inv[(i, i)] = zp.inv(matrix[(i, i)]);
    }

    for j in 0..n {
        for i in (0..j).rev() {
            let mut sum = 0;
            for k in (i + 1)..=j {
                sum = zp.add(sum, zp.mul(matrix[(i, k)], inv[(k, j)]));
            }
            inv[(i, j)] = zp.neg(zp.mul(sum, inv[(i, i)]));
        }
    }

    inv
}
