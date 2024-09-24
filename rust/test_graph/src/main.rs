use std::fs::File;
use std::io::{Read};

// FBE CppGenerator : https://github.com/chronoxor/FastBinaryEncoding/blob/master/source/generator_cpp.cpp
// FBE C# Generator: https://github.com/chronoxor/FastBinaryEncoding/blob/master/source/generator_csharp.cpp

#[derive(Debug)]
struct Book {
    id: i32,
    changed_id: u64,
    instrument_id: u32,
}

fn to_path(name: &str) -> String {
    format!("d:\\code_samples\\serialized\\{}.cpp.bin", name)
}

fn deserialize(buf: &Vec<u8>) {
    let start_offset = 0;
    let size = buf.len();

    let mut offset = start_offset;

    let bytes: [u8; 4] = buf[offset..offset + 4].try_into().expect("Eeek");
    let size = i32::from_le_bytes(bytes);

//     let full_size = u32::from_le_bytes(buf[offset..32].try_into().expect("msg"));
    println!("Read the Int: {}", size);

    offset += 4;

    let next_bytes: [u8; 4] = buf[offset..offset + 4].try_into().expect("Eeek");
    let id = i32::from_le_bytes(next_bytes);
    println!("Read the Int: {}", id);

    offset += 4;

    let next_bytes2: [u8; 4] = buf[offset..offset + 4].try_into().expect("Eeek");
    let id = i32::from_le_bytes(next_bytes2);
    println!("Read the Int: {}", id);

    offset += 4;

    let next_bytes3: [u8; 4] = buf[offset..offset + 4].try_into().expect("Eeek");
    let id = i32::from_le_bytes(next_bytes3);
    println!("Read the Int: {}", id);

}

fn main() {
    let file_name = "FbeEncoding";
    let file_path = to_path(file_name);

    print!("Opening file: {}", file_path);

    let file = File::open(file_path);
    let result = 
        file.and_then(|mut file_contents| {
            let mut buffer = Vec::new();
            let result = file_contents.read_to_end(&mut buffer).and_then(|size | { Ok((size, buffer)) });
            return result;
        })
        .and_then(|(size, buf)| {
            println!("Read {:?} bytes from file", size);
            deserialize(&buf);
            return Ok(size)
        });

    print!("Finished with Result: {:?}", result);
} 
