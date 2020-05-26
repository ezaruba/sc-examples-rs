use elrond_wasm::Vec;
use elrond_wasm::esd_light::*;

/// Copied from elrond-wasm serialization tests. 
pub struct SerExample1 {
    pub int: u16,
    pub seq: Vec<u8>,
    pub another_byte: u8,
}

impl Encode for SerExample1 {
    fn dep_encode_to<O: Output>(&self, dest: &mut O) {
        self.int.dep_encode_to(dest);
        self.seq.dep_encode_to(dest);
        self.another_byte.dep_encode_to(dest);
    }
}

impl Decode for SerExample1 {
    fn dep_decode<I: Input>(input: &mut I) -> Result<Self, DeError> {
        Ok(SerExample1{
            int: u16::dep_decode(input)?,
            seq: Vec::<u8>::dep_decode(input)?,
            another_byte: u8::dep_decode(input)?,
        })
    }
}
