pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn color_255_to_f32(data: &[u8]) -> Vec<f32> {
    let result = data.iter().map(|v| (*v as f32 / 255.)).collect();
    result
}

pub fn color_f32_to_255(data: &[f32]) -> Vec<u8> {
    let result = data.iter().map(|v| (v * 255.) as u8).collect();
    result
}
