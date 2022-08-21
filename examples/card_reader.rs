use pcsc::{Attribute, Context, Error, Protocols, Scope, ShareMode, MAX_ATR_SIZE, MAX_BUFFER_SIZE};

#[tokio::main]
async fn main() {
    let ctx = Context::establish(Scope::User).unwrap();

    let mut readers_buffer = [0; 2048];

    let mut readers = ctx.list_readers(&mut readers_buffer).unwrap();

    let reader = match readers.next() {
        Some(reader) => reader,
        None => {
            println!("No readers are connected");
            std::process::exit(0);
        }
    };
}
