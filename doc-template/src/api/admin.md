
# Admin Endpoints
Endpoints that are only be accessible for users
who have a admin role previlages

### Table of contents


<table class="html-table">
    <thead>
        <tr>
            <th>Users</th>
            <th>Products</th>
            <th>Bundles</th>
            <th>Service</th>
        </tr>
    </thead>
    <tbody>
    <tr>
        <td>   
            <!-- Column for User  -->
            <a href="#create-user-employeeadmin-previlage">
                Create
            </a>
            <a href="#create-user-employeeadmin-previlage">
                Promote
            </a>
            <a href="#promote-or-demote-user-role">
                Edit Role
            </a>
            <a href="#">
                User's cart
            </a>
        </td>
            <!-- Column for Product  -->
        <td>
            <a href="#create-product">
                Create
            </a>
            <a href="#edit-product">
                Edit
            </a>
        </td>
            <!-- Column for Bundles  -->
        <td>
            <a href="#create-bundle">
                Create
            </a>
            <a href="#edit-bundle">
                Edit
            </a>
        </td>
            <!-- Column for Services  -->
        <td>
            <a href="#create-services">
                Create
            </a>
            <a href="#edit-services">
                Edit
            </a>
        </td>
    </tr>
    </tbody>
</table>


---


## Create User (Employee/Admin Previlage)
<div class="endpoint">
    <div class="method-post">post</div>
    https://127.0.0.1:8080/admin/create-user   
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

```json
{
    "status": "success" | "fail" | "error",
    "message": String
}   
```
---

## Promote or Demote user role

<div class="endpoint">
    <div class="method-patch">patch</div>
    https://127.0.0.1:8080/admin/edit-user-role  
</div>

#### 🔸 Request body (Json): 
```json
{
    "user_id": String,
    "role" : "regular" | "employee" | "admin" 
}
```

#### 🔹 Response body (Json)

```json
{
    "status": "success" | "fail" | "error",
    "message": String
}   
```


---


## Create Product
<div class="endpoint">
    <div class="method-post">post</div>
    https://127.0.0.1:8080/product/create   
</div>

#### 🔸 Request body (Json): 
```json
{
    "product_name": String,
    "description"?: String,
    "price": number,
    "stocks": number,
    "status"?: "unavailable" | "available" 
}

```

#### 🔹 Response body (Json)
```json
{
    "status": "success" | "fail" | "error",
    "message": String
}   
```

---

## Edit Product

<div class="endpoint">
    <div class="method-patch">patch</div>
    https://127.0.0.1:8080/product/{product_id}/edit
</div>

#### 🔸 Path: 
| Name        | Type   | 
|-------------|--------|
|`product_id` | string | 

#### 🔸 Request body (Json): 
```json
{
    "product_name"?: String,
    "description"?: String,
    "price"?: number,
    "stocks"?: number,
    "status"?: "unavailable" | "available" 
}
```

#### 🔹 Response body (Json)
```json
{
    "status": "success" | "fail" | "error",
    "message": String
}   
```

---


## Create Bundle

<div class="endpoint">
    <div class="method-post">post</div>
    https://127.0.0.1:8080/bundle/create   
</div>

#### 🔸 Request body (Json): 
```json
{
    "name": String,
    "price": number,
    "status"?: "unavailable" | "available",
    "products": [
        {
            "product_id": numbers,
            "quantity": number
        }
    ]
}
```

#### 🔹 Response body (Json)
```json
{
    "status": "success" | "fail" | "error",
    "message": String
}   
```

---

## Edit Bundle

<div class="endpoint">
    <div class="method-patch">patch</div>
    https://127.0.0.1:8080/bundle/{bundle_id}/edit
</div>

#### 🔸 Path: 
| Name        | Type   | 
|-------------|--------|
|`bundle_id` | string | 

