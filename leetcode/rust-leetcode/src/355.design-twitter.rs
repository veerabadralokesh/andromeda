use std::collections::{HashMap,HashSet};
struct Twitter {
    follows: HashMap<i32,HashSet<i32>>,
    tweets: HashMap<i32,Vec<(i32, i32)>>,
    counter: i32,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Twitter {

    fn new() -> Self {
        Twitter {
            follows: HashMap::new(),
            tweets: HashMap::new(),
            counter: 0i32,
        }
    }
    
    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.tweets.entry(user_id).or_insert(Vec::new()).push((self.counter, tweet_id));
        self.counter += 1;
    }
    
    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        let mut feed = match (self.tweets.get(&user_id)) {
            None => Vec::new(),
            Some(feed) => feed.to_vec(),
        };
        let following = self.follows.get(&user_id);
        match (following) {
            None => (),
            Some(set) => {
                for user in set {
                    // println!("following {:?}", user);
                    match (self.tweets.get(&user)) {
                        None => {},
                        Some(tweets) => {
                            let n = tweets.len();
                            // println!("{:?}", tweets[(n-10).max(0)..]);
                            feed.extend(tweets[(((n as i32)-10).max(0) as usize)..n].to_vec());
                        }
                    }
                }
            }
        }
        feed.sort_by_key(|t| t.0);
        feed.reverse();
        feed[..(10.min(feed.len()))].to_vec().iter().map(|t| t.1).collect()
    }
    
    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        self.follows.entry(follower_id).or_insert(HashSet::new()).insert(followee_id);
    }
    
    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        self.follows.entry(follower_id).or_insert(HashSet::new()).remove(&followee_id);
    }
}

/**
 * Your Twitter object will be instantiated and called as such:
 * let obj = Twitter::new();
 * obj.post_tweet(userId, tweetId);
 * let ret_2: Vec<i32> = obj.get_news_feed(userId);
 * obj.follow(followerId, followeeId);
 * obj.unfollow(followerId, followeeId);
 */
