//! Utility functions
use once_cell::sync::Lazy;
use uuid::Uuid;

/// Generate a v1 UUID
pub fn v1_uuid() -> Uuid {
    static MAC: Lazy<[u8; 6]> = Lazy::new(get_mac_address);
    Uuid::now_v1(&MAC)
}

fn get_mac_address() -> [u8; 6] {
    if let Ok(Some(addr)) = mac_address::get_mac_address() {
        addr.bytes()
    } else {
        let mut mac = [0u8; 6];
        getrandom::getrandom(&mut mac).ok();

        mac
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v1_uuid() {
        let uuid = v1_uuid();

        assert_eq!(uuid.get_version(), Some(uuid::Version::Mac));
        assert_eq!(uuid.get_variant(), uuid::Variant::RFC4122);
    }

    #[test]
    fn test_get_mac_address() {
        let mac = get_mac_address();

        assert_eq!(mac.len(), 6);
    }
}
