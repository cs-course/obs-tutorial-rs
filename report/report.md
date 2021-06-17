# 华中科技大学对象存储系统实验报告
计科校交 1801  
车春池  
U201816030  
2021-6-17  

## 实验目的
* 熟悉对象存储技术，代表性系统及其特性
* 实践对象存储系统，部署实验环境，进行初步测试
* 基于对象存储系统，架设实际应用，示范主要功能

## 实验背景
对象存储是一种计算机数据存储结构，它将数据管理为对象，而不是像文件系统那样管理数据作为文件层次结构的其他存储架构，以及将数据作为扇区和路径中的块来管理数据的块存储。  
每个对象通常包括对象本身，一个变量元数据和一个全局唯一标识符。  
对象存储可以在多个级别实现，包括设备级别，系统级别和接口级别。在每种情况下，对象存储都试图启用其他存储架构所没有处理的功能，比如可以通过应用程序直接编程的接口，一个可以跨越物理硬件多个实例的名称空间，以及数据管理功能，如数据复制和对象级粒度的数据分布。  
本次实验基于 `Minio`，一个 Apache License v2.0 开源协议的对象存储系统服务进行。该系统兼容 AWS S3 云存储服务接口，非常适合于存储大容量非结构化的数据。  
在 `Minio` 的基础上，笔者使用 Rust 语言重写了一个针对亚马逊 S3 云存储服务的性能测试程序，并在 `Minio` 系统上进行了测试，完成了实验二。  

## Rust，新时代的系统编程语言
在传统系统编程领域，比如操作系统，嵌入式开发，C/C++ 占据领导地位，因其可以做到精准控制内存，并且拥有非常小的运行时，相比之下 Java/Python 这些运行时中带有 GC 机制的编程语言大多用于应用编程领域，不适用于系统编程。  
C/C++ 虽然运行时极小，但是由于缺乏规范的内存管理机制，一直以来都被诟病内存安全问题，比如悬浮指针，二次释放问题。  
一直以来编程语言都在**高效运行时**和**安全性**之间做权衡，但以往的大部分编程语言都只是偏向于其中一方，要不是追求高性能，安全性问题交给程序员解决，要不是通过 GC 机制管理内存，但性能由此下降。  
Rust 语言则做到了同时兼顾高性能和内存安全，它通过独特的语法机制，比如说所有权和生命周期机制，既做到了管理内存也做到了较小运行时。它一样可以精准控制内存，并且是在保证安全的情况下。  
Rust 正在系统编程领域，比如操作系统，嵌入式，数据库开发等领域中，发挥它的作用。  
## 实验环境
操作系统：WSL2  
```
Linux DESKTOP-LD8BM4D 5.4.72-microsoft-standard-WSL2 #1 SMP Wed Oct 28 23:40:43 UTC 2020 x86_64 x86_64 x86_64 GNU/Linux
```

Rust 环境：1.52.1  
```bash
$ rustc --version
rustc 1.52.1 (9bc8c42bb 2021-05-09)
```

Minio 版本：  
```bash
$ ./minio --version
minio version RELEASE.2021-06-09T18-51-39Z
```

## 实验内容
* 搭建 `Minio` 环境
* 编写 `s3-bench-rs`
* 使用 `s3-bench-rs` 测试 `Minio` 性能，并进行分析
## 实验过程
### Minio 环境搭建
直接下载二进制文件：  
```bash
$ wget https://dl.min.io/server/minio/release/linux-amd64/minio
```

添加执行权限：  
```bash
$ sudo chmod +x minio
```

配置 ~/.zshrc 文件：  
```
export MINIO_ACCESS_KEY=ccc
export MINIO_SECRET_KEY=WXZFwxzf123
```

运行 Minio:  
```bash
$ ./minio server data
```

运行结果：  
```
Endpoint: http://172.25.42.33:9000  http://127.0.0.1:9000
RootUser: ccc
RootPass: WXZFwxzf123

Browser Access:
   http://172.25.42.33:9000  http://127.0.0.1:9000

Command-line Access: https://docs.min.io/docs/minio-client-quickstart-guide
   $ mc alias set myminio http://172.25.42.33:9000 ccc WXZFwxzf123

Object API (Amazon S3 compatible):
   Go:         https://docs.min.io/docs/golang-client-quickstart-guide
   Java:       https://docs.min.io/docs/java-client-quickstart-guide
   Python:     https://docs.min.io/docs/python-client-quickstart-guide
   JavaScript: https://docs.min.io/docs/javascript-client-quickstart-guide
   .NET:       https://docs.min.io/docs/dotnet-client-quickstart-guide
IAM initialization complete
```

在浏览器打开上面的链接，进入登录画面：  
<img src="./login.png" width="600" height="500" alt="login" align=center />  

输入公钥和密钥，登录进去：  
<img src="./browser.png" width="800" height="500" alt="login" align=center />  

尝试创建一些 `Bucket` 和上传一些文件，Minio 环境搭建基本完成。  

### s3-bench-rs 实现

### 性能测试

## 实验总结
## 参考文献