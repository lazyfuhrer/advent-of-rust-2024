use std::collections::HashMap;

pub struct SantaList {
    records: HashMap<String, bool>
}

impl SantaList {
   
    pub fn new() -> Self {
        Self {records: HashMap::new()}
    }

    pub fn add(&mut self, key: &str, val: bool) {
        self.records.insert(key.to_string(), val);
    }

    pub fn remove(&mut self, key: &str) {
        self.records.remove(key);
    }

    pub fn get(&self, key: &str) -> Option<bool> {
        self.records.get(key).copied()
    }

    pub fn count(&self) -> (usize, usize) {
        let nice_count = self.records.values().filter(|&&val| val).count();
        let naughty_count = self.records.values().filter(|&&val| !val).count();
        (nice_count, naughty_count)
    }

    pub fn list_by_behavior(&self, nice: bool) -> Vec<String> {
        self.records
            .iter()
            .filter(|&(_, &val)| val == nice)
            .map(|(key, _)| key.clone())
            .collect()
    }
}

pub fn main() {
    let mut santa_list = SantaList::new();

    santa_list.add("Alice", true);
    santa_list.add("Bob", false);
    santa_list.add("Charlie", true);

    if let Some(behavior) = santa_list.get("Alice") {
        println!(
            "Alice is on the {} list.",
            if behavior { "Nice" } else { "Naughty" }
        );
    }

    let (nice, naughty) = santa_list.count();
    println!(
        "Santa's list contains {} nice and {} naughty children.",
        nice, naughty
    );

    let nice_list = santa_list.list_by_behavior(true);
    println!("Nice children: {:?}", nice_list);

    let naughty_list = santa_list.list_by_behavior(false);
    println!("Naughty children: {:?}", naughty_list);

    santa_list.remove("Bob");
    let (nice, naughty) = santa_list.count();
    println!(
        "After removing Bob, Santa's list contains {} nice and {} naughty children.",
        nice, naughty
    );
}
