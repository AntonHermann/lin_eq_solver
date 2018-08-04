use std::fmt::{self, Display};
use num_traits::identities::Zero;
use std::ops::{Deref, DerefMut};
use std::ops::{Div, Mul, Sub};
use super::*;

#[derive(Debug, Clone)]
pub struct RowEcholon<M>(Matrix<M>);
newtype_derive!(RowEcholon<M> [Matrix<M>], M);

#[derive(Debug, Clone)]
pub struct ReducedRowEcholon<M>(RowEcholon<M>);
newtype_derive!(ReducedRowEcholon<M> [RowEcholon<M>], M);

fn _normalize_row<M: Zero + Clone + Div<Output=M>>(matrix: &mut Matrix<M>, row: usize) {
    let new_row: Vec<M> = {
        let r: &Vec<M> = (*matrix).get(row).expect("invalid matrix");
        let factor: &M = r.get(row).expect("invalid matrix");

        if factor.is_zero() {
            r.clone()
        } else {
            r.iter().cloned().map(|v| {
                v / factor.clone()
            }).collect()
        }
    };
    *((*matrix).get_mut(row).expect("invalid matrix")) = new_row;
}
fn _substract_following_rows<M: Clone + Mul<Output=M> + Sub<Output=M>>(matrix: &mut Matrix<M>, row: usize) {
    let mut row_iter = (*matrix).iter_mut().skip(row);
    let r: &Vec<M> = row_iter.next().expect("invalid matrix");
    for mut curr in row_iter {
        let new_row: Vec<M> = {
            let factor = curr.get(row).expect("invalid matrix");
            let new_row: Vec<M> = curr.iter().cloned()
                .zip(r.clone())
                .map(|(v, vr)| { vr * factor.clone() - v})
                .collect();
            new_row
        };
        *curr = new_row;
    }
}

// Matrix => RowEcholon
// impl<M: NumOps + NumAssignOps + Clone + fmt::Debug + Zero> From<Matrix<M>> for RowEcholon<M> {
impl<M> From<Matrix<M>> for RowEcholon<M>
    where M: Clone + Mul<Output=M> + Sub<Output=M> + Div<Output=M> + Zero
{
    fn from(mut m: Matrix<M>) -> Self {
        // -1 because last column is extra vector and not part of the matrix
        let col_count: usize = (*m).get(0).expect("invalid matrix").len() - 1;
        for i in 0..col_count {
            _normalize_row(&mut m, i);
            _substract_following_rows(&mut m, i);
        }
        RowEcholon(m)
    }
}

// RowEcholon => ReducedRowEcholon
impl<M: Clone + Mul<Output=M> + Sub<Output=M>> From<RowEcholon<M>> for ReducedRowEcholon<M> {
    fn from(mut re: RowEcholon<M>) -> Self {
        for i in (0..re.len()).rev() {
            re._substract_preceding_rows(i);
        }
        ReducedRowEcholon(re)
    }
}

impl<M: Clone + Mul<Output=M> + Sub<Output=M>> RowEcholon<M> {
    fn _substract_preceding_rows(&mut self, row: usize) {
        let r: Vec<M> = self.0.get(row).expect("invalid matrix").clone();
        for mut curr in self.0.iter_mut().take(row) {
            let new_row: Vec<M> = {
                let factor = curr.get(row).expect("invalid matrix");

                curr.iter().cloned()
                    .zip(r.clone())
                    .map(|(v, vr)| { v - vr * factor.clone() })
                    .collect()
            };
            *curr = new_row;
        }
    }
}

impl<M: Display> Display for RowEcholon<M> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let matrix: &Matrix<M> = &**self;
        matrix.fmt(f)
    }
}
impl<M: Display> Display for ReducedRowEcholon<M> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let matrix: &RowEcholon<M> = &**self;
        matrix.fmt(f)
    }
}