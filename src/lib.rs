mod base_format {
    #![allow(clippy::never_loop)]
    #![allow(warnings)]
    protospec::include_spec!("format");
}

#[test]
pub fn test_file_parse() {
    use base_format::SaleaePacket;
    use std::io::BufReader;
    let data0 = include_bytes!("../test/digital_0.bin");
    let mut br = BufReader::new(&data0[..]);
    if let Err(e) = SaleaePacket::decode_sync(&mut br) {
        println!("e: {}", e);
    }

    let data1 = include_bytes!("../test/digital_1.bin");
    let mut br = BufReader::new(&data1[..]);
    if let Err(e) = SaleaePacket::decode_sync(&mut br) {
        println!("e: {}", e);
    }

    let data0 = include_bytes!("../test/analog_0.bin");
    let mut br = BufReader::new(&data0[..]);
    if let Err(e) = SaleaePacket::decode_sync(&mut br) {
        println!("e: {}", e);
    }

    let data1 = include_bytes!("../test/analog_1.bin");
    let mut br = BufReader::new(&data1[..]);
    if let Err(e) = SaleaePacket::decode_sync(&mut br) {
        println!("e: {}", e);
    }
}

#[cfg(test)]
mod test {
    #[test]
    pub fn test_file_extraction() {
        use crate::base_format::{ItemBody, SaleaePacket};
        use std::io::BufReader;
        let data0 = include_bytes!("../test/analog_0.bin");
        let mut br = BufReader::new(&data0[..]);
        let packet = match SaleaePacket::decode_sync(&mut br) {
            Err(e) => {
                println!("e: {}", e);
                assert!(false);
                return;
            }
            Ok(a) => a,
        };
        println!("{:#?}", packet);
        match packet.body {
            ItemBody::Analog(a) => {
                println!("Data: {:#?}", a);
            }
            ItemBody::Digital(a) => {
                println!("Data: {:#?}", a);
            }
        }
    }
}
