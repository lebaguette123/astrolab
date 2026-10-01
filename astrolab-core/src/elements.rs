#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Elements{
    /// Length, in meters, of the semi-major axis of the orbit
    pub a: f64,
    /// Eccentricity of the orbit, dimensionless. 0 ≤ e < 1 for the elliptical orbits Astrolab supports.f
    pub e: f64,
    /// Inclination, how far the orbit plane is tilted from the reference plane, in radians.
    /// Ranges: [0, π], where 0 is equatorial, and π/2 is polar.
    pub i: f64,
    /// Right ascension of the ascending node: the angle, measured in the reference plane,
    /// from the reference direction to the point where the orbit crosses upward through that plane.
    /// It sets which way the tilt is oriented. Range [0, 2π).
    pub raan: f64,
    /// Argument of periapsis: the angle, measured within the orbit plane, from that ascending-node crossing
    /// to the closest point of the orbit. It sets where the ellipse points. Range [0, 2π)
    pub argp: f64,
    /// True anomaly: the angle, within the orbit plane, from periapsis to where the body is right now.
    /// Range [0, 2π)
    pub nu: f64,
}