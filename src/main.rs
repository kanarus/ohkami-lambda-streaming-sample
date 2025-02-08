use ohkami::prelude::*;
use ohkami::sse::DataStream;

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    let o = Ohkami::new((
        "/".GET(get_streaming_hello),
        "/:name".GET(get_streaming_hello_with_name),
    ));

    lambda_runtime::run(o).await
}

async fn get_streaming_hello() -> DataStream {
    hello_stream(None)
}

async fn get_streaming_hello_with_name(name: String) -> DataStream {
    hello_stream(Some(name))
}

fn hello_stream(name: Option<String>) -> DataStream {
    use tokio::time::{sleep, Duration};

    let name = name.unwrap_or_else(|| String::from("world"));

    DataStream::new(move |mut s| async move {
        s.send("Hello, ".to_owned());

        for c in name.chars() {
            sleep(Duration::from_secs(1)).await;
            s.send(c.to_string());
        }
    })
}
