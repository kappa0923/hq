// use html5ever::driver::ParseOpts;
// use html5ever::parse_document;
// use html5ever::rcdom::{RcDom, Node, NodeData, Handle};
// use html5ever::serialize;
// use html5ever::serialize::SerializeOpts;
// use html5ever::tendril::TendrilSink;
use scraper::{Selector, Html};
// use html5ever::rcdom::{RcDom, Handle, Element, ElementEnum, NodeEnum, Node, Text};

fn main() {
    // 引数を受け取る
    let args: Vec<String> = std::env::args().collect();
    let element = &args[1];

    // DOM操作部分
    let html_data = r#"
        <!DOCTYPE html>
        <head>
            <meta name="referrer" content="origin">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <link rel="stylesheet" type="text/css" href="news.css?yeOCTq4uYdV5J7KbDSdX">
            <link rel="shortcut icon" href="favicon.ico">
            <link rel="alternate" type="application/rss+xml" title="RSS" href="rss">
            <title>
                Hacker News
            </title>
        </head>
        <body></body>
        </html>"#;

    let result = extract_element(html_data, element);
    println!("{}", &result);
}

fn extract_element(html_data: &str, element: &str) -> String {
    // let parser = parse_document(RcDom::default(), ParseOpts::default());
    // let dom = parser.one(html_data);
    // let dom = parse_document(RcDom::default(), ParseOpts::default())
    //     .from_utf8()
    //     .read_from(&mut html_data.as_bytes())
    //     .unwrap();
    // let node = &dom.document.children.borrow()[0];
    // if let NodeData::Element(ref name, ref element_enum, ref attrs) = node.data {
    //     println!("{:?}", name, element_enum, attrs);
    // }

    let document = Html::parse_document(html_data);
    let selector = Selector::parse(&element).unwrap();

    // let head = document.select(&selector).next().unwrap();
    // println!("{}", head.html());

    let mut result = "".to_string();

    for elem in document.select(&selector) {
        // println!("{}", elem.html());
        result.push_str(&elem.html());
        result.push('\n')
    }

    // match &node.data {
    //     NodeData::Document => {}
    //     NodeData::Doctype { name, public_id, system_id } => {}
    //     NodeData::Text { contents } => {}
    //     NodeData::Comment { contents } => {}
    //     NodeData::Element { name, attrs, .. } => {
    //         println!("{:?}", name.prefix.unwrap());
    //     }
    //     NodeData::ProcessingInstruction { target, contents } => {}
    // }

    // let mut bytes = vec![];
    // serialize(&mut bytes, node, SerializeOpts::default()).unwrap();
    // // serialize(&mut bytes, &dom.document, SerializeOpts::default()).unwrap();
    // String::from_utf8(bytes).unwrap()
    // "hogehoge".to_string()
    // head.html()
    result
}
