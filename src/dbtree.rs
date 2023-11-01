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
    cell_start: u16, //May add fragmented free bytes later
    right_pointer: Option<u32>

}

//Represents a single row
struct Record {
    header: [u64]
}

struct DBTreeNode<'a> {
    page_num: u32,
    db_header: &'a mut DBHeader, //this must be mut to enable file seek stuff
    page_header: PageHeader,
    cell_pointers: Option<Vec<u16>>, //don't need the actual cell data till we start looking through things, so probably best to not store cell data in memory
    columns: Vec<String> //TODO: See if we can get this to an array. Currently attempts to make it an array are causing compiler issues
}

impl<'a> DBTreeNode<'a> {
    fn new(db_header: &'a mut DBHeader, page_num: u32, columns: Vec<String>) -> Result<DBTreeNode, io::Error> {
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

        let free_bytes  = buf[7]; //not actually stored in the in-memory header, just used for some optimizations in reading cells

        

        let right_pointer: Option<u32> = match node_type {
            NodeType::InteriorIndex | NodeType::InteriorTable => {
                let mut rp_buf: [u8; 4] = [0; 4];
                reader.read(&mut rp_buf)?;
                Some(u32::from_be_bytes(rp_buf.try_into().expect("invalid length")))
            },
            _ => None
        };
        //read pointer array
        let cell_pointers: Option<Vec<u16>> = match num_cells {
            0 => None,
            _ => {
                let mut pointers = Vec::new();

                //Annoying workarounds bc dynamically-sized arrays aren't stable yet
                let mut pointer_buf_vec: Vec<u8> = Vec::with_capacity((num_cells * 2) as usize);
                pointer_buf_vec.resize((num_cells * 2) as usize, 0);
                let pointer_buf = pointer_buf_vec.as_mut_slice();

                reader.read(pointer_buf)?;
                println!("boop");
                println!("debug info on the pointer vec thingy{:?}", pointer_buf_vec);
                Some(pointers)
            }
        };
        
        let page_header = PageHeader {
            node_type,
            freeblock_start,
            num_cells,
            cell_start,
            right_pointer
        };

        Ok(DBTreeNode {
            page_num,
            db_header,
            page_header,
            cell_pointers,
            columns
        })
    }

    ///Gets a collection of rows from a table, obeying where clausues passed
    fn select(&mut self, fields: Vec<FieldDefinitionExpression>, where_clause: Option<ConditionExpression>) -> Result<HashMap<String, Vec<String>>, io::Error> {
        match self.page_header.node_type {
            NodeType::InteriorIndex | NodeType::InteriorTable => {

            }
            NodeType::LeafIndex => {
                return Ok(HashMap::new()) //TODO: properly implement this
            }
            NodeType::LeafTable => {
                let mut return_table: HashMap<String, Vec<String>> = HashMap::new();
                let mut file = &self.db_header.file;

                //save our starting position and move to the start for this node's page
                let orig_file_pos = file.stream_position()?;
                let page_start = self.db_header.page_size as u64 * self.page_num as u64;
                file.seek(io::SeekFrom::Start(page_start))?;

                for pointer in &self.cell_pointers {
                    
                }
                file.seek(io::SeekFrom::Start(orig_file_pos))?;
                return Ok(return_table);
            }
        }
        todo!()
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

        let mut db_header = DBHeader {
            file,
            page_size,
            ff_write,
            ff_read,
            page_reserve_bytes,
            num_pages,
            freelist_start,
            free_pages,
        };
        let mut column_arr: Vec<String> = Vec::new(); 
        let tree_root = DBTreeNode::new(&mut db_header, 1, column_arr)?;

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