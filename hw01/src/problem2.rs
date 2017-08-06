/// Represents a matrix in row-major order
pub type Matrix = Vec<Vec<f32>>;

/// Computes the product of the inputs `mat1` and `mat2`.
pub fn mat_mult(mat1: &Matrix, mat2: &Matrix) -> Matrix {
    let mut ans: Matrix;
    let n = mat1.len();
    let p = mat2[0].len();
    let m = mat2.len();

    // Initialize ans matrix with zeros
    ans = vec![vec![0.0;p]; n];

    for i in 0..n {
        for j in 0..p {
            for k in 0..m {
                ans[i][j] += mat1[i][k] * mat2[k][j];
            }
        }
    }
    ans
}

//
// Problem 2
//

#[test]
fn test_mat_mult_identity() {
    let mut mat1 = vec![vec![0.;3]; 3];
    for i in 0..mat1.len() {
        mat1[i][i] = 1.;
    }
    let mat2 = vec![vec![5.;3]; 3];
    let result = mat_mult(&mat1, &mat2);
    for i in 0..result.len() {
        for j in 0..result[i].len() {
            assert_eq!(result[i][j], mat2[i][j]);
        }
    }
}
