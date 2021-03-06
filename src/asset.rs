use std::io;
use al_type;
use decode;
use reader::Reader;
use util;
use super::rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable)]
pub struct AssetInfo {
    hash: [String; 2],
    asset_type: u8,
    file_size: u32,
    filename: String,
}

pub fn gen_file_info(path: &str) -> io::Result<()> {
    let mut data = try!(util::read_file(path));
    decode::decode_file_list(&mut data);

    let content = String::from_utf8(data).unwrap();
    let mut arr = Vec::new();
    for line in content.split_whitespace() {
        let fields: Vec<&str> = line.split(',').collect();
        arr.push(AssetInfo {
                     hash: [fields[0].to_string(), fields[1].to_string()],
                     asset_type: fields[2].parse().unwrap(),
                     file_size: fields[3].parse().unwrap(),
                     filename: fields[4].to_string(),
                 });
    }

    try!(util::write_str("generated/asset.json", json::encode(&arr).unwrap()));

    Ok(())
}

pub fn decode_file(path: &str, filename: &str) -> io::Result<()> {
    let data = try!(util::read_file(path));
    let mut reader = Reader::create(&data);

    let file_type = reader.read_chars(4);
    match file_type.as_ref() {
        "ALLZ" => {
            al_type::allz::decode(reader, filename);
        }
        other => println!("Unrecognizable file type {}", other),
    }

    Ok(())
}
