use crate::Outpoint;
use extended_primitives::Buffer;
use handshake_protocol::encoding::{Decodable, DecodingError, Encodable};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Input {
    pub prevout: Outpoint,
    pub sequence: u32,
    ////TODO this should probably be a custom type, but can be implemented later.
    //TODO should be custom type, and also an option.
    //pub witness: Vec<Vec<u8>>,
}

impl Encodable for Input {
    fn size(&self) -> u32 {
        //prevout (36) + sequence (4)
        40
    }

    fn encode(&self) -> Buffer {
        let mut buffer = Buffer::new();

        buffer.extend(self.prevout.encode());
        buffer.write_u32(self.sequence);

        buffer
    }
}

impl Decodable for Input {
    type Error = DecodingError;

    fn decode(buffer: &mut Buffer) -> Result<Self, Self::Error> {
        let prevout = Outpoint::decode(buffer)?;
        let sequence = buffer.read_u32()?;

        Ok(Input { prevout, sequence })
    }
}
