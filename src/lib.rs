use std::cmp;
use std::time::Instant;

pub struct TokenBucket {
    capacity: u64,
    refill_rate: u64,
    tokens: u64,
    last_refill: Instant,
}

impl TokenBucket {
    pub fn new(capacity: u64, refill_rate: u64) -> Self {
        Self::new_at(capacity, refill_rate, Instant::now())
    }

    pub fn new_at(capacity: u64, refill_rate: u64, now: Instant) -> Self {
        Self {
            capacity,
            refill_rate,
            tokens: capacity,
            last_refill: now,
        }
    }

    pub fn try_consume(&mut self, tokens: u64) -> bool {
        self.try_consume_at(tokens, Instant::now())
    }

    pub fn try_consume_at(&mut self, tokens: u64, now: Instant) -> bool {
        if tokens == 0 {
            return false;
        }
        let time_from_last_refill = (now - self.last_refill).as_secs();
        if time_from_last_refill > 0 {
            self.tokens = cmp::min(
                self.capacity,
                self.tokens + self.refill_rate * time_from_last_refill,
            );
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
        let mut bucket = TokenBucket::new(3, 1);
        let start = bucket.last_refill;
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

    #[test]
    fn consume_zero_tokens() {
        let mut bucket = TokenBucket::new(3, 1);
        assert!(!bucket.try_consume(0));
    }

    #[test]
    fn consume_more_than_capacity() {
        let mut bucket = TokenBucket::new(3, 1);
        assert!(!bucket.try_consume(10));
    }

    #[test]
    fn refill_does_not_exceed_capacity() {
        let mut bucket = TokenBucket::new(3, 1);
        let start = bucket.last_refill;
        assert!(bucket.try_consume_at(3, start));
        assert!(!bucket.try_consume_at(3, start));

        let one_second_later = start + std::time::Duration::from_secs(2);
        assert!(bucket.try_consume_at(1, one_second_later));
        assert_eq!(bucket.tokens, 1);

        let three_second_later = start + std::time::Duration::from_secs(4);
        assert!(bucket.try_consume_at(1, three_second_later));
        assert_eq!(bucket.tokens, 2);
        assert_eq!(bucket.tokens, bucket.capacity - 1);
    }

    #[test]
    fn partial_refill_large_request_fails() {
        let mut bucket = TokenBucket::new(8, 2);
        let start = bucket.last_refill;
        assert!(bucket.try_consume_at(8, start));

        let two_second_later = start + std::time::Duration::from_secs(2);
        assert!(!bucket.try_consume_at(30, two_second_later));
        assert_eq!(bucket.tokens, 4);
    }

    #[test]
    fn refill_boundaries_valid() {
        let mut bucket = TokenBucket::new(5, 2);
        let start = bucket.last_refill;
        assert!(bucket.try_consume_at(5, start));

        let two_second_later = start + std::time::Duration::from_secs(2);
        assert!(bucket.try_consume_at(4, two_second_later));
        assert!(!bucket.try_consume_at(1, two_second_later));
    }
}
