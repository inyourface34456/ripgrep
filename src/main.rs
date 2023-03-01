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
        let s1 = args.read_file();
        let contents = s1.as_str();

        assert_eq!(
            vec!["Rust is productive"],
            search(args.query, contents, args.case, args.regex)
        );
    }

    #[test]
    fn case_insesative() {
        let args = Config {
            query: "rUsT",
            file_path: "src/test.txt",
            case: true,
            regex: false,
        };
        let s1 = args.read_file();
        let contents = s1.as_str();

        assert_eq!(
            vec!["coding in rust can be fun", "Rust is productive"],
            search(args.query, contents, args.case, args.regex)
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
        let s1 = args.read_file();
        let contents = s1.as_str();

        assert_eq!(
            vec!["2012-03-14", "2013-01-01", "2014-07-05"],
            search(args.query, contents, args.case, args.regex)
        );
    }
}

fn main() {
    let arg = env::args().collect::<Vec<String>>();

    let args = Config::parce_args(&arg);
    let s1 = args.read_file();
    let query = args.query;
    let contents = s1.as_str();

    let results = search(query, contents, args.case, args.regex);

    for i in results {
        println!("{i}");
    }
}
