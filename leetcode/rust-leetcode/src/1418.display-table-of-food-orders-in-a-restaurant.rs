use std::collections::{HashMap,HashSet};
impl Solution {
    pub fn display_table(orders: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut tables:HashMap<String, HashMap<String, i32>> = HashMap::new();
        let mut items = HashSet::new();
        for order in &orders {
            let table = &order[1];
            let item = &order[2];
            items.insert(item.to_string());
            *((tables.entry(table.to_string()).or_insert(HashMap::new())).entry(item.to_string()).or_insert(0)) += 1;
        }
        let mut items = items.into_iter().collect::<Vec<String>>();
        items.sort();
        let mut table_numbers = tables.keys().collect::<Vec<_>>();
        table_numbers.sort_by_key(|n| n.parse::<i32>().unwrap());
        let mut ans:Vec<Vec<String>> = Vec::with_capacity(table_numbers.len() + 1);
        for _ in 0..=table_numbers.len() {ans.push(Vec::with_capacity(items.len()+1))};
        ans[0].push("Table".to_string());
        ans[0].extend(items.to_vec());
        for (i, table) in table_numbers.iter().enumerate() {
            ans[i+1].push(table.to_string());
            for item in &items {
                ans[i+1].push(tables.get(*table).unwrap().get(item).unwrap_or(&0).to_string());
            }
        }
        ans
    }
}


/* */

// LEARN

use std::collections::{HashMap, BTreeMap, BTreeSet};
use std::str::FromStr;
use std::iter;

impl Solution {
    pub fn display_table(orders: Vec<Vec<String>>) -> Vec<Vec<String>> {
        // Use BTreeMap because traversal order is important
        let mut foods = BTreeMap::new();
        let mut tables = BTreeSet::new();
        for (table, food) in orders
            .into_iter()
            .map(|mut v| (i32::from_str(&v[1]).unwrap(), v.pop().unwrap()))
        {
            // Store tables to iterate later
            tables.insert(table);
            // Count how many at a table ordered a certain food
            *foods
                .entry(food)
                .or_insert(HashMap::new())
                .entry(table)
                .or_insert(0) += 1;
        }
        // We know how long the Vec is going to be, so may as well preallocate
        let mut ans = Vec::with_capacity(tables.len() + 1);
        ans.push(
            // Start with "Table" and add all the other foods
            iter::once("Table".to_owned())
                .chain(foods.keys().cloned())
                .collect(),
        );
        ans.extend(tables.into_iter().map(|t| {
            // Start with the table number and get the number of orders for each food from that table
            iter::once(t.to_string())
                .chain(foods.values().map(|f| f.get(&t).unwrap_or(&0).to_string()))
                .collect()
        }));
        ans
    }
}