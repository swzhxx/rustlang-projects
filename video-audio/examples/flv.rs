use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use anyhow::{anyhow, Result};
use bytes::{Buf, BufMut, BytesMut};

#[derive(Debug, Default)]

struct FlvHeader {
    signature: [u8; 3],
    version: u8,
    flags: u8,
    data_offset: u32,
}

impl From<&mut BytesMut> for FlvHeader {
    fn from(bytes: &mut BytesMut) -> Self {
        let signature_version = bytes.get_u32();
        let signature = (signature_version & 0xFFFFFF00).to_be_bytes();
        let version = (signature_version & 0x0000000F) as u8;
        let flags = bytes.get_u8();
        let data_offset = bytes.get_u32();

        Self {
            signature: *b"FLV",
            version,
            flags,
            data_offset,
        }
    }
}

const FLV_HEADER_SIZE: usize = 9;

#[derive(Debug)]
struct VideoTag {
    frame_type: VideoFrameType,
    encode_type: VideoEnCodeType,
}

impl From<&mut BytesMut> for VideoTag {
    fn from(bytes: &mut BytesMut) -> Self {
        let frame_encode_type = bytes.get_u8();
        let frame_type = match frame_encode_type >> 4 {
            1 => VideoFrameType::KeyFrame,
            2 => VideoFrameType::InterFrame,
            3 => VideoFrameType::DisposableInterFrame,
            4 => VideoFrameType::GeneratedKeyFrame,
            5 => VideoFrameType::VideoInfoFrame,
            _ => unreachable!(),
        };

        let encode_type = match (frame_encode_type & 0x0F) {
            1 => VideoEnCodeType::JPEG,
            2 => VideoEnCodeType::SorensonH263,
            3 => VideoEnCodeType::ScreenVideo,
            4 => VideoEnCodeType::On2VP6,
            5 => VideoEnCodeType::On2VP6WithAlphaChannel,
            6 => VideoEnCodeType::ScreenVideoVersion2,
            7 => VideoEnCodeType::AVC,
            _ => unreachable!(),
        };

        VideoTag {
            frame_type,
            encode_type,
        }
    }
}

#[derive(Debug)]
struct AudioTag {
    encode_type: AudioEncodeType,
    sample_frequency: AudioSampleFrequency,
    sample_precision: AudioSamplePrecision,
    audio_type: AudioType,
}

impl From<&mut BytesMut> for AudioTag {
    fn from(bytes: &mut BytesMut) -> Self {
        let data = bytes.get_u8();
        let encode_type = match (data >> 4) {
            0 => AudioEncodeType::LinearPCMPlatformEndian,
            1 => AudioEncodeType::ADPCM,
            2 => AudioEncodeType::MP3,
            3 => AudioEncodeType::LinearPCMLitterEndian,
            4 => AudioEncodeType::Nelly16kHzMono,
            5 => AudioEncodeType::Nelly8kHzMono,
            6 => AudioEncodeType::Nelly,
            7 => AudioEncodeType::G711AlawPCM,
            8 => AudioEncodeType::G711MulawPC,
            9 => AudioEncodeType::REVERSED,
            10 => AudioEncodeType::AAC,

            14 => AudioEncodeType::MP38kHz,
            15 => AudioEncodeType::DeviceSpecialSound,
            _ => {
                unreachable!()
            }
        };

        let sample_frequency = match (data & 0x03 >> 2) {
            0 => AudioSampleFrequency::Hz5k,
            1 => AudioSampleFrequency::Hz11k,
            2 => AudioSampleFrequency::Hz22k,
            3 => AudioSampleFrequency::Hz44k,
            _ => unreachable!(),
        };

        let sample_precision = match ((data & 0x02) >> 1) {
            0 => AudioSamplePrecision::bits8,
            1 => AudioSamplePrecision::bits16,
            _ => unreachable!(),
        };

        let audio_type = match (data & 0x01) {
            0 => AudioType::SandMono,
            1 => AudioType::SandStetreo,
            _ => unreachable!(),
        };

        AudioTag {
            encode_type,
            sample_frequency,
            sample_precision,
            audio_type,
        }
    }
}

#[derive(Debug)]
enum TagType {
    AUDIO = 0x08,
    VIDEO = 0x09,
    SCRIPT = 0x12,
    UNKNOWN,
}

#[derive(Debug)]

struct TagHeader {
    tag_type: TagType,
    data_size: u32,
    time_stamp: u32,
    time_stamp_ex: u8,
    // resverved: u32,
    stream_id: u32,
}

#[derive(Debug)]
enum VideoFrameType {
    KeyFrame = 1,
    InterFrame = 2,
    DisposableInterFrame = 3,
    GeneratedKeyFrame = 4,
    VideoInfoFrame = 5,
}

#[derive(Debug)]

enum VideoEnCodeType {
    JPEG = 1,
    SorensonH263 = 2,
    ScreenVideo = 3,
    On2VP6 = 4,
    On2VP6WithAlphaChannel = 5,
    ScreenVideoVersion2 = 6,
    AVC = 7,
}

