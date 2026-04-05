use std::time::Instant;

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
}
