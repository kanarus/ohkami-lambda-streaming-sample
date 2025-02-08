use ohkami::prelude::*;
use ohkami::sse::DataStream;

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    let o = Ohkami::new((
        "/".GET(streaming_hello),
    ));

    lambda_runtime::run(o).await
}

async fn streaming_hello(name: String) -> DataStream {
    use tokio::time::{sleep, Duration};

    DataStream::new(move |mut s| async move {
        s.send("Hello, ".to_owned());

        for c in name.chars() {
            sleep(Duration::from_secs(1)).await;
            s.send(c.to_string());
        }
    })
}
