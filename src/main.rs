use bitcoin_from_scratch::finite_fields::FieldElement;
use bitcoin_from_scratch::point::Point;
use num_bigint::BigInt;

fn main() {
    let fe = FieldElement::new(BigInt::from(255)).unwrap();
    let p = Point::new(None, None).unwrap();
    println!("{}", fe);
    println!("{}", p);
    let x = FieldElement::new(BigInt::parse_bytes(b"79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",16).unwrap()).unwrap();
    let y = FieldElement::new(BigInt::parse_bytes(b"483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",16).unwrap()).unwrap();
    let p = Point::new(Some(x), Some(y)).unwrap();
    println!("{}", p);
}
