use std::fs::File;
use std::io::Write;

pub struct LogQuery<'a> {
    logs: &'a Vec<String>,
}

impl<'a> LogQuery<'a> {
    pub fn new(logs: &'a Vec<String>) -> Self {
        LogQuery { logs }
    }

    pub fn search(&self, keyword: &str) -> Vec<&'a String> {
        self.logs
            .iter()
            .filter(|log| log.contains(keyword))
            .collect()
    }

    pub fn export_to_file(&self, keyword: &str, path: &str) -> std::io::Result<()> {
        // 🎁 Your code here! 🎁
        let filtered_logs = self.search(keyword);
        let mut file = File::create(path)?;

        for log in filtered_logs {
            writeln!(file, "{}", log)?;
        }

        Ok(())
    }
}

fn main() {
    let logs = vec![
        String::from("Error: Failed to connect to the server"),
        String::from("Info: User logged in"),
        String::from("Warning: Disk space is low"),
        String::from("Error: Database connection lost"),
    ];

    let log_query = LogQuery::new(&logs);

    // Example usage
    if let Err(e) = log_query.export_to_file("Error", "errors.log") {
        eprintln!("Failed to export logs: {}", e);
    }
}