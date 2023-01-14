use chrono::{Duration, NaiveTime};

/// Trait for the representation of movable objects, such as a
/// cars, human beings, animals, robots etc.
pub trait MovableObjects {

    /// Gets number of movable objects
    fn get_number_of_movable_objects(&self) -> u32;

    /// Gets number of moves, that should be created by the simulation.
    fn get_number_of_random_moves(&self) -> u32;

    /// Gets time of creation.
    fn get_time_of_creation(&self) -> NaiveTime;

    /// Gets time of deletion
    fn get_time_of_deletion(&self) -> NaiveTime;

    /// Gets speed
    fn get_speed(&self) -> Duration;
}