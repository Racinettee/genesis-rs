use std::{str::{from_utf8_unchecked, FromStr}};

use super::rom_fmt::Format;

// Ref: https://www.zophar.net/fileuploads/2/10614uauyw/Genesis_ROM_Format.txt
#[derive(Debug)]
pub struct Rom {
    pub data: Vec<u8>,
    pub format: Format,
}

pub fn load_from_file(path: &str) -> Option<Rom> {
    let path = std::path::Path::new(path);
    let format = Format::from_str(path.extension()?.to_str()?).ok()?;
    let data = std::fs::read(path).ok()?;
    Some(Rom {
        data: data,
        format: format,
    })
}

impl Rom {
    pub fn console(&self) -> &str {
        unsafe { from_utf8_unchecked(&self.data[0x100..0x110]) }
    }
    pub fn copyright(&self) -> &str {
        unsafe { from_utf8_unchecked(&self.data[0x110..0x120]) }
    }
    pub fn title(&self) -> &str {
        unsafe { from_utf8_unchecked(&self.data[0x120..0x150]) }
    }
    pub fn name_overseas(&self) -> &str {
        unsafe { from_utf8_unchecked(&self.data[0x150..0x180]) }
    }
    pub fn product_type(&self) -> &str {
        unsafe { from_utf8_unchecked(&self.data[0x180..0x182]) }
    }
    pub fn product_code(&self) -> &str {
        unsafe { from_utf8_unchecked(&self.data[0x182..0x18E]) }
    }
    pub fn checksum(&self) -> u16 {
        let mut chk = (self.data[0x18E] as u16) << 8;
        chk += self.data[0x18F] as u16;
        chk
    }
    pub fn start(&self) -> usize {
        u32::from_be_bytes(self.data[0x1A0..0x1A4].try_into()
            .expect("error casting rom data into start address")) as usize
    }
    pub fn end(&self) -> usize {
        u32::from_be_bytes(self.data[0x1A4..0x1A8].try_into()
            .expect("error casting rom data into end address")) as usize
    }
    pub fn ram_start(&self) -> usize {
        u32::from_be_bytes(self.data[0x1A8..0x1AC].try_into()
            .expect("error casting rom data to ram start address")) as usize
    }
    pub fn ram_end(&self) -> usize {
        u32::from_be_bytes(self.data[0x1AC..0x1B0].try_into()
            .expect("error casting rom data to ram end address")) as usize
    }
    pub fn valid_checksum(&self) -> bool {
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