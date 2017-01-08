use super::hash::Hash;
use rustc_serialize::{Decodable, Encodable};
use bincode::SizeLimit;
use bincode::rustc_serialize::{encode, decode};
use std::marker::PhantomData;

/// Type Content represents the encoded version of the caller's data.
#[derive(Debug, PartialEq)]
pub struct Content<T: Encodable + Decodable>(Vec<u8>, PhantomData<T>);

impl<T: Encodable + Decodable> Content<T> {
    pub fn encode(value: &T) -> (Hash, Content<T>) {
        let encoded = encode(value, SizeLimit::Infinite).unwrap();
        let hash = Hash::for_bytes(&encoded);
        return (hash, Content(encoded, PhantomData));
    }

    pub fn decode(&self) -> T {
        decode(&self.0).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Content;
    use std::marker::PhantomData;

    #[test]
    fn encode() {
        let (hash, encoded) = Content::encode(&"abcd".to_string());
        assert_eq!(hash.to_hex(),
                   "9481cd49061765e353c25758440d21223df63044352cfde1775e0debc2116841");
        assert_eq!(encoded,
                   Content(vec![0u8, 0, 0, 0, 0, 0, 0, 4, 97, 98, 99, 100], PhantomData));
    }

    #[test]
    fn decode_content_abcd() {
        assert_eq!(Content::<String>(vec![0u8, 0, 0, 0, 0, 0, 0, 4, 97, 98, 99, 100], PhantomData)
                       .decode(),
                   "abcd".to_string());
    }
}
