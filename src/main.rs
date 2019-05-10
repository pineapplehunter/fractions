use fractions::Fraction;

fn main() {
    let f = Fraction::new(-1, -3);

    let f2 = f * -4;
    let f3 = f2 * 20;

    println!("{:?}", f3);
}
