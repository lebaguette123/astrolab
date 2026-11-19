use crate::body::Body;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Orbit{
    pub body: Body,
    pub semi_major_axis: f64,
    pub eccentricity: f64,
}
impl Orbit{
    pub fn new(body: Body, semi_major_axis: f64, eccentricity: f64) -> Result<Self, OrbitError>{
        if !semi_major_axis.is_finite() || semi_major_axis <= 0.0{
            return Err(OrbitError::InvalidSemiMajorAxis)
        }
        if !eccentricity.is_finite() || eccentricity < 0.0 || eccentricity >= 1.0{
            return Err(OrbitError::InvalidEccentricity)
        }
        let periapsis = semi_major_axis * (1.0 - eccentricity);
        if periapsis <= body.radius(){
            return Err(OrbitError::PeriapsisBelowSurface)
        }

        Ok(Self{ body, semi_major_axis, eccentricity })
    }

    /// Calculates the orbital period for an object in a Keplerian orbit.
    ///
    /// The period is computed using:
    ///
    /// `T = 2π * sqrt(a³ / μ)`
    ///
    /// where:
    /// - `a` is the semi-major axis
    /// - `μ` is the standard gravitational parameter of the central body
    ///
    /// # Parameters
    ///
    /// * `semi_major_axis` - The semi-major axis of the orbit.
    ///
    /// # Returns
    ///
    /// The orbital period in time units consistent with `self.mu` and `semi_major_axis`.
    ///
    /// # Notes
    ///
    /// `semi_major_axis` and `self.mu` should be positive values. Invalid or non-physical
    /// values may result in `NaN` or infinite results.
    pub fn orbital_period(&self, semi_major_axis: f64) -> f64{
        2.0 * std::f64::consts::PI *(semi_major_axis.powi(3)/self.body.mu()).sqrt()
    }

    fn periapsis_distance(&self, semi_major_axis: f64, eccentricity: f64) -> f64{
        semi_major_axis*(1.0 - eccentricity)
    }

    fn apoapsis_distance(&self, semi_major_axis: f64, eccentricity: f64) -> f64{
        semi_major_axis*(1.0 + eccentricity)
    }

    pub fn apoapsis_altitude(&self, semi_major_axis: f64, eccentricity: f64) -> f64{
        self.apoapsis_distance(semi_major_axis, eccentricity) - self.body.radius()
    }

    pub fn periapsis_altitude(&self, semi_major_axis: f64, eccentricity: f64) -> f64{
        self.periapsis_distance(semi_major_axis, eccentricity) - self.body.radius()
    }

    pub fn speed_at_periapsis(&self, semi_major_axis: f64, eccentricity: f64) -> f64{
        (self.body.mu() * (2.0 / self.periapsis_distance(semi_major_axis, eccentricity) - 1.0 / semi_major_axis)).sqrt()
    }

    pub fn speed_at_apoapsis(&self, semi_major_axis: f64, eccentricity: f64) -> f64{
        (self.body.mu() * (2.0 / self.apoapsis_distance(semi_major_axis, eccentricity) - 1.0 / semi_major_axis)).sqrt()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OrbitError{
    InvalidSemiMajorAxis,
    InvalidEccentricity,
    PeriapsisBelowSurface,
}
impl std::fmt::Display for OrbitError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            OrbitError::InvalidSemiMajorAxis => write!(f, "Invalid semi-major axis"),
            OrbitError::InvalidEccentricity => write!(f, "Invalid eccentricity"),
            OrbitError::PeriapsisBelowSurface => write!(f, "Periapsis is below the surface of the body"),
        }
    }
}

impl std::error::Error for OrbitError{}

#[cfg(test)]
mod tests {
    use super::{Orbit, OrbitError};
    use crate::body::Body;

    #[test]
    fn earth_low_earth_orbit_period() {
        let orbit = Orbit::new(Body::EARTH, 6_778_000.0, 0.0).expect("valid orbit");

        assert!((orbit.orbital_period(6_778_000.0) - 5_553.0).abs() <= 1.0);
    }

