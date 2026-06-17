use serde::{Deserialize, Serialize};

/// A client's cursor position in canvas/world coordinates. Sent client → server
/// as a bare-CBOR **datagram** (unreliable, self-framed — no length prefix), so
/// high-frequency cursor traffic never head-of-line-blocks the reliable stream.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
pub struct CursorUpdate {
    pub x: f32,
    pub y: f32,
}

/// Another user's cursor, broadcast server → client. Travels over a datagram on
/// the wire; within the room it rides [`super::TaskboardEvent::Cursor`].
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(into_wasm_abi, from_wasm_abi))]
pub struct UserCursor {
    pub user: i32,
    pub x: f32,
    pub y: f32,
}

impl CursorUpdate {
    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        ciborium::into_writer(self, &mut buf).expect("cursor encodes");
        buf
    }

    pub fn decode(bytes: &[u8]) -> anyhow::Result<Self> {
        Ok(ciborium::from_reader(bytes)?)
    }
}

impl UserCursor {
    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        ciborium::into_writer(self, &mut buf).expect("cursor encodes");
        buf
    }

    pub fn decode(bytes: &[u8]) -> anyhow::Result<Self> {
        Ok(ciborium::from_reader(bytes)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cursor_update_datagram_roundtrips() {
        let c = CursorUpdate { x: 12.5, y: -7.25 };
        let back = CursorUpdate::decode(&c.encode()).unwrap();
        assert_eq!((back.x, back.y), (c.x, c.y));
    }

    #[test]
    fn user_cursor_datagram_roundtrips() {
        let c = UserCursor {
            user: 42,
            x: 1.0,
            y: 2.0,
        };
        let back = UserCursor::decode(&c.encode()).unwrap();
        assert_eq!((back.user, back.x, back.y), (c.user, c.x, c.y));
    }

    #[test]
    fn malformed_cursor_datagram_is_rejected() {
        assert!(UserCursor::decode(&[0xff, 0x00, 0x13]).is_err());
    }
}
