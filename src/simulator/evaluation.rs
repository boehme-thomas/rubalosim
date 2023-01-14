use chrono::{DateTime, Local};

pub struct Evaluation {
    simulation_startup: DateTime<Local>,
    simulation_ending: DateTime<Local>,
    rule_execution_started: DateTime<Local>,
    rule_execution_ended: DateTime<Local>,
    //calculated_energy_consumption: todo!(),
    uplink_messages: u64,
    downlink_messages: u64,
    downlink_uplink_messages_per_sensor_type: Vec<(u64, u64)>
}

impl Evaluation {
    pub fn new() -> Evaluation {
        let local = Local::now();
        return Evaluation {
            simulation_startup: local,
            simulation_ending: local,
            rule_execution_started: local,
            rule_execution_ended: local,
            uplink_messages: 0,
            downlink_messages: 0,
            downlink_uplink_messages_per_sensor_type: Vec::new()
        }
    }

    pub fn set_downlink_uplink_messages_per_sensor_type(&mut self, tupel_vec: Vec<(u64, u64)>) {
        self.downlink_uplink_messages_per_sensor_type = tupel_vec;
    }

    pub fn get_downlink_uplink_messages_per_sensor_type(&self) -> &Vec<(u64, u64)> {
        &self.downlink_uplink_messages_per_sensor_type
    }

    pub fn set_simulation_star_up(&mut self, start: DateTime<Local>){
        self.simulation_startup = start;
    }

    pub fn get_simulation_star_up(&self) -> DateTime<Local> {
        self.simulation_startup
    }

    pub fn set_simulation_ending(&mut self, start: DateTime<Local>){
        self.simulation_ending = start;
    }

    pub fn get_simulation_ending(&self) -> DateTime<Local> {
        self.simulation_ending
    }

    pub fn set_rule_execution_started(&mut self, time: DateTime<Local>) {
        self.rule_execution_started = time;
    }

    pub fn get_rule_execution_started(&self) -> DateTime<Local> {
        self.rule_execution_started
    }

    pub fn set_rule_execution_ended(&mut self, time: DateTime<Local>) {
        self.rule_execution_ended = time;
    }

    pub fn get_rule_execution_ended(&self) -> DateTime<Local> {
        self.rule_execution_ended
    }

    pub fn get_uplink_messages(&self) -> u64 {
        self.uplink_messages
    }

    pub fn set_uplink_messages(&mut self, number: u64) {
        self.uplink_messages = number;
    }

    pub fn get_downlink_messages(&self) -> u64 {
        self.downlink_messages
    }

    pub fn set_downlink_messages(&mut self, number: u64) {
        self.downlink_messages = number;
    }
}