    #[test]
    fn earth_geostationary_orbit_period() {
        let orbit = Orbit::new(Body::EARTH, 42_164_000.0, 0.0).expect("valid orbit");

        assert!((orbit.orbital_period(42_164_000.0) - 86_164.0).abs() <= 5.0);
    }

    #[test]
    fn earth_circular_orbit_altitudes_and_speeds() {
        let orbit = Orbit::new(Body::EARTH, 6_778_000.0, 0.0).expect("valid orbit");

        assert!((orbit.periapsis_altitude(6_778_000.0, 0.0) - 407_000.0).abs() <= 1_000.0);
        assert!((orbit.apoapsis_altitude(6_778_000.0, 0.0) - 407_000.0).abs() <= 1_000.0);
        assert!((orbit.speed_at_periapsis(6_778_000.0, 0.0) - 7_669.0).abs() <= 5.0);
        assert!((orbit.speed_at_apoapsis(6_778_000.0, 0.0) - 7_669.0).abs() <= 5.0);
    }

    #[test]
    fn earth_geostationary_transfer_orbit_altitudes_and_speeds() {
        let orbit = Orbit::new(Body::EARTH, 24_364_000.0, 0.7303).expect("valid orbit");

        assert!((orbit.periapsis_altitude(24_364_000.0, 0.7303) - 200_000.0).abs() <= 1_000.0);
        assert!((orbit.apoapsis_altitude(24_364_000.0, 0.7303) - 35_786_000.0).abs() <= 1_000.0);
        assert!((orbit.speed_at_periapsis(24_364_000.0, 0.7303) - 10_245.0).abs() <= 5.0);
        assert!((orbit.speed_at_apoapsis(24_364_000.0, 0.7303) - 1_597.0).abs() <= 5.0);
    }

    #[test]
    fn earth_transfer_orbit_conserves_specific_angular_momentum() {
        let orbit = Orbit::new(Body::EARTH, 24_364_000.0, 0.7303).expect("valid orbit");
        let periapsis = 24_364_000.0 * (1.0 - 0.7303);
        let apoapsis = 24_364_000.0 * (1.0 + 0.7303);
        let periapsis_angular_momentum = periapsis * orbit.speed_at_periapsis(24_364_000.0, 0.7303);
        let apoapsis_angular_momentum = apoapsis * orbit.speed_at_apoapsis(24_364_000.0, 0.7303);

        assert!((periapsis_angular_momentum - 6.73e10).abs() <= 1.0e8);
        assert!((apoapsis_angular_momentum - 6.73e10).abs() <= 1.0e8);
        assert!((periapsis_angular_momentum - apoapsis_angular_momentum).abs() <= 1.0e-3);
    }

    #[test]
    fn accepts_valid_circular_orbit() {
        let orbit = Orbit::new(Body::EARTH, 7_000_000.0, 0.0).expect("valid orbit");

        assert_eq!(orbit.eccentricity, 0.0);
    }

    #[test]
    fn rejects_non_positive_or_non_finite_semi_major_axis() {
        for semi_major_axis in [0.0, -1.0, f64::NAN] {
            assert_eq!(
                Orbit::new(Body::EARTH, semi_major_axis, 0.0),
                Err(OrbitError::InvalidSemiMajorAxis)
            );
        }
    }

    #[test]
    fn rejects_eccentricity_outside_elliptical_range() {
        for eccentricity in [-f64::EPSILON, 1.0, 1.0 + f64::EPSILON] {
            assert_eq!(
                Orbit::new(Body::EARTH, 7_000_000.0, eccentricity),
                Err(OrbitError::InvalidEccentricity)
            );
        }
    }

    #[test]
    fn rejects_periapsis_below_or_at_surface() {
        let eccentricity = 0.5;
        let surface_periapsis_axis = Body::EARTH.radius() / (1.0 - eccentricity);

        assert_eq!(
            Orbit::new(Body::EARTH, surface_periapsis_axis - 1.0, eccentricity),
            Err(OrbitError::PeriapsisBelowSurface)
        );
        assert_eq!(
            Orbit::new(Body::EARTH, surface_periapsis_axis, eccentricity),
            Err(OrbitError::PeriapsisBelowSurface)
        );
    }
}