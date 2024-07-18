use std::collections::{HashMap,HashSet};
impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut email_graph = HashMap::new();
        let mut emails = HashSet::new();
        let mut email_names = HashMap::new();
        for account in accounts.iter() {
            email_names.insert(account[1].clone(), account[0].clone());
            emails.insert(account[1].clone());
            for i in 1..account.len()-1 {
                for j in i+1..account.len() {
                    email_graph.entry(account[i].clone()).or_insert(HashSet::new()).insert(account[j].clone());
                    email_graph.entry(account[j].clone()).or_insert(HashSet::new()).insert(account[i].clone());
                }
            }
        }
        let mut visited = HashSet::new();
        fn dfs(email: String, graph: &HashMap<String, HashSet<String>>, visited: &mut HashSet<String>, name: String) -> Vec<String> {
            let mut email_set = HashSet::new();
            let mut q = Vec::new();
            q.push(email);
            while let Some(em) = q.pop() {
                email_set.insert(em.to_string());
                match graph.get(&em) {
                    None => {},
                    Some(set) => {
                        for cem in set.into_iter() {
                            if !visited.contains(cem) {
                                visited.insert(cem.clone());
                                q.push(cem.clone());
                            }
                        }
                    }
                }
            }
            let mut emails:Vec<String> = email_set.into_iter().collect();
            emails.sort();
            emails.insert(0, name);
            emails
        }

        let mut ans = Vec::new();

        for email in emails.into_iter() {
            if !visited.contains(&email) {
                let name = email_names.get(&email).unwrap();
                let all_emails = dfs(email, &email_graph, &mut visited, name.to_string());
                ans.push(all_emails);
            }
        }

        ans
    }
}

/* */

struct UF {
    parent: Vec<usize>,
    size: Vec<u32>
}

impl UF {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n]
        }
    }

    fn union(&mut self, x: usize, y: usize) {
        let x = self.find(x);
        let y = self.find(y);
        if self.size[x] < self.size[y] {
            self.parent[x] = self.parent[y];
            self.size[y] += self.size[x];
        } else {
            self.parent[y] = self.parent[x];
            self.size[x] += self.size[y];
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if x != self.parent[x] {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }
}

use std::collections::HashMap;

impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut hm = HashMap::new();
        let mut uf = UF::new(accounts.len());
        for (i, account) in accounts.iter().enumerate() {
            for email in account.iter().skip(1) {
                if let Some(&j) = hm.get(email) {
                    uf.union(i, j);
                } else {
                    hm.insert(email.to_owned(), i);
                }
            }
        }

        let mut components = HashMap::new();
        for (email, i) in hm {
            components
                .entry(uf.find(i))
                .or_insert(Vec::new())
                .push(email);
        }

        let mut answer = vec![];
        for (i, mut emails) in components {
            let mut merged = vec![accounts[i][0].clone()];
            emails.sort_unstable();
            merged.extend(emails);
            answer.push(merged);
        }
        answer
    }
}


