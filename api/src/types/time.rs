use std::ops::Deref;

use async_graphql::scalar;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub struct Time(pub u64);

impl Time {
    pub fn new(time: u64) -> Self {
        Self(time)
    }

    pub fn now() -> Self {
        Self(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        )
    }

    pub fn duration(millis: u64) -> Self {
        Self::now() + millis
    }
}

impl From<u64> for Time {
    fn from(time: u64) -> Self {
        Self(time)
    }
}

impl From<Time> for u64 {
    fn from(time: Time) -> Self {
        time.0
    }
}

impl Deref for Time {
    type Target = u64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::Add for Time {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Time {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl std::ops::Add<u64> for Time {
    type Output = Self;
    fn add(self, rhs: u64) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl std::ops::Add<Time> for u64 {
    type Output = Time;
    fn add(self, rhs: Time) -> Self::Output {
        Time(self + rhs.0)
    }
}

impl std::ops::Sub<u64> for Time {
    type Output = Self;
    fn sub(self, rhs: u64) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl std::ops::Sub<Time> for u64 {
    type Output = Time;
    fn sub(self, rhs: Time) -> Self::Output {
        Time(self - rhs.0)
    }
}

scalar!(Time);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn time_now_is_valid() {
        let sys_time_now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        assert!(Time::now().0 == sys_time_now);
    }

    #[test]
    fn time_add() {
        let time = Time::now();
        let time2 = Time::now();
        let sum_time = time + time2;
        let sum_inner = time.0 + time2.0;
        assert!(*sum_time == sum_inner);
    }

    #[test]
    fn time_sub() {
        let time = Time::now();
        let time2 = Time::now();
        let sum_time = time - time2;
        let sum_inner = time.0 - time2.0;
        assert!(*sum_time == sum_inner);
    }
}
