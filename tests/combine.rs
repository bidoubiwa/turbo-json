use serde_json::json;
use turbo_json_checker::*;

// fn parse(text: &str) -> io::Result<JsonType> {
//     let mut string = String::new();
//     let mut checker = JsonChecker::new(text.as_bytes());
//     checker.read_to_string(&mut string)?;
//     let outer_type = checker.finish()?;
//     Ok(outer_type)
// }

#[test]

fn one_file_combination() {
    let mut json_paths = ["./misc/empty.json", "./misc/hello.json"];
    let mut output = String::new();

    // assert_eq!(&*output, json);
}
