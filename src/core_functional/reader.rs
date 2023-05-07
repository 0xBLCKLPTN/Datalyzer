use std::fs::File;
use std::io::prelude::*;


pub trait Reader<T = String> {
    fn read(filepath: T) -> Self;
    fn file_extension(filepath: T) -> T;
}

struct Csv<T> {
    filepath: T,
    extention: T, 
}

struct OldExcel<T> {
    filepath: T,
    extention: T
}

fn simplify_read<T>(filepath: T) -> String {
    todo!();
}

impl<T> Reader<T> for OldExcel<T> {
    fn read(filepath: T) -> OldExcel<T> {
        todo!();
    }
    fn file_extension(filepath: T) -> T {
        todo!();
    }
}

impl<T> Reader<T> for Csv<T> {
    fn read(filepath: T) -> Csv<T> {
        todo!();
    }

    fn file_extension(filepath: T) -> T {
        todo!();
    }
}