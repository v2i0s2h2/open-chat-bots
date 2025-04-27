// Serde library ka use karte hain serialization aur deserialization ke liye
use serde::{Deserialize, Serialize};
// RefCell ka use karte hain mutable access ke liye immutable context mein
use std::cell::RefCell;

// Thread local storage define karte hain jisme STATE variable store hoga
// Ye ensure karta hai ki har thread ka apna STATE instance ho
thread_local! {
    // STATE ek RefCell hai jo Option<State> ko wrap karta hai
    // RefCell runtime pe borrowing rules enforce karta hai
    // Option<State> ka matlab hai ki STATE null bhi ho sakta hai initially
    static STATE: RefCell<Option<State>> = RefCell::default();
}

// State struct ko serialize aur deserialize karne ki capability dete hain
// Ye canister upgrades ke time pe important hai
#[derive(Serialize, Deserialize)]
pub struct State {
    // OpenChat public key store karne ke liye string
    oc_public_key: String,
}

// Error messages ke liye constants define karte hain
const STATE_ALREADY_INITIALIZED: &str = "State has already been initialized";
const STATE_NOT_INITIALIZED: &str = "State has not been initialized";

// State ko initialize karne ke liye function
pub fn init(state: State) {
    STATE.with_borrow_mut(|s| {
        // Agar STATE pehle se initialized hai
        if s.is_some() {
            // To error throw karo
            panic!("{}", STATE_ALREADY_INITIALIZED);
        } else {
            // Nahi to STATE ko initialize karo
            *s = Some(state);
        }
    })
}

// STATE ko read karne ke liye generic function
// F ek closure hai jo State reference leta hai aur R type return karta hai
pub fn read<F: FnOnce(&State) -> R, R>(f: F) -> R {
    // STATE ko borrow karo read-only mode mein
    STATE.with_borrow(|s| f(s.as_ref().expect(STATE_NOT_INITIALIZED)))
}

// STATE ko modify karne ke liye generic function
// F ek closure hai jo mutable State reference leta hai aur R type return karta hai
pub fn mutate<F: FnOnce(&mut State) -> R, R>(f: F) -> R {
    // STATE ko mutable borrow karo
    STATE.with_borrow_mut(|s| f(s.as_mut().expect(STATE_NOT_INITIALIZED)))
}

// STATE ko consume karne ke liye function (upgrade ke time pe use hota hai)
pub fn take() -> State {
    // STATE ko completely le lo aur return karo
    STATE.take().expect(STATE_NOT_INITIALIZED)
}

// State struct ke methods implement karte hain
impl State {
    // Naya State object create karne ke liye constructor
    pub fn new(oc_public_key: String) -> State {
        State { oc_public_key }
    }

    // State ko update karne ke liye method (upgrade ke baad use hota hai)
    pub fn update(&mut self, oc_public_key: String) {
        self.oc_public_key = oc_public_key;
    }

    // OpenChat public key ko access karne ke liye getter
    pub fn oc_public_key(&self) -> &str {
        &self.oc_public_key
    }
}
