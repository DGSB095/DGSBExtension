#[warn(unused_must_use)]

use image::{DynamicImage,
            ImageReader,
};
use image::{GenericImageView};
use std::{
    env, fmt::Error, fs::File, io::Write, path::PathBuf
};

fn rgba_to_hex(r: u8, g: u8, b: u8, a: u8) -> String {
    // Форматируем в HEX с префиксом #
    format!("#{:02X}{:02X}{:02X}{:02X}", r, g, b, a)
}

fn main() {
    let output_path = "E:/Projects/Rust/DGSBExtension/src/output.dgsb";
    let mut output_file = File::create(output_path);
    let args:Vec<String> = env::args().collect();
    let file_path = &args[1];
    if &args[1] == "convert" {
        if args.len() < 3 {
            panic!("You had to write a 'path' after 'convert'. Example cargo run convert E:/Projects/Rust/DGSBExtension/src/sea.png")
        }
        else{
            let file_path = &args[2];
            match PNG_to_DGSB(output_file.unwrap(), file_path, output_path) {
                Ok(()) => println!("PNG файл успешно декодирован и записан в {}", file_path),
                Err(_) => println!("Something went wrong, try again or IDK try to fix it) "),
            }
        }
    }
    else {
        match DGSB_as_image(file_path) {
            Ok(()) => print!("Succsefully opened .dgsb as an image"),
            Err(_) => print!("Something went wrong. Are you sure that you've chosen .dgsb file.")
        }
    }

}
    // Обрабатываем каждый пиксель
fn PNG_to_DGSB(mut  outputf:File, file_pth: &String, _output_pth: &str) -> Result<(), std::io::Error>{
    let img = ImageReader::open(file_pth).unwrap().decode().unwrap();
    let (w, h) = img.dimensions();
    writeln!(outputf, "{}", w)?;
    writeln!(outputf, "{}", h)?;

    for y in 0..h {                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      
        for x in 0..w {
                let pixel = img.get_pixel(x, y);
                let [r, g, b, a] = pixel.0;
                let hex = rgba_to_hex(r, g, b, a);
                // Записываем данные пикселя в файл
                write!(outputf, "{}", hex)?;
        }
    }
    Ok(())
}
fn DGSB_as_image(fpath: &String) -> Result<(), std::io::Error>{
    Ok(())
}