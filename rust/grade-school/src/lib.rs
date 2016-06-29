pub struct School {
    students: Vec<Student>,
}

pub struct Student {
    grade: u16,
    name: String,
}

impl School {
    pub fn new() -> School {
        School{
            students: vec![],
        }
    }

    pub fn grades(&self) -> Vec<u16> {
        let mut grades : Vec<u16> = self.students.iter()
            .map(|s| s.grade )
            .collect();
        grades.sort();
        grades.dedup();
        grades
    }

    pub fn add(&mut self, grade: u16, name: &str) {
        self.students.push(Student{
            grade: grade,
            name: name.to_string(),
        })
    }

    pub fn grade(&self, grade: u16) -> Option<Vec<String>> {
        let names : Vec<String> = self.students.iter()
            .filter(|s| s.grade == grade)
            .map(|s| s.name.clone())
            .collect();

        if names.len() == 0 {
            None
        }
        else {
            Some(names)
        }
    }
}
