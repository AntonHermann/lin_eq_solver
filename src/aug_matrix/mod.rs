macro_rules! newtype_derive {
    ($N:ty [$T:ty], $M:tt) => {
        impl<$M> Deref for $N {
            type Target = $T;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl <$M> DerefMut for $N {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

pub mod matrix;
pub mod row_echolon;
pub mod access_traits;

#[derive(Debug)]
pub struct MalformedMatrixError;

pub use matrix::Matrix;
pub use row_echolon::{RowEcholon, ReducedRowEcholon};
pub use access_traits::*;