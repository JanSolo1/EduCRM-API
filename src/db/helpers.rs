use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Student {
    pub uuid: Option<String>,
    pub student_id: String,
    pub student_first_name: String,
    pub student_middle_name: Option<String>,
    pub student_last_name: String,
    pub student_medical_aid: Option<String>,
    pub student_doctor: Option<String>,
    pub student_doctor_phone: Option<String>,
    pub student_gender: Option<String>,
    pub student_dob: String,
    pub student_race: Option<String>,
    pub student_phone: Option<String>,
    pub student_email: Option<String>,
    pub student_citizenship: Option<String>,
    pub student_home_language: String,
    pub student_resides_with: String,
    pub student_disabilities: Option<Vec<String>>,
    pub student_learning_disabilities: Option<Vec<String>>,

    pub student_father_figure_id_num: Option<String>,
    pub student_father_figure_title: Option<String>,
    pub student_father_figure_first_name: Option<String>,
    pub student_father_figure_middle_name: Option<String>,
    pub student_father_figure_last_name: Option<String>,
    pub student_father_figure_occupation: Option<String>,
    pub student_father_figure_employer: Option<String>,
    pub student_father_figure_phone: Option<String>,
    pub student_father_figure_work_phone: Option<String>,
    pub student_father_figure_home_phone: Option<String>,
    pub student_father_figure_email: Option<String>,
    pub student_father_figure_address: Option<String>,

    pub student_mother_figure_id_num: Option<String>,
    pub student_mother_figure_title: Option<String>,
    pub student_mother_figure_first_name: Option<String>,
    pub student_mother_figure_middle_name: Option<String>,
    pub student_mother_figure_last_name: Option<String>,
    pub student_mother_figure_occupation: Option<String>,
    pub student_mother_figure_employer: Option<String>,
    pub student_mother_figure_phone: Option<String>,
    pub student_mother_figure_work_phone: Option<String>,
    pub student_mother_figure_home_phone: Option<String>,
    pub student_mother_figure_email: Option<String>,
    pub student_mother_figure_address: Option<String>, 
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StudentSearch {
    pub uuid: Option<String>,
    pub student_id: Option<u32>,
}
