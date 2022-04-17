use super::bit_stream::BitStream;

#[derive(Default)]
pub struct SeqParamter {
    profile_idc: u8,
    /*---编码级别的制约条件START---*/
    constraint_sec0_flag: u8,
    constraint_sec1_flag: u8,
    constraint_set2_flag: u8,
    reserved_zero_5bits: u8,
    /*---编码级别的制约条件END-----*/
    level_idc: u8,
    seq_parameter_set_id: usize,
    log2_max_frame_num_minus4: usize,
    pic_order_cnt_type: usize,
    /*---用来计算POC的句法元素START--- */
    log2_max_pic_order_cnt_lsb_minus4: usize,
    delta_pic_order_always_zero_flag: u8,
    offset_for_non_ref_pic: i64,
    offset_for_top_bottom_field: i64,
    num_ref_frames_in_pic_order_cnt_type: i64,
    offset_for_ref_frame: Vec<i64>,
    /*---用来计算POC的句法元素END-------- */
    num_ref_frames: usize,
    gaps_in_frame_num_value_allowed_flag: u8,
    /*----图像宽高相关START--------- */
    pic_width_in_mbs_minus1: usize,
    pic_height_in_map_units_minus1: usize,
    frame_mbs_only_flag: u8,
    mb_adaptive_frame_field_flag: u8,
    /*----图像宽高相关相关END---------- */
    direct_8x8_interface_flag: u8,
    /*---解码后图像剪裁的几个句法元素START---*/
    frame_cropping_flag: u8,
    frame_crop_left_offset: usize,
    frame_crop_right_offset: usize,
    frame_crop_top_offset: usize,
    frame_crop_bottom_offset: usize,
    /*---解码后图像剪裁的几个句法元素END---*/
    vui_parameters_present_flag: u8,
    vui_parameters: Option<VuiParameters>,
}

impl From<&BitStream<'_>> for SeqParamter {
    fn from(bit_stream: &BitStream) -> Self {
        let mut sqs = Self::default();
        sqs.profile_idc = bit_stream.get_one_bit();
        sqs.constraint_sec0_flag = bit_stream.get_one_bit();
        sqs.constraint_sec1_flag = bit_stream.get_one_bit();
        sqs.constraint_set2_flag = bit_stream.get_one_bit();
        sqs.reserved_zero_5bits = bit_stream.get_n_bit(5) as u8;
        sqs.level_idc = bit_stream.get_one_bit();
        sqs.seq_parameter_set_id = bit_stream.get_ue();
        sqs.log2_max_frame_num_minus4 = bit_stream.get_ue();
        sqs.pic_order_cnt_type = bit_stream.get_ue();
        if sqs.pic_order_cnt_type == 0 {
            sqs.log2_max_pic_order_cnt_lsb_minus4 = bit_stream.get_ue();
        } else if (sqs.pic_order_cnt_type == 1) {
            sqs.delta_pic_order_always_zero_flag = bit_stream.get_one_bit();
            sqs.offset_for_non_ref_pic = bit_stream.get_se();
            sqs.offset_for_top_bottom_field = bit_stream.get_se();
            sqs.num_ref_frames_in_pic_order_cnt_type = bit_stream.get_se();
            for i in 0..sqs.num_ref_frames_in_pic_order_cnt_type {
                let index = i as usize;
                sqs.offset_for_ref_frame[index] = bit_stream.get_se();
            }
        }
        sqs.num_ref_frames = bit_stream.get_ue();
        sqs.gaps_in_frame_num_value_allowed_flag = bit_stream.get_ue() as u8;
        sqs.pic_width_in_mbs_minus1 = bit_stream.get_ue();
        sqs.pic_height_in_map_units_minus1 = bit_stream.get_ue();
        sqs.frame_mbs_only_flag = bit_stream.get_one_bit();
        if sqs.frame_mbs_only_flag == 0 {
            sqs.mb_adaptive_frame_field_flag = bit_stream.get_one_bit();
        }
        sqs.direct_8x8_interface_flag = bit_stream.get_one_bit();
        sqs.frame_cropping_flag = bit_stream.get_one_bit();
        if sqs.frame_cropping_flag > 0 {
            sqs.frame_crop_left_offset = bit_stream.get_ue();
            sqs.frame_crop_right_offset = bit_stream.get_ue();
            sqs.frame_crop_top_offset = bit_stream.get_ue();
            sqs.frame_crop_bottom_offset = bit_stream.get_ue();
        }
        sqs.vui_parameters_present_flag = bit_stream.get_one_bit();
        // vui_parameters
        if sqs.vui_parameters_present_flag > 0 {
            sqs.vui_parameters = Some(VuiParameters::from(bit_stream))
        }
        let _rbsp_stop_one_bit = bit_stream.get_one_bit();
        while !bit_stream.is_aligned() {
            bit_stream.get_one_bit();
        }
        sqs
    }
}

#[derive(Default)]
pub struct VuiParameters {
    aspect_ratio_info_present_flag: u8,
    aspect_ratio_idc: u8,
    sar_width: u16,
    sar_height: u16,
    overscan_info_present_flag: u8,
    overscan_appropriate_flag: u8,
    video_singal_type_present_flag: u8,
    video_format: u8,
    video_full_range_flag: u8,
    colour_description_present_flag: u8,
    colour_primaries: u8,
    transfer_characteristics: u8,
    matrix_coefficients: u8,
    chroma_loc_info_present_flag: u8,
    chroma_sample_loc_type_top_field: usize,
    chroma_sample_loc_type_bottom_field: usize,
    timing_info_present_flag: u8,
    num_units_in_tick: u32,
    time_scale: u32,
    fixed_frame_rate_flag: u8,
    nal_hrd_parameters_present_flag: u8,
    nal_hrd_parameters: Option<HrdParameters>,
    vcl_hrd_parameters_present_flag: u8,
    vcl_hrd_parameters: Option<HrdParameters>,
    low_delay_hrd_flag: u8,
    pic_struct_present_flag: u8,
    bitstream_restriction_flag: u8,
    motion_vectors_over_pic_boundaries_flag: u8,
    max_bytes_per_pic_denom: usize,
    max_bits_per_mb_denom: usize,
    log2_max_mv_length_horizontal: usize,
    log2_max_mv_length_vertical: usize,
    max_num_reorder_frames: usize,
    max_dec_frame_buffering: usize,
}

impl From<&BitStream<'_>> for VuiParameters {
    fn from(bit_stream: &BitStream<'_>) -> Self {
        todo!()
    }
}

#[derive(Default)]
pub struct HrdParameters {
    cpb_cnt_minus1: usize,
    bit_rate_scale: u8,
    cpb_size_scale: u8,
    bit_rate_value_minus1: Vec<usize>,
    cpb_size_value_minus: Vec<usize>,
    cbr_flag: Vec<u8>,
    initial_cpb_removal_delay_length_minus1: u8,
    cpb_removal_delay_length_minus1: u8,
    dpb_output_delay_length_minus1: u8,
    time_offset_length: u8,
}

impl From<&BitStream<'_>> for HrdParameters {
    fn from(_: &BitStream<'_>) -> Self {
        todo!()
    }
}
