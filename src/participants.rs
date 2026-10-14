use std::collections::HashSet;

pub fn unique_count(names: &[String]) -> usize {
    let unique_names: HashSet<&String> = names.iter().collect();
    unique_names.len()
}
