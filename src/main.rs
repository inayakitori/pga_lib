use pga_lib::basis::*;

fn main() {
    let a = *E1 + *E3;
    let b = *E2;
    println!("Result of (e1 + e3) * e2: {:?}", a * b);
}
