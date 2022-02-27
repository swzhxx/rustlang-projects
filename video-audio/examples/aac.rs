use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use anyhow::{anyhow, Result};
use bytes::{Buf, BufMut, BytesMut};

fn simplest_acc_parser<P>(path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let mut data_size = 0;
    let size = 0;
    let mut cnt = 0;
    let mut offset = 0;

    let mut acc_frame = BytesMut::with_capacity(1024 * 5);
    acc_frame.put_slice(&[0; 1024 * 5]);

    let mut acc_buffer = BytesMut::with_capacity(1024 * 1024);
    acc_buffer.put_slice(&[0; 1024 * 1024]);

    let input_file = File::open(path)?;
    let mut reader = BufReader::new(input_file);
    println!(" -----+- ADTS FRAME TABLE -+--------------");
    println!("  NUM  |  Profile  |  Frequency  |  Size  |");
    println!(" -----+------------+-------------+--------+");

    data_size = reader.read(&mut acc_buffer)? as i32;
    let input_data = acc_buffer.clone();
    loop {
        let _ = match get_adts_frame(&mut acc_buffer, data_size, &mut acc_frame, size) {
            Ok(ret) => {
                if ret == 1 {
                    offset = data_size;
                    acc_buffer.copy_from_slice(input_data.split_at(data_size as usize).0);
                    break;
                } else {
                    acc_buffer.get_uint(ret as usize);

                    let mut profile = acc_frame[2] & 0xc0;
                    profile = profile >> 6;
                    let profile_str = match profile {
                        0 => "Main",
                        1 => "Lc",
                        2 => "SSR",
                        _ => "unknown",
                    };

                    let mut sampling_frequency_index = acc_frame[2] & 0x3c;
                    sampling_frequency_index = sampling_frequency_index >> 2;
                    let frequencey_str = match sampling_frequency_index {
                        0 => "96000HZ",
                        1 => "88200HZ",
                        2 => "64000HZ",
                        3 => "48000HZ",
                        4 => "44100HZ",
                        5 => "32000HZ",
                        6 => "24000HZ",
                        7 => "22050HZ",
                        8 => "16000HZ",
                        9 => "12000HZ",
                        10 => "11025HZ",
                        11 => "8000HZ",
                        _ => "unknown",
                    };

                    println!(
                        "{:5} {:8} {:8} {:5}",
                        cnt, profile_str, frequencey_str, size
                    );
                }
            }
            Err(err) => {
                println!("{:?}", err);
                break;
            }
        };
        cnt += 1;
    }

    Ok(())
}

fn get_adts_frame(
    buffer: &mut [u8],
    buf_size: i32,
    data: &mut [u8],
    data_size: i32,
) -> Result<i32> {
    let mut size = 0 as u32;
    let mut data_size = data_size;
    let mut buf_size = buf_size;
    let mut buffer = buffer;
    if buffer.len() == 0 || data.len() == 0 || data_size == 0 {
        return Err(anyhow!("get_adts_frame 0 "));
    }

    loop {
        if buf_size < 7 {
            return Err(anyhow!("get_adts_frame 0 "));
        }

        if buffer[0] == 0xff && (buffer[0] & 0xf0) == 0xf0 {
            size |= ((buffer[3] as u32) & 0x03) << 11;
            size |= (buffer[4] as u32) << 3;
            size != ((buffer[5] as u32) & 0xe0) >> 5;
            break;
        }
        buf_size -= 1;
        buffer = &mut buffer[1..];
    }

    if (buf_size as u32) < size {
        return Ok(1);
    }

    Ok(data_size)
}

fn main() -> Result<()> {
    simplest_acc_parser("./asserts/nocturne.aac")?;
    Ok(())
}
