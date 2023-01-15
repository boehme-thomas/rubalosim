use std::borrow::{Borrow, BorrowMut};
use std::io::{Error, Write};
use chrono::{Duration, Local, NaiveTime, Timelike};
use std::sync::Arc;
use petgraph::graph::{DefaultIx, Graph, NodeIndex};
use petgraph::algo::simple_paths;
use petgraph::Undirected;
use rand_distr::{Distribution, Normal};
use rand;
use rand::{Rng, thread_rng};
use std::fs;
use std::ops::{BitAnd, BitOr, BitXor};
use elorapi::rules::RefValue;
use elorapi::rules::Condition::{Device, Time};
use evaluation::Evaluation;
use crate::structure::{Edge, EdgeData, Node, NodeData};
use crate::simulator::event::{Event, EventList, Events};
use crate::simulator::parameters::Parameters;

/// This module is for an event in the simulation.
pub mod event;

/// This module is for different parameters in the simulation.
pub mod parameters;

/// This mod is for the evaluation of the simulation.
mod evaluation;


pub struct Simulator {
    parameters: Parameters,
    event_list: EventList,
    // needs to be an Arc, since the size must be known at compile time
    evaluation: Evaluation,
    node_of_stays: Vec<(NodeIndex, u32)>,
}

impl Simulator {

    /// Creates a new Simulator object with specific [`Parameters`] and a user specific `Metric`
    /// (for more information about this see [`Metric`])
    pub fn new(parameters: Parameters) -> Simulator {
        let event_list = EventList::new();
        return Simulator {
            parameters,
            event_list,
            evaluation: Evaluation::new(),
            node_of_stays: Vec::new(),
        }
    }

    /// Gets event list.
    pub fn get_event_list_mut(&mut self) -> &mut EventList {
        Self::get_event_list_private(&mut self.event_list)
    }

    fn get_event_list_private(event_list: &mut EventList) -> &mut EventList {
        event_list.borrow_mut()
    }

    /// Gets event list.
    pub fn get_event_list(&self) -> &EventList {
        self.event_list.borrow()
    }

    /// Gets parameters
    pub fn get_parameters(&self) -> &Parameters {
        self.parameters.borrow()
    }

    /// Gets parameters mutable.
    pub fn get_parameters_mut(&mut self) -> &mut Parameters {
        Self::get_parameters_pri(&mut self.parameters)
    }

    fn get_parameters_pri(parameters: &mut Parameters) -> &mut Parameters {
        parameters.borrow_mut()
    }

    /*
    /// Gets node of stays.
    fn get_node_of_movable_object(&mut self) -> &mut [(NodeIndex, u32)] {
        self.node_of_stays.borrow_mut()
    }*/

    fn add_node_of_movable_object(&mut self, node_of_movable_object:(NodeIndex, u32)) {
        self.node_of_stays.push(node_of_movable_object);
    }

    /// Startup of the simulation. Should be executed after the instantiation of [Simulator].
    pub fn start_up_simulation(&mut self, length: i32) {
        self.evaluation.set_simulation_star_up(Local::now());
        self.change_event_list_for_movement(length);
        self.change_event_list_for_sensors().unwrap();
    }

    /// Ending of simulation. Should be executed in the ending. After the rule execution.
    /// <br/> This should also be executed before the writing of the event list.
    pub fn ending_simulation(&mut self, path_for_evaluation:String) -> &EventList {
        self.evaluation.set_simulation_ending(Local::now());
        let sensor_types = self.parameters.get_sensor_types();
        // downlink_uplink
        let mut messages_per_sensor_type :Vec::<(u64, u64)> = Vec::new();
        for _ in sensor_types {
            messages_per_sensor_type.push((0,0));
        }
        let eventlist = &self.event_list.events;
        let mut uplink_counter = 0;
        let mut downlink_counter = 0;
        for event in eventlist {
            let action = event.get_action();
            match action {
                Events::Message(message) => {
                    let mut sensor_type_string = event.get_id();
                    let index = event.get_id().find("SensorType_").unwrap() + 11;
                    sensor_type_string.replace_range(..index, "");
                    let sensor_type = sensor_type_string.parse::<usize>().unwrap();

                    if message.contains("Downlink") {
                        messages_per_sensor_type[sensor_type].0 += 1;
                        downlink_counter += 1;
                    } else if message.contains("Uplink") {
                        messages_per_sensor_type[sensor_type].1 += 1;
                        uplink_counter += 1;
                    }
                }
                _ => {}
            }
        }

        self.evaluation.set_downlink_uplink_messages_per_sensor_type(messages_per_sensor_type);
        self.evaluation.set_downlink_messages(downlink_counter);
        self.evaluation.set_uplink_messages(uplink_counter);

        self.write_evaluation(path_for_evaluation);

        &self.event_list
    }


