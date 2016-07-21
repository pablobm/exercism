use std::collections::HashMap;

pub struct School {
    students: HashMap<u16, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            students: HashMap::new(),
        }
    }

    pub fn grades(&self) -> Vec<u16> {
        let mut grades : Vec<u16> = self.students.keys()
            .cloned()
            .collect();
        grades.sort();
        grades
    }

    pub fn add(&mut self, grade: u16, name: &str) {
        let mut names = self.students.entry(grade).or_insert(vec![]);
        names.push(name.to_string());
        names.sort();
    }

    pub fn grade(&self, grade: u16) -> Option<&Vec<String>> {
        self.students.get(&grade)
    }
}