#### 🔸 Request body (Json): 
```json
{
    "name"?: String,
    "description"?: String,
    "price": number,
    "stocks": BigInt,
    "remove_product"?: [ 
        {
            "product_id": number
        }
    ],
    "add_product"?: [
        {
            "product_id": numbers,
            "quantity": number
        }
    ],
    "edit_product"?: [
        {
            "product_id": numbers,
            "quantity": number
        }
    ]
}
```
Info: "edit_product" does only change the quantity number, not the product itself


#### 🔹 Response body (Json)
```json
{
    "status": "success" | "fail" | "error",
    "message": String
}   
```


---

## Create Services
<div class="endpoint">
    <div class="method-post">post</div>
    https://127.0.0.1:8080/service/create   
</div>

#### 🔸 Request body (Json): 
```json
{
    (Ongoing...)
}
```


#### 🔹 Response body (Json)
```json
{
    "status": "success" | "fail" | "error",
    "message": String
}   
```



---

## Edit Services
<div class="endpoint">
    <div class="method-patch">patch</div>
    https://127.0.0.1:8080/service/edit   
</div>

#### 🔸 Request body (Json): 
```json
{
    (Ongoing...)
}
```


#### 🔹 Response body (Json)
```json
{
    "status": "success" | "fail" | "error",
    "message": String
}   
```



---

## Set discount Product
<div class="endpoint">
    <div class="method-put">put</div>
    https://127.0.0.1:8080/product/{product_id}/set-discount   
</div>

#### 🔸 Path: 
| Name        | Type   | 
|-------------|--------|
|`product_id` | string | 

#### 🔸 Request body (Json): 
```json
{
    "discount_perc": number,
    "expiry_date": String(ISO8601 Date Format),
}
```

#### 🔹 Response body (Json)
```json
{
    "status": "success" | "fail" | "error",
    "message": String
}   
```




---

## Set discount Bundle
<div class="endpoint">
    <div class="method-put">put</div>
    https://127.0.0.1:8080/bundle/{bundle_id}/set-discount   
</div>

#### 🔸 Path: 
| Name        | Type   | 
|-------------|--------|
|`bundle_id` | string | 

#### 🔸 Request body (Json): 
```json
{
    "discount_perc": number,
    "expiry_date": String(ISO8601 Date Format),
}
```

#### 🔹 Response body (Json)
```json
{
    "status": "success" | "fail" | "error",
    "message": String
}   
```


---

## Set discount Service
<div class="endpoint">
    <div class="method-put">put</div>
    https://127.0.0.1:8080/service/{service_id}/set-discount   
</div>

#### 🔸 Path: 
| Name        | Type   | 
|-------------|--------|
|`service_id` | string | 

#### 🔸 Request body (Json): 
```json
{
    "discount_perc": number,
    "expiry_date": String(ISO8601 Date Format),
}
```

#### 🔹 Response body (Json)
```json
{
    "status": "success" | "fail" | "error",
    "message": String
}   
```


---

## Remove discount Product
<div class="endpoint">
    <div class="method-patch">patch</div>
    https://127.0.0.1:8080/product/{product_id}/rmv-discount   
</div>

#### 🔸 Path: 
| Name        | Type   | 
|-------------|--------|
|`product_id` | string | 


#### 🔹 Response body (Json)
```json
{
    "status": "success" | "fail" | "error",
    "message": String
}   
```


---

## Remove discount bundles
<div class="endpoint">
    <div class="method-patch">patch</div>
    https://127.0.0.1:8080/bundle/{bundle_id}/rmv-discount   
</div>

#### 🔸 Path: 
| Name        | Type   | 
|-------------|--------|
|`bundle_id` | string |


#### 🔹 Response body (Json)
```json
{
    "status": "success" | "fail" | "error",
    "message": String
}   
```

---

## Remove discount Service
<div class="endpoint">
    <div class="method-patch">patch</div>
    https://127.0.0.1:8080/service/{service_id}/rmv-discount   
</div>

#### 🔸 Path Parameters: 
| Name        | Type   |
|-------------|--------|
|`service_id` | string |


#### 🔹 Response body (Json)
```json
{
    "status": "success" | "fail" | "error",
    "message": String
}   
```


---

