#[derive(Debug)]
pub struct Duration(u64);

impl Duration {
    pub fn new(seconds: u64) -> Self { Self(seconds) }
    pub fn years(&self) -> f64 {
        self.0 as f64 / 60f64 / 60f64 / 24f64 / 365.25f64
    }
}

impl From<u64> for Duration {
    fn from(seconds: u64) -> Self {
        Duration(seconds)
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

macro_rules! planet {
    ($name:ident,$earth_comparison_quotient:literal) => {
        pub struct $name;
        impl Planet for $name {
            fn years_during(d: &Duration) -> f64 {
                d.years() / $earth_comparison_quotient
            }
        }
    }
}

planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Earth, 1f64);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);
