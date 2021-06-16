# s3-bench-rs
AWS S3 对象存储服务性能测试程序，可用于华中科技大学物联网存储系统实验。  

## 实验方法
1. 配置运行对象存储系统，比如 [minio](https://min.io/)，必须兼容 AWS S3 协议。
2. 配置好 Rust 环境，[安装教程](https://kaisery.github.io/trpl-zh-cn/ch01-01-installation.html)
3. 进入该项目目录下，修改 `benches` 目录下的源代码文件中的 `ENDPOINT`，`KEY` 和 `SECRET`，换成符合自己系统实际情况的值
4. 修改 `BUCKET` 和 `OBJECT`，换成自己系统中存在的 bucket 和 object。
5. 命令行运行 `cargo bench`，项目会自动运行 `GET` 和 `PUT` 测试，并输出运行时间和分析**尾延迟**

## 输出结果分析
下面是一个输出结果例子：  
```
Async GetObject         time:   [9.9090 ms 10.087 ms 10.377 ms]
                        change: [-42.063% -40.860% -39.294%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) low severe
  3 (3.00%) high severe
```

这是对对象存储系统 `Get` 请求的测试，一共重复测试了 100 次，下面对结果进行解释：  
+ time: 每轮测试运行的时间，左边的是最小值，右边的是最大值，中间的是所有运行时间的**最佳估计**
+ change: 相比上次测试的变化值（在该实验中可忽略）
+ outliers: 离群值，表示该结果的值和其他结果相差较大
    - low severe: 严重的低离群值
    - low mild: 轻微的低离群值
    - high severe: 严重的高离群值
    - high mild: 轻微的高离群值
    - 尾延迟指的是后两种

## 补充
该项目基于 [rusty-s3](https://github.com/paolobarbolini/rusty-s3) 和 [tokio](https://github.com/tokio-rs/tokio) 异步运行时编写，有兴趣的同学可以尝试让本项目支持 [async-std](https://github.com/async-rs/async-std) 异步运行时，欢迎 PR。  
本项目编写的过程中引发了一个 Pull Request: <https://github.com/paolobarbolini/rusty-s3/pull/14>，开源社区的魅力就在于此!  
作者邮箱: <linuxgnulover@gmail.com>  

