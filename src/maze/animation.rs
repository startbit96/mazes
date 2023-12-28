use std::{thread, time::Duration};

const ANIMATION_DELAY_SHORT_MILLISECONDS: u64 = 2;
const ANIMATION_DELAY_MIDDLE_MILLISECONDS: u64 = 5;
const ANIMATION_DELAY_LONG_MILLISECONDS: u64 = 10;

pub enum Delay {
    NoDelay,
    Short,
    Middle,
    Long,
}

impl Delay {
    fn to_milliseconds(&self) -> u64 {
        match self {
            Delay::NoDelay => 0,
            Delay::Short => ANIMATION_DELAY_SHORT_MILLISECONDS,
            Delay::Middle => ANIMATION_DELAY_MIDDLE_MILLISECONDS,
            Delay::Long => ANIMATION_DELAY_LONG_MILLISECONDS,
        }
    }
}

pub fn delay(delay: Delay) {
    let duration = Duration::from_millis(delay.to_milliseconds());
    thread::sleep(duration);
}
