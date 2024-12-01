use serde::{Deserialize, Serialize};
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, Stdout};

use serde::ser::Serialize as SerializeTrait;

#[derive(Deserialize)]
struct InitRequest {
    msg_id: u64,
}

#[derive(Serialize)]
struct InitResponse {
    src: String,
    dest: String,
    body: InitResponseBody,
}

#[derive(Serialize)]
struct InitResponseBody {
    msg_id: u64,
    in_reply_to: u64,
    #[serde(rename = "type")]
    message_type: String,
}

#[derive(Deserialize)]
struct EchoRequest {
    msg_id: u64,
    echo: String,
}

#[derive(Serialize)]
struct EchoResponse {
    src: String,
    dest: String,
    body: EchoResponseBody,
}

#[derive(Serialize)]
struct EchoResponseBody {
    msg_id: u64,
    in_reply_to: u64,
    echo: String,
    #[serde(rename = "type")]
    message_type: String,
}

async fn write_to_stdout<T: SerializeTrait>(stdout: &mut Stdout, t: T) -> anyhow::Result<()> {
    let _result = stdout
        .write_all(serde_json::to_string(&t).unwrap().as_bytes())
        .await?;
    stdout.write_all(b"\n").await?;
    return Ok(());
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let stdin = io::BufReader::new(io::stdin());
    let mut stdout = io::stdout();

    let mut lines = stdin.lines();
    while let Ok(Some(line)) = lines.next_line().await {
        let request: serde_json::Value = serde_json::from_str(&line).unwrap();

        // Handle "init" message
        if let Some(body) = request.get("body") {
            if body.get("type") == Some(&serde_json::Value::String("init".into())) {
                let msg: InitRequest = serde_json::from_value(body.clone()).unwrap();
                let reply = InitResponse {
                    src: request["dest"].as_str().unwrap().to_string(),
                    dest: request["src"].as_str().unwrap().to_string(),
                    body: InitResponseBody {
                        msg_id: 1,
                        in_reply_to: msg.msg_id,
                        message_type: String::from("init_ok"),
                    },
                };

                write_to_stdout(&mut stdout, &reply).await?;
            }

            // Handle "echo" message
            if body.get("type") == Some(&serde_json::Value::String("echo".into())) {
                let msg: EchoRequest = serde_json::from_value(body.clone()).unwrap();
                let reply = EchoResponse {
                    src: request["dest"].as_str().unwrap().to_string(),
                    dest: request["src"].as_str().unwrap().to_string(),
                    body: EchoResponseBody {
                        msg_id: 1,
                        in_reply_to: msg.msg_id,
                        echo: msg.echo,
                        message_type: String::from("echo_ok"),
                    },
                };
                write_to_stdout(&mut stdout, &reply).await?;
            }
        }
    }
    Ok(())
}
