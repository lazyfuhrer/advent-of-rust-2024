use std::fmt::{self, Display, Formatter};

pub struct KidsGift {
    pub name: String,
}

pub struct ElvesGift {
    pub name: String,
}

pub struct ReindeerGift {
    pub name: String,
}

impl Display for KidsGift {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "KidsGift: {}", self.name)
    }
}

impl Display for ElvesGift {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ElvesGift: {}", self.name)
    }
}

impl Display for ReindeerGift {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ReindeerGift: {}", self.name)
    }
}

pub fn display_gift<T: Display>(gift: T) {
    println!("{}", gift);
}

pub fn main() {
    let kids_gift = KidsGift {
        name: "toy car".to_string(),
    };
    let elves_gift = ElvesGift {
        name: "vertical monitor".to_string(),
    };
    let reindeer_gift = ReindeerGift {
        name: "carrot".to_string(),
    };

    display_gift(&kids_gift);
    display_gift(&elves_gift);
    display_gift(&reindeer_gift);
}
