<div style="text-align: center"><img title="educrm_api_logo" width="400" src="educrm_api_logo.png"></div>

# EduCRM-API
Overview of the EduCRM-API Objects, calls and interactions 

## Student Object
The student field is only for students, there are other objects for interacting with staff or schedules.
Here is the student object
```json
{
    "uuid": "auto generated string",
    "student_id": "000000000000",
    "student_first_name": "First Name",
    "student_middle_name": "Middle Name",
    "student_last_name": "Last Name",
    ... The rest of the fields
}
```

## Student Requests
> Scope: "/student"

|Function          | Request Method   | Endpoint      |
|   -----------    | :-------------:  |:------------- |
|      Create      |        POST      |    /create    |
|       List       |        GET       |     /get      |
|       Read       |        GET       |     /get      |

> Note: The "/get" endpoint serves dual purposes. With parameters, it fetches a specific student's details. Without parameters, it returns a list of all students.