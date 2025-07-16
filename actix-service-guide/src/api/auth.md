# Chapter 1


### Table of Contents
* [login User](#login-user)
* [SignUp User](#sign-in-user)
* [Change Password]()


  
   

---

## Login User

<div class="endpoint">
    <div class="method-get">get</div>
    https://127.0.0.1:8080/login   
</div>

#### Request JSON Payload    
```json
{
    "username": String,
    "password": String
}
```

#### Response Payload
Response will return a set cookie 
```
{

}
```


---

## Sign in User

<div class="endpoint">
    <div class="method-post">post</div>
    https://127.0.0.1:8080/signup   
</div>

#### Request JSON Payload    
```json
{
    "email": String,
    "password": String,
    "username": String,
    "first_name": String,
    "last_name": String,
    "phone_no": String,
    "location": String | null
}
```
#### Response Payload
```
{

}
```


## Change Password

<div class="endpoint">
    <div class="method-put">put</div>
    https://127.0.0.1:8080/change-password   
</div>

#### Request JSON Payload    
```json
{
    "password": String,
    "new_password": String,
    "confirm_password": String
}
```
#### Response Payload
```
{

}
```


## Forgot Password

<div class="endpoint">
    <div class="method-post">post</div>
    https://127.0.0.1:8080/request-change-password   
</div>

#### Request JSON Payload    
```json
{
    "username": String
}
```
#### Response Payload
```
{
    
}
```




