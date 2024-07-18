impl Solution {
    pub fn reformat_date(date: String) -> String {
        let months = ["", "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
        let date = date.split(' ').collect::<Vec<_>>();
        let day = match date[0].len() {
            3 => date[0].chars().take(1).collect::<String>(),
            _ => date[0].chars().take(2).collect::<String>(),
        };
        let month = months.iter().position(|&r| r == date[1]).unwrap();
        format!("{}-{:0>2}-{:0>2}", date[2], month, day)
    }
}

