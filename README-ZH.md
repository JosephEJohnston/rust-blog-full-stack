<p>
    [<a href="README-ZH.md">中文</a>]
</p>

参考 https://github.com/jcc/blog 设计 UI 和需求

原博客前端为 Vue，后端为 Php + Laravel，地址：https://www.pigjian.com/article?page=1

## 技术栈

本博客前后端都使用 Rust 语言实现，前端为 Yew 并用 Trunk 部署，后端为 Rocket + Diesel + MySql8.0

## 部署
这里部署的操作系统环境为 Windows11，使用 PowerShell。
Linux 下命令差不多。

默认部署者已经安装了 Rust 及相关工具链、MySql8.0
### 前端
#### 1. 配置 WASM Target
```shell
rustup target add wasm32-unknown-unknown
```
#### 2. 安装 trunk
```shell
cargo install trunk
```
#### 3. 启动前端服务
```shell
cd .\frontend\
trunk serve
```
#### 4. 查看页面
http://0.0.0.0:80

### 后端
#### 1. diesel 依赖安装

Linux 系统参照下面指导安装相关依赖

- https://github.com/diesel-rs/diesel/blob/master/guide_drafts/backend_installation.md


Windows：

- 配置环境变量：MYSQLCLIENT_LIB_DIR=C:\Program Files\MySQL\MySQL Server 8.0\lib

  - ![path_variable](/frontend/static/resource/img/path_variable.png)

- PowerShell 下输入：setx MYSQLCLIENT_LIB_DIR "C:\Program Files\MySQL\MySQL Server 8.0\lib"

  - 有问题可参照：https://github.com/diesel-rs/diesel/issues/1286

#### 2. 数据库配置

在 backend 文件夹下添加 .env 文件，补充 mysql 用户配置，参考 .env_bak

#### 3. 数据库表添加
添加 rust_blog 数据库

- ```mysql
  CREATE DATABASE rust_blog
导入 Dump 文件夹
- Mysql Workbench -> Server -> Data Import

  - ![data_import](/frontend/static/resource/img/data_import.png)
  - 然后进 Import Progress 进行导入

#### 4. 启动后端服务
```shell
cd .\backend\
cargo run
```

## 注意事项

默认前后端部署在同一服务器下， 前端配置了本地后端转发， 在 Trunk.toml 可改
```shell
[[proxy]]
backend = "http://0.0.0.0:8000/api"
```