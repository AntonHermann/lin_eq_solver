use std::{fmt, cmp};
use std::slice::Iter;

pub struct MalformedMatrixError;

#[derive(Debug)]
pub struct Matrix<M>(Vec<Vec<M>>);

impl<M> Matrix<M> {
    pub fn from_raw(raw: Vec<Vec<M>>) -> Result<Self, MalformedMatrixError> {
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
    pub fn rows(&self) -> Iter<Vec<M>> {
        self.0.iter()
    }
}

impl<M: fmt::Display> fmt::Display for Matrix<M> {
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

        for row in &self.0 {
            // write!(f, "| ")?;
            for (el, width) in row.into_iter().zip(col_widths) {
                write!(f, "{:width$} ", el, width = width)?;
                // write!(f, "{:width$} | ", el, width = width)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}