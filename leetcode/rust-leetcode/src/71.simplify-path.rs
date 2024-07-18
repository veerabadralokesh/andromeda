impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut paths = path.split("/").collect::<Vec<&str>>();
        // println!("{:?}", paths);
        let mut ans:Vec<&str> = Vec::new();
        ans.push(paths[0]);
        for &p in paths.iter().skip(1) {
            if p == ".." {
                if ans.len() > 1 {
                    ans.pop();
                }
                continue;
            } else if p == "" || p == "." {
                continue;
            } else {
                ans.push(p);
            }
        }
        if ans.len() == 1 { return "/".to_string(); }
        ans.join("/")
    }
}
