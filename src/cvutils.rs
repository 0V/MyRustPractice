use std::io;
use std::env;

extern crate cv;
use cv::*;

pub fn do_main(){
        
}


extern crate cv;
use cv::highgui::*;
use cv::imgcodecs::ImageReadMode;
use cv::*;

fn do_show_image() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: display_image ImageToLoadAndDisplay");
        std::process::exit(-1);
    }

    let mat = Mat::from_path(&args[1], ImageReadMode::Color).expect("Failed to read from path");

    if !mat.is_valid() {
        println!("Could not open or find the image");
        std::process::exit(-1);
    }

    highgui_named_window("Display window", WindowFlag::Normal).unwrap();
    mat.show("Display window", 0).unwrap();
}

pub fn do_blur() { 
    let root_path: String = env::args().next().unwrap_or("");
    let args: Vec<String> = env::args().skip(1).collect();

    let file_name = args[0];
}

pub fn wallis_fomula(count : u64) -> f64 {
    let mut pi : f64 = 1.0;

    for i in 1..count {
        let n = (4 * i * i) as f64;
        pi *= n / (n - 1.0);
    }
    let ans = pi * 2.0;
    return ans;
}
