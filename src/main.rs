#![allow(unused_must_use)]
#![allow(non_camel_case_types)]

use std::{
    fmt,
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
fn main() {
    let mut holder = String::new();
    let mut another: f32 = 0.0;
    cin >> &mut holder >> &mut another;
    cout << holder << another;
}
