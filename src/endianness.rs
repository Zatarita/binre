use std::sync::LazyLock;

#[derive(Debug, Clone, Copy)]
pub enum Endianness {
    Little,
    Big,
}

pub fn swap_inplace(bytes: &mut [u8], endianness: Endianness) {
    match (endianness, *SYS_ENDIANNESS) {
        (Endianness::Little, Endianness::Big) => bytes.reverse(),
        (Endianness::Big, Endianness::Little) => bytes.reverse(),
        (_, _) => {}
    }
}

fn native_endianness() -> Endianness {
    union TestMemory {
        short: u16,
        bytes: [u8; 2],
    }

    let shared_mem: TestMemory = TestMemory { short: 0x1234 };

    match unsafe { shared_mem.bytes[0] } {
        0x34 => Endianness::Little,
        0x12 => Endianness::Big,
        _ => panic!("Invalid value passed to testing memory!"), // This shouldn't be able to happen
    }
}

pub static SYS_ENDIANNESS: LazyLock<Endianness> = LazyLock::new(|| native_endianness());
