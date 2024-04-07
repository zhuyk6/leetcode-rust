use std::collections::{HashMap, HashSet};

struct ThroneInheritance {
    king: String,
    sons: HashMap<String, Vec<String>>,
    death_set: HashSet<String>,
}

#[allow(unused)]
impl ThroneInheritance {
    fn new(king_name: String) -> Self {
        ThroneInheritance {
            king: king_name.clone(),
            sons: HashMap::from([(king_name, vec![])]),
            death_set: HashSet::new(),
        }
    }

    fn birth(&mut self, parent_name: String, child_name: String) {
        self.sons.entry(parent_name).or_default().push(child_name);
    }

    fn death(&mut self, name: String) {
        self.death_set.insert(name);
    }

    fn dfs(&self, x: &str, que: &mut Vec<String>) {
        if !self.death_set.contains(x) {
            que.push(x.to_string());
        }
        if let Some(sons) = self.sons.get(x) {
            for y in sons {
                self.dfs(y, que);
            }
        }
    }

    fn get_inheritance_order(&self) -> Vec<String> {
        let mut ans = vec![];
        self.dfs(&self.king, &mut ans);
        ans
    }
}
