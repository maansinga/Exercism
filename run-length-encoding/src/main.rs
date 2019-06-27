use run_length_encoding::{encode, decode};

fn main(){
//    let encoded = encode("zzz ZZ  zZ");
//    let decoded = decode(encoded.as_ref());
//    println!("Encoded: {}", encoded);
//    println!("Decoded: {}", decoded);

    let encoded =  "XYZ";
    let decoded = "XYZ";
    println!("original Encoded: {}", encoded);
    println!("original Decoded: {}", decoded);
    println!("Decoded: {}", decode(encoded));
    println!("Encoded: {}", encode(decoded));
}