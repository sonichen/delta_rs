use std::process;

mod constants {
    pub const BLOCK_SIZE: usize = 10;
}

/// Structure for a data block
#[derive(Debug, Clone, PartialEq)]
pub struct DataBlock {
    /// Block number of the data block
    pub  block_number: usize,
    /// Content of the data block
    pub  data: Vec<u8>,
}

impl DataBlock {
    /// Create a data block
    fn new(block_number: usize, data: Vec<u8>) -> Self {
        DataBlock { block_number, data }
    }
}

///  Splitting large-scale data into fixed-size data blocks and recording the block numbers.
fn split_data_into_blocks(data: Vec<u8>, block_size: usize) -> (Vec<DataBlock>, Vec<usize>) {
    let mut blocks = Vec::new();
    let mut index = 0;
    let mut block_number = 0;
    let mut numbers: Vec<usize> = Vec::new();
    while index < data.len() {
        numbers.push(block_number);

        let end = std::cmp::min(index + block_size, data.len());
        blocks.push(DataBlock::new(block_number, data[index..end].to_vec()));
        index = end;
        block_number += 1;
    }

    (blocks, numbers)
}

// Comparing data block lists to find newly added data blocks.
fn find_different_blocks(
    id: u8,
    track: &Vec<Delta>,
    current_data: &[u8],
    _block_size: usize,
) -> Vec<DataBlock> {
    let blocks1 = get_data_blocks_up_to_id(id, track);
    let (blocks2, _data_indices) =
        split_data_into_blocks(current_data.clone().to_vec(), constants::BLOCK_SIZE);
    // Find elements in block1 that are not in block2
    let elements_not_in_block1: Vec<DataBlock> = blocks2
        .iter()
        .filter(|block2_item| {
            blocks1
                .iter()
                .find(|block1_item| block1_item.data == block2_item.data)
                .is_none()
        })
        .cloned()
        .collect();
    elements_not_in_block1
}

/// Add new blocks to blocklist
fn add_to_block_list(
    mut block_list: Vec<DataBlock>,
    different_blocks: Vec<DataBlock>,
) -> (Vec<DataBlock>, Vec<usize>) {
    let mut diff_number = Vec::<usize>::new();
    for mut block in different_blocks {
        let last_block_number = block_list.last().map_or(0, |block| block.block_number);

        block.block_number = 1 + last_block_number;
        diff_number.push(block.block_number);
        block_list.push(block);
    }

    // block_list
    (block_list, diff_number)
}

/// Extract the index of data block
fn extract_index(vec_data1: &Vec<DataBlock>, vec_data2: &[DataBlock]) -> Vec<usize> {
    let mut index: Vec<usize> = Vec::new();
    for data_block1 in vec_data1.iter() {
        if let Some(index_in_vec_data2) = vec_data2
            .iter()
            .position(|data_block2| data_block1.data == data_block2.data)
        {
            index.push(vec_data2[index_in_vec_data2].block_number);
        }
    }

    index
}
/// Track, store the track for data
#[derive(Debug, Clone)]
pub struct Track{
    pub track: Vec<Delta>,
}
impl Track {
     // First Store     
     pub fn create( context: &str) ->Track {
        Delta::init(context)
     }

     //  Delta Store  
     pub fn modify(&mut self, content: &str){
        let track=Delta::add(content, self.track.clone(), true);
        self.track=track;
       
     }
    
}
/// Delta, record block info
#[derive(Debug, Clone)]
pub struct Delta {
    pub id: u8,
    pub index: Vec<usize>,
    pub blocks: Vec<DataBlock>,
    pub snapshot: bool,
}
impl Delta {
    /// Create delta
    fn new(id: u8, index: Vec<usize>, blocks: Vec<DataBlock>, snapshot: bool) -> Self {
        Delta {
            id,
            index,
            blocks,
            snapshot,
        }
    }
    /// Create the first store
    pub fn init(content: &str) ->Track {
        let data: Vec<u8> = content.as_bytes().to_vec();
        let (blocks, data_indices) = split_data_into_blocks(data.clone(), constants::BLOCK_SIZE);
        let delta = Delta::new(0, data_indices, blocks, true);
        let mut track: Vec<Delta> =Vec::new();
        track.push(delta);
        // track
        Track{
            track
        }
        
    }
   
