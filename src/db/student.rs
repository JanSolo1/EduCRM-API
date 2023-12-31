use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Student {
    pub uuid: Option<String>,
    pub student_id: u32,
    pub student_first_name: String,
    pub student_middle_name: Option<String>,
    pub student_last_name: String,
    pub student_dob: String,
    pub student_email: String,
    pub student_phone: String,
    pub student_gender: Option<String>,
}
