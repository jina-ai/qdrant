pub mod jina {
    tonic::include_proto!("jina");
}
use jina::DocumentProto;

fn main() {
    let doc = DocumentProto::default();
    println!("Hello World!");
}