    /// Store data
    pub fn add(content: &str, mut record_table: Vec<Delta>, snapshot: bool) -> Vec<Delta> {
        // let mut record_table=self;
        // Check the last data
        let last = record_table.last().unwrap_or_else(|| {
            println!("The last data is empty!");
            process::exit(1);
        });
        let last_id = last.id;


        // Process the current data
        let current_id = last_id + 1;

        // Convert the content to data
        let current_data: Vec<u8> = content.as_bytes().to_vec();
        let (current_data_blocks, _data_indices) = split_data_into_blocks(current_data.clone(), constants::BLOCK_SIZE);
       
        // Build a block list and record the construction number of the original data
        let different_blocks =
            find_different_blocks(last_id, &record_table, &current_data, constants::BLOCK_SIZE);

        let block_list = get_data_blocks_up_to_id(last_id, &record_table);
        let (records, diff) = add_to_block_list(block_list, different_blocks);
        
        // assign id to diff blocks
        let diff_blocks: Vec<DataBlock> = records
        .iter()
        .filter_map(|record| {
            if diff.contains(&record.block_number) {
                Some(DataBlock {
                    block_number: record.block_number,
                    data: record.data.clone(),
                })
            } else {
                None
            }
        })
        .collect();
    
        // get current index
        let matching_block_numbers = extract_index(&current_data_blocks, &records);

        let delta = Delta {
            id: current_id,
            index: matching_block_numbers,
            blocks: diff_blocks,
            snapshot,
        };
        record_table.push(delta);
        record_table
    }
}

/// Function to combine Vec<DataBlock> into text
fn combine_data_blocks_to_text(data_blocks: &Vec<DataBlock>) -> String {
    let mut combined_text = String::new();
    for data_block in data_blocks {
        combined_text.push_str(std::str::from_utf8(&data_block.data).unwrap());
    }
    combined_text
}

/// Find the corresponding indexes by ID.
fn find_index_by_id(id: u8, delta_list: &[Delta]) -> Option<Vec<usize>> {
    let delta_to_find = delta_list.iter().find(|delta| delta.id == id);

    delta_to_find.map(|delta| delta.index.clone())
}

/// Get all data blocks from ID 0 to the input ID.
fn get_data_blocks_up_to_id(id: u8, delta_list: &Vec<Delta>) -> Vec<DataBlock> {
    let mut data_blocks = Vec::new();
    for delta in delta_list {
        if delta.id <= id {
            data_blocks.extend(delta.blocks.iter().cloned());
        }
    }
    data_blocks
}

/// Get the Vec<DataBlock> corresponding to the indexes.
fn get_data_blocks_by_index(index: &Vec<usize>, data_blocks: &[DataBlock]) -> Vec<DataBlock> {
    let mut result_blocks = Vec::new();
    for &idx in index {
        if let Some(data_block) = data_blocks.iter().find(|block| block.block_number == idx) {
            result_blocks.push(data_block.clone());
        }
    }
    result_blocks
}
/// Get full data(string)
pub fn get_content(id: u8, detlas: Vec<Delta>) -> String {
    if let Some(index) = find_index_by_id(id, &detlas) {
        let data_blocks = get_data_blocks_up_to_id(id, &detlas);
        let selected_blocks = get_data_blocks_by_index(&index, &data_blocks);
        combine_data_blocks_to_text(&selected_blocks)
    } else {
        println!("No data blocks found for ID {}", id);
        process::exit(1);
    }
}
