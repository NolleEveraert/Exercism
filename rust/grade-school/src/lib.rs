use std::collections::{HashMap, HashSet};

pub struct School<'a> {
    roster: HashMap<u32, HashSet<&'a str>>,
}

impl<'a> School<'a> {
    pub fn new() -> School<'a> {
        School { roster: HashMap::new() }
    }

    pub fn add(&mut self, grade: u32, student: &'a str) {
        if self.roster.values().any(|g| g.contains(&student)) {
            return;
        }

        if let Some(students) = self.roster.get_mut(&grade) {
            students.insert(student);
        } else {
            let mut set: HashSet<&'a str> = HashSet::new();
            set.insert(student);
            self.roster.insert(grade, set);
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades: Vec<u32> = self.roster.keys().cloned().collect();
        grades.sort();
        grades
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        if let Some(students) = self.roster.get(&grade) {
            let mut result: Vec<String> = students.iter().map(|s| s.to_string()).collect();
            result.sort();
            result
        } else {
            return Vec::<String>::new();
        }
    }
}
