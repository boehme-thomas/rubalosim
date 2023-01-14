/// Struct for a type of sensor
#[derive(Clone)]
pub struct SensorType {
    id: String,
    device_profile_id: String,
    uplink_interval_in_sec: u64
}
impl SensorType {
    /// Creates new SensorType.
    /// <br/>Id has to be of patter: "SensorType_" + _number of sensor type_
    pub fn new(id: String, device_profile_id: String, uplink_interval_in_sec: u64) -> SensorType {
        return SensorType {
            id,
            device_profile_id,
            uplink_interval_in_sec
        }
    }

    /// Gets SensorType id.
    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    /// Gets device profile id.
    pub fn get_device_profile_id(&self) -> String {
        self.device_profile_id.clone()
    }

    /// Gets uplink interval in sec.
    pub fn get_uplink_interval_in_sec(&self) -> u64 {
        self.uplink_interval_in_sec
    }
}

/// Struct for the representation of sensors.
#[derive(Clone)]
pub struct Sensor {
    id: String,
    sensor_type: SensorType,
    no: i64
}

impl Sensor {
    /// Creates new Sensor.
    /// <br/>Id has to be of pattern: "Sensor_ " + _Id of the node_ + "_no. _" + _number of sensor of all same sensors_ + "_of_type _" + _sensor type id_.
    pub fn new(id: String, sensor_type: SensorType, no:i64) -> Sensor {
        return Sensor {
            id,
            sensor_type,
            no
        }
    }

    /// Gets id. This should be unique under all sensors.
    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    /// Gets SensorType
    pub fn get_sensor_type(&self) -> SensorType {
        self.sensor_type.clone()
    }

    /// Sets id.
    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    /// Gets number. This should be unique under all sensors.
    pub fn get_number(&self) -> i64{
        self.no
    }
}