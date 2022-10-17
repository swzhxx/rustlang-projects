use ash::version::{EntryV1_0, InstanceV1_0};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let entry = ash::Entry::new()?;
    let instance = unsafe { entry.create_instance(&Default::default(), None)? };
    unsafe { instance.destroy_instance(None) };
    Ok(())
}
