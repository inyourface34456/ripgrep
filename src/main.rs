mod utils;
use std::env;
use utils::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result<'a>() {
        let args = Config {
            query: "duct",
            file_path: "src/test.txt",
            case: false,
            regex: false,
        };

        assert_eq!(vec!["Rust is productive"], args.search());
    }

    #[test]
    fn case_insesative() {
        let args = Config {
            query: "rUsT",
            file_path: "src/test.txt",
            case: true,
            regex: false,
        };

        assert_eq!(
            vec!["coding in rust can be fun", "Rust is productive"],
            args.search()
        );
    }

    #[test]
    fn regep() {
        let args = Config {
            query: r"(\d{4})-(\d{2})-(\d{2})",
            file_path: "src/test.txt",
            case: false,
            regex: true,
        };

        assert_eq!(
            vec!["2012-03-14", "2013-01-01", "2014-07-05"],
            args.search()
        );
    }
}

fn main() {
    let arg = env::args().collect::<Vec<String>>();

    let args = Config::parce_args(&arg);

    let results = args.search();

    for i in results {
        println!("{i}");
    }
}
