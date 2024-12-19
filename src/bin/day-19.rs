use std::marker::PhantomData;

// Define the states as empty structs
pub struct Empty;
pub struct Ready;
pub struct Flying;

// Define the Sleigh struct to hold the state
pub struct Sleigh<State> {
    _state: PhantomData<State>,
}

// Implement methods for the Sleigh struct
impl Sleigh<Empty> {
    // Create a new Sleigh in the Empty state
    pub fn new() -> Self {
        Sleigh {
            _state: PhantomData,
        }
    }

    // Transition the Sleigh from Empty to Ready
    pub fn load(self) -> Sleigh<Ready> {
        Sleigh {
            _state: PhantomData,
        }
    }
}

impl Sleigh<Ready> {
    // Transition the Sleigh from Ready to Flying
    pub fn take_off(self) -> Sleigh<Flying> {
        Sleigh {
            _state: PhantomData,
        }
    }

    // Transition the Sleigh from Ready to Empty
    pub fn unload(self) -> Sleigh<Empty> {
        Sleigh {
            _state: PhantomData,
        }
    }
}

impl Sleigh<Flying> {
    // Transition the Sleigh from Flying to Ready
    pub fn land(self) -> Sleigh<Ready> {
        Sleigh {
            _state: PhantomData,
        }
    }
}

fn main() {
    let sleigh = Sleigh::<Empty>::new();
    let sleigh = sleigh.load();
    let sleigh = sleigh.take_off();
    let sleigh = sleigh.land();
    let _sleigh = sleigh.unload();
}
