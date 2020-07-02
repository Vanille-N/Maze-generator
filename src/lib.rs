pub mod dfs;
pub mod kruskal;
pub mod prim;
pub mod grid;
pub mod union_find;

use crate::grid::Grid;


enum Algorithms {
    Dfs,
    Kruskal,
    Prim,
}

pub struct Config {
    algo: Algorithms,
    width: usize,
    height: usize,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Self, String> {
        if args.len() == 1 {
            return Err(String::from("Not enough arguments. Type `help` for more information."));
        } else if args[1] == "help" {
            println!("$ maze ALGORITHM HEIGHT WIDTH");
            println!("Where");
            println!("    ALGORITHM : (prim | kruskal | dfs)");
            println!("    HEIGHT    : usize");
            println!("    WIDTH     : usize");
            return Err(String::from("Done."));
        } else if args.len() < 4 {
            return Err(String::from("Not enough arguments. Type `help` for more information."));
        }
        let algo = match args[1].as_str() {
            "prim" => Algorithms::Prim,
            "kruskal" => Algorithms::Kruskal,
            "dfs" => Algorithms::Dfs,
            s => return Err(format!("Unknown algorithm '{}'", s)),
        };
        let height = match args[2].parse::<usize>() {
            Ok(n) => n,
            Err(_) => return Err(format!("'{}' not recognized as usize", args[2])),
        };
        let width = match args[3].parse::<usize>() {
            Ok(n) => n,
            Err(_) => return Err(format!("'{}' not recognized as usize", args[3])),
        };
        Ok(Config { algo, width, height })
    }
}

pub fn run(cfg: Config) {
    let mut g = Grid::new(cfg.height, cfg.width);
    match cfg.algo {
        Algorithms::Dfs => dfs::build(&mut g),
        Algorithms::Kruskal => kruskal::build(&mut g),
        Algorithms::Prim => prim::build(&mut g),
    }
    println!("{}", g);
}
