pub(crate) trait Packet {
    const PACKET_ID: u8;
    const PACKET_STRUCTURE: &'static [PacketTypes];
}

pub(crate) mod server_bound {
    use super::{
        Packet,
        PacketTypes::{self, *},
    };
    struct Handshake;
    impl Packet for Handshake {
        const PACKET_ID: u8 = 0x00;
        const PACKET_STRUCTURE: &'static [PacketTypes] = &[VarInt, Str(255), UnsignedShort, VarInt];
    }
}

pub(crate) enum PacketTypes {
    Boolean,
    Byte,
    UnsignedByte,
    Short,
    UnsignedShort,
    Int,
    Long,
    Float,
    Double,
    Str(usize),
    Chat,
    Identifier,
    VarInt,
    VarLong,
    EntityMetadata,
    Slot,
    NBTTag,
    Position,
    Angle,
    UUID,
    OptionalX,
    ArrayofX,
    XEnum,
    ByteArray,
}
pub(crate) enum Size {
    Fixed(usize),
    Range(usize, usize),
}

impl PacketTypes {
    pub(crate) fn size(&self) -> Size {
        match self {
            Self::Boolean => Size::Fixed(1),
            Self::Byte => Size::Fixed(1),
            Self::UnsignedByte => Size::Fixed(1),
            Self::Short => Size::Fixed(2),
            Self::UnsignedShort => Size::Fixed(2),
            Self::Int => Size::Fixed(4),
            Self::Long => Size::Fixed(8),
            Self::Float => Size::Fixed(4),
            Self::Double => Size::Fixed(8),
            Self::Str(n) => Size::Range(1, (n * 4) + 3),
            Self::Chat => Size::Range(1, (32767 * 4) + 3),
            Self::Identifier => Size::Range(1, (32767 * 4) + 3),
            Self::VarInt => Size::Range(1, 5),
            Self::VarLong => Size::Range(1, 10),
            Self::EntityMetadata => Size::Range(1, !0),
            Self::Slot => Size::Range(1, !0),
            Self::NBTTag => Size::Range(1, !0),
            Self::Position => Size::Fixed(0),
            Self::Angle => Size::Fixed(0),
            Self::UUID => Size::Fixed(0),
            Self::OptionalX => Size::Fixed(0),
            Self::ArrayofX => Size::Fixed(0),
            Self::XEnum => Size::Fixed(0),
            Self::ByteArray => Size::Fixed(0),
        }
    }
    pub(crate) fn read_as_var_int(&self, mut data: &mut [u8]) -> Option<i32> {
        match &self {
            Self::VarInt => {
                let mut num_read = 0;
                let mut result = 0i32;
                let mut read: Option<u8> = None;
                while if read != None {
                    read.unwrap() & 0b10000000 != 0
                } else {
                    true
                } {
                    read = Some(data[0]);
                    data = &mut data[1..];
                    let value = read.unwrap() & 0b01111111;
                    result |= (value as i32) << (7 * num_read);
                    num_read += 1;
                    if num_read > 5 {
                        panic!("Var Int Too big");
                    }
                }
                Some(result)
            }
            _ => None,
        }
    }
    pub(crate) fn read_as_var_long(&self, mut data: &mut [u8]) -> Option<i64> {
        match &self {
            Self::VarLong => {
                let mut num_read = 0;
                let mut result = 0i64;
                let mut read: Option<u8> = None;
                while if read != None {
                    read.unwrap() & 0b10000000 != 0
                } else {
                    true
                } {
                    read = Some(data[0]);
                    data = &mut data[1..];
                    let value = read.unwrap() & 0b01111111;
                    result |= (value as i64) << (7 * num_read);
                    num_read += 1;
                    if num_read > 10 {
                        panic!("Var Long Too big");
                    }
                }
                Some(result)
            }
            _ => None,
        }
    }
}
