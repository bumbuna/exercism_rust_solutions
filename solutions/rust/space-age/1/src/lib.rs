// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    seconds: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self { seconds: s as f64 }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        d.seconds / SECONDS_IN_EARTH_YEAR
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

const SECONDS_IN_EARTH_YEAR: f64 = 31_557_600_f64;

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        d.seconds / (0.2408467 * SECONDS_IN_EARTH_YEAR)
    }
}
impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        d.seconds / (0.61519726 * SECONDS_IN_EARTH_YEAR)
    }
}
impl Planet for Earth {}
impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        d.seconds / (1.8808158 * SECONDS_IN_EARTH_YEAR)
    }
}
impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        d.seconds / (11.862615 * SECONDS_IN_EARTH_YEAR)
    }
}
impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        d.seconds / (29.447498 * SECONDS_IN_EARTH_YEAR)
    }
}
impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        d.seconds / (84.016846 * SECONDS_IN_EARTH_YEAR)
    }
}
impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        d.seconds / (164.79132 * SECONDS_IN_EARTH_YEAR)
    }
}
