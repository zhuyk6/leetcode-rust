use std::collections::{HashMap, HashSet};

pub struct ThroneInheritance {
    king: String,
    sons: HashMap<String, Vec<String>>,
    death_set: HashSet<String>,
}

impl ThroneInheritance {
    pub fn new(king_name: String) -> Self {
        ThroneInheritance {
            king: king_name.clone(),
            sons: HashMap::from([(king_name, vec![])]),
            death_set: HashSet::new(),
        }
    }

    pub fn birth(&mut self, parent_name: String, child_name: String) {
        self.sons.entry(parent_name).or_default().push(child_name);
    }

    pub fn death(&mut self, name: String) {
        self.death_set.insert(name);
    }

    pub fn dfs(&self, x: &str, que: &mut Vec<String>) {
        if !self.death_set.contains(x) {
            que.push(x.to_string());
        }
        if let Some(sons) = self.sons.get(x) {
            for y in sons {
                self.dfs(y, que);
            }
        }
    }

    pub fn get_inheritance_order(&self) -> Vec<String> {
        let mut ans = vec![];
        self.dfs(&self.king, &mut ans);
        ans
    }
}
