mod tcp;

use std::env;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let protocol: &str = &args[1];
    let role: &str = &args[2];
    match protocol {
        "tcp" => match role {
            "server" => {
                tcp::server::serve();
            }
            _ => {}
        },
        _ => {}
    }
}
