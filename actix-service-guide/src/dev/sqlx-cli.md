

## How to use sqlx-cli
sqlx-cli created a [📁 migration](./migrations/)
folder that contains up and down sql script files

- up apply changes to the database
- down revert/delete changes to the database 

**Make new migration**
```
sqlx migrate add <title>
```

**Apply changes/migrate to database**
```
sqlx migrate run
```


**Revert changes/migrate to database**
```
sqlx migrate revert
```
