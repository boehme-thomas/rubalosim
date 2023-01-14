use std::borrow::Borrow;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use chirpstack_api::as_pb::external::api::{GetDeviceResponse, Device as ChirpstackDevice};
use chrono::{NaiveTime, Weekday};
use elorapi::devices::Device;
use elorapi::rules;
use elorapi::rules::{Action, Condition, DeviceCondition, RefValue, TimeCondition};

/*
/// Struct for a type of sensor
#[derive(Clone)]
pub struct RuleType {
    id: String,
    sensor_type_id: String,
    rule: rules::Rule,
}
impl RuleType {
    /// Creates new SensorType.
    pub fn new(id: String, sensor_type_id: String) -> RuleType {
        return RuleType {
            id,
            sensor_type_id
        }
    }

    /// Gets RuleType id.
    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn create_device_condition(sensor_id: String, data_to_be_measured_index: usize, operator: String, threshold: RefValue) -> DeviceCondition {
        let device = Self::create_device_with_sensor_id(sensor_id);
        DeviceCondition::new(device, data_to_be_measured_index, operator, threshold)
    }

    pub fn create_time_condition(weekday: Option<Weekday>, timespan_start: NaiveTime, timespan_end: NaiveTime) -> TimeCondition {
        TimeCondition::new(weekday, timespan_start, timespan_end)
    }

    pub fn create_device_action(sensor_id: String, payload_indices: Vec<usize>) -> Action {
        let device = Self::create_device_with_sensor_id(sensor_id);
        Action::new(device, payload_indices, "".to_string(), 0)
    }

    pub fn create_rule(conditions: Vec<Condition>, bool_ops: Vec<String>, actions: Vec<Action>) -> &rules::Rule {
        let rule_arc = rules::Rule::new(conditions, bool_ops, actions);
        let rule = rule_arc.lock().unwrap().deref();
        return rule.clone();
    }


    /// Create dummy device for the usage of elorapi api, without using actual devices and a
    /// connection to a chirpstack server.
    fn create_device_with_sensor_id(sensor_id:String) -> Device {
        let chirpstack_device = ChirpstackDevice {
            dev_eui: sensor_id,
            name: "".to_string(),
            application_id: 0,
            description: "".to_string(),
            device_profile_id: "".to_string(),
            skip_f_cnt_check: false,
            reference_altitude: 0.0,
            variables: HashMap::new(),
            tags: HashMap::new(),
            is_disabled: false,
        };
        let dev_resp = GetDeviceResponse {
            device: Some(chirpstack_device),
            last_seen_at: None,
            device_status_battery: 0,
            device_status_margin: 0,
            location: None,
        };
        let dev = Device::new(dev_resp);
        return dev;
    }

    /*
    /// Gets device profile id.
    pub fn get_device_profile_id(&self) -> String {
        self.device_profile_id.clone()
    }

    /// Gets uplink interval in sec.
    pub fn get_uplink_interval_in_sec(&self) -> u64 {
        self.uplink_interval_in_sec
    }*/
}*/

/// Struct for the representation of sensors.
#[derive(Clone)]
pub struct Rule {
    id: String,
    pub rule: Arc<Mutex<rules::Rule>>,
}

impl Rule {

    /// Gets id.
    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    /// Gets rule.
    pub fn get_rule(&self) -> &Mutex<rules::Rule> {
        self.rule.borrow()
    }

    pub fn create_device_condition(sensor_id: String, sensor_number:i64, data_to_be_measured_index: usize, operator: String, threshold: RefValue) -> DeviceCondition {
        let device = Self::create_device_with_sensor_id(sensor_id, sensor_number);
        DeviceCondition::new(device, data_to_be_measured_index, operator, threshold)
    }

    pub fn create_time_condition(weekday: Option<Weekday>, timespan_start: NaiveTime, timespan_end: NaiveTime) -> TimeCondition {
        TimeCondition::new(weekday, timespan_start, timespan_end)
    }

    pub fn create_device_action(sensor_id: String, sensor_number:i64, payload_indices: Vec<usize>) -> Action {
        let device = Self::create_device_with_sensor_id(sensor_id, sensor_number);
        Action::new(device, payload_indices, "".to_string(), 0)
    }

    pub fn create_rule(id: String, conditions: Vec<Condition>, bool_ops: Vec<String>, actions: Vec<Action>) -> Self {
        let rule = rules::Rule::new(conditions, bool_ops, actions);
        return Rule {
            id,
            rule
        }
    }

    /*
    pub fn create_RefValue(Option<bool>, Option<i32>, Option<f32>, Option<>, Option<(String, usize)>) -> RefValue {
        IntNumber(i32),
        String(String),
        FloatNumber(f32),
        Bool(bool),
        Uplink((Device, usize)),
    }*/

    /// Create dummy device for the usage of elorapi api, without using actual devices and a
    /// connection to a chirpstack server.
    fn create_device_with_sensor_id(sensor_id:String, sensor_number:i64) -> Device {
        let chirpstack_device = ChirpstackDevice {
            dev_eui: sensor_id,
            name: "".to_string(),
            application_id: sensor_number,
            description: "".to_string(),
            device_profile_id: "".to_string(),
            skip_f_cnt_check: false,
            reference_altitude: 0.0,
            variables: HashMap::new(),
            tags: HashMap::new(),
            is_disabled: false,
        };
        let dev_resp = GetDeviceResponse {
            device: Some(chirpstack_device),
            last_seen_at: None,
            device_status_battery: 0,
            device_status_margin: 0,
            location: None,
        };
        let dev = Device::new(dev_resp);
        return dev;
    }

    /// Gets sensor id and sensor number.
    pub fn get_sensor_information_from_conditions(&self) -> Vec<(String, i64)> {
        let rule = self.rule.lock().unwrap();
        let conditions = rule.get_conditions();
        let mut sensor_infos = Vec::<(String, i64)>::new();
        for condition in conditions {
            match condition {
                Condition::Device(condition) => {
                    let device = condition.get_device();
                    let sensor_id= device.get_chirpstack_device().device.unwrap().dev_eui;
                    let sensor_number = device.get_chirpstack_device().device.unwrap().application_id;
                    sensor_infos.push((sensor_id, sensor_number))
                }
                Condition::Time(_) => {}
            }

        }
        sensor_infos
    }

    /*
    /// Gets SensorType
    pub fn get_sensor_type(&self) -> SensorType {
        self.sensor_type.clone()
    }

    /// Sets id.
    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }*/

}