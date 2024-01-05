use std::io::Read;

use iron_exec::{job::Command, worker::Worker};
use tonic::{Request, Response, Status};

pub mod runner {
    tonic::include_proto!("runner");
}

use runner::runner_server::Runner;
pub use runner::runner_server::RunnerServer;
use runner::{
    QueryJobRequest, QueryJobResponse, StartJobRequest, StartJobResponse, StopJobRequest,
    StopJobResponse, StreamJobRequest, StreamJobResponse,
};
use uuid::Uuid;

pub struct JobRunner {
    worker: Box<Worker>,
}

impl JobRunner {
    pub fn new(worker: Box<Worker>) -> Self {
        JobRunner { worker }
    }
}

#[tonic::async_trait]
impl Runner for JobRunner {
    type StreamJobStream = std::pin::Pin<
        Box<dyn futures::stream::Stream<Item = Result<StreamJobResponse, Status>> + Send + Sync>,
    >;

    async fn start_job(
        &self,
        request: Request<StartJobRequest>,
    ) -> Result<Response<StartJobResponse>, Status> {
        let StartJobRequest { name, args } = request.into_inner();
        let job_id = match self.worker.start(Command::new(name, args), Uuid::new_v4()) {
            Ok(result) => result,
            Err(e) => return Err(Status::internal(e.as_str())),
        };
        let response = StartJobResponse {
            job_id: job_id.to_string(),
        };
        Ok(Response::new(response))
    }

    async fn stop_job(
        &self,
        request: Request<StopJobRequest>,
    ) -> Result<Response<StopJobResponse>, Status> {
        let StopJobRequest {
            job_id,
            owner_id,
            gracefully,
        } = request.into_inner();
        if let Err(e) = self.worker.stop(
            Uuid::parse_str(&job_id).unwrap(),
            Uuid::parse_str(&owner_id).unwrap(),
            gracefully,
        ) {
            return Err(Status::internal(e.as_str()));
        };
        let response = StopJobResponse {};
        Ok(Response::new(response))
    }

    async fn query_job(
        &self,
        request: Request<QueryJobRequest>,
    ) -> Result<Response<QueryJobResponse>, Status> {
        let QueryJobRequest { job_id, owner_id } = request.into_inner();
        let (job_id, owner_id) = match get_job_and_owner_uuids_from_str(job_id, owner_id) {
            Ok(ids) => ids,
            Err(e) => return Err(e),
        };
        let job_info = match self.worker.query(job_id, owner_id) {
            Ok(job_info) => job_info,
            Err(e) => return Err(Status::internal(e.as_str())),
        };
        let response = QueryJobResponse {
            status: job_info.status(),
            pid: job_info.pid(),
            exit_code: job_info.exit_code(),
        };
        Ok(Response::new(response))
    }

    async fn stream_job(
        &self,
        request: Request<StreamJobRequest>,
    ) -> Result<Response<Self::StreamJobStream>, Status> {
        let StreamJobRequest { job_id, owner_id } = request.into_inner();
        let (job_id, owner_id) = match get_job_and_owner_uuids_from_str(job_id, owner_id) {
            Ok(ids) => ids,
            Err(e) => return Err(e),
        };
        let reader = match self.worker.stream(job_id, owner_id) {
            Ok(reader) => reader,
            Err(e) => return Err(Status::internal(e.as_str())),
        };

        let output_stream = async_stream::stream! {
            let mut reader = reader;
            let mut buf = [0;4096];
            loop {
                let size = match reader.read(&mut buf) {
                    Ok(0) => break,
                    Ok(size) => size,
                    Err(e) => {
                        yield Err(Status::internal(format!("failed to read job logs: {:?}", e)));
                        break;
                    },
                };

                yield Ok(StreamJobResponse { output: buf[..size].to_vec()})
            };
        };
        Ok(Response::new(Box::pin(output_stream)))
    }
}

fn get_job_and_owner_uuids_from_str(
    job_id: String,
    owner_id: String,
) -> Result<(Uuid, Uuid), Status> {
    let job_id = match Uuid::parse_str(&job_id) {
        Ok(job_id) => job_id,
        Err(e) => {
            return Err(Status::invalid_argument(format!(
                "invalid job_id provided, please provide valid UUID: {:?}",
                e
            )))
        }
    };
    let owner_id = match Uuid::parse_str(&owner_id) {
        Ok(owner_id) => owner_id,
        Err(e) => {
            return Err(Status::invalid_argument(format!(
                "invalid owner_id provided, please provide valid UUID: {:?}",
                e
            )))
        }
    };
    Ok((job_id, owner_id))
}
