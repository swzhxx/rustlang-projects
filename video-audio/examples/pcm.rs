use std::{
    fs::File,
    io::{BufRead, BufReader, Read, Write},
    path::Path,
};

use bytes::{Buf, BufMut, BytesMut};

use anyhow::*;

fn simplest_pcm16le_split(url: &Path) -> Result<()> {
    let mut file = File::open(url)?;
    let mut file_l = File::create("./asserts/output_pcm_l.pcm")?;
    let mut file_r = File::create("./asserts/output_pcm_r.pcm")?;
    let mut buff_reader = BufReader::new(file);
    loop {
        let mut data = vec![0; 4];
        let size = buff_reader.read(&mut data)?;
        if size == 0 {
            break;
        }
        if size != data.len() {
            return Err(anyhow!("read error"));
        }
        file_l.write(&data[0..2])?;
        file_r.write(&data[2..])?;
    }

    Ok(())
}

fn simplest_pcm16le_halfvolumeleft(url: &Path) -> Result<()> {
    let mut file = File::open(url)?;
    let mut file_half = File::create("./asserts/output_pcm_half.pcm")?;
    let mut buf_reader = BufReader::new(file);
    let mut cnt = 0;
    loop {
        // let mut data = vec![0; 4];
        let mut data = BytesMut::with_capacity(4);
        data.put_slice(&[0; 4]);
        let size = buf_reader.read(&mut data)?;
        if size == 0 {
            break;
        }

        let mut left_sample = data.get_u16();

        left_sample = left_sample / 2;
        println!("{}", left_sample.to_ne_bytes().len());
        file_half.write(&left_sample.to_ne_bytes())?;
        file_half.write(&data)?;
        // let right_sample = data.get_u16();

        // data.put_u16(left_sample);
        // file_half.write(&left_sample.to_be_bytes())?;

        // data.put_u16(right_sample);
        // file_half.write(&right_sample.to_be_bytes())?;
        // println!("{}", data.len());
        // let half_data: Vec<u8> = data.iter().map(|signal| return *signal / 2).collect();
        // file_half.write(&data)?;
        cnt += 1;
    }
    println!("{:?}", cnt);
    Ok(())
}
fn main() -> Result<()> {
    // split pcm16le
    //simplest_pcm16le_split(Path::new("./asserts/NocturneNo2inEflat_44.1k_s16le.pcm"))?;

    // 减低一半音量
    simplest_pcm16le_halfvolumeleft(Path::new("./asserts/NocturneNo2inEflat_44.1k_s16le.pcm"))?;
    Ok(())
}
