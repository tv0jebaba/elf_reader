use::std::fs;
use::thiserror::Error;




#[derive(Error,Debug)]
pub enum Error {
    #[error("Can't read")]
    Io(#[from]std::io::Error),
    #[error("Reading error, bad path?")]
    ReadingErr,
    #[error("Header must be 64 bytes")]
    TooSmall,
}

#[repr(C)]
#[derive(Debug)]

struct Header64 {
    e_ident: [u8; 16],  //Magic number and other info       16 - 0x00 
    e_type: u16,        // Object file type                  2 - 0x10
    e_machine: u16,     // Architecture                      2 - 0x12
    e_version: u32,     // Object file version               4 - 0x14
    e_entry: u64,       // Entry point virtual adress        8 - 0x18
    e_phoff: u64,       // Program header table file offset  8 - 0x20
    e_shoff: u64,       // Section header table file offset  8 - 0x28
    e_flags: u32,       // Processor specific flags          4 - 0x30
    e_ehsize: u16,      // ELF header size in bytes          2 - 0x34
    e_phentsize: u16,   // Program Header table entry size   2 - 0x36
    e_phnum: u16,       // Program header entry count        2 - 0x38
    e_shentsize: u16,   // Section header table entry size   2 - 0x3A
    e_shnum: u16,       // Section header table entry count  2 - 0x3C
    e_shstrndx: u16,    // Section header string table index 2 - 0x3E


}


fn main () {
match read(){
    Ok(_) => println!("OK"),
        Err(e) => println!("{:?}", e),
}

}

fn read () -> Result<(), Error> {
    let data: Vec<u8> = fs::read("target/release/test_projekt")?;
    
 
    //let header = &data[0..63]; 
    let header = parse_data(&data[0..64]);
    println!("{:?}", header); 


Ok(()) 

}
 
 fn parse_data (data: &[u8]) -> Result<Header64, Error> {
    if data.len() < 64 {
        return Err(Error::TooSmall);
    }

    Ok(Header64 {
        e_ident:data[0..16].try_into().unwrap(),
        e_type: u16::from_le_bytes(data[16..18].try_into().unwrap()),
        e_machine: u16::from_le_bytes(data[18..20].try_into().unwrap()),
        e_version: u32::from_le_bytes(data[20..24].try_into().unwrap()),
        e_entry: u64::from_le_bytes(data[24..32].try_into().unwrap()),
        e_phoff: u64::from_le_bytes(data[32..40].try_into().unwrap()),
        e_shoff: u64::from_le_bytes(data[40..48].try_into().unwrap()),
        e_flags: u32::from_le_bytes(data[48..52].try_into().unwrap()),
        e_ehsize: u16::from_le_bytes(data[52..54].try_into().unwrap()),
        e_phentsize: u16::from_le_bytes(data[54..56].try_into().unwrap()),
        e_phnum: u16::from_le_bytes(data[56..58].try_into().unwrap()),
        e_shentsize: u16::from_le_bytes(data[58..60].try_into().unwrap()),
        e_shnum: u16::from_le_bytes(data[60..62].try_into().unwrap()),
        e_shstrndx: u16::from_le_bytes(data[62..64].try_into().unwrap())
            


    })


 }


