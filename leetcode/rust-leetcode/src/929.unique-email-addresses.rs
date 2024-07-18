use std::collections::HashSet;
impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut set:HashSet<String> = HashSet::new();
        for email in emails.iter() {
            match email.split_once('@') {
                Some((addr, tld)) => {
                    match addr.split_once('+') {
                        Some((act_addr, _)) => {
                            let act_addr = act_addr.replace(".", "");
                            set.insert(format!("{}_{}", act_addr, tld));
                        },
                        d @ _ => {
                            let addr = addr.replace(".", "");
                            set.insert(format!("{}_{}", addr, tld));
                        }
                    }
                }
                _ => {},
            }
        }
        set.len() as _
    }
}