    fn write_evaluation(&mut self, path:String) {
        let eventlist_len = self.event_list.events.len();

        let ending = self.evaluation.get_simulation_ending();
        let start = self.evaluation.get_simulation_star_up();
        let length = (ending-start).to_string().replace("PT", "").replace("S"," sec");

        let rule_ending = self.evaluation.get_rule_execution_ended();
        let rule_start = self.evaluation.get_rule_execution_started();
        let rule_length = (rule_ending-rule_start).to_string().replace("PT", "").replace("S"," sec");

        let dow_ups_vec = self.evaluation.get_downlink_uplink_messages_per_sensor_type();
        let downs = self.evaluation.get_downlink_messages();
        let ups = self.evaluation.get_uplink_messages();

        let number_of_messages = downs+ups;

        let date = Local::now();
        let path = path + "Evaluation_" + date.date_naive().to_string().as_str() + "_" + date.time().hour().to_string().as_str() + "_"+ date.time().minute().to_string().as_str() + "_" + date.time().second().to_string().as_str() +".txt";
        let mut f = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(path).unwrap();

        let data = "Start of the simulation: ".to_owned() + start.to_string().as_str()+"\n";
        f.write(data.as_bytes()).unwrap();

        let data = "End of the simulation: ".to_owned() + ending.to_string().as_str() + "\n";
        f.write(data.as_bytes()).unwrap();

        let data = "Length of simulation: ".to_owned() + length.to_string().as_str() + "\n\n";
        f.write(data.as_bytes()).unwrap();

        let data = "Start of rule execution: ".to_owned() + rule_start.to_string().as_str()+ "\n";
        f.write(data.as_bytes()).unwrap();

        let data = "End of rule execution: ".to_owned() + rule_ending.to_string().as_str() + "\n";
        f.write(data.as_bytes()).unwrap();

        let data = "Length of rule execution: ".to_owned() + rule_length.to_string().as_str() + "\n\n";
        f.write(data.as_bytes()).unwrap();

        let data = "Length of event list: ".to_owned() + eventlist_len.to_string().as_str() + "\n\n";
        f.write(data.as_bytes()).unwrap();

        let data = "Number of messages: ".to_owned() + number_of_messages.to_string().as_str() + "\n";
        f.write(data.as_bytes()).unwrap();

        let data = "Number of uplink messages: ".to_owned() + ups.to_string().as_str()+ "\n";
        f.write(data.as_bytes()).unwrap();

        let data = "Number of downlink messages: ".to_owned() + downs.to_string().as_str() + "\n\n";
        f.write(data.as_bytes()).unwrap();

        let data = "Number of downlink and uplink messages per sensor type:".to_owned();
        f.write(data.as_bytes()).unwrap();

        for i in 0..dow_ups_vec.len() {
            let data = "\n\t ".to_owned() + "Sensor type " + i.to_string().as_str() + ": " + dow_ups_vec[i].1.to_string().as_str() + " uplink messages, " + dow_ups_vec[i].0.to_string().as_str() + " downlink_messages,";
            f.write(data.as_bytes()).unwrap();
        }
    }

    /// Searches a path from the given start node to the given end not in the specific graph.
    /// It is necessary to give possible nodes, these are nodes which can be used.
    fn search_path(graph: &Graph<Node<Arc<dyn NodeData>>, Edge<Arc<dyn EdgeData>>, Undirected, DefaultIx>, start_node_id: NodeIndex, possible_nodes: Vec<NodeIndex>, end_node_id: Option<NodeIndex>) -> Vec<NodeIndex> {
        let mut rng = thread_rng();
        let len_nodes_to_move_to = possible_nodes.len();
        let node_id:NodeIndex;
        if end_node_id.is_none() {
            let v = rng.gen_range(0..len_nodes_to_move_to);
            node_id = *possible_nodes.get(v).unwrap();
        } else {
            node_id = end_node_id.unwrap();
        }
        let vecs = simple_paths::all_simple_paths::<Vec<NodeIndex>, &Graph<Node<Arc<dyn NodeData>>, Edge<Arc<dyn EdgeData>>, Undirected, DefaultIx>>(graph, start_node_id, node_id, 0, None).collect::<Vec<_>>();
        let path_index = rng.gen_range(0..vecs.len());
        let path:&Vec<NodeIndex> = vecs.get(path_index).unwrap();
        return path.clone();
    }

    /// Creates events for movable objects.
    fn create_movement_events(&self, speed: Duration, mut moves:i32, path: Vec<NodeIndex>, mut next_time:NaiveTime, number_of_movable_object: u32, event_list: &mut EventList ) -> (i32, NaiveTime) {
        let mut path_clone = path.clone();
        path_clone.remove(0);
        for node_index in path_clone {
            let action = Events::Move(node_index);
            next_time = next_time + speed;
            let new_event = Event::new("Movable_object_".to_owned() + number_of_movable_object.to_string().as_str()+"_Move_no._"+moves.to_string().as_str(), next_time, action);
            event_list.add_event(new_event);
            moves = moves+1;
        }
        return (moves, next_time);
    }

