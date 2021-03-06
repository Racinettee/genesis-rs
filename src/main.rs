mod m68k;
mod md;

fn main() -> Result<(), &'static str> {
    println!("Registers:");
    for register in m68k::cpu::REGISTER_NAMES {
        println!("{}", register)
    }
    println!("Opcodes:");
    for code in m68k::OP_CODES {
        println!("{}", code)
    }
    let rom = md::rom::load_from_file("sonic.md").ok_or("could not load rom")?;

    println!("ROM: {}, FORMAT: {}", "sonic.md", rom.format);

    println!("Console: {}", rom.console());
    println!("Copyright: {}", rom.copyright());
    println!("Title: {}", rom.title());
    println!("Overseas: {}", rom.name_overseas());
    println!("Product type: {}", rom.product_type());
    println!("Product code: {}", rom.product_code());
    println!("Product type: {}", rom.checksum());
    println!("Valid checksum: {:?}", rom.valid_checksum());
    Ok(())
}
