use html5ever::driver::ParseOpts;
use html5ever::parse_document;
use html5ever::rcdom::RcDom;
use html5ever::serialize;
use html5ever::serialize::SerializeOpts;
use html5ever::tendril::TendrilSink;

fn main() {
    let html_data = "これは<span>テスト</span>です。";
    let parser = parse_document(RcDom::default(), ParseOpts::default());
    let dom = parser.one(html_data);
    let mut bytes = vec![];
    serialize(&mut bytes, &dom.document, SerializeOpts::default()).unwrap();
    println!("{}", String::from_utf8(bytes).unwrap());
}
