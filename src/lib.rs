#[macro_use]
mod measurement;
pub use measurement::Measurement;

pub mod length;
pub use length::Length;

pub mod temperature;
pub use temperature::{Temperature, TemperatureDelta};

pub mod mass;
pub use mass::Mass;

pub mod volume;
pub use volume::Volume;

pub mod pressure;
pub use pressure::Pressure;

pub mod speed;
pub use speed::Speed;

pub mod acceleration;
pub use acceleration::Acceleration;

pub mod energy;
pub use energy::Energy;

pub mod power;
pub use power::Power;

pub mod force;
pub use force::Force;

pub mod area;
pub use area::Area;

/// For given types A, B and C, implement, using base units:
///     - A = B * C 
///     - A = C * B 
///     - B = A / C 
///     - C = A / B
macro_rules! impl_maths {
    ($a:ty, $b:ty) => {
        impl ::std::ops::Mul<$b> for $b {
            type Output = $a;

            fn mul(self, rhs: $b) -> Self::Output {
                Self::Output::from_base_units(self.get_base_units() * rhs.get_base_units())
            }
        }

        impl ::std::ops::Div<$b> for $a {
            type Output = $b;

            fn div(self, rhs: $b) -> Self::Output {
                Self::Output::from_base_units(self.get_base_units() / rhs.get_base_units())
            }
        }
    };

    ($a:ty, $b:ty, $c:ty) => {
        impl ::std::ops::Mul<$b> for $c {
            type Output = $a;

            fn mul(self, rhs: $b) -> Self::Output {
                Self::Output::from_base_units(self.get_base_units() * rhs.get_base_units())
            }
        }

        impl ::std::ops::Mul<$c> for $b {
            type Output = $a;

            fn mul(self, rhs: $c) -> Self::Output {
                Self::Output::from_base_units(self.get_base_units() * rhs.get_base_units())
            }
        }

        impl ::std::ops::Div<$c> for $a {
            type Output = $b;

            fn div(self, rhs: $c) -> Self::Output {
                Self::Output::from_base_units(self.get_base_units() / rhs.get_base_units())
            }
        }

        impl ::std::ops::Div<$b> for $a {
            type Output = $c;

            fn div(self, rhs: $b) -> Self::Output {
                Self::Output::from_base_units(self.get_base_units() / rhs.get_base_units())
            }
        }
    }
}

impl Measurement for std::time::Duration {
    fn get_base_units(&self) -> f64 {
        self.as_secs() as f64 + ((self.subsec_nanos() as f64) * 1e-9)
    }

    fn from_base_units(units: f64) -> Self {
        let subsec_nanos = ((units * 1e9) % 1e9) as u32;
        let secs = units as u64;
        std::time::Duration::new(secs, subsec_nanos)
    }

    fn get_base_units_name(&self) -> &'static str {
        "s"
    }
}

impl_maths!(Area, Length);
impl_maths!(Energy, Force, Length);
impl_maths!(Energy, std::time::Duration, Power);
impl_maths!(Force, Mass, Acceleration);
impl_maths!(Force, Pressure, Area);
impl_maths!(Length, std::time::Duration, Speed);
impl_maths!(Power, Force, Speed);
impl_maths!(Speed, std::time::Duration, Acceleration);
impl_maths!(Volume, Length, Area);

// Include when running tests, but don't export them
#[cfg(test)]
#[allow(dead_code)]
mod tests;
