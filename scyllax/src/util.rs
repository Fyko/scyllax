use once_cell::sync::Lazy;
use uuid::Uuid;

/// Generate a v1 UUID
pub fn v1_uuid() -> Uuid {
    static MAC: Lazy<[u8; 6]> = Lazy::new(get_mac_address);
    Uuid::now_v1(&MAC)
}

fn get_mac_address() -> [u8; 6] {
    match mac_address::get_mac_address() {
        Ok(Some(addr)) => addr.bytes(),
        _ => {
            let mut mac = [0u8; 6];
            getrandom::getrandom(&mut mac).ok();
            mac
        }
    }
}