    /// Adds events to the event list, that represents the movement of the movable objects.
    /// <br/> Where length is the number of time units that the simulation should run.
    pub fn change_event_list_for_movement(&mut self, length: i32) -> EventList {
        let normal = Normal::new(0.0, 0.5).unwrap();
        let mut event_list = EventList::new();

        let mut node_of_movable_objects = Vec::new();
        let speed = self.parameters.get_movable_objects().get_speed();
        let number_of_movable_objects = self.parameters.get_movable_objects().get_number_of_movable_objects();
        let start = self.parameters.get_movable_objects().get_time_of_creation();
        let end = self.parameters.get_movable_objects().get_time_of_deletion();
        let number_of_moves = self.parameters.get_movable_objects().get_number_of_random_moves();
        let underlying_structure = self.parameters.get_underlying_structure();

        let start_nodes = underlying_structure.get_start_nodes();
        let end_nodes = underlying_structure.get_end_nodes();
        let graph = underlying_structure.get_graph_structure();
        let nodes_to_move_to = underlying_structure.get_nodes_to_move_to();
        let len_nodes_to_move_to = nodes_to_move_to.len();

        let range_start_nodes = start_nodes.len();
        let range_end_nodes = end_nodes.len();

        let wait_vector:Vec<i64> = vec![6, 13, 25];

        let mut rng = thread_rng();

        // create all events for each movable_object
        for i in 0..number_of_movable_objects {
            let v:f32 = normal.sample(rng.borrow_mut());
            // create creation event with a random start node
            // get a normal distributed value, add the start time and round to two decimals
            let milliseconds = (v*60.0*60.0*1_000.0).round() as i64;
            let duration = Duration::milliseconds(milliseconds);
            let creation_time = start+duration;
            // get a uniform distributed index of the start nodes
            let creation_node_index = rng.gen_range(0..range_start_nodes);
            // get id of start node
            let creation_node_id = start_nodes[creation_node_index].clone();
            // create event with specific node id
            let create_action = Events::Create(creation_node_id);

            // create new Event and add it to event list
            let new_event = Event::new("Movable_object_".to_owned() + i.to_string().as_str() + "_Creation", creation_time, create_action);
            event_list.add_event(new_event);


            let path = Simulator::search_path(graph, creation_node_id, nodes_to_move_to.clone(), None);
            let node_index_to_get_back_to = *path.last().unwrap();

            node_of_movable_objects.push((node_index_to_get_back_to.clone(), i.clone()));

            let mut moves = 0;

            let mut next_time = creation_time;
            (moves, next_time) = self.create_movement_events(speed, moves, path, next_time, i, event_list.borrow_mut());

            for _ in 0..number_of_moves {
                // get a uniform random index
                let random_node_index = rng.gen_range(0..len_nodes_to_move_to);
                // get random node
                let random_node = nodes_to_move_to[random_node_index];
                // path from node_index_to_get_back_to to random_node
                let path = Simulator::search_path(graph, node_index_to_get_back_to, nodes_to_move_to.clone(), Some(random_node));

                // when the next movement should be
                next_time = next_time + Duration::minutes(90);

                let last_node = *path.last().unwrap();

                // add all vertices from path to the eventlist
                (moves, next_time) = self.create_movement_events(speed, moves, path, next_time, i, event_list.borrow_mut());

                let wait_random_time_index = rng.gen_range(0..wait_vector.len());
                let wait_random_time = wait_vector.get(wait_random_time_index).unwrap();
                next_time = next_time + Duration::minutes(*wait_random_time);
                // path from the last random_node back to the node_index_to_get_back_to
                // if to not performant enough, change this to the reversed path from above
                let path = Simulator::search_path(graph, last_node, nodes_to_move_to.clone(), Some(node_index_to_get_back_to));
                (moves, next_time) = self.create_movement_events(speed, moves, path, next_time, i, event_list.borrow_mut());
            }


            let v = normal.sample(rng.borrow_mut());

            // create deletion event with a random end node
            // get a normal distributed value, add the end time and round to two decimals
            let milliseconds = (v*60.0*60.0*1_000.0).round() as i64;
            let duration = Duration::milliseconds(milliseconds);
            let deletion_time = end + duration;
            // get a uniform distributed index of the end nodes
            let deletion_node_index = rng.gen_range(0..range_end_nodes);
            // get id of end node
            let deletion_node_id = end_nodes[deletion_node_index].clone();
            // create event with specific id
            let delete_action = Events::Delete(deletion_node_id);

            // create new Event and add it to the event list
            let new_event = Event::new("Movable_object_".to_owned() + i.to_string().as_str() + "_Deletion", deletion_time, delete_action);
            event_list.add_event(new_event);

            // create path from 'node_index_to_get_back_to' to the deletion_node_id node
            let path = Simulator::search_path(graph, node_index_to_get_back_to, nodes_to_move_to.clone(), Some(deletion_node_id));
            let mut path_length = path.len() as i64;
            // for each node in the path add an event to the list, the time is from the path will end at the deletion time
            //moves = moves + path_length as i32;
            for node_index in path {
                let action = Events::Move(node_index);
                let dur = path_length * speed.num_seconds();
                let time = deletion_time - Duration::seconds(dur);
                let new_event = Event::new("Movable_object_".to_owned() + i.to_string().as_str()+"_Move_no._"+moves.to_string().as_str(), time, action);
                event_list.add_event(new_event);
                moves = moves + 1;
                path_length = path_length - 1;
            }

        }
        self.event_list = event_list.clone();

        for i in node_of_movable_objects {
            self.add_node_of_movable_object(i);
        }

        return event_list;
    }

