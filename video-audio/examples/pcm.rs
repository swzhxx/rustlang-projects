use std::{
    fs::File,
    io::{BufRead, BufReader, Read, Write},
    os::raw::c_char,
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
        if size != data.len() {
            return Err(anyhow!("read error"));
        }
        let mut left_sample = data.get_i16_le();
        left_sample = left_sample / 2;
        let right_sample = data.get_i16();
        data.put_i16_le(left_sample);
        data.put_i16(right_sample);
        file_half.write(&data)?;
        // file_half.write(&data)?;
        cnt += 1;
    }
    println!("{:?}", cnt);
    Ok(())
}

fn simplest_pcm16le_doublespeed(url: &Path) -> Result<()> {
    let mut file = File::open(url)?;
    let mut file_double = File::create("./asserts/output_double_speed.pcm")?;
    let mut buf_reader = BufReader::new(file);
    let mut cnt = 0;
    loop {
        let mut data = BytesMut::with_capacity(4);
        data.put_slice(&[0; 4]);
        let size = buf_reader.read(&mut data)?;
        if size == 0 {
            break;
        }
        if size != data.len() {
            return Err(anyhow!("read error"));
        }

        if cnt % 2 == 0 {
            file_double.write(&data)?;
        }

        cnt = cnt + 1;
    }
    Ok(())
}

fn simplest_pcm16le_to_pcm8(url: &Path) -> Result<()> {
    let mut file = File::open(url)?;
    let mut file_8bit = File::create("./asserts/16_to_8.pcm")?;
    let mut buf_reader = BufReader::new(file);
    loop {
        let mut data = BytesMut::with_capacity(4);
        data.put_slice(&[0; 4]);
        let size = buf_reader.read(&mut data)?;
        if size == 0 {
            break;
        }
        if size != data.len() {
            return Err(anyhow!("read error"));
        }
        let samplenum = data.get_i16_le();
        let samplenum = ((samplenum >> 8) + 128) as u8;
        data.put_u8(samplenum);
        let samplenum = data.get_i16_le();
        let samplenum = ((samplenum >> 8) + 128) as u8;
        data.put_u8(samplenum);
        file_8bit.write(&data)?;
    }
    Ok(())
}

#[repr(C)]
struct WaveHeader<'a> {
    fccId: &'a [u8; 4],
    dwSize: u32,
    fccType: &'a [u8; 4],
}
#[repr(C)]
struct WaveFmt<'a> {
    fccId: &'a [u8; 4],
    dwSize: u32,
    wFormatTag: u16,
    wChannels: u16,
    dwSamplesPerSec: u32,
    dwAvgBytesPerSec: u32,
    wBlockAlign: u16,
    uiBitsPerSample: u16,
}
#[repr(C)]
struct WaveData<'a> {
    fccId: &'a [u8; 4],
    dwSize: u32,
    data: Vec<u8>,
}

fn pcm16le_to_wave(path: &Path, channels: Option<usize>, sample_rate: Option<usize>) -> Result<()> {
    let mut pcmHeader = WaveHeader {
        fccId: b"RIFF", //RIFF,
        dwSize: 0,
        fccType: b"WAVE",
    };
    let channels = channels.unwrap_or(2);
    let sample_rate = sample_rate.unwrap_or(44100);
    let mut pcmFmt = WaveFmt {
        fccId: b"fmt ",
        dwSize: 16,
        wFormatTag: 1,
        wChannels: channels as u16,
        dwSamplesPerSec: sample_rate as u32,
        dwAvgBytesPerSec: (sample_rate * std::mem::size_of::<u16>() as usize) as u32,
        wBlockAlign: 2,
        uiBitsPerSample: 16,
    };

    let mut pcmData = WaveData {
        fccId: b"data",
        dwSize: 0,
        data: vec![],
    };
    let mut file = File::open(path)?;
    let mut file_wave = File::create("./asserts/pcm_to_wave.wav")?;
    let mut buf_reader = BufReader::new(file);
    loop {
        let mut data = BytesMut::with_capacity(2);
        data.put_slice(&[0; 2]);
        let size = buf_reader.read(&mut data).unwrap();
        if size == 0 {
            break;
        }
        pcmData.dwSize += size as u32;
        pcmData.data.append(&mut data.to_vec());
    }
    pcmHeader.dwSize = 44 + pcmData.dwSize;
    let mut output = BytesMut::new();
    output.put_slice(pcmHeader.fccId);
    output.put_u32_le(pcmHeader.dwSize);
    output.put_slice(pcmHeader.fccType);

    output.put_slice(pcmFmt.fccId);
    output.put_u32_le(pcmFmt.dwSize);
    output.put_u16_le(pcmFmt.wFormatTag);
    output.put_u16_le(pcmFmt.wChannels);
    output.put_u32_le(pcmFmt.dwSamplesPerSec);
    output.put_u32_le(pcmFmt.dwAvgBytesPerSec);
    output.put_u16_le(pcmFmt.wBlockAlign);
    output.put_u16_le(pcmFmt.uiBitsPerSample);

    output.put_slice(pcmData.fccId);
    output.put_u32_le(pcmData.dwSize);
    output.put_slice(&pcmData.data[..]);

    file_wave.write(&output)?;
    Ok(())
}

fn main() -> Result<()> {
    // split pcm16le
    //simplest_pcm16le_split(Path::new("./asserts/NocturneNo2inEflat_44.1k_s16le.pcm"))?;

    // 减低一半音量
    //simplest_pcm16le_halfvolumeleft(Path::new("./asserts/NocturneNo2inEflat_44.1k_s16le.pcm"))?;

    // 双倍速
    //simplest_pcm16le_doublespeed(Path::new("./asserts/NocturneNo2inEflat_44.1k_s16le.pcm"))?;

    // 16bit -> 8bit
    //simplest_pcm16le_to_pcm8(Path::new("./asserts/NocturneNo2inEflat_44.1k_s16le.pcm"))?;

    // pcm -> wave
    pcm16le_to_wave(
        Path::new("./asserts/NocturneNo2inEflat_44.1k_s16le.pcm"),
        Some(2),
        Some(44100),
    )?;
    Ok(())
}
