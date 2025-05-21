use anyhow::{Context, Result};
use clap::Parser;
use reqwest::multipart::{Form, Part};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct Config {
    telegram: TelegramConfig,
}

#[derive(Debug, Deserialize)]
struct TelegramConfig {
    token: String,
    #[serde(rename = "chatId")]
    chat_id: i64,
}

#[derive(Parser, Debug)]
#[command(author, version, about = "Upload images to Telegram channel")]
struct Args {
    /// Path to the image file
    #[arg(short, long)]
    file: PathBuf,

    /// Optional caption for the image
    #[arg(short, long)]
    caption: Option<String>,

    /// Path to config file (default: config.json)
    #[arg(short = 'C', long, default_value = "config.json")]
    config: PathBuf,
}

#[derive(Serialize, Deserialize)]
struct TelegramResponse {
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // 加载配置文件
    let config_data = std::fs::read_to_string(&args.config)
        .with_context(|| format!("无法读取配置文件: {}", args.config.display()))?;
    let config: Config = serde_json::from_str(&config_data)
        .with_context(|| "解析配置文件失败")?;

    // 检查文件是否存在
    if !args.file.exists() {
        anyhow::bail!("文件不存在: {}", args.file.display());
    }

    // 构建 API URL
    let api_url = format!(
        "https://api.telegram.org/bot{}/sendDocument",
        config.telegram.token
    );

    println!("正在上传文件: {}", args.file.display());

    // 读取文件
    let mut file = File::open(&args.file)
        .with_context(|| format!("无法打开文件: {}", args.file.display()))?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .with_context(|| format!("无法读取文件内容: {}", args.file.display()))?;

    // 文件名
    let filename = args.file.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("image.jpg");

    // 构建 multipart 表单
    let file_part = Part::bytes(buffer)
        .file_name(filename.to_string());

    let mut form = Form::new()
        .part("document", file_part)
        .text("chat_id", config.telegram.chat_id.to_string());

    // 添加可选的图片说明
    if let Some(caption) = args.caption {
        form = form.text("caption", caption);
    }

    // 发送请求
    let client = reqwest::Client::new();
    let response = client.post(api_url)
        .multipart(form)
        .send()
        .await
        .with_context(|| "发送请求失败")?;

    // 解析响应
    let response_text = response.text().await?;
    
    match serde_json::from_str::<TelegramResponse>(&response_text) {
        Ok(telegram_response) => {
            if telegram_response.ok {
                println!("文件上传成功!");
            } else {
                println!("上传失败: {}", telegram_response.description.unwrap_or_else(|| "未知错误".to_string()));
            }
        },
        Err(e) => {
            println!("解析响应失败: {}", e);
            println!("原始响应: {}", response_text);
        }
    }

    Ok(())
}
