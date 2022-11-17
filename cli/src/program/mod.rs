use crate::prelude::*;

pub trait Program {
    type Output;
    type Error: std::error::Error;

    fn run(&self) -> Result<Self::Output, Self::Error>;

}