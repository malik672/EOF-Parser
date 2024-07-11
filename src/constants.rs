pub const EOF_MAGIC: [u8; 2] = [0xEF, 0x00];
pub const MAX_CODE_SECTIONS: usize = 1024;
pub const MAX_RETURN_STACK_SIZE: usize = 1024;
pub use byteorder::{LittleEndian, ReadBytesExt};
pub use std::io::{Cursor, Read};
