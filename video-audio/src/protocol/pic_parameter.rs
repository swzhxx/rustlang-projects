use super::bit_stream::BitStream;

#[derive(Default)]
pub struct PicParameter {
    pic_parameter_set_id: usize,
    seq_parameter_set_id: usize,
    entropy_coding_mode_flag: u8,
    pic_order_present_flag: u8,
    num_slice_groups_minus1: usize,
    slice_group_map_type: usize,
    run_length_minus1: Vec<usize>,
    top_left: Vec<usize>,
    bottom_right: Vec<usize>,
    slice_group_change_direction_flag: u8,
    slice_group_change_rate_minus1: usize,
    pic_size_in_map_units_minus: usize,
    slice_group_id: Vec<u8>,
    num_ref_idx_10_active_minus1: usize,
    num_ref_idx_11_active_minus1: usize,
    weighted_pred_flag: u8,
    weighted_bipred_dic: u8,
    pic_init_qp_minus26: i64,
    pic_init_qs_minus26: i64,
    chroma_qp_index_offset: i64,
    deblocking_filter_control_present_flag: u8,
    constrained_intra_pred_flag: u8,
    redundant_pic_cnt_present_flag: u8,
}

impl From<&BitStream<'_>> for PicParameter {
    fn from(bs: &BitStream<'_>) -> Self {
        let mut pps = Self::default();
        let mut bits_number_of_each_slice_group_id = 0;

        pps.pic_parameter_set_id = bs.get_ue();
        pps.seq_parameter_set_id = bs.get_ue();
        pps.entropy_coding_mode_flag = bs.get_one_bit();
        pps.pic_order_present_flag = bs.get_one_bit();
        pps.num_slice_groups_minus1 = bs.get_ue();
        if pps.num_slice_groups_minus1 > 0 {
            pps.slice_group_map_type = bs.get_ue();
            if pps.slice_group_map_type == 0 {
                for i in 0..pps.num_slice_groups_minus1 {
                    pps.run_length_minus1[i] = bs.get_ue();
                }
            } else if pps.slice_group_map_type == 2 {
                for i in 0..pps.num_slice_groups_minus1 {
                    pps.top_left[i] = bs.get_ue();
                    pps.bottom_right[i] = bs.get_ue();
                }
            } else if (pps.slice_group_map_type == 3
                || pps.slice_group_map_type == 4
                || pps.slice_group_map_type == 5)
            {
                pps.slice_group_change_direction_flag = bs.get_one_bit();
                pps.slice_group_change_rate_minus1 = bs.get_ue();
            } else if (pps.slice_group_map_type == 6) {
                pps.pic_size_in_map_units_minus = bs.get_ue();
                if pps.num_slice_groups_minus1 + 1 > 4 {
                    bits_number_of_each_slice_group_id = 3;
                } else if pps.num_slice_groups_minus1 + 1 > 2 {
                    bits_number_of_each_slice_group_id = 2;
                } else {
                    bits_number_of_each_slice_group_id = 1
                }

                for i in 0..pps.pic_size_in_map_units_minus {
                    pps.slice_group_id[i] =
                        bs.get_n_bit(bits_number_of_each_slice_group_id as usize) as u8;
                }
            }
        }
        pps.num_ref_idx_10_active_minus1 = bs.get_ue();
        pps.num_ref_idx_11_active_minus1 = bs.get_ue();
        pps.weighted_pred_flag = bs.get_one_bit();
        pps.weighted_bipred_dic = bs.get_n_bit(2) as u8;
        pps.pic_init_qp_minus26 = bs.get_se();
        pps.pic_init_qs_minus26 = bs.get_se();
        pps.chroma_qp_index_offset = bs.get_se();
        pps.deblocking_filter_control_present_flag = bs.get_one_bit();
        pps.constrained_intra_pred_flag = bs.get_one_bit();
        pps.redundant_pic_cnt_present_flag = bs.get_one_bit();

        let rbsp_stop_one_bit = bs.get_one_bit();
        while !bs.is_aligned() {
            bs.get_one_bit();
        }
        pps
    }
}
