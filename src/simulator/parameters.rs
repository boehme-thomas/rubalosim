use std::borrow::{Borrow, BorrowMut};
use std::io::{Error, ErrorKind};
use std::sync::Arc;
use elorapi::devices::{DeviceProfile, DeviceProfileContainer};
use crate::movable_object::MovableObjects;
use crate::rule::{Rule};
use crate::structure::UnderlyingStructure;
use crate::sensor::{SensorType};


/// Struct for adding parameters to the [Simulator].
/// <br/> Be aware that `movable_objects` needs an underlying struct,
/// which implements the trait [MovableObject].
/// <br/> The same holds for the `underlying_structure`,
/// which needs a representation of a graph as underlying struct, that has to implement the [UnderlyingStructure] trait.
pub struct Parameters {
    underlying_structure: Option<Arc<dyn UnderlyingStructure>>,
    movable_objects: Arc<dyn MovableObjects>,
    rules: Vec<Rule>,
    device_profile_container: DeviceProfileContainer,
    sensor_types: Vec<SensorType>,
    number_of_sensors: i64
}

impl Parameters {
    /// Create new Parameters, with a specific struct that have to implement `UnderlyingStructure`.
    /// For more information see the [UnderlyingStructure] trait.
    pub fn new(movable_objects: Arc<dyn MovableObjects>) -> Parameters {
        return Parameters {
            underlying_structure: None,
            movable_objects,
            rules: Vec::<Rule>::new(),
            //rule_types: Vec::<RuleType>::new(),
            device_profile_container: DeviceProfileContainer::new(),
            sensor_types: Vec::<SensorType>::new(),
            number_of_sensors: 0,
        }
    }

    /// Adds a specific struct to Parameters, which have to implement the `MovableObject` trait.
    /// <br/> For more information see [MovableObject].
    pub fn change_movable_objects(&mut self, movable_objects: Arc<dyn MovableObjects>) {
        self.movable_objects = movable_objects;
    }

    /// Gets the movable objects.
    pub fn get_movable_objects(&mut self) -> &Arc<dyn MovableObjects> {
        self.movable_objects.borrow_mut()
    }

    /// Changes the underlying structure.
    /// <br/> The new structure has to implement the `UnderlyingStructure` trait. For more information see [UnderlyingStructure].
    pub fn change_underlying_structure(&mut self, underlying_structure: Arc<dyn UnderlyingStructure>) {
        self.underlying_structure = Some(underlying_structure);
    }

    /// Gets the underlying structure.
    pub fn get_underlying_structure(&self) -> &Arc<dyn UnderlyingStructure> {
        self.underlying_structure.as_ref().unwrap().borrow()
    }

    /// Sets underlying structure.
    pub fn set_underlying_structure(&mut self, underlying_structure: Arc<dyn UnderlyingStructure>) {
        self.underlying_structure = Some(underlying_structure);
    }

    /// Gets the device profiles. This contains the specification of sensors
    pub fn get_device_profiles(&mut self) -> &mut [DeviceProfile] {
        self.device_profile_container.get_device_profiles()
    }

    /// Gets a device profile index via an id.
    pub fn get_device_profile_index_via_id(&self, device_profile_id: &str) -> Result<usize, Error> {
        self.device_profile_container.get_device_profile_index_via_dev_prof_id(device_profile_id)
    }

    /// Gets sensor types.
    pub fn get_sensor_types(&self) -> Vec<SensorType> {
        self.sensor_types.clone()
    }

    /// Adds sensor with a specific description. Therefor a json file with the path
    /// `downlink_specification_file` or `upload_specification_file` will be loaded.
    /// For more information on how the files have to look like see the elorapi crate
    /// or under this [link](elorapi).
    /// <br/> DeviceProfile is created with id of pattern: "DevProf_ _number_of_dev_prof_added_".
    /// <br/> SensorTyp is created with id of pattern: "SensorType_ _number_of_sensor_types_added_".
    pub fn add_device_profile_via_file(&mut self, uplink_interval_in_sec: u64, downlink_specification_file: Option<&str>, uplink_specification_file: Option<&str>) -> Result<(), Error>{
        if downlink_specification_file.is_none() & uplink_specification_file.is_none() {
            return Err(Error::new(ErrorKind::Other, "No specification file was given!!"));
        }
        let mut len = self.device_profile_container.get_device_profiles().len();
        len = len + 1;
        let id = "DevProf_".to_owned() + &len.to_string();
        let id_copy = id.clone();
        let mut def_prof = DeviceProfile::new(id.as_str(), None, None);
        if downlink_specification_file.is_some() {
            def_prof.read_downlink(downlink_specification_file.unwrap())?;
        }
        if uplink_specification_file.is_some() {
            def_prof.read_uplink(uplink_specification_file.unwrap())?;
        }
        let len_sensors = self.sensor_types.len();
        let id  = "SensorType_".to_owned() + len_sensors.to_string().as_str();
        let sensor_type = SensorType::new(id, id_copy, uplink_interval_in_sec);
        self.sensor_types.push(sensor_type);
        self.device_profile_container.add_device_profile(def_prof);
        Ok(())
    }

    /// Set rules.
    pub fn set_rule(&mut self, rules: Vec<Rule>) {
        self.rules = rules;
    }

    /// Gets rules.
    pub fn get_rules(&self) -> &[Rule] {
        self.rules.borrow()
    }

    /// Sets number of sensors.
    pub fn set_number_of_sensors(&mut self, number_of_sensors:i64) {
        self.number_of_sensors = number_of_sensors;
    }

    /// Gets number of sensors.
    pub fn get_number_of_sensors(&self) -> i64 {
        self.number_of_sensors
    }

}