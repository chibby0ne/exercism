use std::collections::HashMap;

#[derive(Default)]
pub struct School {
    roster: HashMap<String, u32>,
}

impl School {
    pub fn new() -> School {
        School {
            roster: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.roster.insert(student.to_owned(), grade);
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut v = self.roster.values().copied().collect::<Vec<u32>>();
        v.dedup_by_key(|m| *m);
        v
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        if self.roster.is_empty() || !self.roster.values().any(|&g| g == grade) {
            return None;
        }
        let mut vec = self
            .roster
            .keys()
            .filter(|&n| self.roster.get(n) == Some(&grade))
            .map(|n| n.to_owned())
            .collect::<Vec<String>>();
        vec.sort_unstable();
        Some(vec)
    }
}
