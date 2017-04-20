extern crate argparse;
extern crate rustc_serialize;

use argparse::{ArgumentParser, Store};

mod al_type;
mod asset;
mod decode;
mod reader;
mod util;

fn main() {
    let mut file_list = String::new();
    let mut decode_file = String::new();
    let mut output_name = String::new();
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut file_list)
            .add_option(&["-l", "--list"], Store, "File list for assets");
        ap.refer(&mut decode_file)
            .add_option(&["-f", "--file"], Store, "File to decode");
        ap.refer(&mut output_name)
            .add_option(&["-o", "--output"], Store, "Filename to create");
        ap.parse_args_or_exit();
    }

    if !file_list.is_empty() {
        match asset::gen_file_info(&file_list) {
            Err(why) => println!("Fail to generate asset.json due to: {}", why),
            Ok(_) => println!("asset.json generated successfully."),
        }
    }

    if !decode_file.is_empty() {
        match asset::decode_file(&decode_file, &output_name) {
            Err(why) => println!("Fail to generate asset.json due to: {}", why),
            Ok(_) => println!("{} decoded successfully.", decode_file),
        }
    }
}