    /// Changes the event list for sensors, so messages on a regular basis are created.
    fn change_event_list_for_sensors(&mut self) -> Result<EventList, Error> {
        let parameters = self.parameters.borrow_mut();
        let underlying_structure= parameters.get_underlying_structure().clone();
        let start_time = self.event_list.get_event_list().first().unwrap().get_time();
        let end_time = self.event_list.get_event_list().last().unwrap().get_time();
        let mut event_id_count = 0;
        for node in underlying_structure.get_graph_structure().node_weights() {
            for sensor in node.get_sensors() {
                let now = Local::now().nanosecond() / 1_000_000;
                let time = Duration::milliseconds(now as i64);

                /*let def_prof_id = sensor.get_sensor_type().get_device_profile_id();
                let res = parameters.get_device_profile_index_via_id(def_prof_id.as_str());
                let index = match res {
                    Ok(index) => index,
                    Err(error) => return Err(error)
                };

                let device_profiles = parameters.get_device_profiles();*/
                let time_interval = sensor.get_sensor_type().get_uplink_interval_in_sec();
                let time_interval_duration = Duration::seconds(time_interval as i64);
                let mut send_time = start_time - time_interval_duration + time;
                while send_time <= end_time + time_interval_duration + time {
                    let sensor_id = sensor.get_id();
                    // create and send actual message
                    // maybe that has to be done in building automation
                    let message = Simulator::create_empty_uplink_sensor_message(parameters, sensor.get_sensor_type().get_device_profile_id())?;
                    let action = Events::Message(message);
                    let event_id = "Message_of_".to_owned() + sensor.get_number().to_string().as_str() + "_" + sensor_id.as_str();
                    let event = Event::new(event_id, send_time, action);
                    self.event_list.add_event(event);
                    send_time = send_time + time_interval_duration;
                    event_id_count = event_id_count + 1;
                }
            }
        }
        let eventlist = self.event_list.clone();
        Ok(eventlist)
    }

    /// Creates sensor message out of the device profile with following pattern, the _**_ has to be exchanged with the specific data:
    /// <br/>For uplink messages: "Uplink_message_ _payload-one_ :_**_, _payload-two_ :_**_,...,".
    fn create_empty_uplink_sensor_message(parameters: &mut Parameters, dev_prof_id: String) -> Result<String, Error>{
        let dev_prof_index = parameters.get_device_profile_index_via_id(dev_prof_id.as_str())?;
        let dev_prof_uplink = parameters.get_device_profiles()[dev_prof_index].get_uplink();
        let mut uplink_message = "Uplink_Message_".to_string();
        if dev_prof_uplink.is_some() {
            let mut uplink = dev_prof_uplink.unwrap();
            let payloads = uplink.get_payloads();
            for payload in payloads {
                uplink_message = uplink_message + payload.as_str() + ":**,";
            }
        } else {
            uplink_message = "Uplink_Message_dummy_message".to_string();
        }
        Ok(uplink_message)
    }

    /// Adds standard values to the uplink messages in the event list, these have as message "**".
    pub fn add_standard_values_to_uplink_messages(&mut self, sensor_type: Vec<String>, data: Vec<Vec<String>>) {
        let eventlist_object = self.get_event_list_mut();
        let eventlist = eventlist_object.get_event_list_mut();
        let length = eventlist.len();
        for j in 0..length {
            let i = eventlist.get(j).unwrap();
            let old_message = i.get_action().get_message();
            if old_message != "" {
                let new_event_id = i.get_id();
                let new_time = i.get_time();
                let mut new_message = old_message;
                for sens in 0..sensor_type.len() {
                    if new_event_id.contains(("SensorType_".to_owned() + sensor_type[sens].clone().as_str()).as_str()) {
                        for replacement in &data[sens] {
                            new_message = new_message.replacen("**", replacement.as_str(), 1);
                        }
                    }
                }
                let new_action = Events::Message(new_message);
                let new_event = Event::new(new_event_id, new_time, new_action);
                eventlist[j] = new_event;
            }
        }
    }

