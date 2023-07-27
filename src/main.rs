use std::process;

mod constants {
    pub const BLOCK_SIZE: usize = 10;
}

// 数据块的结构
#[derive(Debug, Clone, PartialEq)]
struct DataBlock {
    block_number: usize, // 数据块的编号
    data: Vec<u8>,       // 数据块的内容
}
impl DataBlock {
    fn new(block_number: usize, data: Vec<u8>) -> Self {
        DataBlock { block_number, data }
    }
}

// 将大规模数据切分成固定大小的数据块，并记录数据块的编号
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

// 对比数据块列表，找到新增的数据块
fn find_different_blocks(
    id: u8,
    deltas: &Vec<Delta>,
    current_data: &[u8],
    _block_size: usize,
) -> Vec<DataBlock> {
    let blocks1 = get_data_blocks_up_to_id(id, deltas);
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

// 将 Vec<DataBlock> 转换为 Vec<u8> 的函数
fn extract_data_from_data_blocks(data_blocks: &[DataBlock]) -> Vec<u8> {
    let mut extracted_data: Vec<u8> = Vec::new();
    for data_block in data_blocks {
        extracted_data.extend(data_block.data.iter().cloned());
    }
    extracted_data
}
fn extract_index(vec_data1: &Vec<DataBlock>, vec_data2: &Vec<DataBlock>) -> Vec<usize> {
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

fn find_blocks_by_id(id: u8, record_table: &Vec<Delta>) -> Option<Vec<DataBlock>> {
    // 在 record_table 中查找与给定 id 匹配的 Delta
    let delta_to_find = record_table.iter().find(|delta| delta.id == id);
    // println!("{:?}=====",delta_to_find);
    // 如果找到了匹配的 Delta，则获取该 Delta 中的 index
    if let Some(delta) = delta_to_find {
        let data_indices = &delta.index;
        let blocks = get_data_blocks_up_to_id(id, record_table);
        // 使用 data_indices 在 Delta 的 blocks 中找到对应的数据块
        let mut data_blocks = Vec::new();
        for &data_index in data_indices {
            if let Some(data_block) = blocks.get(data_index) {
                data_blocks.push(data_block.clone());
            }
        }
        // 返回找到的数据块
        Some(data_blocks)
    } else {
        // 如果找不到匹配的 Delta，则返回 None
        None
    }
}
#[derive(Debug, Clone)]
struct Delta {
    id: u8,
    index: Vec<usize>,
    blocks: Vec<DataBlock>,
    snapshot: bool,
}
impl Delta {
    fn new(id: u8, index: Vec<usize>, blocks: Vec<DataBlock>, snapshot: bool) -> Self {
        Delta {
            id,
            index,
            blocks,
            snapshot: snapshot,
        }
    }
    fn init(content: &str) -> Vec<Delta> {
        // 转换数据
        let data: Vec<u8> = content.as_bytes().to_vec();
        // 构建块列表，记录原始数据的构建编号
        let (blocks, data_indices) = split_data_into_blocks(data.clone(), constants::BLOCK_SIZE);
        let delta = Delta::new(0, data_indices, blocks, true);
        let mut deltas: Vec<Delta> = Vec::new();
        deltas.push(delta);
        deltas
    }

    fn add(content: &str, mut record_table: Vec<Delta>) -> Vec<Delta> {
        // 转换数据
        let current_data: Vec<u8> = content.as_bytes().to_vec();
        let (current_data_blocks, _data_indices) =
            split_data_into_blocks(current_data.clone(), constants::BLOCK_SIZE);

        //查看上一个数据
        let last;
        if let Some(last_element) = record_table.last() {
            // 将最后一个元素的值存储在变量 last_value 中
            last = last_element;
        } else {
            // 向量为空时的处理
            println!("The vector is empty!");
            process::exit(1);
        }
        let last_id = last.id;

        // 处理现在数据
        let current_id = last_id + 1;

        // 构建块列表，记录原始数据的构建编号
      
        let different_blocks =
            find_different_blocks(last_id, &record_table, &current_data, constants::BLOCK_SIZE);
     
        let block_list=get_data_blocks_up_to_id(last_id, &record_table);
        let (records, diff) = add_to_block_list(block_list, different_blocks);
       
        let mut diff_blocks: Vec<DataBlock> = Vec::new();
        for item in &diff {
            for record in &records {
                if &record.block_number == item {
                    let db = DataBlock {
                        block_number: *item,
                        data: record.data.clone(),
                    };
                    diff_blocks.push(db);
                }
            }
        }

        let matching_block_numbers = extract_index(&current_data_blocks, &records);

        let detla = Delta {
            id: current_id,
            index: (matching_block_numbers),
            blocks: (diff_blocks),
            snapshot: false,
        };
        record_table.push(detla);
        record_table
    }
}
// 将 Vec<DataBlock> 组合为文本的函数

// 组合 Vec<DataBlock> 为文本
fn combine_data_blocks_to_text(data_blocks: &Vec<DataBlock>) -> String {
    let mut combined_text = String::new();
    for data_block in data_blocks {
        combined_text.push_str(std::str::from_utf8(&data_block.data).unwrap());
    }
    combined_text
}
// 通过ID找到对应的index
fn find_index_by_id(id: u8, delta_list: &Vec<Delta>) -> Option<Vec<usize>> {
    // 在 delta_list 中查找与给定 id 匹配的 Delta
    let delta_to_find = delta_list.iter().find(|delta| delta.id == id);

    // 如果找到了匹配的 Delta，则返回该 Delta 中的 index
    if let Some(delta) = delta_to_find {
        Some(delta.index.clone())
    } else {
        // 如果找不到匹配的 Delta，则返回 None
        None
    }
}

// 获取从id=0开始到输入id的所有数据块
fn get_data_blocks_up_to_id(id: u8, delta_list: &Vec<Delta>) -> Vec<DataBlock> {
    let mut data_blocks = Vec::new();
    for delta in delta_list {
        if delta.id <= id {
            data_blocks.extend(delta.blocks.iter().cloned());
        }
    }
    data_blocks
}

// 获取index对应的Vec<DataBlock>
fn get_data_blocks_by_index(index: &Vec<usize>, data_blocks: &[DataBlock]) -> Vec<DataBlock> {
    let mut result_blocks = Vec::new();
    for &idx in index {
        if let Some(data_block) = data_blocks.iter().find(|block| block.block_number == idx) {
            result_blocks.push(data_block.clone());
        }
    }
    result_blocks
}

fn get_full_data(id: u8, detlas: Vec<Delta>) -> String {
    if let Some(index) = find_index_by_id(id, &detlas) {
        let data_blocks = get_data_blocks_up_to_id(id, &detlas);
        let selected_blocks = get_data_blocks_by_index(&index, &data_blocks);
         combine_data_blocks_to_text(&selected_blocks)
    } else {
        println!("No data blocks found for ID {}", id);
        "Wrong".to_owned()
    }
}

fn main() {
    let mut detlas = Delta::init("ABCDEFGHIJKLMNOPQRSTUVWXYJ");
    detlas = Delta::add("1BCDEFGHIJKLMNOPQRSTUVWXY1", detlas);
    detlas = Delta::add("ABCDEFGHIJKLMNOPQRSTUVWXY8", detlas);
    detlas = Delta::add("ABCDEFG5IJKLMN7PQRSTUVWXYJ", detlas);
    detlas = Delta::add("ABCDEFGHIJKLMNOPQRSTUVWXYJ", detlas);

    // println!("{:?}",get_full_data(3, detlas));
    // println!("{:?}",get_full_data(1, detlas.clone()));
    // println!("{:?}",get_full_data(2, detlas.clone()));

    for item in &detlas {
         println!("{:?}\n", item);
    }
}
