pub struct LogQuery<'a> {
    pub logs: &'a Vec<String>,
}

// 1. Finish the implementation of LogQuery
impl<'a> LogQuery<'a> {
    pub fn new(vec_of_strings: &'a Vec<String>) -> LogQuery<'a> {
        LogQuery{logs: vec_of_strings}
    }
    pub fn search(&self, string_slice: &str) -> Vec<&'a String> {
        self.logs.iter()
            .filter(|log| log.contains(string_slice))
            .collect()
    }
}
// 2. Create a public associated function named `new()` that will take a reference to a vector of strings
//
// 3. Create a public method named `search` that accepts a string slice and finds it from the logs and
//    returns a vector of references to those logs.

fn main() {
    let logs = vec![
        "INFO: System started".to_string(),
        "ERROR: Disk full".to_string(),
        "WARNING: Low memory".to_string(),
        "INFO: User logged in".to_string(),
    ];

    let log_query = LogQuery::new(&logs);

    let search_results = log_query.search("ERROR");
    for result in search_results {
        println!("{}", result);
    }
}