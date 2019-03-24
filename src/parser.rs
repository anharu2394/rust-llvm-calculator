use failure::Error;

pub mod syntax {
    include!(concat!(env!("OUT_DIR"), "/calculator.rs"));
}
