use std::str;

use include_assets::{include_dir, NamedArchive};

pub fn load(token: &str) {
    let archive = NamedArchive::load(include_dir!("assets"));
    let lns = str::from_utf8(archive.get("data.txt").unwrap()).unwrap().lines();
    let _ = lns.for_each(|x| {
        let inx = x.find(token);
        match inx {
            Some(_inx) => {
                let v: Vec<&str> = x.split('|').collect();
                print!("[\\{}]  {} \t", v[1], v[2]);
            },
            None => {}
        }
    });
    print!("\r\n");
}