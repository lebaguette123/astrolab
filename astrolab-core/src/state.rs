use crate::Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StateVector{
    /// Position in meters, relative to the central body's center, in an inertial frame
    pub r: Vec3,
    /// Velocity in meters per second, relative to the central body's center, in an inertial frame
    pub v: Vec3,
}