    /// Gets a matrix of nodes where the movable objects moved to.
    pub fn get_matrix_of_nodes_of_movable_objects(&mut self) -> Vec<Vec<(usize, NaiveTime, NodeIndex)>>{
        let number_of_movable_objects = self.get_parameters_mut().get_movable_objects().get_number_of_movable_objects();
        let eventlist = self.event_list.events.clone();
        //let number_of_movable_objects = self.parameters.get_movable_objects().get_number_of_movable_objects();
        let mut matrix:Vec::<Vec<(usize, NaiveTime, NodeIndex)>> = Vec::new();

        for _ in 0..number_of_movable_objects {
            matrix.push(Vec::<(usize, NaiveTime, NodeIndex)>::new());
        }

        for event in eventlist {
            let action = event.get_action();
            match action {
                Events::Move(node_index) => {
                    let mut message = event.get_id();
                    message = message.trim_start_matches("Movable_object_").to_string();
                    let index_of_ = message.find("_");
                    message.replace_range(index_of_.unwrap()..message.len(), "");
                    let number = message.parse::<usize>().unwrap();
                    matrix[number].push((number, event.get_time(), node_index));
                }
                _ => {
                    continue
                }
            }
        }
        return matrix;
    }

    /// Starts the rule execution.
    pub fn rule_execution(&mut self) {
        self.evaluation.set_rule_execution_started(Local::now());
        let number_of_sensors = self.parameters.get_number_of_sensors();
        let sensor_types = self.parameters.get_sensor_types();
        let sensor_data_not_there_placeholder = "Ü_Ü".to_string();
        let used_by = Vec::<String>::new();


        let _device_profiles = self.parameters.get_device_profiles();
        let mut downlinks = Vec::new();
        let mut uplinks = Vec::new();
        for index in 0.._device_profiles.len() {
            downlinks.push((index, _device_profiles[index].get_downlink()));
            uplinks.push((index, _device_profiles[index].get_uplink()));
        }


        let rules = self.parameters.get_rules();

        let mut eventlist = self.event_list.events.clone();
        let mut eventlist_len = eventlist.len();
        let mut event_index = 0;


        //let between = Uniform::new_inclusive(0, 1000);
        //let rng = thread_rng();

        let mut sensor_data = Vec::new();
        for _ in 0..number_of_sensors+1 {
            sensor_data.push((used_by.clone(), NaiveTime::from_hms_opt(0,0,0).unwrap(), sensor_data_not_there_placeholder.clone()));
        }

        while event_index < eventlist_len {

            // get event
            let event = eventlist.get(event_index).unwrap();

            // check if action is a message
            let action_message = match event.get_action() {
                Events::Move(_) => {
                    event_index +=1;
                    continue
                },
                Events::Delete(_) => {
                    event_index +=1;
                    continue
                },
                Events::Create(_) => {
                    event_index +=1;
                    continue
                },
                Events::Message(message) => message
            };

            // get the sensor number out of the event message (event_id)
            let event_message = event.get_id();

            let mut sensor_id_str = event_message.trim_start_matches("Message_of_").to_string();
            let no_number_index = sensor_id_str.find("_").unwrap();
            sensor_id_str.replace_range(no_number_index.., "");

            let sensor_index = sensor_id_str.parse::<usize>().unwrap();

            // set sensor data for sensor_index
            sensor_data[sensor_index] = (Vec::<String>::new(), event.get_time(), action_message);

            'rule: for rule_sim in rules {

                let necessary_sensors = rule_sim.get_sensor_information_from_conditions();

                let rule = rule_sim.get_rule().lock().unwrap();

                let mut time_vec = Vec::<NaiveTime>::new();

                for index in necessary_sensors {
                    if (sensor_data[index.1 as usize].0.contains(&rule_sim.get_id())) | (sensor_data[index.1 as usize].2 == sensor_data_not_there_placeholder) {
                        continue 'rule
                    } else {
                        time_vec.push(sensor_data[index.1.clone() as usize].1);
                    }
                }

                let mut bool_values = Vec::<bool>::new();

                let conditions = rule.get_conditions();

                // condition
                for condition_index in 0..conditions.len() {
                    match conditions.get(condition_index).unwrap() {
                        Device(condition) => {
                            let device = condition.get_device();
                            let sensor_id= device.get_chirpstack_device().device.unwrap().dev_eui;
                            let sensor_number= device.get_chirpstack_device().device.unwrap().application_id;
                            let operator = condition.get_operator();
                            let threshold = condition.get_threshold();

                            // get the number of sensor type
                            let sensor_type_number_index = sensor_id.find("SensorType_").unwrap() + 11;
                            let mut sensor_number_string = sensor_id.clone();
                            sensor_number_string.replace_range(..sensor_type_number_index, "");
                            let sensor_type_number = sensor_number_string.parse::<usize>().unwrap();

                            //let sensor_type = self.parameters.get_sensor_types().get(sensor_type_number).unwrap().clone();
                            let sensor_type = sensor_types.get(sensor_type_number).unwrap();

                            let device_profile_index = self.get_parameters().get_device_profile_index_via_id(sensor_type.get_device_profile_id().as_str()).unwrap();

                            let mut uplink_opt = None;
                            for up in 0..uplinks.len() {
                                if (uplinks[up].0 != device_profile_index) | uplinks[up].1.is_none() {
                                    continue
                                } else {
                                    uplink_opt = uplinks[up].1.clone();
                                }
                            }

                            if uplink_opt.is_none() {
                                continue 'rule
                            }

                            let mut uplink = uplink_opt.unwrap();

                            let data = condition.get_measure_data();
                            let payload = uplink.get_payloads().get(data).unwrap();

                            // get name of measured data and get measured data
                            let action_message:String = sensor_data[sensor_number as usize].2.clone();
                            let mut action_payload = action_message.trim_start_matches("Uplink_Message_").to_string();
                            action_payload = action_payload.trim_end_matches(",").to_string();
                            let name_data_vec: Vec<&str> = action_payload.split(":").collect();
                            let name = name_data_vec[0].to_string();
                            let data = name_data_vec[1].to_string();


                            if payload != &name {
                                continue 'rule
                            }

                            let mut bool_res = false;

                            match threshold {
                                RefValue::String(thresh) => {

                                    let operator_fn = thresh.get_operator(operator).unwrap();

                                    if operator_fn(&data, thresh) {
                                        bool_res = true;
                                    }

                                }

                                RefValue::IntNumber(thresh) => {
                                    let operator_fn = thresh.get_operator(operator).unwrap();

                                    let data_int = data.parse::<i32>().unwrap();

                                    if operator_fn(&data_int, thresh) {
                                        bool_res = true;
                                    }
                                }

                                RefValue::FloatNumber(thresh) => {
                                    let operator_fn = thresh.get_operator(operator).unwrap();

                                    let data_float = data.parse::<f32>().unwrap();

                                    if operator_fn(&data_float, thresh) {
                                        bool_res = true;
                                    }
                                }

                                RefValue::Bool(thresh) => {
                                    let operator_fn = thresh.get_operator(operator).unwrap();

                                    let data_bool = data.parse::<bool>().unwrap();

                                    if operator_fn(&data_bool, thresh) {
                                        bool_res = true;
                                    }
                                }
                                _ => {}
                            }
                            bool_values.push(bool_res);
                            sensor_data[sensor_number as usize].0.push(rule_sim.get_id());
                        },

                        Time(condition) => {
        //                    println!("time condition");
                            let timespan = condition.get_timespan();

                            let start = timespan.get(0).unwrap();
                            let end = timespan.get(1).unwrap();

                            let mut time_vec_bool = Vec::<bool>::new();

                            for time in &time_vec {
                                // the following 28 lines of code are from [elorapi::rules::RuleContainer::start_rule_execution]

                                let mut bool_time = false;

                                // check time
                                // e.g. time: 15:00; range: 12:00-15:00
                                // 12:00 < 15:00 < 16:00
                                let bool_1 = (start < time) & (time < end) & (start < end);

                                // e.g. time: 15:00; range: 23:00-16:00
                                // 23:00 > 15:00 < 16:00
                                let bool_2 = (start > time) & (time < end) & (start > end);

                                // e.g. time: 15:00; range: 14:00-2:00
                                // 14:00 < 15:00 > 2:00
                                let bool_3 = (start < time) & (time > end) & (start > end);

                                if bool_1 | bool_2 | bool_3 {
                                    bool_time = true;
                                }
                                // code from elorapi ends here...................................

                                let bool_res = bool_time;
                                time_vec_bool.push(bool_res);
                            }
                            if time_vec_bool.contains(&true){
                                bool_values.push(true);
                            } else {
                                continue 'rule
                            }
                        },
                    }
                }

                let bool_ops = rule.get_bool_ops();

                let bool_functions = Simulator::parse_bool_ops(bool_ops);

                let bool_values_len = bool_values.len();


                if bool_values_len == 0 {
                    continue 'rule
                }


                let mut bool_result = bool_values[0];


                if bool_values_len > 1 {
                    for bool_res_index in 1..bool_values_len {
                        let operator = bool_functions[bool_res_index-1];

                        bool_result = operator(bool_result, bool_values[bool_res_index]);
                    }
                }

                if !bool_result {
                    continue 'rule
                }

                let mut time = time_vec[0];
                for time_one in time_vec {
                    if time_one > time {
                        time = time_one;
                    }
                }


                let actions = rule.get_action();


                // execute actions
                for action in actions {
                    // dev_eui is where sensor_id is saved -> rubalosim -> rule
                    let sensor_id_string = action.get_device().get_chirpstack_device().device.unwrap().dev_eui;
                    // application is is where sensor_number is save -> rubalosim -> rule
                    let sensor_number = action.get_device().get_chirpstack_device().device.unwrap().application_id;

                    let sensor_type_str_index  = sensor_id_string.find("SensorType_").unwrap();
                    let mut sensor_id_string_copy = sensor_id_string.clone();
                    sensor_id_string_copy.replace_range(..sensor_type_str_index+11,"");
                    let sensor_type_index = sensor_id_string_copy.parse::<usize>().unwrap();
                    let device_payload_indices = action.get_payload_indices();

                    let sensor_type = self.parameters.get_sensor_types().get(sensor_type_index).unwrap().clone();
                    let device_profile_index = self.get_parameters().get_device_profile_index_via_id(sensor_type.get_device_profile_id().as_str()).unwrap();

                    let mut downlink_ops = None;
                    for down in 0..downlinks.len() {
                        if (downlinks[down].0 != device_profile_index) | downlinks[down].1.is_none() {
                            continue
                        } else {
                            downlink_ops = downlinks[down].1.clone();
                        }
                    }
                    let mut downlink = downlink_ops.unwrap();
                    let payloads = downlink.get_payloads();

                    let mut message = "Downlink_Message_command:".to_string();
                    let mut command_names_downlink = Vec::new();
                    for index in device_payload_indices {
                        let payload = payloads.get(*index).unwrap();
                        let command_name = payload.get_command_name();
                        command_names_downlink.push(command_name.clone());
                        message = message + command_name.as_str() + ","
                    }
                    let event_message = Events::Message(message.clone());

          //          let range = between.sample(&mut rng);

                    //let time_calc_end = Local::now().time();
                    //let duration = time_calc_end - time_calc_start;
                    //println!("{}", duration);
                    let new_time = time + Duration::milliseconds(1);
               //     println!("time: {}", time);

                    // create uplink messages
                    let mut uplink_ops = None;
                    for up in 0..uplinks.len() {
                        if (uplinks[up].0 != device_profile_index) | uplinks[up].1.is_none() {
                            continue
                        } else {
                            uplink_ops = uplinks[up].1.clone();
                        }
                    }

                    let id = "Message_of_".to_owned()+ sensor_number.to_string().as_str() + "_" + &sensor_id_string;
                    let event = Event::new(id.clone(), new_time, event_message);

                //    println!("new downlink message created");
                    self.event_list.add_event(event);

                    let mut uplink = uplink_ops.unwrap();

                    let mut uplink_message = "Uplink_Message_".to_string();
                    for payload_index in 0..uplink.get_payloads().len() {
                        uplink_message = uplink_message + uplink.get_payloads().get(payload_index).unwrap() + ":" + command_names_downlink.get(payload_index).unwrap() + ",";
                    }

                    //println!("event_list len {}", self.event_list.events.len());
                    let event_list_len = self.event_list.events.len();
                    // make new
                    'make_new: for event_index_new in event_index+1..event_list_len {

                        let event = self.event_list.events.get(event_index_new).unwrap();
                        let event_time = event.get_time();
            //            println!("make new: {}", event_time);
                        if event.get_action().get_message().contains("Downlink_") {
                            continue 'make_new;
                        }
                        if event.get_id() == id {
                            //println!("actually changed message {}", event.get_id());
                            let action_message = Events::Message(uplink_message.clone());
                            let new_event = Event::new(event.get_id().clone(), event_time, action_message);
                            self.event_list.events[event_index_new] = new_event;
                        }
                    }

                }
            }
            eventlist = self.event_list.events.clone();
            eventlist_len = eventlist.len();
            event_index += 1;
        }
        self.evaluation.set_rule_execution_ended(Local::now());
    }

    /// Parses the string of boolean operators to the actual function.
    /// </br> Parts of this method are taken from [`elorapi::rules::RuleContainer::start_rule_execution`].
    fn parse_bool_ops(bool_ops: &[String]) -> Vec<fn(bool, bool) -> bool> {
        let mut function_vec = Vec::<fn(bool, bool) -> bool>::new();
        for bool_ops in bool_ops {

            // this part is taken from elorapi.
            let function: fn(bool, bool) -> bool = match bool_ops.as_str() {
                "&" => BitAnd::bitand,
                "|" => BitOr::bitor,
                "^" => BitXor::bitxor,
                _ => panic!(),
            };
            function_vec.push(function);
        }
        return function_vec;
    }


    /// Prints event list.
    pub fn print_event_list(&mut self) {
        let event_list = self.event_list.get_event_list();
        for i in event_list.into_iter() {
            println!("{}: {}, {:?}", i.get_time(), i.get_id(), i.get_action());
        }
        println!("Length: {}", event_list.len());
    }

    /// Prints event list for specific movable object.
    pub fn print_event_list_of_movable_object(&mut self, id:String) {
        for i in self.event_list.get_event_list() {
            if i.get_id().contains(&id) {
                println!("{}: {}", i.get_time(), i.get_id());
            }
        }
    }

    /// Prints event list for specific sensor.
    pub fn print_event_list_sensor(&mut self, id:String) {
        for i in self.event_list.get_event_list(){
            if i.get_id().contains(&id) {
                println!("{}: {}, {}", i.get_time(), i.get_id(), i.get_action().to_string());
            }
        }
    }

    /// Writes the event list to the given file. If no file is found a new one is created.
    pub fn write_event_list(&mut self, path:String) -> Result<(), Error> {
        let date = Local::now();
        let path = path + "Event_List_" + date.date_naive().to_string().as_str() + "_" + date.time().hour().to_string().as_str() + "_"+ date.time().minute().to_string().as_str() + "_" + date.time().second().to_string().as_str() +".txt";
        let mut f = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)?;
        for event in self.event_list.get_event_list() {
            let data = "Time: ".to_owned() + event.get_time().to_string().as_str() + ", id: " + event.get_id().as_str() + ", action: " + event.get_action().to_string().as_str()+"\n";
            f.write(data.as_bytes())?;
        }
        Ok(())
    }

    /// Writes the events of the given movable object.
    /// <\br> It is also possible to write all movable obejects.
    pub fn write_events_of_movable_object(&mut self, path:String, id:String) -> Result<(), Error> {
        let date = Local::now();
        let path = path + id.as_str() + date.date_naive().to_string().as_str() + "_" + date.time().hour().to_string().as_str() + "_" + date.time().minute().to_string().as_str() + "_" +date.time().second().to_string().as_str() + ".txt";
        let mut f = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)?;
        for event in self.event_list.get_event_list() {
            if event.get_id().contains(&id) {
                let data = "Time: ".to_owned() + event.get_time().to_string().as_str() + ", id: " + event.get_id().as_str() + ", action: " + event.get_action().to_string().as_str() + "\n";
                f.write(data.as_bytes())?;
            }
        }
        Ok(())
    }

    /// Wirtes downlink-Messages.
    pub fn write_events_downlink_message(&mut self, path:String) -> Result<(), Error> {
        let date = Local::now();
        let path = path + "Downlink_Messsages" + "_" + date.time().hour().to_string().as_str() + "_" + date.time().minute().to_string().as_str() + "_" +date.time().second().to_string().as_str() + ".txt";
        let mut f = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)?;
        for event in self.event_list.get_event_list() {
            if event.get_action().get_message().contains("Downlink_Message_") {
                let data = "Time: ".to_owned() + event.get_time().to_string().as_str() + ", id: " + event.get_id().as_str() + ", action: " + event.get_action().to_string().as_str() + "\n";
                f.write(data.as_bytes())?;
            }
        }
        Ok(())
    }

}

