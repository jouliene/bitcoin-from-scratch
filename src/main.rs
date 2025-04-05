use bitcoin_from_scratch::finite_fields::FieldElement;
use num_bigint::BigInt;

fn main() {
    let fe = FieldElement::new(BigInt::from(255)).unwrap();
    println!("{}", fe);
}
