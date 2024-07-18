impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let parse_version = |v: &String| v.split('.').collect::<Vec<&str>>().iter().map(|n| n.parse::<i32>().unwrap()).collect::<Vec<_>>();
        let v1 = parse_version(&version1);
        let v2 = parse_version(&version2);
        // println!("{:?}", v1);
        // println!("{:?}", v2);
        let mut i = 0;
        while i < (v1.len()).min(v2.len())  {
            if v1[i] > v2[i] {
                return 1;
            } else if v1[i] < v2[i] {
                return -1;
            }
            i += 1;
        }
        if i < v1.len() {
            for j in i..v1.len() {
                if v1[j] != 0 {
                    return 1;
                }
            }
            return 0;
        } else if i < v2.len() {
            for j in i..v2.len() {
                if v2[j] != 0 {
                    return -1;
                }
            }
            return 0;
        }
        0
    }
}
