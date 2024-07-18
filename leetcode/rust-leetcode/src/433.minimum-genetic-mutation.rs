use std::collections::{HashMap,HashSet,VecDeque};
impl Solution {
    pub fn min_mutation(start_gene: String, end_gene: String, bank: Vec<String>) -> i32 {
        if start_gene == end_gene {
            return 0;
        }
        let bases = [b'A', b'C', b'G', b'T'];
        let sg = start_gene.into_bytes();
        let eg = end_gene.into_bytes();
        let bank:HashSet<Vec<u8>> = bank.into_iter().map(|g| g.into_bytes()).collect();
        let mut visited = HashSet::with_capacity(bank.len()*2);
        let mut q = VecDeque::new();
        q.push_back(sg);
        let mut mutations = 0;
        let mut temp = b'a';
        while !q.is_empty() {
            for _ in 0..q.len() {
                if let Some(mut gene) = q.pop_front() {
                    if gene == eg {
                        return mutations;
                    }
                    if visited.contains(&gene) {
                        continue;
                    }
                    visited.insert(gene.to_vec());
                    // let mut new_gene = gene.to_vec();
                    for i in 0..gene.len() {
                        // if gene[i] == eg[i] {
                        //     continue;
                        // }
                        let orig = gene[i];
                        for j in 0..4 {
                            if orig != bases[j] {
                                temp = gene[i];
                                gene[i] = bases[j];
                                // println!("{i} {} {:?} {}", gene[i], gene, bank.contains(&gene));
                                if !visited.contains(&gene) && bank.contains(&gene) {
                                    q.push_back(gene.to_vec());
                                }
                                gene[i] = temp;
                            }
                        }
                    }
                }
            }
            // println!("{:?}", q);
            mutations += 1;
        }
        -1
    }
}


// LATER
// See this test case
// startGene =
// "AACCGGTT"
// endGene = 
// "AACCGGTA"
// bank =
// ["AAACGGTT","AACCGCTA","AAACGGTA"]
//
