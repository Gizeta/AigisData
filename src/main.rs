extern crate argparse;
extern crate rustc_serialize;

use argparse::{ArgumentParser, Store};

mod asset;
mod decode;
mod util;

fn main() {
    let mut file_list = "cache/1fp32igvpoxnb521p9dqypak5cal0xv0".to_string();
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut file_list)
            .add_option(&["-l", "--list"], Store, "File list for assets");
        ap.parse_args_or_exit();
    }

    match asset::gen_file_info(&file_list) {
        Err(why) => println!("Fail to generate asset.json due to: {}", why),
        Ok(_) => println!("asset.json generated successfully."),
    }
}
