/// The type of work this job represents.
#[derive(Debug)]
pub enum JobType {
    Reconcile,
    HTTPRequest,
    Metric,
}

/// A unit of work flowing through the pipeline. #[derive(Debug]
#[derive(Debug)]
pub struct Job {
    pub id: u8,
    pub job_type: JobType,
    pub payload: String,
}

impl Job {
    pub fn new(id: u8, job_type: JobType) -> Self {
        Job {
            payload: format!("{:?} job #{id}", job_type),
            id,
            job_type,
        }
    }
}
