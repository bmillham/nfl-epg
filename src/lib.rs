use std::{fs, io};
use std::fs::File;
use std::io::{BufReader, Write};
use libflate::gzip::Decoder;
use quick_xml::de::from_reader;
use xmltv::*;

fn to_old(fname: &str) -> bool {
    match fs::exists(fname) {
        Ok(true) => {
            let metadata = fs::metadata(fname).unwrap();
            let et = metadata.modified().unwrap().elapsed().unwrap().as_secs();
            if et < 60*60*23 {
                false
            } else { true }
        }
        Ok(false) => true,
        Err(_) => true,
    }
}

pub async fn get_and_unzip(url: &str) -> Tv {
    let fname = url.split("/").last().unwrap();
    if !to_old(fname) {
        println!("Using local file {fname} as it's less than 23 hours old");
    } else {
        println!("Downloading sports epg from {url} to {fname}");
        let resp = reqwest::get(url).await.unwrap();
        println!("Status: {}", resp.status());
        let mut out = File::create(fname).expect("Unable to create file");
        //let content = resp.bytes().expect("Request failed");
        let content = resp.bytes().await.unwrap();
        out.write_all(&content).expect("Unable to write file");
    }
    let gzf = File::open(fname).expect("Unable to open gz file");
    let mut gzbr = BufReader::new(gzf);
    let mut decoder = Decoder::new(&mut gzbr).unwrap();
    let outfile = decoder.header().filename().unwrap().to_str().unwrap().to_string();
    println!("Unzipping to: {}", outfile);
    let mut of = File::create(outfile.clone()).expect("Unable to \
    open file");
    io::copy(&mut decoder, &mut of).expect("Unable to copy file");
    println!("Unzipping done");
    let f = File::open(outfile).expect("file not found");
    let mut br = BufReader::new(f);
    println!("Reading");
    let item: Tv = from_reader(&mut br).expect("couldn't read item");
    println!("Done");
    item
}