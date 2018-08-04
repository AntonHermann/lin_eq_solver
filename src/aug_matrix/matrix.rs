use std::cmp;
use std::fmt::{self, Display, Debug};
use num_traits::Zero;
use num_traits::ops::inv::Inv;
use std::ops::*;
use super::*;
use super::access_traits::*;
use super::row_echolon::*;

#[derive(Debug, Clone)]
pub struct Matrix<Mv>(Vec<Vec<Mv>>);

impl<Mv> Matrix<Mv> {
    pub fn try_from_raw(raw: Vec<Vec<Mv>>) -> Option<Self> {
        let valid = {
            let mut iter = raw.iter().map(Vec::len);
            let first_length = match iter.next() {
                Some(len) => len,
                None => return None // empty matrix is considered falsy
            };
            iter.all(|length| length == first_length) // do all rows have same length as first?
        };
        if valid {
            Some(Matrix(raw))
        } else {
            None
        }
    }
}
impl<Mv> MatrixRead for Matrix<Mv> {
    type Mv = Mv;
    fn inner(&self) -> &Vec<Vec<Mv>> {
        &self.0
    }
}
impl<Mv: Clone + Zero + Mul<Output=Mv>> MatrixRowOps for Matrix<Mv> {
    fn swap_rows(&mut self, row1: usize, row2: usize) {
        self.0.swap(row1, row2)
    }
    fn scale_row(&mut self, row: usize, scalar: &Mv) {
        assert!(!scalar.is_zero());

        self.0[row] = self.0[row].iter()
            .cloned()
            .map(|v| { v * scalar.clone() })
            .collect()
    }
    fn add_scaled_row(&mut self, row1: usize, row2: usize, scalar: &Mv) {
        let r1: Vec<Mv> = self.0[row1].clone();
        let r2: Vec<Mv> = self.0[row2].clone();
        self.0[row1] = r1.into_iter()
            .zip(r2)
            .map(|(v1, v2)| {
                v1 + (scalar.clone() * v2)
            })
            .collect()
    }
}
impl<Mv> MatrixToSquare for Matrix<Mv> {
    fn crop_matrix(&mut self) {
        let num_cols: usize = self.0[0].len() - 1;
        self.0.truncate(num_cols)
    }
}
impl<Mv> Matrix<Mv>
    where Mv: Clone + Zero + Debug +
        Div<Output=Mv> +
        Inv<Output=Mv> +
        Neg<Output=Mv> +
        Mul<Output=Mv>
{
    pub fn try_solve(&self) -> Option<Vec<Mv>> {
        let row_echolon = RowEcholon::from(self.clone());
        let square_matrix = SquareMatrix::try_from_row_echolon(row_echolon)?;
        let reduced_row_echolon = ReducedRowEcholon::from(square_matrix);
        let solution = reduced_row_echolon.solve();
        Some(solution)
    }
}

// impl<Mv: Signed + PartialOrd> Matrix<Mv> {
//     pub fn pivot_swap(&mut self, step: usize) {
//         debug_assert!(step < self.0.len());
//         debug!("pivot_swap called with step {}", step);
//         let (pivot_row_index, _) = self.0.iter().enumerate()
//             .skip(step) // ignore first {step} rows
//             .map(|(i, row)| (i, row.get(step).map(Signed::abs).expect("invalid matrix"))) // extract {step}th value of each row
//             .max_by(|(_, acc), (_, cur)| {
//                 acc.partial_cmp(cur) // try to compare
//                     .unwrap_or(cmp::Ordering::Greater) // if cmp failes, keep current max
//             }).expect("empty matrix");
//         info!("swapping rows {} and {}.", step, pivot_row_index);
//         self.0.swap(step, pivot_row_index);
//     }
// }

impl<Mv: Display> Display for Matrix<Mv> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let row_count = match self.0.first() {
            Some(first_row) => first_row.len(),
            None => return write!(f, "empty matrix")
        };

        let col_widths: &Vec<_> = &self.0.iter().fold(
            vec![0; row_count], // start fold with column width of 0
            |max_widths, row| {
                max_widths.into_iter()
                    // zip current max-widths with widths of current row
                    .zip(row.into_iter().map(|val| val.to_string().len()))
                    // and take the maximum of both as new maximum width for each column
                    .map(|(max_width, cur_width)| cmp::max(max_width, cur_width)) 
                    .collect()
            }
        );

        let last_index = self.0.len();
        for row in &self.0 {
            for (i, (el, width)) in row.into_iter().zip(col_widths).enumerate() {
                if i == last_index {
                    write!(f, "| ")?;
                }
                write!(f, "{:width$} ", el, width = width)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}