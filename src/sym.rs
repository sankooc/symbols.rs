use std::str;

use include_assets::{include_dir, NamedArchive};
use term_grid::{Grid, GridOptions, Direction, Filling, Cell};


pub fn load(token: &str) {
    let archive = NamedArchive::load(include_dir!("assets"));
    let lns = str::from_utf8(archive.get("data.txt").unwrap()).unwrap().lines();
    let mut grid = Grid::new(GridOptions {
        filling:     Filling::Spaces(2),
        direction:   Direction::LeftToRight,
    });
    let _ = lns.for_each(|x| {
        let inx = x.find(token);
        match inx {
            Some(_inx) => {
                let v: Vec<&str> = x.split('|').collect();
                grid.add(Cell::from(v[1]));
                grid.add(Cell::from(v[2]));
            },
            None => {}
        }
    });
    println!("{}", grid.fit_into_width(100).unwrap());
}