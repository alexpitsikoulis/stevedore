extern crate iron_exec;
mod api;
mod auth;

use api::{JobRunner, RunnerServer};
use auth::{client::CertificateAuthorityClient, get_ca_certificate, get_certificate};
use tonic::transport::{Certificate, Identity, Server, ServerTlsConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ca_client = CertificateAuthorityClient::new("http://127.0.0.1:50052".parse()?).await?;
    let ca_certificate = get_ca_certificate(&mut ca_client).await?;
    let (certificate, private_key) = get_certificate(&mut ca_client).await?;
    let certificate = Certificate::from_pem(certificate);

    let server_identity = Identity::from_pem(certificate, private_key);
    let tls_config = ServerTlsConfig::new()
        .identity(server_identity)
        .client_ca_root(Certificate::from_pem(ca_certificate));

    let addr = "127.0.0.1:50051".parse()?;
    let worker = iron_exec::worker::Worker::new(iron_exec::worker::Config::default())?;
    let job_runner = JobRunner::new(Box::new(worker));

    Server::builder()
        .tls_config(tls_config)?
        .add_service(RunnerServer::new(job_runner))
        .serve(addr)
        .await?;

    Ok(())
}
