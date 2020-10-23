use scraper::{Selector, Html};
use structopt::StructOpt;
use std::io::{self, Read};

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(name = "ELEMENT NAME")]
    element: String,
}

fn main() {
    // 引数を受け取る
    let element = &Opt::from_args().element;

    let html_data = read_from_stdin().unwrap();

    let result = extract_element(&html_data, element);
    println!("{}", &result);
}

/// htmlから指定した要素を抜き出す
fn extract_element(html_data: &str, element: &str) -> String {
    let document = Html::parse_document(html_data);
    let selector = Selector::parse(&element).unwrap();

    let mut result = String::new();

    for elem in document.select(&selector) {
        result.push_str("\x1b[32m");
        result.push_str(&elem.html());
        result.push_str("\x1b[m");
        result.push('\n');
    }

    return result;
}

/// 標準入力を読み込む
fn read_from_stdin() -> io::Result<String> {
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer)
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn parse_check_html() {
//         let html_src = r#"
//           <!DOCTYPE html>
//           <head>
//               <meta name="referrer" content="origin">
//               <meta name="viewport" content="width=device-width, initial-scale=1.0">
//               <link rel="stylesheet" type="text/css" href="news.css?yeOCTq4uYdV5J7KbDSdX">
//               <link rel="shortcut icon" href="favicon.ico">
//               <link rel="alternate" type="application/rss+xml" title="RSS" href="rss">
//               <title>
//                   hq is a Html Query.
//               </title>
//           </head>
//           <body></body>
//           </html>"#;

//         let expect_html = r#"<meta name="referrer" content="origin">
// "#;

//         let result = extract_element(html_src, "meta[name=\"referrer\"]");
//         assert_eq!(result, expect_html.to_string());
//     }
// }
