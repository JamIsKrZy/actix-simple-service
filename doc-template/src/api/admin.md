
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
            <a href="#edit-product">
                List
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
    https://127.0.0.1:8080/admin/user/new   
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
    "location"?: String,
    "role:" "admin" | "employee" |"regular"
}
```

#### 🔹 Response body (Json)

```json
{

    // successful service
    "success"?: {
        "query": 
    },
    
    // fail to comply service
    "fail"?: {
        "message": String
    },

    // Any internal service problem
    "error"?: {
        "message": String
    }

}   
```
---

## Edit user

<div class="endpoint">
    <div class="method-patch">patch</div>
    https://127.0.0.1:8080/admin/user/edit/{user-id}
</div>

#### 🔸 Path: 
| Name        | Type   | 
|-------------|--------|
|`user_id` | string | 

#### 🔸 Request body (Json): 
```json
{
    "email"?: String,
    "first_name"?: String,
    "last_name"?: String,
    "phone_no"?: String,
    "location"?: String,
    "role:" "admin" | "employee" |"regular"
}
```

#### 🔹 Response body (Json)
```json
{

    // successful service
    "success"?: {
        "query": 
    },
    
    // fail to comply service
    "fail"?: {
        "message": String
    },

    // Any internal service problem
    "error"?: {
        "message": String
    }

}   
```


---


## Create Product
<div class="endpoint">
    <div class="method-post">post</div>
    https://127.0.0.1:8080/admin/product/new   
</div>

#### 🔸 Request body (Json): 
```json
{
    "name": String,
    "description"?: String,
    "price": number | String,
    "stocks": number,
    "status"?: "unavailable" | "available" 
}

```

#### 🔹 Response body (Json)
```json
{
}   
```

---

## Edit Product

<div class="endpoint">
    <div class="method-patch">patch</div>
    https://127.0.0.1:8080/admin/product/edit/{product_id}
</div>

#### 🔸 Path: 
| Name        | Type   | 
|-------------|--------|
|`product_id` | number | 

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
    
}   
```

---


## Product List

<div class="endpoint">
    <div class="method-get">get</div>
    https://127.0.0.1:8080/admin/product/list
</div>

#### 🔸 Query: 
| Name        | Type   | Required | Description | 
|-------------|--------|----------|-------------|
|`limit`  | number | Yes | Maximum nummber of list to return | 
|`offset` | number | Yes | Index to start in the list |


#### 🔹 Response body (Json)
```json
{
    
}   
```

---



## Create Bundle

<div class="endpoint">
    <div class="method-post">post</div>
    https://127.0.0.1:8080/admin/bundle/new   
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
    https://127.0.0.1:8080/admin/bundle/edit/{bundle_id}
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
    https://127.0.0.1:8080/service/new   
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

