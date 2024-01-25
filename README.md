<div style="text-align: center"><img title="educrm_api_logo" width="400" src="educrm_api_logo.png"></div>

# EduCRM-API
Overview of the EduCRM-API Objects, calls and interactions 

## Student Object
The student field is only for students, there are other objects for interacting with staff or schedules.
Here is the student object
```json
{
    "student_id": "000000000000",
    "student_first_name": "First Name",
    "student_middle_name": "Middle Name",
    "student_last_name": "Last Name",
    ... The rest of the fields
}
```

## Student Requests
> Scope: "/student"

|Function          | Request Method   |       Endpoint       |
|   -----------    | :-------------:  |    :-------------    |
|      Create      |        POST      |        /create       |
|       List       |        GET       |         /get         |
|       Read       |        GET       |   /get/{student_id}  |
|      Update      |        GET       | /update/{student_id} |
|      Delete      |       DELETE     | /delete/{student_id} |

