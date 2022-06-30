use std::{fs};
use std::process::{Command};
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};

fn main() {
    const OUT_PATH: &str = "./output";
    const MAX_VALUE: u64 = 10000000;
    let main_template = fs::read_to_string("./resources/main_template.txt").expect("Unable to read main_template");
    let out_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(format!("{OUT_PATH}/main.rs"))
        .expect("Unable to create output");
    let mut out = BufWriter::new(out_file);
    out.write_all(main_template.as_bytes()).expect("Unable to write data");
    for value in 0..MAX_VALUE {
        println!("Current iteration:    {value}");
        println!("Remaining iterations: {}", MAX_VALUE - value);
        let result = is_even(value);
        let expression = format!("\
        if value == {value} {{ \
            return {result};\
        }} \n");
        out.write_all(expression.as_bytes()).expect("Unable to write data");
    }
    out.write_all("panic!();\n}".as_bytes()).expect("Unable to write data");
    out.flush().expect("Unable to flush output");
    let out_file_metadata = out.into_inner().unwrap().metadata().expect("Can't get output file metadata");
    println!("Output file size: {} Mb", (out_file_metadata.len() as f64 / 1000000 as f64));
    let exit_status = Command::new("cmd")
        .current_dir(OUT_PATH)
        .args(&["/C", "cargo build --release --timings --verbose --verbose"])
        .status()
        .expect("Can't compile output");
    println!("Build exit status: {}", exit_status.code().unwrap())
}


fn is_even(value: u64) -> bool {
    return value % 2 == 0;
}

//fn is_even(value: u64) -> bool {
//    if value == 1 {
//        return false;
//    }
//    true
//}

