fn main() -> Result<(), Box<dyn std::error::Error>> {
    let hardware = hwinfo_rs::collect()?;
    println!("{hardware:#?}");
    Ok(())
}
