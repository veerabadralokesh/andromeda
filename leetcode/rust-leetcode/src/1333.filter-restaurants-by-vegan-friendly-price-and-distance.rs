impl Solution {
    pub fn filter_restaurants(restaurants: Vec<Vec<i32>>, vegan_friendly: i32, max_price: i32, max_distance: i32) -> Vec<i32> {
        let mut filtered = restaurants.iter().filter(|r| {
            r[2] * vegan_friendly == vegan_friendly && r[3] <= max_price && r[4] <= max_distance
        }).cloned().collect::<Vec<_>>();
        filtered.sort_by_key(|f| (-f[1], -f[0]));
        filtered.iter().map(|f| f[0]).collect::<Vec<_>>()
    }
}

