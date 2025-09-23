// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

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
    fn years_during(d: &Duration) -> f64 {
        d.years() / Self::earth_comparison_quotient()
    }

    fn earth_comparison_quotient() -> f64;
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn earth_comparison_quotient() -> f64 {
        0.2408467
    }
}
impl Planet for Venus {
    fn earth_comparison_quotient() -> f64 {
        0.61519726
    }
}
impl Planet for Earth {
    fn earth_comparison_quotient() -> f64 {
        1f64
    }
}
impl Planet for Mars {
    fn earth_comparison_quotient() -> f64 {
        1.8808158
    }
}
impl Planet for Jupiter {
    fn earth_comparison_quotient() -> f64 {
        11.862615
    }
}
impl Planet for Saturn {
    fn earth_comparison_quotient() -> f64 {
        29.447498
    }
}
impl Planet for Uranus {
    fn earth_comparison_quotient() -> f64 {
        84.016846
    }
}
impl Planet for Neptune {
    fn earth_comparison_quotient() -> f64 {
        164.79132
    }
}
