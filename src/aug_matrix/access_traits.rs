pub trait MatrixRead {
    type Mv;
    fn inner(&self) -> &Vec<Vec<Self::Mv>>;
}
pub trait MatrixRowOps: MatrixRead {

    /// swap rows {row1} and {row2}
    fn swap_rows(&mut self, row1: usize, row2: usize);
    /// M(row) = M(row) * scalar
    fn scale_row(&mut self, row: usize, scalar: &Self::Mv);
    /// M(row1) = M(row1) + (scalar * M(row2))
    fn add_scaled_row(&mut self, row1: usize, row2: usize, scalar: &Self::Mv);
}
pub trait MatrixToSquare: MatrixRead {
    fn crop_matrix(&mut self);
}