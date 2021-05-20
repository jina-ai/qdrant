// fn main() {
//     prost_build::compile_protos(&["src/jina.proto", "src/items.proto"],
//                                 &["src/"]).unwrap();
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("src/jina.proto")?;
    Ok(())
}
