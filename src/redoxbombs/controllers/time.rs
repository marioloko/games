use events::GameEvent;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::time::{Duration, Instant};

/// A `Timeout` wraps an `GameEvent` which shall be executed
/// after a period of time.
#[derive(Debug)]
struct Timeout {
    /// The `creation_time` is the instant in which the timeout was created.
    creation_time: Instant,

    /// The `expire_after` shows the milliseconds that shall pass for the
    /// timeout to expire.
    expire_after: Duration,

    /// The `event` field is the `GameEvent` to be released after the
    /// timeout expiration.
    event: GameEvent,
}

impl Timeout {
    /// Create a new `Timeout` given some milliseconds to wait and
    /// an `GameEvent` to wrap.
    fn new(millis: u64, event: GameEvent) -> Timeout {
        Timeout {
            creation_time: Instant::now(),
            expire_after: Duration::from_millis(millis),
            event,
        }
    }

    /// Return whether or not the `Timeout` has expired. That means that
    /// the seconds to expire has passed.
    fn expired(&self) -> bool {
        self.creation_time.elapsed() >= self.expire_after
    }

    /// Return the `Instant` in which the `Timeout` shall expire.
    ///
    /// panics:
    /// - This function panics if the addition overflows.
    fn expiration_time(&self) -> Instant {
        self.creation_time + self.expire_after
    }
}

impl PartialEq for Timeout {
    /// Two `Timeout` are equal if their `expiration_time` is equal.
    fn eq(&self, other: &Timeout) -> bool {
        self.expiration_time() == other.expiration_time()
    }
}

impl Eq for Timeout {}

impl PartialOrd for Timeout {
    /// The partial order of two `Timeout` is the opposite of the
    /// partial order of their `expiration_time`.
    fn partial_cmp(&self, other: &Timeout) -> Option<Ordering> {
        other.expiration_time().partial_cmp(&self.expiration_time())
    }
}

impl Ord for Timeout {
    /// The order of two `Timeout` is equivalent to its partial order of their `expiration_time`.
    fn cmp(&self, other: &Timeout) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

/// The `TimeController` manages the events that shall be run in a period
/// of time in the futuer.
#[derive(Debug)]
pub struct TimeController {
    /// The `scheduled_events` is a priority queue, which pop always the
    /// the timeout with sooner expiration date. It stores the timeouts to
    /// be executed in the future
    scheduled_events: BinaryHeap<Timeout>,
}

impl TimeController {
    /// Create a new `TimeController`.
    pub fn new() -> TimeController {
        TimeController {
            scheduled_events: BinaryHeap::new(),
        }
    }

    /// Schedule a new `Timeout` to return an `GameEvent` after some millis.
    pub fn schedule_event_in(&mut self, millis: u64, event: GameEvent) {
        let timeout = Timeout::new(millis, event);
        self.scheduled_events.push(timeout);
    }

    /// Pop the `GameEvent` corresponding to the `Timeout` whose expiration
    /// time is the sooner, if it expiration time has due.
    pub fn pop_event(&mut self) -> Option<GameEvent> {
        if !self.has_any_expired_timeout() {
            return None;
        }

        let timeout = self.scheduled_events.pop()?;
        Some(timeout.event)
    }

    /// Remove all the scheduled events.
    pub fn clear(&mut self) {
        self.scheduled_events.clear();
    }

    /// Check if there is any expired `Timeout` in the queue.
    fn has_any_expired_timeout(&self) -> bool {
        match self.scheduled_events.peek() {
            None => false,
            Some(next_timeout) => next_timeout.expired(),
        }
    }
}
