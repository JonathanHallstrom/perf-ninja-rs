#[cfg(test)]
mod tests;

pub type MatrixOfDoubles = Vec<Vec<f64>>;

pub fn init_matrix(matrix: &mut MatrixOfDoubles) {
    let size = matrix.len();

    for i in 0..size {
        for j in 0..size {
            matrix[i][j] = ((i + j) % 1024) as f64
        }
    }
}

pub fn solution(matrix_in: &MatrixOfDoubles, matrix_out: &mut MatrixOfDoubles) {
    let size = matrix_in.len();

    // around 8MB chunks, should fit in L3 cache on most machines
    const N: usize = 1024;

    let mut starti = 0;
    let mut endi = 0;
    while starti < size {
        let mut endj = 0;
        let mut startj = 0;
        while startj < size {
            for i in starti..endi {
                for j in startj..endj {
                    matrix_out[i][j] = matrix_in[j][i];
                }
            }
            startj = endj;
            endj = size.min(endj + N);
        }
        starti = endi;
        endi = size.min(endi + N);
    }
}
