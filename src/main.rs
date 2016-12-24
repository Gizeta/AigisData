extern crate argparse;

mod decode;

use std::io;
use std::io::Read;
use std::io::Write;
use std::fs::File;

use argparse::{ArgumentParser, Store};

fn read_file(path: &str) -> io::Result<Vec<u8>> {
    let mut f = try!(File::open(path));
    let mut buffer = Vec::new();

    try!(f.read_to_end(&mut buffer));

    Ok(buffer)
}

fn main() {
    let mut file_list = "cache/1fp32igvpoxnb521p9dqypak5cal0xv0".to_string();
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut file_list)
            .add_option(&["-l", "--list"], Store, "File list for assets");
        ap.parse_args_or_exit();
    }

    match gen_file_list(&file_list) {
        Err(why) => println!("Fail to generate file list due to: {}", why),
        Ok(_) => println!("File list generated successfully."),
    }
}

fn gen_file_list(path: &str) -> io::Result<()> {
    let mut data = try!(read_file(path));
    decode::decode_file_list(&mut data);

    let mut f = try!(File::create("./generated/file_list.txt"));
    try!(f.write_all(&data));

    Ok(())
}
