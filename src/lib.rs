#![allow(unused_must_use)]
#![allow(non_camel_case_types)]

use std::{
    fmt,
    ops::{Shl, Shr},
    str::FromStr,
};

pub struct cin;
pub struct cout;

impl<T> Shl<T> for cout
where
    T: fmt::Display,
{
    type Output = cout;
    fn shl(self, printer: T) -> Self::Output {
        print!("{printer}");
        cout
    }
}

impl<T> Shr<&mut T> for cin
where
    T: FromStr + Default,
{
    type Output = cin;
    fn shr(self, consumer: &mut T) -> Self::Output {
        let mut temp_string = String::new();
        std::io::stdin().read_line(&mut temp_string).unwrap();
        *consumer = temp_string.trim().parse::<T>().unwrap_or_default();
        cin
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cout_test() {
        let holder = 12;
        let string_holder = String::from("Working");
        cout << "This is working" << "\n" << holder << "\n" << string_holder;
    }
    #[test]
    fn cin_test() {
        let mut holder = 0;
        cin >> &mut holder;
    }
}
