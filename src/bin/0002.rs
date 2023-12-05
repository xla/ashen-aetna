fn main() -> Result<(), Box<dyn std::error::Error>> {
    let entry = unsafe { ash::Entry::load()? };
    let instance = unsafe { entry.create_instance(&Default::default(), None)? };
    unsafe { instance.destroy_instance(None) };
    Ok(())
}
