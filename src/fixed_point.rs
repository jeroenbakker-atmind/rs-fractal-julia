use std::ops::{Add, Div, Mul, Shl, Shr, Sub};

const PRECISION: i8 = 16;
const PRECISION_MULTIPLIER: f32 = 65536.0;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct FixedPoint<T>
where
    T: Copy + PartialOrd,
{
    value: T,
}

impl<T> From<f32> for FixedPoint<T>
where
    T: From<i32> + Copy + PartialOrd,
{
    fn from(src: f32) -> FixedPoint<T> {
        FixedPoint {
            value: T::from((src * PRECISION_MULTIPLIER) as i32),
        }
    }
}

impl<T> Div<FixedPoint<T>> for FixedPoint<T>
where
    T: Div<T, Output = T> + Shl<i8, Output = T> + Copy + PartialOrd,
{
    type Output = FixedPoint<T>;

    fn div(self, other: FixedPoint<T>) -> FixedPoint<T> {
        FixedPoint::<T> {
            value: (self.value << PRECISION) / other.value,
        }
    }
}

impl<T> Mul<FixedPoint<T>> for FixedPoint<T>
where
    T: Mul<T, Output = T> + Shr<i8, Output = T> + Copy + PartialOrd,
{
    type Output = FixedPoint<T>;

    fn mul(self, other: FixedPoint<T>) -> FixedPoint<T> {
        FixedPoint::<T> {
            value: (self.value * other.value) >> PRECISION,
        }
    }
}

impl<T> Add<FixedPoint<T>> for FixedPoint<T>
where
    T: Add<T, Output = T> + Copy + PartialOrd,
{
    type Output = FixedPoint<T>;

    fn add(self, other: FixedPoint<T>) -> FixedPoint<T> {
        FixedPoint::<T> {
            value: self.value + other.value,
        }
    }
}

impl<T> Sub<FixedPoint<T>> for FixedPoint<T>
where
    T: Sub<T, Output = T> + Copy + PartialOrd,
{
    type Output = FixedPoint<T>;

    fn sub(self, other: FixedPoint<T>) -> FixedPoint<T> {
        FixedPoint::<T> {
            value: self.value - other.value,
        }
    }
}
