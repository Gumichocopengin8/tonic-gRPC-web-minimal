pub mod pb {
    tonic::include_proto!("grpc.examples.echo");
}

use pb::{EchoRequest, EchoResponse};
use std::{net::ToSocketAddrs, pin::Pin, time::Duration};
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};
use tonic::{transport::Server, Request, Response, Status};
use tower_http::cors::{Any, CorsLayer};

type EchoResult<T> = Result<Response<T>, Status>;

#[derive(Debug)]
pub struct EchoServer {}

#[tonic::async_trait]
impl pb::echo_server::Echo for EchoServer {
    async fn unary_echo(&self, req: Request<EchoRequest>) -> EchoResult<EchoResponse> {
        let message = pb::EchoResponse {
            message: format!("Hello {}!", req.into_inner().message),
        };
        Ok(Response::new(message))
    }

    type ServerStreamingEchoStream =
        Pin<Box<dyn Stream<Item = Result<EchoResponse, Status>> + Send>>;

    async fn server_streaming_echo(
        &self,
        req: Request<EchoRequest>,
    ) -> EchoResult<Self::ServerStreamingEchoStream> {
        println!("EchoServer::server_streaming_echo");
        println!("\tclient connected from: {:?}", req.remote_addr());

        // creating infinite stream with requested message
        let repeat = std::iter::repeat(EchoResponse {
            message: format!("Hello {}!", req.into_inner().message),
        });
        let mut stream = Box::pin(tokio_stream::iter(repeat).throttle(Duration::from_millis(200)));

        // spawn and channel are required if you want handle "disconnect" functionality
        // the `out_stream` will not be polled after client disconnect
        let (tx, rx) = mpsc::channel(128);
        tokio::spawn(async move {
            while let Some(item) = stream.next().await {
                match tx.send(Result::<_, Status>::Ok(item)).await {
                    Ok(_) => {
                        // item (server response) was queued to be send to client
                    }
                    Err(_item) => {
                        // output_stream was build from rx and both are dropped
                        break;
                    }
                }
            }
            println!("\tclient disconnected");
        });

        let output_stream = ReceiverStream::new(rx);
        Ok(Response::new(
            Box::pin(output_stream) as Self::ServerStreamingEchoStream
        ))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let echo_server = EchoServer {};

    println!("start server");
    Server::builder()
        .accept_http1(true)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_headers(Any)
                .allow_methods(Any)
                .max_age(Duration::from_secs(60) * 30),
        )
        .layer(tonic_web::GrpcWebLayer::new())
        .add_service(pb::echo_server::EchoServer::new(echo_server))
        .serve("0.0.0.0:8000".to_socket_addrs().unwrap().next().unwrap())
        .await
        .unwrap();
    Ok(())
}
