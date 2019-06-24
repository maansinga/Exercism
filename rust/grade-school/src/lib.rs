use std::collections::HashMap;

pub struct School {
    table: HashMap<String, u32>
}

impl School {
    pub fn new() -> School {
        School { table: HashMap::new() }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        *self.table.entry(student.to_string()).or_insert(grade) = grade;
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut result = self
            .table
            .values()
            .fold(vec![], |mut a, c| {
                if a.contains(c) {
                    a
                } else {
                    a.push(c.clone());
                    a
                }
            });

        result.sort();
        result
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        let mut result: Vec<String> = self
            .table
            .iter()
            .filter(|x| *x.1 == grade)
            .map(|x| x.0.clone())
            .collect();
        if result.is_empty() {
            None
        } else {
//            result.sort_by(|x, y| x.cmp(y));
            result.sort();
            Some(result)
        }
    }
}
