use encode;
use decode;
#[test]
fn encode_test1() {
    let origin = String::from("Äiti on ötökkä, ålåf ei");
    let encoded = encode(&origin);
    assert_eq!(encoded, "%C3%84iti%20on%20%C3%B6t%C3%B6kk%C3%A4%2C%20%C3%A5l%C3%A5f%20ei");
}
#[test]
fn encode_test2() {
    let origin = String::from("normaalistring");
    let encoded = encode(&origin);
    assert_eq!(encoded, "normaalistring");
}

#[test]
#[should_panic]
fn encode_test_panic() {
    let origin = String::from("Äiti on ÷ötökkä, ålåf ei");
    let _encoded = encode(&origin);
   
}

#[test]
fn decode_test1() {
    let origin = String::from("%C3%84iti%20on%20%C3%B6t%C3%B6kk%C3%A4%2C%20%C3%A5l%C3%A5f%20ei");
    let decoded = decode(&origin);
    assert_eq!(decoded, "Äiti on ötökkä, ålåf ei");
}

#[test]
#[should_panic]
fn failing_decode_test1() {
    let origin = String::from("%Cei");
    let decoded = decode(&origin);   
}

#[test]
#[should_panic]
fn failing_decode_test2() {
    let origin = String::from("%");
    let decoded = decode(&origin); 
}

#[test]
fn decode_test_nochange() {
    let origin = String::from("normaaliststring");
    let decoded = decode(&origin);
    assert_eq!(decoded, "normaaliststring");
}