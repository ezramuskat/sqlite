use std::{
    fs::File,
    io::{self, BufReader, Error, Read, Seek},
};

#[derive(Debug)]
enum NodeType {
    InteriorIndex,
    LeafIndex,
    InteriorTable,
    LeafTable,
}
struct DBTreeHeader {
    file: File,
    page_size: u16,
    ff_write: bool,
    ff_read: bool,
    page_reserve_bytes: u8,
    num_pages: u32,
    freelist_start: u32,
    free_pages: u32,
}

struct PageHeader {
    node_type: NodeType,
    freeblock_start: u16,
    num_cells: u16,
    cell_start: u16, //May add fragmented free bytes and rightmost pointer later
}

pub struct DBTreeRoot {
    db_header: DBTreeHeader,
    page_header: PageHeader,
}

impl DBTreeRoot {
    pub fn new(file_name: &str) -> Result<DBTreeRoot, io::Error> {
        //open file
        let file = File::open(file_name)?;
        let mut reader = BufReader::new(&file);
        //read db header
        let mut buf: [u8; 4] = [0; 4];

        //get page size and file format data
        reader.seek(io::SeekFrom::Start(16))?;
        reader.read(&mut buf)?;

        println!("{:?}", &buf);

        let page_size = u16::from_be_bytes((&buf[0..2]).try_into().expect("incorrect length"));

        let ff_write = buf[2] == 2;
        let ff_read = buf[3] == 2;

        //get reserve space
        //the payload fractions also get read here; in the future we might
        //make multiple buffers to reduce the actual amount of reading, but for now
        //we'll just ignore them
        reader.read(&mut buf)?;
        let page_reserve_bytes = (&buf[0]).clone();

        //get num pages
        reader.seek(io::SeekFrom::Current(4))?;
        reader.read(&mut buf)?;

        let num_pages = u32::from_be_bytes(buf.try_into().expect("invalid length"));

        //get first freelist page
        reader.read(&mut buf)?;

        let freelist_start = u32::from_be_bytes(buf.try_into().expect("invalid length"));

        //get num free pages
        reader.read(&mut buf)?;

        let free_pages = u32::from_be_bytes(buf.try_into().expect("invalid length"));

        //start reading page header
        let mut buf: [u8; 8] = [0; 8];
        reader.seek(io::SeekFrom::Start(100))?;
        reader.read(&mut buf)?;

        let node_type: NodeType = match &buf[0] {
            2 => NodeType::InteriorIndex,
            5 => NodeType::InteriorTable,
            10 => NodeType::LeafIndex,
            13 => NodeType::LeafTable,
            _ => return Err(Error::new(io::ErrorKind::InvalidData, "invalid node type")),
        };

        let freeblock_start =
            u16::from_be_bytes((&buf[1..3]).try_into().expect("incorrect length"));

        let num_cells = u16::from_be_bytes((&buf[3..5]).try_into().expect("incorrect length"));

        let cell_start = u16::from_be_bytes((&buf[5..7]).try_into().expect("incorrect length"));

        let page_header = PageHeader {
            node_type,
            freeblock_start,
            num_cells,
            cell_start,
        };

        let db_header = DBTreeHeader {
            file,
            page_size,
            ff_write,
            ff_read,
            page_reserve_bytes,
            num_pages,
            freelist_start,
            free_pages,
        };

        Ok(DBTreeRoot {
            db_header,
            page_header,
        })
    }

    pub fn get_debug_info(&self) {
        println!("This database has {} pages", self.db_header.num_pages);
        println!("This root node is of type {:?}", self.page_header.node_type);
    }
}

struct Node {
    page_header: PageHeader,
}
