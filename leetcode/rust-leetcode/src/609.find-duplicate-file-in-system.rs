use std::collections::HashMap;
impl Solution {
    pub fn find_duplicate(paths: Vec<String>) -> Vec<Vec<String>> {
        let mut map = HashMap::new();
        for path in paths.iter() {
            let path_files = path.split_whitespace().collect::<Vec<&str>>();
            let dir = path_files[0];
            for file_content in path_files.iter().skip(1) {
                match file_content.split_once('(') {
                    None => {},
                    Some((file_name, content)) => {
                        let file_path = format!("{dir}/{file_name}");
                        map.entry(content).or_insert(Vec::new()).push(file_path);
                    }
                }
            }
        }
        let mut ans = vec![];
        for (content, file_paths) in map.into_iter() {
            if file_paths.len() > 1 {
                ans.push(file_paths.to_vec());
            }
        }
        ans
    }
}

