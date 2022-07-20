use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use wasm_compresso::{zlib_encode, zlib_decode, zlib_encode_raw, zlib_decode_raw};

fn main() {
    // stirng compress(zlib)
	let raw: &str = "uncompressed";
    println!("{}", raw);
    let aaa: String = zlib_encode(raw);
    println!("{}", aaa);
    let bbb: String = zlib_decode(&aaa);
    println!("{}", bbb);

    // stirng compress(zlib)
    let org_path = Path::new("data/koupen.jpg");
    let encode_path = Path::new("data/koupen_en.jpg");
    let decode_path = Path::new("data/koupen_de.jpg");
    let display = org_path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    // pathを読み込み専用モードで開く。これは`io::Result<File>`を返す。
    let mut file: File = match File::open(&org_path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        // `io::Error`の`description`メソッドはエラーを説明する文字列を返す。
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    // ファイルの中身を文字列に読み込む。`io::Result<useize>`を返す。
    let mut buffer: Vec<u8> = Vec::new();

    // read the whole file
    match file.read_to_end(&mut buffer) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{:?}", display, buffer),
    }

    let aaa: Vec<u8> = zlib_encode_raw(&buffer);
    let mut file = match File::create(&encode_path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };
    match file.write_all(&aaa) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }


    let bbb: Vec<u8> = zlib_decode_raw(&aaa);
    let mut file = match File::create(&decode_path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };
    match file.write_all(&bbb) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }

}
