# Chapter 1


### Table of Contents
* [Login User](#login-user)
* [SignUp User](#sign-in-user)
* [Change Password](#change-password)
* [Forgot Password](#forgot-password)



---

## Login User

<div class="endpoint">
    <div class="method-get">get</div>
    https://127.0.0.1:8080/login   
</div>
 
#### 🔸 Request body (Json): 

```json
{
    "username": String,
    "password": String
}
```

#### 🔹 Response body (Json)
Response will return a set cookie 
```json
{
    "status": String,
    "message": String
}
```


---



## Sign in User

<div class="endpoint">
    <div class="method-post">post</div>
    https://127.0.0.1:8080/signup   
</div>

#### 🔸 Request body (Json): 
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


#### 🔹 Response body (Json)
```
{

}
```


---

## Change Password

<div class="endpoint">
    <div class="method-put">put</div>
    https://127.0.0.1:8080/change-password   
</div>

#### 🔸 Request body (Json): 
  
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

---

## Forgot Password

<div class="endpoint">
    <div class="method-post">post</div>
    https://127.0.0.1:8080/request-change-password   
</div>

#### 🔸 Request body (Json): 
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




