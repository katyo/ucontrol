use core::ops::{Div, Sub, Mul};
use typenum::{IsLess, Abs, AbsVal, Unsigned, Integer, Z0, Diff};
use crate::{FromUnsigned, BitsType, Pow, Fix, Cast};

macro_rules! cast_from {
    ($type: ty) => {
        impl<Bits, Base, Exp> Cast<$type> for Fix<Bits, Base, Exp>
        where
            $type: Cast<Bits::Type>,
            Bits: BitsType,
            Bits::Type: FromUnsigned + Pow + Cast<$type> + Mul<Bits::Type, Output = Bits::Type> + Div<Bits::Type, Output = Bits::Type>,
            Base: Unsigned,
            Z0: IsLess<Exp>,
            Exp: Abs,
            AbsVal<Exp>: Integer,
        {
            fn cast(value: $type) -> Self {
                Self::from(value)
            }
        }
    }
}

cast_from!(u8);
cast_from!(u16);
cast_from!(u32);
cast_from!(u64);
#[cfg(feature = "i128")]
cast_from!(u128);

cast_from!(i8);
cast_from!(i16);
cast_from!(i32);
cast_from!(i64);
#[cfg(feature = "i128")]
cast_from!(i128);

cast_from!(f32);
cast_from!(f64);

impl<Bits, ToBits, Base, Exp, ToExp> Cast<Fix<Bits, Base, Exp>> for Fix<ToBits, Base, ToExp>
where
    Bits: BitsType + IsLess<ToBits>,
    Bits::Type: FromUnsigned + Pow + Mul<Output = Bits::Type> + Div<Output = Bits::Type>,
    ToBits: BitsType,
    ToBits::Type: FromUnsigned + Pow + Mul<Output = ToBits::Type> + Div<Output = ToBits::Type> + Cast<Bits::Type>,
    Base: Unsigned,
    Exp: Sub<ToExp>,
    Diff<Exp, ToExp>: Abs + IsLess<Z0>,
    AbsVal<Diff<Exp, ToExp>>: Integer
{
    fn cast(value: Fix<Bits, Base, Exp>) -> Self {
        value.convert()
    }
}

#[cfg(test)]
mod test {
    use typenum::*;
    use crate::{bin::{IFix32, IFix64}, Cast};

    type F32 = IFix32<N16>;
    type F64 = IFix64<N16>;
    type F64D = IFix64<N32>;

    #[test]
    fn mul() {
        let a = F32::from(123.456);
        let b = F32::from(78.9);
        let c = F32::cast(a * b);

        assert_eq!(c, F32::from(9740.67715));
    }

    #[test]
    fn div() {
        let a = F32::from(6789.12);
        let b = F32::from(12.345);
        let c = F32::cast(F64D::cast(a) / b);

        assert_eq!(c, F32::from(549.9496));
    }
}