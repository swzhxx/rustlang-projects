use ash::{
    version::{EntryV1_0, InstanceV1_0},
    vk,
};

unsafe extern "system" fn vulkan_debug_utils_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _p_user_data: *mut std::ffi::c_void,
) -> vk::Bool32 {
    let message = std::ffi::CStr::from_ptr((*p_callback_data).p_message);
    let severity = format!("{:?}", message_severity).to_lowercase();
    let ty = format!("{:?}", message_type).to_lowercase();
    println!("[Debug][{}][{}] {:?}", severity, ty, message);
    vk::FALSE
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let entry = ash::Entry::new()?;
    let instance_create_info = vk::InstanceCreateInfo {
        ..Default::default()
    };
    dbg!(&instance_create_info);
    let instance = unsafe { entry.create_instance(&instance_create_info, None)? };
    unsafe { instance.destroy_instance(None) };
    Ok(())
}
