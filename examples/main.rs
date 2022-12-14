// As the cinner crate contains cin, cout and endl only, we can include it all
use cinner::*;

fn main() {
    let mut i = 0;
    let _ = cin >> &mut i;
    let _ = cout << "The value of i is: " << i << endl;
}
