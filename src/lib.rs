//! This crate is a framework for the simulation of a rule based LoRaWAN.
//! <br/>An example for the implementation for the UseCase _building automation_ can be found like the following:
//! - in the crate building-automation
//! - in the github repo building-automation

/// This module is the main module of this crate.
/// <br/>To create a simulator and run a simulation.
pub mod simulator;

/// This module is for the representation of movable objects in the simulation,
/// e.g. cars, humans, animals, etc.
pub mod movable_object;

/// This module is for the representation of a structure used by the simulation.
/// <br/> The structure on which the simulation should be executed has to implement the [`UnderlyingStructure`](structure::UnderlyingStructure) trait.
pub mod structure;

/// This module is for the representation of sensors.
pub mod sensor;

/// This module is for the representation of a rule.
pub mod rule;

