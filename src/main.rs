// CC:IA project by SimaDude/MyBoiSima/Stepanov Aleksei
//
// ComputerCraft:Image Application (CC:IA in further) is a program
// that converts an image to a readable by ComputerCraft computers format.
//
// This CC:IA is written in Rust programming language.
//
// You can use, modify or translate to other language this code.
//
// Contact: fictov.game@gmail.com
//

extern crate image;


use std::fs;
use std::env;
use std::time::Instant;
use std::io;
use image::imageops::FilterType;
use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};


static mut GRGBVAL: [[[u8; 256];256];256] = [[[0u8; 256];256];256];
const RGBVAL: [[i32; 3];16] = [
    [240,240,240],
    [242,178,51],
    [229,127,216],
    [153,178,242],
    [222,222,108],
    [127,204,25],
    [242,178,204],
    [76,76,76],
    [153,153,153],
    [76,153,178],
    [178,102,229],
    [51,102,204],
    [127,102,76],
    [87,166,78],
    [204,76,76],
    [25,25,25]];

fn truecolor(r:i32, g:i32, b:i32) -> u8 {
    let mut dist:i32 = 196608; // calculated with { 3 * (256**2) }
    let mut color:i8 = -1;     // in case the variable d gotta be more than dist in every way then we return -1

    for i in 0..16{
        let d:i32 = 
           i32::pow(RGBVAL[i][0] - r, 2) // imagine this is a pythagoras theorem
         + i32::pow(RGBVAL[i][1] - g, 2) // and it will be easier to understand.
         + i32::pow(RGBVAL[i][2] - b, 2);
        if d < dist {  // here we check if d (current distance) is less than a distance that we had
            dist = d;  // and if it is then it means it closer to the color we want to get.
            color = i as i8; // color variable is the number  of  the color we want to get.
        }
    }
    return color as u8
}

fn toCCImage(img: &mut image::DynamicImage){ 
    unsafe{ // needs to be in unsafe block because of the static variable
        for x in 0..img.width(){                            // we are using
            for y in 0..img.height(){
                img.put_pixel(x, y, image::Rgba([   RGBVAL[(GRGBVAL[img.get_pixel(x, y)[0] as usize]
                                                    [img.get_pixel(x, y)[1] as usize]
                                                    [img.get_pixel(x, y)[2] as usize]) as usize][0] as u8,
                                                    RGBVAL[(GRGBVAL[img.get_pixel(x, y)[0] as usize]
                                                    [img.get_pixel(x, y)[1] as usize]
                                                    [img.get_pixel(x, y)[2] as usize]) as usize][1] as u8,
                                                    RGBVAL[(GRGBVAL[img.get_pixel(x, y)[0] as usize]
                                                    [img.get_pixel(x, y)[1] as usize]
                                                    [img.get_pixel(x, y)[2] as usize]) as usize][2] as u8,
                                                255
                                                ]))
            }}};
}

fn toCCText(img: &image::DynamicImage) -> String{
    let mut retval = String::new();
    let hex = "0123456789abcdef";


    unsafe{ // needs to be in unsafe block because of the static variable   -
        for y in 0..img.height(){                           // we are using -
            for x in 0..img.width(){
                retval.push(
                hex.chars().nth(
                    GRGBVAL[img.get_pixel(x, y)[0] as usize]
                    [img.get_pixel(x, y)[1] as usize]
                    [img.get_pixel(x, y)[2] as usize]
                    as usize).unwrap());
            }
            retval.push('\n');}
        };

    return retval
}

fn ditherImage(img: &mut image::DynamicImage){
    let w1 = 7.0/16.0;
    let w2 = 1.0/16.0;
    let w3 = 5.0/16.0;
    
    for x in 1..img.width()-1{
        for y in 1..img.height()-1{
            let t = truecolor(img.get_pixel(x, y)[0] as i32, img.get_pixel(x, y)[1] as i32, img.get_pixel(x, y)[2] as i32) as usize;
            let quantR = img.get_pixel(x, y)[0] as f64 - RGBVAL[t][0] as f64;
            let quantG = img.get_pixel(x, y)[1] as f64 - RGBVAL[t][1] as f64;
            let quantB = img.get_pixel(x, y)[2] as f64 - RGBVAL[t][2] as f64;

            img.put_pixel(x, y, image::Rgba([
            
                RGBVAL[t][0] as u8,
                RGBVAL[t][1] as u8,
                RGBVAL[t][2] as u8,
                
                255
            ]));
            
            img.put_pixel(x+1, y, image::Rgba([
            
                (img.get_pixel(x+1, y)[0] as f64 + quantR * w1) as u8,
                (img.get_pixel(x+1, y)[1] as f64 + quantG * w1) as u8,
                (img.get_pixel(x+1, y)[2] as f64 + quantB * w1) as u8,
                
                255
            ]));

            img.put_pixel(x+1, y+1, image::Rgba([
            
                (img.get_pixel(x+1, y+1)[0] as f64 + quantR * w2) as u8,
                (img.get_pixel(x+1, y+1)[1] as f64 + quantG * w2) as u8,
                (img.get_pixel(x+1, y+1)[2] as f64 + quantB * w2) as u8,
                
                255
            ]));

            img.put_pixel(x, y+1, image::Rgba([
            
                (img.get_pixel(x, y+1)[0] as f64 + quantR * w3) as u8,
                (img.get_pixel(x, y+1)[1] as f64 + quantG * w3) as u8,
                (img.get_pixel(x, y+1)[2] as f64 + quantB * w3) as u8,
                
                255
            ]));

        }
    }
}

fn main() -> io::Result<()> {
    env::set_var("RUST_BACKTRACE", "1");

    let starttime = Instant::now(); // Start measuring time without preparations

    unsafe{
    for x in 0..256{            // Ok I'm not sure if this is SAFE. HAH. GET IT. SAFE. CUZ IT'S UNSAFE BLOCK?
        for y in 0..256{        // anyway it says that using static variables is dangerous but i think it's
            for z in 0..256{    // actually pretty useful as we don't have to use truecolor() function and wait for it
                GRGBVAL[x][y][z] = truecolor(x as i32, y as i32, z as i32);
            }
        }
    }}
    let preparetime = starttime.elapsed();

    let mut img = image::open("input.png").unwrap(); // Open an image as DynamicImage
    let opentime = starttime.elapsed() - preparetime;


    img = img.resize_exact(164, 81, FilterType::Nearest); // resize image
    let resizetime = starttime.elapsed() - opentime - preparetime;

    ditherImage(&mut img);
    toCCImage(&mut img); // pass reference to the image to function so we can convert it to our format
    let converttime = starttime.elapsed() - opentime - preparetime - resizetime;


    let coolstring = toCCText(&img);
    fs::write("output.txt", coolstring).expect("Can't write to file");
    img.save("output.png").unwrap();
    let savetime =  starttime.elapsed() - opentime - preparetime - resizetime -  converttime;

    println!("Statistics for CC:IA-rust (+dithering)...");
    println!("Open, Converted and saved in: {:?}", (starttime.elapsed() - preparetime));
    println!("Program ran in    : {:?}", starttime.elapsed());
    println!("Time to open      : {:?}", opentime);
    println!("Time to resize    : {:?}", resizetime);
    println!("Time to convert   : {:?}", converttime);
    println!("Time to save stuff: {:?}", savetime);

    Ok(())
}
