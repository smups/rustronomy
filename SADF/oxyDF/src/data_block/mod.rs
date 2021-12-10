//Imports
use crate::data_types::{EncodeAndConsume, Decode};

//Module structure
mod metadata;

trait DataBlock: EncodeAndConsume + Decode {
    const MDI: u16; //Meta Data Index
    const DCA: u16; //Data Compression Algorithm
}
//This abstract DataBlock trait is implemented by all Data Blocks (they must
// all be encodable and decodable)