impl Solution {
    pub fn maximum_units(mut box_types: Vec<Vec<i32>>, mut truck_size: i32) -> i32 {
        box_types.sort_by_key(|bt| -bt[1]);
        let mut max_units = 0;
        let mut i = 0;
        while truck_size > 0 && i < box_types.len() {
            let box_type = &box_types[i];
            let package = truck_size.min(box_type[0]);
            max_units += box_type[1] * package;
            truck_size -= package;
            i += 1;
        }
        max_units
    }
}

