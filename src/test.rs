use encode;
use decode;
#[test]
fn encode_test1() {
    let origin = String::from("Äiti on ötökkä, ålåf ei");
    let encoded = encode(&origin);
    assert_eq!(encoded, "%C3%84iti%20on%20%C3%B6t%C3%B6kk%C3%A4%2C%20%C3%A5l%C3%A5f%20ei");
}

#[test]
fn decode_test1() {
    let origin = String::from("%C3%84iti%20on%20%C3%B6t%C3%B6kk%C3%A4%2C%20%C3%A5l%C3%A5f%20ei");
    let decoded = decode(&origin);
    assert_eq!(decoded, "Äiti on ötökkä, ålåf ei");
}