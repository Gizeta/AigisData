use std::io;
use std::io::Read;
use std::io::Write;
use std::fs::File;

pub fn read_file(path: &str) -> io::Result<Vec<u8>> {
    let mut f = try!(File::open(path));
    let mut buffer = Vec::new();

    try!(f.read_to_end(&mut buffer));

    Ok(buffer)
}

pub fn write_str(path: &str, data: String) -> io::Result<()> {
    let f = try!(File::create(path));
    try!(write!(&f, "{}", data));

    Ok(())
}

pub fn write_bytes(path: &str, data: Vec<u8>) -> io::Result<()> {
    let mut f = try!(File::create(path));
    try!(f.write_all(&data));

    Ok(())
}
