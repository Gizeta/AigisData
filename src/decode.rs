pub fn decode_file_list(buffer: &mut Vec<u8>) {
    for i in buffer {
        *i = *i ^ 0xda;
    }
}
