use emne::utils::tracing_setup;

#[tokio::main]
async fn main() {
   tracing_setup::init_tracing_subscriber();
}
