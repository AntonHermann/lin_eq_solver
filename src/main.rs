#![allow(dead_code)]

mod matrix;

use std::io::{self, BufRead};
use std::str::FromStr;

use matrix::{Matrix, MalformedMatrixError};

enum Error<M: FromStr> {
    // IOError(io::Error),
    ParseError(M::Err),
    MalformedMatrixError(MalformedMatrixError)
}

fn main() {
    let matrix: Matrix<f32> = loop {
        println!("Enter the matrix. Enter an empty line to finish");
        if let Ok(m) = read_matrix() {
            break m
        }
        println!("This wasn't a valid input!")
    };
    println!("{}", matrix);
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
        .and_then(|m| Matrix::from_raw(m).map_err(|err| Error::MalformedMatrixError(err)))
}