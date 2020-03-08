#[async_std::main]
async fn main() {
    match feather_packet_parser::compile_def("packets/1.15.2.def", "src/packets/v1_15_2.rs").await {
        Ok(_) => (),
        Err(e) => {
            panic!("{}", e);
        }
    }
}
