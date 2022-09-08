<p>
    [<a href="README-ZH.md">中文</a>]
</p>

Reference https://github.com/jcc/blog to design UI and functions.

Original blog frontend is  Vue，backend is Php + Laravel，address：https://www.pigjian.com/article?page=1

## Technology Stack

The frontend and backend of this blog is implemented with Rust, frontend is Yew and deployed by Trunk,
backend is Rocket + Diesel + MySql8.0.

## Deployment
The operating system environment deployed here is windows11, using PowerShell.

The commands under Linux are similar.

It is assumed that the deployer has installed rust and related tool chains, and MySQL 8.0.
### Frontend
#### 1. Config WASM Target
```shell
rustup target add wasm32-unknown-unknown
```
#### 2. Install Trunk
```shell
cargo install trunk
```
#### 3. Start Frontend Server
```shell
cd .\frontend\
trunk serve
```
#### 4. View Pages
http://0.0.0.0:80

### Backend
#### 1. Diesel Deps Install

For Linux system, please refer to the following instructions to install relevant dependencies:

- https://github.com/diesel-rs/diesel/blob/master/guide_drafts/backend_installation.md


Windows：

- Configuring environment variables：MYSQLCLIENT_LIB_DIR=C:\Program Files\MySQL\MySQL Server 8.0\lib

  - ![path_variable](/frontend/static/resource/img/path_variable.png)

- On PowerShell, input：setx MYSQLCLIENT_LIB_DIR "C:\Program Files\MySQL\MySQL Server 8.0\lib"

  - More questions refer to：https://github.com/diesel-rs/diesel/issues/1286

#### 2. Database Configuration

Add .env file under the backend folder to supplement the MySQL user configuration, see .env_bak

#### 3. Adding Database Tables
Add rust_blog database

- ```mysql
  CREATE DATABASE rust_blog
Import Dump folder
- Mysql Workbench -> Server -> Data Import

  - ![data_import](/frontend/static/resource/img/data_import.png)
  - Then move to Import Progress tab and do import.

#### 4. Start backend Server
```shell
cd .\backend\
cargo run
```

## Notices

By default, the front-end and back-end are deployed on the same server. 
The front-end is configured with local back-end forwarding, which can be changed in Trunk.toml.
```shell
[[proxy]]
backend = "http://0.0.0.0:8000/api"
```