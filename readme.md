# track_rs

`track_rs` is a **version control**  based on Rust, allowing you to store only data blocks that have not been stored in previous versions, effectively managing and restoring different versions of data. 

## Versions

track_rs 0.1.0 can do data version control and use incremental data storage. It has been released. https://crates.io/crates/track_rs

**!!Note: track_rs 0.2.0 is under development.** 

- [ ] add snapshot to improve efficiency 
- [ ] Optimize the data block separation rule so that it cuts according to data length 
- [ ] Optimize Data Block Identification
- [ ] Testing and documents

## Design Overview

The design of track_rs is centered around the following key principles:

- **Version Control**: The project allows users to maintain and manage multiple versions of data. Each version is represented as a Delta.

- **Incremental Data Storage**: To reduce storage space usage, track_rs stores only the data blocks that have not been stored in previous versions. This is achieved through a block-based approach.

- **Block-Based Storage**: Data is divided into blocks, and each block is associated with a unique identifier. When a new version is created, only the new and previously unsaved blocks are stored, along with the composition order. This design minimizes redundant data storage.

- **Quick Data Restoration**: Users can efficiently restore data to a specific version by specifying the version number and data block index.

## Example Use Cases

track_rs is designed to be useful in various scenarios, including:

- Managing versioned documents or files efficiently.
- Reducing storage space for data that evolves over time.
- Quickly restoring data to specific versions for analysis or historical purposes.

## Quick Start

### Installation

Add track_rs as a dependency in your Cargo.toml:

```toml
[dependencies]
track_rs = "0.1.0"
```

### Usage Example

```rust
 

use track_rs::track_rs::{ get_content, Track};

fn main() {
    // Write data
    let mut track=Track::create("Hello, I am a rust developer.");
    
    // First modify
    track.modify("Hello, I am a Java developer.");

    // Second modify
    track.modify("Hello, I am a Python developer.");

    // Third modify
    track.modify("Hello, I am a Go developer.");

    // print the data block and the content
    for item in &track.track {
        println!("The text in {} time is '{}'",item.id, get_content(item.id, track.track.clone()));
        println!("{:?}\n", item);
    }
    
}
```

Output

```shell
The text in 0 time is 'Hello, I am a rust developer.'
Delta { id: 0, index: [0, 1, 2], blocks: [DataBlock { block_number: 0, data: [72, 101, 108, 108, 111, 44, 32, 73, 32, 97] }, DataBlock { block_number: 1, data: [109, 32, 97, 32, 114, 117, 115, 116, 32, 100] }, DataBlock { block_number: 2, data: [101, 118, 101, 108, 111, 112, 101, 114, 46] }], snapshot: true }

The text in 1 time is 'Hello, I am a Java developer.'
Delta { id: 1, index: [0, 3, 2], blocks: [DataBlock { block_number: 3, data: [109, 32, 97, 32, 74, 97, 118, 97, 32, 100] }], snapshot: true }

The text in 2 time is 'Hello, I am a Python developer.'
Delta { id: 2, index: [0, 4, 5, 6], blocks: [DataBlock { block_number: 4, data: [109, 32, 97, 32, 80, 121, 116, 104, 111, 110] }, DataBlock { block_number: 5, data: [32, 100, 101, 118, 101, 108, 111, 112, 101, 114] }, DataBlock { block_number: 6, data: [46] }], snapshot: true }

The text in 3 time is 'Hello, I am a Go developer.'
Delta { id: 3, index: [0, 7, 8], blocks: [DataBlock { block_number: 7, data: [109, 32, 97, 32, 71, 111, 32, 100, 101, 118] }, DataBlock { block_number: 8, data: [101, 108, 111, 112, 101, 114, 46] }], snapshot: true }
```

## License

`track_rs` is licensed under this licensed:

- MIT LICENSE ( https://opensource.org/licenses/MIT)
