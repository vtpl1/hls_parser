#[macro_use]
extern crate clap;
extern crate reqwest;
use clap::{App, Arg};

fn main() {
    /*
     -h, --help         show this help message and exit
  -s SEGMENTS        Number of segments to be analyzed per playlist
  -l FRAME_INFO_LEN  Max number of frames per track whose information will be
                     reported
    */
    let matches = App::new("HLS Parser")
        .version(crate_version!())
        .get_matches();

    // let segments = matches.value_of("segments").unwrap_or("1");
    // println!("Value for segments: {}", segments);

    // let url = matches.value_of("url").unwrap();
    // println!("Value for segments: {}", url);

    // more program logic goes here...
    let client = reqwest::Client::new();
    let _res = client
        .post("http://httpbin.org/post")
        .body("the exact body that is sent")
        .send();
}
