use std::{thread, time::Duration};

const ANIMATION_DELAY_SHORT_MILLISECONDS: u64 = 8;
const ANIMATION_DELAY_MIDDLE_MILLISECONDS: u64 = 15;
const ANIMATION_DELAY_LONG_MILLISECONDS: u64 = 30;
const ANIMATION_DELAY_VERY_LONG_MILLISECONDS: u64 = 1000;

pub enum Delay {
    NoDelay,
    Short,
    Middle,
    Long,
    VeryLong,
}

impl Delay {
    fn to_milliseconds(&self) -> u64 {
        match self {
            Delay::NoDelay => 0,
            Delay::Short => ANIMATION_DELAY_SHORT_MILLISECONDS,
            Delay::Middle => ANIMATION_DELAY_MIDDLE_MILLISECONDS,
            Delay::Long => ANIMATION_DELAY_LONG_MILLISECONDS,
            Delay::VeryLong => ANIMATION_DELAY_VERY_LONG_MILLISECONDS,
        }
    }
}

pub fn delay(delay: Delay) {
    let duration = Duration::from_millis(delay.to_milliseconds());
    thread::sleep(duration);
}
