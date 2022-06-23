use std::collections::HashMap;

pub fn add_people_to_department(
    company: &mut HashMap<String, Vec<String>>,
    department: String,
    person: String,
) {
    company
        .entry(department)
        .or_insert_with(Vec::new)
        .push(person);
}

pub fn get_people_from_department(
    company: &mut HashMap<String, Vec<String>>,
    department: String,
) -> Option<&Vec<String>> {
    company.get(&department)
}
