use anyhow::Result;
use mongodb::bson::oid::ObjectId;
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub enum JobState {
    #[default]
    Created,
    Selected,
    Deployed,
    Running,
    Error,
    Finish,
    Clean,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Job {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub graph_json: String,
    pub state: JobState,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JobUpdateInfo {
    pub state: Option<JobState>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListJobParams {
    pub state: Option<JobState>,
}
pub trait JobRepo {
    fn insert(&self, job: &Job) -> impl std::future::Future<Output = Result<Job>> + Send;

    fn get(&self, id: &ObjectId) -> impl std::future::Future<Output = Result<Option<Job>>> + Send;

    fn delete(&self, id: &ObjectId) -> impl std::future::Future<Output = Result<()>> + Send;

    fn get_job_for_running(&self) -> impl std::future::Future<Output = Result<Option<Job>>> + Send;

    fn update(
        &self,
        id: &ObjectId,
        info: &JobUpdateInfo,
    ) -> impl std::future::Future<Output = Result<()>> + Send;

    fn list_jobs(
        &self,
        list_job_params: &ListJobParams,
    ) -> impl std::future::Future<Output = Result<Vec<Job>>> + Send;
}

pub trait MainDbRepo = JobRepo + Clone + Send + Sync + 'static;
