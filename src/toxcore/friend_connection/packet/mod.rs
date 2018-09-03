/*! Top-level Friend connection Packets
*/

use toxcore::binary_io::*;

mod id_alive;

pub use self::id_alive::*;

/** Friend connection packet enum that encapsulates all types of Friend connection packets.
*/
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Packet {
    /// [`IdAlive`](./struct.IdAlive.html) structure.
    IdAlive(IdAlive),
}

impl ToBytes for Packet {
    fn to_bytes<'a>(&self, buf: (&'a mut [u8], usize)) -> Result<(&'a mut [u8], usize), GenError> {
        match *self {
            Packet::IdAlive(ref p) => p.to_bytes(buf),
        }
    }
}

impl FromBytes for Packet {
    named!(from_bytes<Packet>, alt!(
        map!(IdAlive::from_bytes, Packet::IdAlive)
    ));
}