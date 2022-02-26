use std::{
    default,
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use anyhow::*;
use bytes::{BufMut, BytesMut};

#[derive(Debug)]
enum NaluType {
    NALU_TYPE_SLICE = 1,
    NALU_TYPE_DPA = 2,
    NALU_TYPE_DPB = 3,
    NALU_TYPE_DPC = 4,
    NALU_TYPE_IDR = 5,
    NALU_TYPE_SEI = 6,
    NALU_TYPE_SPS = 7,
    NALU_TYPE_PPS = 8,
    NALU_TYPE_AUD = 9,
    NALU_TYPE_EOSEQ = 10,
    NALU_TYPE_EOSTREAM = 11,
    NALU_TYPE_FILL = 12,
}

#[derive(Debug)]
enum NaluPriority {
    NALU_PRIORITY_DISPOSABLE = 0,
    NALU_PRIRITY_LOW = 1,
    NALU_PRIORITY_HIGH = 2,
    NALU_PRIORITY_HIGHEST = 3,
}

#[repr(C)]
#[derive(Debug, Default)]
struct Nalu {
    startcodeprefix_len: i32,
    len: u32,
    max_size: u32,
    forbidden_bit: i32,
    nal_reference_idc: i32,
    nal_unit_type: i32,
    buf: Vec<u8>,
}

fn simplest_h264_parser<P>(path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let max_buffer_size = 100000;
    let mut file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut nalu = Nalu::default();
    nalu.max_size = max_buffer_size;
    nalu.buf = vec![0; max_buffer_size as usize];
    let mut data_offset = 0usize;
    let mut nal_num = 0usize;

    println!("-------+----- NALU Table -------+-------+");
    println!(" NUM   |  POS  |  IDC  |  TYPE  |  LEN  |");
    println!("-------+-------+-------+--------+-------+");
    loop {
        let data_length = get_annexb_nalu(&mut nalu, &mut reader)?;
        println!(
            " {:?}    |  {:?}     |       |   {:?}",
            nal_num, data_offset, nalu.len
        );
        nal_num += 1;
    }
    Ok(())
}

// 判断0x00 00 01
fn find_start_code2(buf: &[u8]) -> bool {
    if buf[0] != 0 || buf[1] != 0 || buf[2] != 1 {
        false
    } else {
        true
    }
}

// 判断0x00 00 00 01
fn find_start_code3(buf: &[u8]) -> bool {
    if buf[0] != 0 || buf[1] != 0 || buf[2] != 0 || buf[3] != 1 {
        false
    } else {
        true
    }
}

fn get_annexb_nalu(nalu: &mut Nalu, reader: &mut BufReader<File>) -> Result<i32> {
    let mut pos = 0;
    let mut rewind = 0i32;
    nalu.startcodeprefix_len = 3;
    let mut buffer = BytesMut::with_capacity(3);
    buffer.put_slice(&[0; 3]);
    let size = reader.read(&mut buffer)?;

    if size != 3 {
        return Err(anyhow!("get_annexb_nalu err"));
    }
    if !find_start_code2(&buffer[..]) {
        let mut buffer_2 = BytesMut::with_capacity(1);
        buffer_2.put_u8(0);
        let size = reader.read(&mut buffer_2)?;
        if size != 1 {
            return Err(anyhow!("read error"));
        }
        buffer.put_slice(&buffer_2[..]);
        if !find_start_code3(&buffer[..]) {
            return Err(anyhow!("get_annexb_nalu err"));
        } else {
            pos = 4;
            nalu.startcodeprefix_len = 4
        }
    } else {
        pos = 3;
        nalu.startcodeprefix_len = 3
    }

    let mut info2 = false;
    let mut info3 = false;
    let mut start_code_found = false;

    loop {
        let mut buffer_3 = BytesMut::with_capacity(1);
        buffer_3.put_u8(0);
        let size = reader.read(&mut buffer_3[..])?;
        if size == 0 {
            nalu.len = pos - 1 - nalu.startcodeprefix_len as u32;
            nalu.buf = buffer[nalu.startcodeprefix_len as usize..nalu.len as usize]
                .into_iter()
                .map(|v| *v)
                .collect();
            nalu.forbidden_bit = (nalu.buf[0] & 0x80) as i32;
            nalu.nal_reference_idc = (nalu.buf[0] & 0x60) as i32;
            nalu.nal_unit_type = (nalu.buf[0] & 0x1f) as i32;

            return Ok((pos - 1) as i32);
        }
        pos += 1;
        buffer.put_slice(&buffer_3[..]);
        // println!("{:2X}", buffer);
        info3 = find_start_code3(&buffer[buffer.len() - 4..]);
        if !info3 {
            info2 = find_start_code2(&buffer[buffer.len() - 3..]);
        }
        start_code_found = info2 || info3;
        if start_code_found {
            break;
        }
    }

    rewind = if info3 { -4 } else { -3 };
    reader.seek_relative(rewind as i64)?;

    nalu.len = (pos as i32 + rewind - nalu.startcodeprefix_len as i32) as u32;
    nalu.buf = buffer[nalu.startcodeprefix_len as usize..nalu.len as usize]
        .into_iter()
        .map(|v| *v)
        .collect();
    nalu.forbidden_bit = (nalu.buf[0] & 0x80) as i32;
    nalu.nal_reference_idc = (nalu.buf[0] & 0x60) as i32;
    nalu.nal_unit_type = (nalu.buf[0] & 0x1f) as i32;

    return Ok(pos as i32 + rewind);
}

fn main() -> Result<()> {
    simplest_h264_parser("./asserts/sintel.h264")
    // todo!()
}
