use std::time::Instant;
use std::cmp;

pub struct TokenBucket {
    capacity: u64,
    refill_rate: u64,
    tokens: u64,
    last_refill: Instant,
}

impl TokenBucket {
    pub fn new(capacity: u64, refill_rate: u64) -> Self {
        Self {
            capacity,
            refill_rate,
            tokens: capacity,
            last_refill: Instant::now(),
        }
    }

    pub fn try_consume(&mut self, tokens: u64) -> bool {
        self.try_consume_at(tokens, Instant::now())
    }

    fn try_consume_at(&mut self, tokens: u64, now: Instant) -> bool {
        let time_from_last_refill = (now - self.last_refill).as_secs();
        if time_from_last_refill > 0 {
            self.tokens = cmp::min(self.capacity, self.tokens + self.refill_rate * time_from_last_refill);
            self.last_refill = now;
        }

        if self.tokens >= tokens {
            self.tokens -= tokens;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_bucket_starts_full() {
        let mut bucket = TokenBucket::new(3, 1);

        assert!(bucket.try_consume(1));
        assert!(bucket.try_consume(1));
        assert!(bucket.try_consume(1));
        assert!(!bucket.try_consume(1));
    }

    #[test]
    fn refills_after_time_passed() {
        let start = Instant::now();
        let mut bucket = TokenBucket::new(3, 1);
        assert!(bucket.try_consume_at(3, start));
        assert!(!bucket.try_consume_at(1, start));

        let one_second_later = start + std::time::Duration::from_secs(1);
        assert!(bucket.try_consume_at(1, one_second_later));
    }

    #[test]
    fn refills_only_after_time_passed() {
        let mut bucket = TokenBucket::new(3, 1);
        let start = bucket.last_refill;
        assert!(bucket.try_consume_at(3, start));


        let half_second_later = start + std::time::Duration::from_millis(500);
        assert!(!bucket.try_consume_at(1, half_second_later));

        let one_second_later = half_second_later + std::time::Duration::from_millis(500);
        assert!(bucket.try_consume_at(1, one_second_later));
    }
}
