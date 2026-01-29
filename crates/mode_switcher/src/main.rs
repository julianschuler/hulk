use std::io::stdin;

use cdr::{CdrLe, Infinite};
use high_level_interface::{Mode, Request};
use zenoh::Config;

#[tokio::main]
async fn main() {
    let session = zenoh::open(Config::default()).await.unwrap();
    let publisher = session
        .declare_publisher("booster/high_level_interface")
        .await
        .unwrap();

    let mut buffer = String::new();
    let stdin = stdin();

    loop {
        buffer.clear();
        stdin.read_line(&mut buffer).unwrap();

        let request = match buffer.trim() {
            "d" => Some(Request::change_mode(Mode::Damping)),
            "c" => Some(Request::change_mode(Mode::Custom)),
            _ => None,
        };

        if let Some(request) = request {
            dbg!(&request);

            let serialized_request = cdr::serialize::<_, _, CdrLe>(&request, Infinite).unwrap();

            publisher.put(serialized_request).await.unwrap()
        }
    }
}
