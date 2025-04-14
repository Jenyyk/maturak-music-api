
# SERVER SIDE:
before launch, create a `.env` file and insert a variable for the admin password:
```
DATABASE_PASSWORD = 12345
```
If no variable is found, the password defaults to `admin`  

# CLIENT SIDE API:
**ENDPOINT `/music`**
## `GET` Method:
The get method is used to list all songs currently in the queue.  
Answers are:  

### `200 OK` + Json  
Returns Json array of all song structures.  
Currently the song structure returns only the link supplied when adding and a unique UUID.
## `POST` Method:
The post method is used to add a song to the queue.  
The API expects a `Content-Type: application/json` header.
The request body should be a raw String regardless, supplied as such: `'"example link"'`  
Answers are:  

### `201 CREATED` + Json  
The link was successfully aded to queue.  
### `409 CONFLICT` + Json  
A song with the same link already exists in the queue.
### `400 BAD REQUEST` + Json  
A malformed request body was sent that could not be deserialized.  

## `DELETE` Method:
The delete method is used by queue administrators to delete an entry from the queue.  
The API expects a `Content-Type: application/json` header.
The request body should be a JSON structure supplied as such: `'{"id": "song uuid", "password": "server side password"}'`  
The song id is returned by the `GET` method.  
Answers are:  

### `200 OK` + Json  
The song was successfully deleted from the queue.  
### `400 BAD REQUEST` + Json  
Recieved malformed data, see response body for more information.  
### `401 UNAUTHORIZED` + Json  
An incorrect password was entered.  
### `404 NOT FOUND` + Json  
The song was not found inside the database, or another database error has arisen.
