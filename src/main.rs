#![allow(dead_code)]

extern crate num_traits;
#[macro_use]
extern crate log;
extern crate simple_logger;

mod aug_matrix;

use std::io::{self, BufRead};
use std::str::FromStr;

use aug_matrix::*;

enum Error<M: FromStr> {
    // IOError(io::Error),
    ParseError(M::Err),
    MalformedMatrixError(MalformedMatrixError)
}

fn main() {
    simple_logger::init().unwrap();

    // println!("Enter the augmented matrix. Enter an empty line to finish.");
    // println!("Each row has the following format: 1 2 3 5");
    // println!("which corresponds to this equation: 1x + 2y + 3z = 5");
    // let mut matrix: Matrix<f32> = loop {
    //     if let Ok(m) = read_matrix() {
    //         break m
    //     }
    //     println!("This wasn't a valid input! Try again:")
    // };
    // let mut matrix: Matrix<f32> = Matrix::from_raw(vec![
    //     vec![  2.0,  1.0, -1.0, /* = */   8.0 ],
    //     vec![ -3.0, -1.0,  2.0, /* = */ -11.0 ],
    //     vec![ -2.0,  1.0,  2.0, /* = */  -3.0 ],
    // ]).unwrap()
    let matrix: Matrix<f32> = Matrix::try_from_raw(vec![
        vec![  1.0,  3.0,  1.0, /* = */   9.0 ],
        vec![  1.0,  1.0, -1.0, /* = */   1.0 ],
        vec![  3.0, 11.0,  5.0, /* = */  35.0 ],
    ]).unwrap();
    println!("{}", matrix);

    let row_echolon = RowEcholon::from(matrix);
    println!("{}", row_echolon);

    let reduced_row_echolon = ReducedRowEcholon::from(row_echolon);
    println!("{}", reduced_row_echolon);

}

fn read_matrix<M: FromStr>() -> Result<Matrix<M>, Error<M>> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let m: Result<Vec<Vec<M>>, M::Err> = handle
        .lines() // Iterator<Result<String>>
        .filter_map(Result::ok) // Iterator<String>
        .take_while(|line| { !line.is_empty() }) // take until empty line
        .map(|line: String| -> Result<Vec<M>, M::Err> { 
            line
                .split_whitespace()
                .map(str::parse)
                .collect()
        })
        .collect();
    m
        .map_err(|err| Error::ParseError(err)) // Result<_, Parse???Error> -> Result<_, Error>
        .and_then(|m| Matrix::try_from_raw(m).map_err(|err| Error::MalformedMatrixError(err)))
}