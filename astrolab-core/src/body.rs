#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Body{
    name: &'static str,
    /// Standard gravitational parameter (G × M), in m³/s² 
    mu: f64,
    /// Mean radius, in meters
    radius: f64,
}
impl Body{
    pub const EARTH: Self = Self{ name: "Earth", mu: 3.986004418e14, radius: 6.371e6};
    pub const MOON: Self = Self{ name: "Moon", mu:4.9028e12, radius: 1.7374e6};
    pub const MARS: Self = Self{ name: "Mars", mu: 4.282837e13, radius: 3.3895e6};
    pub const SUN: Self = Self{ name: "Sun", mu: 1.32712440018e20, radius: 6.957e8};
    pub const ALL: &[Body] = &[Self::EARTH, Self::MOON, Self::MARS, Self::SUN];

    pub fn from_name(name: &str) -> Option<Self>{
        for body in Self::ALL{
            if body.name.eq_ignore_ascii_case(name){
                return Some(*body);
            }
        }
        None
    }
    
    pub fn name(&self) -> &'static str{
        self.name
    }
    pub fn mu(&self) -> f64{
        self.mu
    }
    pub fn radius(&self) -> f64{
        self.radius
    }
}
