
use unicode_normalization::UnicodeNormalization;

use std::fs::File;
// use std::io::Read;
use std::env::args;

// fn main() -> std::io::Result<()> {
//     let mut text = String::new();

//     let filename: String = args().nth(1).unwrap();

//     let mut file = File::open(filename)?;

//     file.read_to_string(&mut text)?;

//     let normalized: String = text.nfkd().filter(|c| c.is_ascii()).collect();

//     println!("{}", normalized);

//     Ok(())
// }

use std::io::BufReader;
use std::io::BufRead;

use std::io::Write;
use std::io::BufWriter;

use std::fs::OpenOptions;

const DEV_NULL: &str = "/dev/null";


fn get_input() -> BufReader<File> {
    let input: String = args().nth(1).unwrap();

    let input = File::open(input).expect("couldn't open input");
    
    return BufReader::new(input);
}

fn get_output() -> BufWriter<File> {
    let output = args().nth(2).unwrap_or(String::from(DEV_NULL));
    let output = OpenOptions::new().write(true).open(output)
                            .expect("couldn't open output");

    return BufWriter::new(output);
}


fn main() -> std::io::Result<()> {
    let s = "ÅΩ";

    let nfkd 

    println!("=================");

    let input = get_input();

    let mut output = get_output();

    for line in input.lines() {
        let buffer: String = line?.nfkd().filter(|c| c.is_ascii()).chain(vec!['\n']).collect();

        output.write_all(buffer.as_bytes()).unwrap();
    }

    output.flush()?;

    Ok(())
}

// while let Ok(_) = buf_reader.read_line(&mut buffer) {
//     buffer = buffer.nfkd().filter(|c| c.is_ascii()).chain(vec!['\n']).collect();

//     output.write_all(buffer.as_bytes()).unwrap();

//     buffer.clear();
// }
