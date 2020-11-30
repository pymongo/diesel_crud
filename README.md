# diesel CRUD(Create, Read, Update, Delete) example with datetime

How to run:

```
DATABASE_URL=file:db.sqlite diesel setup
DATABASE_URL=file:db.sqlite diesel migration generate create_users
# (edit migration/xxx/up.sql and migration/xxx/down.sql)
DATABASE_URL=file:db.sqlite diesel migration run
cargo run
```
