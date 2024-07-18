use std::collections::HashMap;
impl Solution {
    pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
        let mut map:HashMap<String, i32> = HashMap::new();
        for cpdomain in cpdomains {
            let (count, mut domain) = cpdomain.split_once(' ').unwrap();
            let count = count.parse::<i32>().unwrap();
            *map.entry(domain.to_string()).or_insert(0) += count;
            while let Some((subdomain, rdomain)) = domain.split_once('.') {
                domain = rdomain;
                *map.entry(rdomain.to_string()).or_insert(0) += count;
            }
        }
        let mut cpsubdomains:Vec<String> = Vec::new();
        for (k, v) in map.into_iter() {
            cpsubdomains.push(format!("{v} {k}"));
        }
        cpsubdomains
    }
}
