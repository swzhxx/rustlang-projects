use std::collections::HashMap;

use super::{
    bit_stream::BitStream,
    nalu::{Nalu, NaluType},
    pic_parameter::PicParameter,
    seq_parameter::SeqParameter,
};

#[derive(Default)]
pub struct SliceHeader {
    first_mb_in_slice: usize,
    slice_type: usize,
    pic_parameter_set_id: usize,
    frame_num: usize,
    field_pic_flag: u8,
    bottom_field_flag: u8,
    idr_pic_id: usize,
    pic_order_cnt_lsb: usize,
    delta_pic_order_cnt_bottom: i64,
    delta_pic_order_cnt: Vec<i64>,
    redundant_pic_cnt: usize,
    direct_spatial_mv_pred_flag: u8,
    num_ref_idx_active_override_flag: u8,
    num_ref_idx_10_active_minus1: usize,
    unm_ref_idx_11_activ3_minus1: usize,
    cabac_init_idc: usize,
    slice_qb_delta: i64,
    spl_for_switch_flag: u8,
    slice_qs_delta: usize,
    disable_deblocking_filter_idc: usize,
    slice_alpha_c0_offset_div2: i64,
    slice_beta_offset_div2: i64,
    slice_group_change_cycle: usize,
}

impl SliceHeader {
    fn new(
        bs: &BitStream,
        sqs_map: HashMap<usize, SeqParameter>,
        pps_map: HashMap<usize, PicParameter>,
        nalu: &Nalu,
    ) -> Self {
        let mut slice_header = Self::default();
        slice_header.first_mb_in_slice = bs.get_ue();
        slice_header.slice_type = bs.get_ue();
        slice_header.pic_parameter_set_id = bs.get_ue();
        let pps_default = PicParameter::default();
        let sqs_default = SeqParameter::default();
        let pps = pps_map
            .get(&slice_header.pic_parameter_set_id)
            .unwrap_or(&pps_default);
        let sqs = sqs_map
            .get(&pps.seq_parameter_set_id)
            .unwrap_or(&sqs_default);
        if sqs.frame_mbs_only_flag == 0 {
            slice_header.field_pic_flag = bs.get_one_bit();
            if slice_header.field_pic_flag > 0 {
                slice_header.bottom_field_flag = bs.get_one_bit();
            }
        }
        if nalu.nal_unit_type == NaluType::NALU_TYPE_IDR {
            slice_header.idr_pic_id = bs.get_ue();
        }
        if sqs.pic_order_cnt_type == 0 {
            slice_header.pic_order_cnt_lsb =
                bs.get_n_bit(sqs.log2_max_pic_order_cnt_lsb_minus4 + 4);
            if pps.pic_order_present_flag > 0 && slice_header.field_pic_flag == 0 {
                slice_header.delta_pic_order_cnt_bottom = bs.get_se();
            }
        }

        if sqs.pic_order_cnt_type == 1 && sqs.delta_pic_order_always_zero_flag > 0 {
            slice_header.delta_pic_order_cnt.push(bs.get_se());
            if sqs.pic_order_cnt_type > 0 && slice_header.field_pic_flag == 0 {
                slice_header.delta_pic_order_cnt.push(bs.get_se());
            }
        }

        if pps.redundant_pic_cnt_present_flag > 0 {
            slice_header.redundant_pic_cnt = bs.get_ue();
        }

        //todo
        slice_header
    }
}

pub enum SliceLayer {
    SliceWithoutPartitioning(SliceWithoutPartitioning),
    SliceA(SliceA),
    SliceB(SliceB),
    SliceC(SliceC),
}

#[derive(Default)]
pub struct SliceWithoutPartitioning {
    slice_header: SliceHeader,
    slice_data: SliceData,
}
impl SliceWithoutPartitioning {
    fn new(bs: &BitStream, nalu_type: NaluType) -> SliceWithoutPartitioning {
        todo!()
    }
}

#[derive(Default)]
pub struct SliceA {
    slice_id: usize,
    slice_data: SliceData,
}

#[derive(Default)]
pub struct SliceB {
    slice_id: usize,
    redundant_pic_cnt: usize,
    slice_data: SliceData,
}
#[derive(Default)]
pub struct SliceC {
    slice_id: usize,
    redundant_pic_nat: usize,
    slice_data: SliceData,
}

#[derive(Default)]
pub struct SliceData {}

#[derive(Default)]
pub struct Slice {}
