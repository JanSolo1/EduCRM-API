# EduCRM-API
> Overview of the EduCRM-API Objects, calls and interactions 

## Student Object
The student field is only for students, there are other objects for interacting with staff or schedules.
Here is the student objects
```json
{
    "uuid": "Primary ID",
    "student_id": "ID number of student as string",
    "student_first_name": "First Name",
    "student_middle_name": "Optional, Middle Name",
    "student_last_name": "Last Name",
    "student_dob": "YYYY-MM-DD",
    "student_gender": Boolean
}
```

### Create Student

Create a student with a post request.

Method: **Post**

Endpoint: **/student/create**

Payload:
```
- uuid (Not Required)
- student_first_name (Required)
- student_middle_name (Optional)
- student_last_name (Required)
- student_dob": (Required)
- student_gender (Optional)
```