#[derive(Debug)]
enum AudioEncodeType {
    LinearPCMPlatformEndian = 0,
    ADPCM = 1,
    MP3 = 2,
    LinearPCMLitterEndian = 3,
    Nelly16kHzMono = 4,
    Nelly8kHzMono = 5,
    Nelly = 6,
    G711AlawPCM = 7,
    G711MulawPC = 8,
    REVERSED = 9,
    AAC = 10,
    MP38kHz = 14,
    DeviceSpecialSound = 15,
}

#[derive(Debug)]
enum AudioSampleFrequency {
    Hz5k = 0,
    Hz11k = 1,
    Hz22k = 2,
    Hz44k = 3,
}

#[derive(Debug)]
enum AudioSamplePrecision {
    bits8 = 0,
    bits16 = 1,
}

#[derive(Debug)]
enum AudioType {
    SandMono = 0,
    SandStetreo = 1,
}

impl From<&mut BytesMut> for TagHeader {
    fn from(bytes: &mut BytesMut) -> Self {
        let tag_type = bytes.get_u8();
        let tag_type = match tag_type {
            0x8 => TagType::AUDIO,
            0x9 => TagType::VIDEO,
            0x12 => TagType::SCRIPT,
            _ => TagType::UNKNOWN,
        };
        let data_size = bytes.get(0..3).unwrap();
        let data_size =
            data_size[0] as u32 * 65536 + data_size[1] as u32 * 256 + data_size[2] as u32;
        bytes.advance(3);
        let time_stamp = bytes.get(0..3).unwrap();
        let time_stamp =
            time_stamp[0] as u32 * 65536 + time_stamp[1] as u32 * 256 + time_stamp[2] as u32;
        bytes.advance(3);
        let time_stamp_ex = bytes.get_u8();
        let _stream_id = bytes.get(0..3).unwrap();

        TagHeader {
            tag_type,
            data_size: data_size,
            time_stamp,
            time_stamp_ex,
            stream_id: 0,
        }
    }
}

fn simplest_flv_parser<P>(path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let output_a = 1;
    let output_v = 1;

    // let mut flv = FlvHeader::default();
    // let tag_header = TagHeader::default();

    let file = File::open(path)?;
    let mut previous_tag_size = 0u32;
    let mut previous_tags_size_z = 0u32;

    let mut reader = BufReader::new(file);

    // let header_size = std::mem::size_of::<FlvHeader>();
    let mut header_data = BytesMut::with_capacity(FLV_HEADER_SIZE);
    header_data.put_slice(&vec![0; FLV_HEADER_SIZE]);

    let size = reader.read(&mut header_data)?;
    if size != header_data.len() {
        return Err(anyhow!("read error"));
    }
    let flv = FlvHeader::from(&mut header_data);

    println!("***************FLV HEADER************************");
    println!(
        "Signature   :{:?}",
        std::str::from_utf8(&flv.signature).unwrap()
    );
    println!("Version   :{:?}", flv.version);
    println!("Flags   :{:?}", flv.flags);
    println!("HeaderSize :{:?}", flv.data_offset);
    // println!("{:?}", flv);

    loop {
        let mut previous_tag_buffer = BytesMut::new();
        previous_tag_buffer.put_slice(&[0; 4]);
        let read_size = reader.read(&mut previous_tag_buffer)?;
        if read_size == 0 {
            break;
        }
        let _previous_tag_size = previous_tag_buffer.get_u32_le();

        let mut tag_header_buffer = BytesMut::new();
        tag_header_buffer.put_slice(&[0; 11]);
        let read_size = reader.read(&mut tag_header_buffer)?;
        if read_size != tag_header_buffer.len() {
            return Err(anyhow!("read error"));
        }
        let tag_header = TagHeader::from(&mut tag_header_buffer);
        println!(
            " [{:6?}] {:6?} {:6?} |",
            tag_header.tag_type, tag_header.data_size, tag_header.time_stamp
        );

        match tag_header.tag_type {
            TagType::AUDIO => {
                let mut buffer = BytesMut::new();
                buffer.put_slice(&[0; 1]);
                reader.read(&mut buffer)?;
                let audio_header_tag = AudioTag::from(&mut buffer);
                println!("{:?}", audio_header_tag);
                // tag_data 略过
                reader.seek_relative(tag_header.data_size as i64 - 1)?;
            }
            TagType::VIDEO => {
                let mut buffer = BytesMut::new();
                buffer.put_slice(&[0; 1]);
                reader.read(&mut buffer)?;
                let video_header_tag = VideoTag::from(&mut buffer);
                println!("{:?}", video_header_tag);
                // tag_data 略过
                reader.seek_relative(tag_header.data_size as i64 - 1)?;
            }
            _ => {
                reader.seek_relative(tag_header.data_size as i64)?;
            }
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    simplest_flv_parser("./asserts/cuc_ieschool.flv")?;
    Ok(())
}
