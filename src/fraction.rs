use std::ops::Mul;

#[derive(Debug)]
pub struct Fraction {
    sign: bool,
    numerator: u64,
    denominator: u64,
}

impl<I> Mul<I> for Fraction
where I: IntoInteger {
    type Output = Fraction;

    fn mul(self, rhs: I) -> Self::Output {
        let n = rhs.into_int();
        Self {
            sign: self.sign ^ n.sign,
            numerator: self.numerator * n.num,
            denominator: self.denominator,
        }
    }
}

impl <I: IntoInteger> Mul<Fraction> for I{
    type Output = Fraction;

    fn mul(self, rhs: Fraction) -> Self::Output {
        rhs * self
    }
}

impl Fraction {
    pub fn new<N, D>(numerator: N, denominator: D) -> Self
    where
        N: IntoInteger,
        D: IntoInteger,
    {
        let n = numerator.into_int();

        let d = denominator.into_int();
        Self {
            sign: n.sign ^ d.sign,
            numerator: n.num,
            denominator: d.num,
        }
    }
}

#[derive(Debug)]
pub struct Integer{
    pub sign: bool,
    pub num: u64,
}

pub trait IntoInteger {
    fn into_int(self) -> Integer;
}

impl IntoInteger for i64 {
    fn into_int(self) -> Integer {
        let sign = self < 0;
        Integer {
            sign,
            num: if sign {-self} else {self} as u64,
        }
    }
}

impl IntoInteger for i32 {
    fn into_int(self) -> Integer {
        let sign = self < 0;
        Integer {
            sign,
            num: if sign {-self} else {self} as u64,
        }
    }
}

impl IntoInteger for i16 {
    fn into_int(self) -> Integer {
        let sign = self < 0;
        Integer {
            sign,
            num: if sign {-self} else {self} as u64,
        }
    }
}

impl IntoInteger for i8 {
    fn into_int(self) -> Integer {
        let sign = self < 0;
        Integer {
            sign,
            num: if sign {-self} else {self} as u64,
        }
    }
}

impl IntoInteger for isize {
    fn into_int(self) -> Integer {
        let sign = self < 0;
        Integer {
            sign,
            num: if sign {-self} else {self} as u64,
        }
    }
}

impl IntoInteger for u64 {
    fn into_int(self) -> Integer {
        Integer {
            sign: true,
            num: self as u64
        }
    }
}

impl IntoInteger for u32 {
    fn into_int(self) -> Integer {
        Integer {
            sign: true,
            num: self as u64
        }
    }
}
impl IntoInteger for u16 {
    fn into_int(self) -> Integer {
        Integer {
            sign: true,
            num: self as u64
        }
    }
}
impl IntoInteger for u8 {
    fn into_int(self) -> Integer {
        Integer {
            sign: true,
            num: self as u64
        }
    }
}
impl IntoInteger for usize {
    fn into_int(self) -> Integer {
        Integer {
            sign: true,
            num: self as u64
        }
    }
}
