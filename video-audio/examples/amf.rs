use amf::{Amf0Value, Value, Version};

fn main() {
    // Encodes a AMF0's number
    let number = Value::from(Amf0Value::Number(1.23));
    let mut buf = Vec::new();
    number.write_to(&mut buf).unwrap();

    // Decodes above number
    let decoded = Value::read_from(&mut &buf[..], Version::Amf0).unwrap();
    assert_eq!(number, decoded);
}
