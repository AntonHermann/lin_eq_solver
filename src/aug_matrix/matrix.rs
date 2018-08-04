use std::cmp;
use std::fmt::{self, Display};
use num_traits::Signed;
use num_traits::Zero;
use std::ops::{Deref, DerefMut};
use std::ops::*;
use MalformedMatrixError;
use super::*;

#[derive(Debug, Clone)]
pub struct Matrix<M>(Vec<Vec<M>>);
newtype_derive!(Matrix<M> [Vec<Vec<M>>], M);

impl<M> Matrix<M> {
    pub fn try_from_raw(raw: Vec<Vec<M>>) -> Result<Self, MalformedMatrixError> {
        let valid = {
            let mut iter = raw.iter().map(Vec::len);
            let first_length = match iter.next() {
                Some(len) => len,
                None => return Err(MalformedMatrixError) // empty matrix is considered falsy
            };
            iter.all(|length| length == first_length) // do all rows have same length as first?
        };
        if valid {
            Ok(Matrix(raw))
        } else {
            Err(MalformedMatrixError)
        }
    }
}
impl<M> MatrixRead<M> for Matrix<M> {
    fn inner(&self) -> &Vec<Vec<M>> {
        &self.0
    }
}
impl<M: Clone + Zero + Mul<Output=M>> MatrixRowOps<M> for Matrix<M> {
    fn swap_rows(&mut self, row1: usize, row2: usize) {
        self.0.swap(row1, row2)
    }
    fn scale_row(&mut self, row: usize, scalar: &M) {
        assert!(!scalar.is_zero());

        self.0[row] = self.0[row].iter()
            .cloned()
            .map(|v| { v * scalar.clone() })
            .collect()
    }
    fn add_scaled_row(&mut self, row1: usize, row2: usize, scalar: &M) {
        let r1: Vec<M> = self.0[row1].clone();
        let r2: Vec<M> = self.0[row2].clone();
        self.0[row1] = r1.into_iter()
            .zip(r2)
            .map(|(v1, v2)| {
                v1 + (scalar.clone() * v2)
            })
            .collect()
    }
}

impl<M: Signed + PartialOrd> Matrix<M> {
    pub fn pivot_swap(&mut self, step: usize) {
        debug_assert!(step < self.0.len());
        debug!("pivot_swap called with step {}", step);
        let (pivot_row_index, _) = self.0.iter().enumerate()
            .skip(step) // ignore first {step} rows
            .map(|(i, row)| (i, row.get(step).map(Signed::abs).expect("invalid matrix"))) // extract {step}th value of each row
            .max_by(|(_, acc), (_, cur)| {
                acc.partial_cmp(cur) // try to compare
                    .unwrap_or(cmp::Ordering::Greater) // if cmp failes, keep current max
            }).expect("empty matrix");
        info!("swapping rows {} and {}.", step, pivot_row_index);
        self.0.swap(step, pivot_row_index);
    }
}

// impl<M: NumOps + Clone + Debug + Zero> Matrix<M> {
//     fn _substract_mul_row(&mut self, r1: usize, r2: usize, factor: M) {
//         let new_r1: Vec<M> = {
//             let r1 = self.0.get(r1).expect("invalid matrix");
//             let r2 = self.0.get(r2).expect("invalid matrix");
//             r1.iter().cloned().zip(r2.clone()).map(|(v1, v2)| {
//                 v1 - factor.clone() * v2
//             }).collect()
//         };
//         *self.0.get_mut(r1).expect("invalid matrix") = new_r1;
//     }
// }


impl<M: Display> Display for Matrix<M> {
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