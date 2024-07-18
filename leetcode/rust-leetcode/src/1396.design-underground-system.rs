use std::collections::HashMap;
struct UndergroundSystem {
    customers: HashMap<i32, (String, i32)>,
    stations: HashMap<String, HashMap<String, Vec<i32>>>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl UndergroundSystem {

    fn new() -> Self {
        UndergroundSystem {
            customers: HashMap::new(),
            stations: HashMap::new(),
        }
    }
    
    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.customers.insert(id, (station_name, t));
    }
    
    fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        let (start, start_time) = self.customers.get(&id).unwrap();
        self.stations.entry(station_name).or_insert(HashMap::new())
            .entry(start.to_string()).or_insert(Vec::new()).push((t-start_time));
        self.customers.remove(&id);
    }
    
    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        let v = self.stations.get(&end_station).unwrap().get(&start_station).unwrap();
        (v.iter().sum::<i32>() as f64) / (v.len() as f64)
    }
}

/**
 * Your UndergroundSystem object will be instantiated and called as such:
 * let obj = UndergroundSystem::new();
 * obj.check_in(id, stationName, t);
 * obj.check_out(id, stationName, t);
 * let ret_3: f64 = obj.get_average_time(startStation, endStation);
 */

/* */

// LEARN

use std::{collections::{hash_map, BTreeMap, HashMap}};

struct UndergroundSystem {
  progress : HashMap<i32, (String, i32)>,
  station : HashMap<(String, String), Vec<i32>>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl UndergroundSystem {

  fn new() -> Self {
    UndergroundSystem {
      progress : HashMap::new(),
      station: HashMap::new()
    }
  }
  
  fn check_in(&mut self, id: i32, station_name: String, t: i32) {
    self.progress.insert(id, (station_name, t));
  }
  
  fn check_out(&mut self, id: i32, station_name: String, t: i32) {
    let (station_in, t_in) = self.progress.remove(&id).unwrap();
    let t_diff = t - t_in;
    self.station.entry((station_in, station_name))
      .and_modify(|v| v.push(t_diff))
      .or_insert(vec![t_diff]);
  }
  
  fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
    match self.station.get(&(start_station, end_station))  {
      None => 0.0,
      Some(v) => (v.iter().sum::<i32>() as f64) / (v.len() as f64)
    }
  }
}

/**
 * Your UndergroundSystem object will be instantiated and called as such:
 * let obj = UndergroundSystem::new();
 * obj.check_in(id, stationName, t);
 * obj.check_out(id, stationName, t);
 * let ret_3: f64 = obj.get_average_time(startStation, endStation);
 */