// These lines of code, till the end, are from the elorapi crate.
/// Trait for the selection of an comparison operator for a specific type.
trait Operator<T> {

    /// Gets the comparison operator function for a specific datatype.
    fn get_operator(&self, operator: String) -> Result<fn(&T, &T)->bool, ()>;
}

impl Operator<i32> for i32 {
    fn get_operator(&self, operator: String) -> Result<fn(&i32, &i32) -> bool ,()> {
        let op = match operator.as_str() {
            "<" => <i32 as PartialOrd<i32>>::lt,
            "<=" => <i32 as PartialOrd<i32>>::le,
            ">" => <i32 as PartialOrd<i32>>::gt,
            ">=" => <i32 as PartialOrd<i32>>::ge,
            "==" => <i32 as PartialEq<i32>>::eq,
            "!=" => <i32 as PartialEq<i32>>::ne,
            _ => return Err(())
        };
        return Ok(op);
    }
}

impl Operator<f32> for f32 {
    fn get_operator(&self, operator: String) -> Result<fn(&f32, &f32) -> bool,()> {
        let op = match operator.as_str() {
            "<" => <f32 as PartialOrd<f32>>::lt,
            "<=" => <f32 as PartialOrd<f32>>::le,
            ">" => <f32 as PartialOrd<f32>>::gt,
            ">=" => <f32 as PartialOrd<f32>>::ge,
            "==" => <f32 as PartialEq<f32>>::eq,
            "!=" => <f32 as PartialEq<f32>>::ne,
            _ => return Err(())
        };
        return Ok(op);
    }
}

impl Operator<bool> for bool {
    fn get_operator(&self, operator: String) -> Result<fn(&bool, &bool) -> bool ,()> {
        let op = match operator.as_str() {
            "<" => <bool as PartialOrd<bool>>::lt,
            "<=" => <bool as PartialOrd<bool>>::le,
            ">" => <bool as PartialOrd<bool>>::gt,
            ">=" => <bool as PartialOrd<bool>>::ge,
            "==" => <bool as PartialEq<bool>>::eq,
            "!=" => <bool as PartialEq<bool>>::ne,
            _ => return Err(())
        };
        return Ok(op);
    }
}

impl Operator<String> for String {
    fn get_operator(&self, operator: String) -> Result<fn(&String, &String) -> bool ,()> {
        let op = match operator.as_str() {
            "<" => <String as PartialOrd<String>>::lt,
            "<=" => <String as PartialOrd<String>>::le,
            ">" => <String as PartialOrd<String>>::gt,
            ">=" => <String as PartialOrd<String>>::ge,
            "==" => <String as PartialEq<String>>::eq,
            "!=" => <String as PartialEq<String>>::ne,
            _ => return Err(())
        };
        return Ok(op);
    }
}
