use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap};

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd)]
struct Food {
    name    : String,
    cuisine : String,
    rating  : i32,
}
impl Food {
    fn new(name: &str, cuisine: &str, rating: i32) -> Self {
        Self {
            name    : name.into(),
            cuisine : cuisine.into(),
            rating,
        }
    }
}
impl Ord for Food {
    fn cmp(&self, other: &Self) -> Ordering {
        match other.rating.cmp(&self.rating) {
            Ordering::Equal => self.name.cmp(&other.name),
            ord => ord,
        }
    }
}

struct FoodRatings {
    cuisines : HashMap<String, BTreeSet<Food>>,
    foods    : HashMap<String, Food>,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FoodRatings {

    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) 
        -> Self 
    {
        let mut cuisines_map = HashMap::new();
        let mut foods_map    = HashMap::new();

        let foods_iter = foods.into_iter().zip(cuisines).zip(ratings);

        for ((food, cuisine), rating) in foods_iter {
            let food = Food::new(&food, &cuisine, rating);
            cuisines_map.entry(cuisine.clone())
                        .or_insert_with(BTreeSet::new)
                        .insert(food.clone());
            foods_map.insert(food.name.clone(), food);
        }
        Self { cuisines: cuisines_map, foods: foods_map }
    }
    
    fn change_rating(&mut self, food: String, new_rating: i32) {
        let food = self.foods.get_mut(&food).unwrap();
        let list = self.cuisines.get_mut(&food.cuisine).unwrap();
        list.remove(food);
        food.rating = new_rating;
        list.insert(food.clone());
    }
    
    fn highest_rated(&self, cuisine: String) -> String {
        self.cuisines.get(&cuisine).unwrap()
                     .iter().next().unwrap().name.clone()
    }
}

/**
 * Your FoodRatings object will be instantiated and called as such:
 * let obj = FoodRatings::new(foods, cuisines, ratings);
 * obj.change_rating(food, newRating);
 * let ret_2: String = obj.highest_rated(cuisine);
 */


/* */

use std::collections::{HashMap, BinaryHeap};
use std::cmp::Reverse;

#[derive(Default)]
struct FoodRatings {
    // food => (cuisine, rating)
    foods: HashMap<String, (String, i32)>,
    // cuisine => Heap<(rating, food)>
    cuisine_ratings: HashMap<String, BinaryHeap<(i32, Reverse<String>)>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FoodRatings {
    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let mut new = Self::default();
        for ((food, cuisine), rating) in foods.into_iter().zip(cuisines).zip(ratings) {
            new.foods.insert(food.clone(), (cuisine.clone(), rating));
            new.cuisine_ratings.entry(cuisine).or_default().push((rating, Reverse(food)));
        }
        new
    }
    
    fn change_rating(&mut self, food: String, new_rating: i32) {
        let Some((cuisine, rating)) = self.foods.get_mut(&food) else {
            return;
        };

        *rating = new_rating;

        let Some(q) = self.cuisine_ratings.get_mut(cuisine.as_str()) else {
            return;
        };

        q.push((new_rating, Reverse(food)));
    }
    
    fn highest_rated(&mut self, cuisine: String) -> String {
        let q = self.cuisine_ratings.get_mut(&cuisine).unwrap();
        while let Some(top) = q.peek_mut() {
            let (rating, Reverse(food)) = &*top;
            if self.foods[food].1 == *rating {
                return food.clone();
            }

            std::collections::binary_heap::PeekMut::pop(top);
        }

        panic!()
    }
}

/**
 * Your FoodRatings object will be instantiated and called as such:
 * let obj = FoodRatings::new(foods, cuisines, ratings);
 * obj.change_rating(food, newRating);
 * let ret_2: String = obj.highest_rated(cuisine);
 */

