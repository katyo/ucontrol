use super::{BitsType, Cast, Fix, FromUnsigned, Pow};
use core::ops::{Div, Mul};
use typenum::{Abs, AbsVal, Bit, Integer, IsLess, Le, Unsigned, Z0};

macro_rules! from_num {
    ($TYPE: ty, $KIND: tt) => {
        impl<Bits, Base, Exp> From<$TYPE> for Fix<Bits, Base, Exp>
        where
            $TYPE: Cast<Bits::Type>,
            Bits: BitsType<Base>,
            Bits::Type: FromUnsigned + Pow + Cast<$TYPE> + Mul<Bits::Type, Output = Bits::Type> + Div<Bits::Type, Output = Bits::Type>,
            Base: Unsigned,
            Z0: IsLess<Exp>,
            Exp: Abs,
            AbsVal<Exp>: Integer,
        {
            fn from(value: $TYPE) -> Self {
                let base = Bits::Type::from_unsigned::<Base>(); // e
                let abs_exp = AbsVal::<Exp>::to_i32() as u32; // |exp|
                let exp_pos = Le::<Z0, Exp>::to_bool(); // 0 < exp

                // FIXME: Would like to do this with typenum::Pow, but that
                // seems to result in overflow evaluating requirements.
                let ratio = base.pow(abs_exp); // base^|exp|

                // TODO: Add rounding

                Self::new(if exp_pos {
                    from_num!(@$KIND, /, $TYPE, Bits::Type, value, ratio)
                } else {
                    from_num!(@$KIND, *, $TYPE, Bits::Type, value, ratio)
                })
            }
        }
    };

    (@float, $OP: tt, $TYPE: ty, $BITS: ty, $value: ident, $ratio: ident) => {
        <$BITS>::cast($value $OP <$TYPE>::cast($ratio))
    };

    (@int, $OP: tt, $TYPE: ty, $BITS: ty, $value: ident, $ratio: ident) => {
        <$BITS>::cast($value) $OP $ratio
    };
}

from_num!(i8, int);
from_num!(i16, int);
from_num!(i32, int);
from_num!(i64, int);
#[cfg(feature = "i128")]
from_num!(i128, int);

from_num!(u8, int);
from_num!(u16, int);
from_num!(u32, int);
from_num!(u64, int);
#[cfg(feature = "i128")]
from_num!(u128, int);

from_num!(f32, float);
from_num!(f64, float);

#[cfg(test)]
mod test {
    use crate::si::Milli;
    use typenum::*;

    #[test]
    fn from_u16() {
        let a = Milli::<P4>::from(9u16);
        assert_eq!(a, Milli::new(9_000));
    }

    #[test]
    fn from_i16() {
        let a = Milli::<P4>::from(11i16);
        assert_eq!(a, Milli::new(11_000));
    }

    #[test]
    fn from_i16_neg() {
        let a = Milli::<P4>::from(-11i16);
        assert_eq!(a, Milli::new(-11_000));
    }

    #[test]
    fn from_u32() {
        let a = Milli::<P8>::from(9u32);
        assert_eq!(a, Milli::new(9_000));
    }

    #[test]
    fn from_i32_neg() {
        let a = Milli::<P8>::from(-11i32);
        assert_eq!(a, Milli::new(-11_000));
    }

    #[test]
    fn from_u64() {
        let a = Milli::<P16>::from(9u64);
        assert_eq!(a, Milli::new(9_000));
    }

    #[test]
    fn from_i64_neg() {
        let a = Milli::<P16>::from(-11i64);
        assert_eq!(a, Milli::new(-11_000));
    }

    #[test]
    fn from_f32() {
        let a = Milli::<P4>::from(0.1f32);
        assert_eq!(a, Milli::new(0_100));

        let a = Milli::<P4>::from(-2.5f32);
        assert_eq!(a, -Milli::new(2_500));
    }

    #[test]
    fn from_f64() {
        let a = Milli::<P4>::from(0.1f64);
        assert_eq!(a, Milli::new(0_100));

        let a = Milli::<P4>::from(-2.5f64);
        assert_eq!(a, -Milli::new(2_500));
    }
}
