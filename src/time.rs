use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};

pub fn get_nanos_timestamp() -> Result<u64, SystemTimeError> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_nanos_timestamp() {
        let timestamp = get_nanos_timestamp().unwrap();
        assert!(timestamp > 0);
    }
}
