use std::marker::PhantomData;
pub struct Empty;
pub struct Ready;
pub struct Flying;
pub trait State {
    fn status() -> &'static str;
}

// TODO: Define the `status` method for all states
pub struct Sleigh<T:State> {
    // This is only public for testing purposes
    // In real-world scenarios, this should be private
    pub state: PhantomData<T>,
}

impl <T:State> Sleigh<T> {
    pub fn status(&self) -> &'static str {
        T::status()
    }
}

impl State for Empty {
    fn status() -> &'static str {
        "Empty"
    }
}

impl Sleigh<Empty> {
    pub fn new() -> Self {
        Self { state: PhantomData }
    }
    pub fn load(self) -> Sleigh<Ready> {
        Sleigh { state: PhantomData }
    }
}

impl State for Ready {
    fn status() -> &'static str {
        "Ready"
    }  
}

impl Sleigh<Ready> {
    pub fn take_off(self) -> Sleigh<Flying> {
        Sleigh { state: PhantomData }
    }
    pub fn unload(self) -> Sleigh<Empty> {
        Sleigh { state: PhantomData }
    }
}

impl State for Flying {
    fn status() -> &'static str {
        "Flying"
    }
}

impl Sleigh<Flying> {
    pub fn land(self) -> Sleigh<Ready> {
        Sleigh { state: PhantomData }
    }
}

fn main() {
    let sleigh = Sleigh::<Empty>::new();
    println!("Sleigh status: {}", sleigh.status());

    let sleigh = sleigh.load();
    println!("Sleigh status: {}", sleigh.status());

    let sleigh = sleigh.take_off();
    println!("Sleigh status: {}", sleigh.status());

    let sleigh = sleigh.land();
    println!("Sleigh status: {}", sleigh.status());

    let sleigh = sleigh.unload();
    println!("Sleigh status: {}", sleigh.status());
}