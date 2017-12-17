
#[derive(Debug, Clone, Builder)]
pub struct Star {
    pub mass: f64,
    pub luminosity: f64,
    pub metalicity: f64,
}

impl Star {
    pub fn new(mass: f64, luminosity: f64, metalicity: f64) -> Self {
        Star {
            mass,
            luminosity,
            metalicity,
        }
    }
}
