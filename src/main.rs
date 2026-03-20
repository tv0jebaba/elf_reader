use std::env;
use::std::fs;
use::thiserror::Error;
use std::convert::TryInto;

const EI_MAG0: u8 = 0;		/* File identification byte 0 index */
const ELFMAG0: u8 = 0x7f;		/* Magic number byte 0 */

const EI_MAG1: u8 = 1;		/* File identification byte 1 index */
const ELFMAG1: u8 = b'E';		/* Magic number byte 1 */

const EI_MAG2: u8 =  2;		/* File identification byte 2 index */
const ELFMAG2: u8 = b'L';		/* Magic number byte 2 */

const EI_MAG3: u8 = 3;		/* File identification byte 3 index */
const ELFMAG3: u8 = b'F';		/* Magic number byte 3 */

/* Conglomeration of the identification bytes, for easy testing as a word.  */
const ELFMAG: &str = "\x7FELF";
const	SELFMAG: u8 =		4;

const EI_CLASS: u8 =	4;		/* File class byte index */
const ELFCLASSNONE: u8 =0;		/* Invalid class */
const ELFCLASS32: u8 =	1;		/* 32-bit objects */
const ELFCLASS64: u8 =	2;		/* 64-bit objects */
const ELFCLASSNUM: u8 =	3;

const EI_DATA: u8 =		5;		/* Data encoding byte index */
const ELFDATANONE: u8 =	0;		/* Invalid data encoding */
const ELFDATA2LSB: u8 =	1;		/* 2's complement, little endian */
const ELFDATA2MSB: u8 =	2;		/* 2's complement, big endian */
const ELFDATANUM: u8 =	3;

const EI_VERSION: u8 =	6;		/* File version byte index */
 			/* Value must be EV_CURRENT */
const EI_OSABI: u8 = 7;		/* OS ABI identification */
const ELFOSABI_NONE: u8 =		0;	/* UNIX System V ABI */
const ELFOSABI_SYSV: u8 =		0;	/* Alias.  */
const ELFOSABI_HPUX: u8 =		1;	/* HP-UX */
const ELFOSABI_NETBSD: u8 =		2;	/* NetBSD.  */
const ELFOSABI_GNU: u8 =		3;	/* Object uses GNU ELF extensions.  */
const ELFOSABI_LINUX: u8 =		ELFOSABI_GNU; /* Compatibility alias.  */
const ELFOSABI_SOLARIS: u8 =	6;	/* Sun Solaris.  */
const ELFOSABI_AIX: u8 =	    7;	/* IBM AIX.  */
const ELFOSABI_IRIX: u8 =       8;	/* SGI Irix.  */
const ELFOSABI_FREEBSD: u8 =    9;	/* FreeBSD.  */
const ELFOSABI_TRU64: u8 =      10;	/* Compaq TRU64 UNIX.  */
const ELFOSABI_MODESTO: u8 =    11;	/* Novell Modesto.  */
const ELFOSABI_OPENBSD: u8 =    12;	/* OpenBSD.  */
const ELFOSABI_ARM_AEABI: u8 =  64;	/* ARM EABI */
const ELFOSABI_ARM: u8 =        97;	/* ARM */
const ELFOSABI_STANDALONE: u8 =	255;	/* Standalone (embedded) application */

const EI_ABIVERSION : u8 =	8;		/* ABI version */

const EI_PAD : u8 =		9;		/* Byte index of padding bytes */


#[derive(Error,Debug)]
pub enum Error {
    #[error("Can't read")]
    Io(#[from]std::io::Error),
    #[error("Conversion error from slice")]
    Slice(#[from] std::array::TryFromSliceError),
    #[error("Invalid padding - e_ident")]
    InvalidPadding,
    #[error("Invalid ELF - e_ident")]
    InvalidElf,
    #[error("Reading error, bad path?")]
    ReadingErr,
    #[error("Header must be 64 bytes")]
    TooSmall,
}



    
#[derive(Debug)]
struct EIdent {
    magic: [u8; 4],
    class: u8,
    endian: u8,
    version: u8,
    osabi: u8,
    abi_version: u8,
    padding: [u8; 7]
} 



#[repr(C)]
#[derive(Debug)]

struct Header32 {
    e_ident: [u8; 16],  //Magic number and other info       16 - 0x00 
    e_type: u16,        // Object file type                  2 - 0x10
    e_machine: u16,     // Architecture                      2 - 0x12
    e_version: u32,     // Object file version               4 - 0x14
    e_entry: u32,       // Entry point virtual adress        4 - 0x18
    e_phoff: u32,       // Program header table file offset  4 - 0x1C
    e_shoff: u32,       // Section header table file offset  4 - 0x20
    e_flags: u32,       // Processor specific flags          4 - 0x24
    e_ehsize: u16,      // ELF header size in bytes          2 - 0x28
    e_phentsize: u16,   // Program Header table entry size   2 - 0x2A
    e_phnum: u16,       // Program header entry count        2 - 0x2C
    e_shentsize: u16,   // Section header table entry size   2 - 0x2E
    e_shnum: u16,       // Section header table entry count  2 - 0x30
    e_shstrndx: u16,    // Section header string table index 2 - 0x32


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

pub enum Architecture {
    Elf32,
    Elf64,
}



fn main () {
match read(){
    Ok(_) => println!("OK"),
        Err(e) => println!("{:?}", e),
}


}

fn read () -> Result<(), Error> {
    let path = env::current_exe()?;
    let data: Vec<u8> = fs::read(path)?;
    
    //let ident: [u8; 16] = data.get(0..16); // slice 16b e_ident 
    let ident:[u8; 16] = match data.get(0..16) {
        Some(slice) => {
            match slice.try_into() {
                Ok(ident) => {
                    println!("Ok ident");
                ident
                }
           
                Err(_) => {
                    println!("Error ident");
                    [0; 16]
                }
            }
       
        
        }
   
        None => {
            println!("Data moc krátká");
            [0; 16]
        }
    };
     
    let header = parse_data(&data[0..64]); //slice 64b header 
    println!("{:?}", header); 
   
    
    let e_ident = parse_ident(ident);  //validation ELF
    println!("Tohle je E_IDENT {:?}", e_ident);
   
    


    Ok(()) 

}


fn parse_ident(ident:[u8; 16]) -> Result <EIdent, Error> {
      
    match ident  { [0x7f, b'E', b'L', b'F', class, endian, version, osabi, abi_version, padding @..] =>{
    let padding_array: [u8; 7] = padding; //padding e_ident 7b
    
    if padding.len() != 7 {
        return Err(Error::InvalidPadding);
  
    }

        Ok(EIdent {
        magic:[0x7f, b'E', b'L', b'F'],
        class:      ident[4],
        endian:     ident[5],
        version:    ident[6],
        osabi:      ident[7],
        abi_version:ident[8],
        padding: padding_array
        }) 
      
    }
   

    _=> Err(Error::InvalidElf),

        }
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


