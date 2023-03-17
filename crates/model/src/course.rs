use super::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Course {
  pub title: String,
  pub credits: String,
  pub subject: String,
  pub code: String,
  pub level: String,
  pub url: String,
  pub department: String,
  pub faculty: String,
  pub faculty_url: String,
  pub terms: Vec<String>,
  pub description: String,
  pub instructors: Vec<Instructor>,
  pub prerequisites: Vec<String>,
  pub corequisites: Vec<String>,
  pub schedule: Vec<Schedule>,
}