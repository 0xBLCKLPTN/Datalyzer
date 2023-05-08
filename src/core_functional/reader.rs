use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::prelude::*;

pub struct Csv;
pub struct OldExcel;
pub struct NewExcel;


pub trait Reader {
    fn new() -> Self;
    
    fn simply_reader(&self, filepath: String) {
        todo!();
    }

    fn read(&self, filepath: String);
}


impl Reader for OldExcel {
    fn new() -> Self {
        Self 
    }
    fn read(&self, filepath: String) {
        todo!();
    }
}

impl Reader for Csv {
    fn new() -> Self {
        Self
    }

    fn read(&self, filepath: String) {
        todo!();
    }
}