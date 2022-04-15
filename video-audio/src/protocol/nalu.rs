use anyhow::anyhow;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::io::BufReader;

#[derive(Debug)]
pub enum NaluType {
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

impl TryFrom<u8> for NaluType {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> anyhow::Result<Self> {
        match value {
            1 => Ok(NaluType::NALU_TYPE_SLICE),
            2 => Ok(NaluType::NALU_TYPE_DPA),
            3 => Ok(NaluType::NALU_TYPE_DPB),
            4 => Ok(NaluType::NALU_TYPE_DPC),
            5 => Ok(NaluType::NALU_TYPE_IDR),
            6 => Ok(NaluType::NALU_TYPE_SEI),
            7 => Ok(NaluType::NALU_TYPE_SPS),
            8 => Ok(NaluType::NALU_TYPE_PPS),
            9 => Ok(NaluType::NALU_TYPE_AUD),
            10 => Ok(NaluType::NALU_TYPE_EOSEQ),
            11 => Ok(NaluType::NALU_TYPE_EOSTREAM),
            12 => Ok(NaluType::NALU_TYPE_FILL),
            _ => Err(anyhow!("try from nalu type error")),
        }
    }
}

// impl<T> TryFrom<T> for NaluType
// where
//     T: Into<u8>,
// {
//     type Error = anyhow::Error;
//     fn try_from(value: T) -> anyhow::Result<Self> {
//         let num: u8 = value.into();
//         match num {
//             1 => Ok(NaluType::NALU_TYPE_SLICE),
//             2 => Ok(NaluType::NALU_TYPE_DPA),
//             3 => Ok(NaluType::NALU_TYPE_DPB),
//             4 => Ok(NaluType::NALU_TYPE_DPC),
//             5 => Ok(NaluType::NALU_TYPE_IDR),
//             6 => Ok(NaluType::NALU_TYPE_SEI),
//             7 => Ok(NaluType::NALU_TYPE_SPS),
//             8 => Ok(NaluType::NALU_TYPE_PPS),
//             9 => Ok(NaluType::NALU_TYPE_AUD),
//             10 => Ok(NaluType::NALU_TYPE_EOSEQ),
//             11 => Ok(NaluType::NALU_TYPE_EOSTREAM),
//             12 => Ok(NaluType::NALU_TYPE_FILL),
//             _ => Err(anyhow!("try from nalu type error")),
//         }
//     }
// }

pub struct Nalu<'a> {
    forbidden_zero_bit: u8,
    nal_ref_idc: u8,
    nal_unit_type: NaluType,
    rbsp: &'a [u8],
}

impl<'a, 'b> TryFrom<&'b [u8]> for Nalu<'a>
where
    'b: 'a,
{
    type Error = anyhow::Error;
    fn try_from(data: &'b [u8]) -> anyhow::Result<Self> {
        let mut buffer = BytesMut::new();
        buffer.put_slice(data);
        let byte = buffer.get_u8();
        let forbidden_zero_bit = (byte & 0b10000000) >> 7;
        let nal_ref_idc = (byte & 0b01100000) >> 5;
        let nal_unit_type: NaluType = (byte & 0b00011111).try_into()?;
        let (_, rbsp) = data.split_at(0);

        Ok(Self {
            forbidden_zero_bit,
            nal_ref_idc,
            nal_unit_type,
            rbsp,
        })
    }
}
