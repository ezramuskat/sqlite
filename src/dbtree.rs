use std::fs::File;

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
	freeblock_start: Option<u16>,
	num_cells: u16,
	cell_start: u16
	//May add fragmented free bytes and rightmost pointer later
}

struct DBTreeRoot {
	db_header: DBTreeHeader,
	page_header: PageHeader
}

impl DBTreeRoot {
}

struct Node {
	page_header: PageHeader
}