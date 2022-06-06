use std::str::from_utf8_unchecked;

mod m68k;

#[derive(Debug)]
struct Rom {
    data: Vec<u8>
}

fn load_rom_from_file(path: &str) -> Option<Rom> {
    let data = std::fs::read(path).ok()?;

    Some(Rom {
        data: data,
    })
}

impl Rom {
    fn console(&self) -> &str {
        unsafe { from_utf8_unchecked(&self.data[0x100..0x110]) }
    }
    fn copyright(&self) -> &str {
        unsafe { from_utf8_unchecked(&self.data[0x110..0x120]) }
    }
    fn title(&self) -> &str {
        unsafe { from_utf8_unchecked(&self.data[0x120..0x150]) }
    }
    fn name_overseas(&self) -> &str {
        unsafe { from_utf8_unchecked(&self.data[0x150..0x180]) }
    }
    fn product_type(&self) -> &str {
        unsafe { from_utf8_unchecked(&self.data[0x180..0x182]) }
    }
    fn product_code(&self) -> &str {
        unsafe { from_utf8_unchecked(&self.data[0x182..0x18E]) }
    }
    fn checksum(&self) -> u16 {
        let mut chk = (self.data[0x18E] as u16) << 8;
        chk += self.data[0x18F] as u16;
        chk
    }
    fn start(&self) -> usize {
        u32::from_be_bytes(self.data[0x1A0..0x1A4].try_into()
            .expect("error casting rom data into start address")) as usize
    }
    fn end(&self) -> usize {
        u32::from_be_bytes(self.data[0x1A4..0x1A8].try_into()
            .expect("error casting rom data into end address")) as usize
    }
    fn ram_start(&self) -> usize {
        u32::from_be_bytes(self.data[0x1A8..0x1AC].try_into()
            .expect("error casting rom data to ram start address")) as usize
    }
    fn ram_end(&self) -> usize {
        u32::from_be_bytes(self.data[0x1AC..0x1B0].try_into()
            .expect("error casting rom data to ram end address")) as usize
    }
    fn valid_checksum(&self) -> bool {
        let mut sum = 0u16;
        for i in (0x200..self.data.len()).step_by(2) {
            let (result, _) = (self.data[i] as u16).overflowing_shl(8);
            let result = result + self.data[i + 1] as u16;
            let (result, _) = sum.overflowing_add(result);
            sum = result;
        }
        sum == self.checksum()
    }
}

fn main() -> Result<(), &'static str> {
    for op_code in m68k::OP_CODES {
        println!("{}", op_code);
    }
    println!("Hello, world!");

    let rom = load_rom_from_file("sonic.md").ok_or("could not load rom")?;

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
