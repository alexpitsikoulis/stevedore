extern crate iron_exec;
mod auth;
mod api;

use api::{JobRunner, RunnerServer};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse()?;
    let worker = iron_exec::worker::Worker::new(iron_exec::worker::Config::default())?;
    let job_runner = JobRunner::new(Box::new(worker));

    Server::builder()
        .add_service(RunnerServer::new(job_runner))
        .serve(addr)
        .await?;

    Ok(())
}
