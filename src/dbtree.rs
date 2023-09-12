use std::{
    fs::File,
    io::{self, BufReader, Error, Read, Seek}, collections::{HashSet, HashMap},
};

use nom_sql::{FieldDefinitionExpression, ConditionExpression};

#[derive(Debug)]
enum NodeType {
    InteriorIndex,
    LeafIndex,
    InteriorTable,
    LeafTable,
}

struct PageHeader {
    node_type: NodeType,
    freeblock_start: u16,
    num_cells: u16,
    cell_start: u16, //May add fragmented free bytes and rightmost pointer later
}

struct DBTreeRoot<'a> {
    db_header: &'a DBHeader,
    page_header: PageHeader,
}

impl<'a> DBTreeRoot<'a> {
    fn new(db_header: &'a DBHeader, page_num: u32) -> Result<DBTreeRoot, io::Error> {
        //open file
        let mut reader = BufReader::new(&db_header.file);

        //start reading page header
        let mut buf: [u8; 8] = [0; 8];
        if page_num == 1 { //reading first page, should only happen on initialization
            reader.seek(io::SeekFrom::Start(100))?;
        } else {
            reader.seek(io::SeekFrom::Start(db_header.page_size as u64 * page_num as u64))?;
        }
        
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

        Ok(DBTreeRoot {
            db_header,
            page_header,
        })
    }
    
    ///Pulls data from a DBTree, in line with a simple SELECT statement.
    /// 
    /// Returns a Hashtable with keys as column names, and values as all results in said column matching the request
    fn select(fields: Vec<FieldDefinitionExpression>, where_clause: Option<ConditionExpression>) -> HashMap<String, Vec<String>>{

        todo!()
    }

    fn get_debug_info(&self) {
        println!("This root node is of type {:?}", self.page_header.node_type);
    }
}

struct TableFmt {
    name: String,
    tbl_name: String,
    root_page: u32,
    sql_text: String
}

struct DBHeader {
    file: File,
    page_size: u16,
    ff_write: bool,
    ff_read: bool,
    page_reserve_bytes: u8,
    num_pages: u32,
    freelist_start: u32,
    free_pages: u32,
}

pub struct DBSchemaTable {
    db_header: DBHeader,
    tables: HashSet<TableFmt> //need to do tests comparing this to a Vec; since tables are small, linear search might outperform hashing
}

impl DBSchemaTable {
    pub fn new(file_name: &str) -> Result<DBSchemaTable, io::Error> {
        //open file
        let file = File::open(file_name)?;
        let mut reader = BufReader::new(&file);
        //read db header
        let mut buf: [u8; 4] = [0; 4];

        //get page size and file format data
        reader.seek(io::SeekFrom::Start(16))?;
        reader.read(&mut buf)?;

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

        

        //read through the BTree and get the various entires

        let tables: HashSet<TableFmt> = HashSet::new();

        let db_header = DBHeader {
            file,
            page_size,
            ff_write,
            ff_read,
            page_reserve_bytes,
            num_pages,
            freelist_start,
            free_pages,
        };

        let tree_root = DBTreeRoot::new(&db_header, 1)?;

        Ok(DBSchemaTable {
            db_header,
            tables
        })
    }

    pub fn list_tables(&self) -> Vec<&String> { //apparently this doesn't need lifetime identifiers???
        return self.tables.iter().map(|table|
            &table.name
        ).collect();
    }
}