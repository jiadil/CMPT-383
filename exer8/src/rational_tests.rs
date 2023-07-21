#[cfg(test)]
use crate::rational::Rational;

#[test]
fn rational_test() {
    let mut r = Rational::new(6, 8);
    assert_eq!(format!("{:?}", r), "Rational { n: 6, d: 8 }");
    r.reduce();
    assert_eq!(format!("{:?}", r), "Rational { n: 3, d: 4 }");
    let four1 = Rational::from(4);
    let four2 = Rational::new(4, 1);
    assert_eq!(four1, four2);

    //test display
    assert_eq!(format!("{}", r), "3/4");
    let mut r1 = Rational::new(6, 8);
    assert_eq!(format!("{}", r1), "6/8");

    // test float
    let mut r2 = Rational::new(3, 4);
    let four3 = f64::from(r2);
    assert_eq!(four3, 0.75);

    let mut r3 = Rational::new(3, 4);
    let four4: f64 = r3.into();
    assert_eq!(four4, 0.75);
}
