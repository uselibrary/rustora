# Rustora

将图片上传到 Telegram 频道，由 Rust 驱动。适用于 Typora 图床或直接命令行上传。

## 功能

- 通过命令行将图片文件上传到 Telegram 频道
- 支持添加图片说明
- 可自定义配置文件

## 配置文件

在 `config.json` 文件中配置 Telegram 信息：

```json
{
  "telegram": {
    "token": "你的机器人令牌",
    "chatId": 你的频道ID
  }
}
```

## 使用方法

### 基本用法

```bash
# 上传图片到 Telegram 频道
rustora --file /path/to/your/image.jpg

# 上传图片并附带说明文字
rustora --file /path/to/your/image.jpg --caption "这是一张美丽的风景照"
```

### 使用自定义配置文件

```bash
rustora --file /path/to/your/image.jpg --config /path/to/custom/config.json
```

## 构建可执行文件

```bash
cargo build --release
```

构建完成后，可执行文件会在 `target/release/rustora` 路径下。

## 全局安装

可以将构建好的可执行文件复制到系统的 PATH 路径中：

```bash
sudo cp target/release/rustora /usr/local/bin/
```

这样就可以在任何位置使用以下命令：

```bash
rustora --file /path/to/your/image.jpg
```
