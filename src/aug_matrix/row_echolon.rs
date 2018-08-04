use std::fmt::{self, Debug, Display};
use num_traits::identities::Zero;
use num_traits::ops::inv::Inv;
use std::ops::*;
use super::*;
use super::access_traits::*;

#[derive(Debug, Clone)]
pub struct RowEcholon<M>(M);

#[derive(Debug, Clone)]
pub struct ReducedRowEcholon<M>(M);

#[derive(Debug, Clone)]
pub struct SquareMatrix<M>(M);

/* ---------------------------- */
/* ------- ROW ECHELON  ------- */
/* ---------------------------- */

// Matrix => RowEcholon
impl<M> From<M> for RowEcholon<M>
    where M: MatrixRowOps + Debug,
          M::Mv: Clone + Zero + Debug +
            Div<Output=M::Mv> +
            Inv<Output=M::Mv> +
            Neg<Output=M::Mv>
{
    fn from(mut m: M) -> Self {
        // -1 because last column is extra vector and not part of the matrix
        let col_count: usize = m.inner()[0].len() - 1;
        let row_count: usize = m.inner().len();
        for i in 0..col_count {
            // 1. normalize row
            let lead_coeff: M::Mv = m.inner()[i][i].clone();
            if lead_coeff.is_zero() {
                // TODO: handle lead_coeff == 0 case
            } else {
                m.scale_row(i, &lead_coeff.inv())
            }

            // substract following rows
            for row in (i+1)..row_count {
                let neg_row_lead_coeff: M::Mv = -m.inner()[row][i].clone();
                m.add_scaled_row(row, i, &neg_row_lead_coeff);
            }
        }
        RowEcholon(m)
    }
}
impl<M> RowEcholon<M>
    where M: MatrixRowOps,
          M::Mv: Zero
{
    pub fn is_solvable(&self) -> bool {
        let non_zero_rows_count: usize = self.0.inner().iter()
            .filter(|row| {
                !row.iter().all(|v| v.is_zero())
            })
            .count();
        let col_count: usize = self.0.inner()[0].len() - 1;
        // matrix is only solvable if there are exactly as many non-zero
        // rows as there are columns
        non_zero_rows_count == col_count
    }
}
impl<M: Display> Display for RowEcholon<M> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let matrix: &M = &self.0;
        matrix.fmt(f)
    }
}


/* ---------------------------- */
/* ------ SQUARE MATRIX  ------ */
/* ---------------------------- */

// RowEcholon => SquareMatrix
impl<M> SquareMatrix<M>
    where M: MatrixRowOps + MatrixToSquare,
          M::Mv: Clone + Zero
{
    pub fn try_from_row_echolon(mut m: RowEcholon<M>) -> Option<SquareMatrix<M>>
    {
        let col_count: usize = m.0.inner()[0].len() - 1;

        let is_valid_square_matrix: bool = m.0.inner()
            .iter()
            .cloned()
            .take(col_count)
            .all(|row| {
                !row.iter().all(|v| v.is_zero())
            });

        if is_valid_square_matrix {
            m.0.crop_matrix();
            Some(SquareMatrix(m.0))
        } else {
            None
        }
    }
}
impl<M: Display> Display for SquareMatrix<M> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let matrix: &M = &self.0;
        matrix.fmt(f)
    }
}

/* ---------------------------- */
/* --- REDUCED ROW ECHELON  --- */
/* ---------------------------- */

// SquareMatrix => ReducedRowEcholon
impl<M> From<SquareMatrix<M>> for ReducedRowEcholon<M>
    where M: MatrixRowOps,
          M::Mv: Clone + Neg<Output=M::Mv> + Debug + Zero + Mul<Output=M::Mv>,
{
    fn from(mut re: SquareMatrix<M>) -> ReducedRowEcholon<M> {
        // -1 because last row is vector of augmented matrix
        let num_cols: usize = re.0.inner()[0].len() - 1;
        // num_cols..1, because first row has no preceding rows => can be skipped
        for col in (0..num_cols).rev() {
            for row in 0..col {
                let neg_coeff: M::Mv = -re.0.inner()[row][col].clone();
                re.0.add_scaled_row(row, col, &neg_coeff);
            }
        }
        ReducedRowEcholon(re.0)
    }
}
impl<M> ReducedRowEcholon<M>
    where M: MatrixRowOps,
          M::Mv: Clone
{
    pub fn solve(&self) -> Vec<M::Mv> {
        let last_col: usize = self.0.inner()[0].len() - 1;
        self.0.inner().iter().map(|row| {
            row[last_col].clone()
        }).collect()
    }
}
impl<M: Display> Display for ReducedRowEcholon<M> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let matrix: &M = &self.0;
        matrix.fmt(f)
    }
}