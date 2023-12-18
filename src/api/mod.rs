use tonic::{transport::Server, Request, Response, Status};

pub mod runner {
    tonic::include_proto!("runner");
}

use runner::runner_server::Runner;
use runner::{StartJobRequest, StartJobResponse, StopJobRequest, StopJobResponse, QueryJobRequest, QueryJobResponse, StreamJobRequest, StreamJobResponse };
pub use runner::runner_server::RunnerServer;

#[derive(Default)]
pub struct JobRunner {}

#[tonic::async_trait]
impl Runner for JobRunner {
    async fn start_job(
        &self,
        request: Request<StartJobRequest>,
    ) -> Result<Response<StartJobResponse>, Status> {
        let response = StartJobResponse {
            job_id: "test string".into(),
        };
        Ok(Response::new(response))
    }

    async fn stop_job(
        &self,
        request: Request<StopJobRequest>,
    ) -> Result<Response<StopJobResponse>, Status> {
        let response = StopJobResponse {

        };
        Ok(Response::new(response))
    }

    async fn query_job(
        &self,
        request: Request<QueryJobRequest>,
    ) -> Result<Response<QueryJobResponse>, Status> {
        let response = QueryJobResponse {
            status: "test status".into(),
            pid: None,
            exit_code: None,
        };
        Ok(Response::new(response))
    }

    async fn stream_job(
        &self,
        request: Request<StreamJobRequest>,
    ) -> Result<Response<StreamJobResponse>, Status> {
        let response = StreamJobResponse {
            output: vec![1,2,3],
        };
        Ok(Response::new(response))
    }
}
