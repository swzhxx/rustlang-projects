pub struct SeqParamter {
    profile_idc: u8,
    constraint_sec0_flag: u8,
    constraint_sec1_flag: u8,
    constraint_set2_flag: u8,
    reserved_zero_5bits: u8,
    level_idc: u8,
    seq_parameter_set_id: usize,
    log2_max_frame_num_minus4: usize,
    pick_order_cnt_type: usize,
    log2_max_pic_order_cnt_lsb_minus4: usize,
    delta_pic_order_always_zero_flag: u8,
}
