# RustGpt

简单的讯飞烽火gpt，用于终端对话。(暂时只接入了讯飞烽火)

## Quik Start

[讯飞开放平台-以语音交互为核心的人工智能开放平台 (xfyun.cn)](https://www.xfyun.cn/)

去讯飞注册，在控制台中获取 app_id, app_secret, app_key，并替换掉 main 中的配置。

##### 编译

```rust
cargo build --release
```

执行

```shell
./target/release/spark_gpt
```

