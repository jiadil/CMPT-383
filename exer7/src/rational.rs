fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rational {
    n: i64,
    d: i64,
}

impl Rational {
    pub fn new(n: i64, d: i64) -> Rational {
        Rational {
            n: n,
            d: d,
        }
    }
    
    pub fn reduce(&mut self) {
       let g = gcd(self.n, self.d);
        self.n /= g;
        self.d /= g;

        if self.d < 0 {
            self.n *= -1;
            self.d *= -1;
        }
    }
}

impl From<i64> for Rational {
    fn from(n: i64) -> Rational {
        Rational {
            n: n,
            d: 1,
        }
    }
}
