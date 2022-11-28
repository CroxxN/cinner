#![allow(non_camel_case_types)]
#![allow(unused_must_use)]
use std::{
    borrow::BorrowMut,
    fmt::{self, Debug, LowerHex, Octal},
    ops::{Shl, Shr},
    str::FromStr,
};

struct cin;
struct cout;

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

impl<T> Shr<T> for cin
where
    T: BorrowMut<T> + FromStr + Debug,
{
    type Output = cin;
    fn shr(self, mut consumer: T) -> Self::Output {
        let mut temp_string = String::new();
        std::io::stdin()
            .read_line(temp_string.borrow_mut())
            .unwrap();

        let mut things = temp_string.parse::<T>().expect("This shit cannot work");
        consumer = things;
        cin
    }
}
fn main() {
    let mut holder: i32;
    cin >> holder;
    cout << holder;
}
