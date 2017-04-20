use std::io;
use reader::Reader;
use util;

enum FinishState {
    Overflow,
    Word,
    Literal,
}

pub fn decode(reader: Reader, filename: &str) -> io::Result<()> {
    let data = decompress(reader);
    try!(util::write_bytes(&filename, data));

    Ok(())
}

pub fn decompress(mut reader: Reader) -> Vec<u8> {
    let ver = reader.read_u8();
    if ver != 1 {
        panic!("Unexpected ver {:}", ver);
    }

    let minbits_length = reader.read_u8();
    let minbits_offset = reader.read_u8();
    let minbits_literal = reader.read_u8();

    let dst_size = reader.read_u32() as usize;
    let mut dst: Vec<u8> = Vec::with_capacity(dst_size);

    let readcontrol_length = |reader: &mut Reader| 3 + read_control(reader, minbits_length);
    let readcontrol_offset = |reader: &mut Reader| 1 + read_control(reader, minbits_offset);
    let readcontrol_literal = |reader: &mut Reader| 1 + read_control(reader, minbits_literal);

    let literal_len = readcontrol_literal(&mut reader) as usize;
    copy_literal(&mut reader, &mut dst, literal_len);

    let mut word_offset = readcontrol_offset(&mut reader) as usize;
    let mut word_len = readcontrol_length(&mut reader) as usize;

    let mut fin_state = FinishState::Overflow;

    while !reader.is_eof() {
        if dst.len() + word_len >= dst_size {
            fin_state = FinishState::Word;
            break;
        }
        if reader.read_bit() == 0 {
            let literal_len = readcontrol_literal(&mut reader) as usize;
            if dst.len() + word_len + literal_len >= dst_size {
                fin_state = FinishState::Literal;
                break;
            }

            let literal_offset = reader.offset;
            reader.offset += literal_len;
            let next_offset = readcontrol_offset(&mut reader) as usize;
            let next_len = readcontrol_length(&mut reader) as usize;
            copy_word(&mut dst, word_offset, word_len);

            let control_offset = reader.offset;
            reader.offset = literal_offset;
            copy_literal(&mut reader, &mut dst, literal_len);

            if reader.offset != literal_offset + literal_len {
                panic!("Unexcepted literal error");
            }

            reader.offset = control_offset;
            word_offset = next_offset;
            word_len = next_len;
        } else {
            let next_offset = readcontrol_offset(&mut reader) as usize;
            let next_len = readcontrol_length(&mut reader) as usize;
            copy_word(&mut dst, word_offset, word_len);

            word_offset = next_offset;
            word_len = next_len;
        }
    }

    match fin_state {
        FinishState::Word => {
            copy_word(&mut dst, word_offset, word_len);
        }
        FinishState::Literal => {
            copy_word(&mut dst, word_offset, word_len);
            copy_literal(&mut reader, &mut dst, literal_len);
        }
        FinishState::Overflow => panic!("Overflow"),
    }

    dst
}

fn read_unary(reader: &mut Reader) -> u8 {
    let mut n = 0;
    while reader.read_bit() == 1 {
        n += 1;
    }

    n
}

fn read_control(reader: &mut Reader, minbits: u8) -> u16 {
    let u = read_unary(reader);
    let n = reader.read_bits(u + minbits) as u16;

    n + (((1 << u as u16) - 1) << minbits)
}

fn copy_literal(reader: &mut Reader, dst: &mut Vec<u8>, len: usize) {
    for _ in 0..len {
        dst.push(reader.read_u8());
    }
}

fn copy_word(dst: &mut Vec<u8>, offset: usize, len: usize) {
    for _ in 0..len {
        let ch = dst[dst.len() - offset];
        dst.push(ch);
    }
}
