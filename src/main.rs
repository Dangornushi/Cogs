use clap::Parser;
use dotenv::dotenv;
use reqwest::blocking::Client;
use serde_json::json;
use std::env;
use std::fs;
use std::process;
use indicatif::{ProgressBar, ProgressStyle};
use std::thread;
use std::time::Duration;

#[derive(Parser)]
#[command(
    author,
    version,
    about = r#"


    ██████╗ ██████╗  ██████╗ ███████╗
   ██╔════╝██╔═══██╗██╔════╝ ██╔════╝
   ██║     ██║   ██║██║  ███╗███████╗
   ██║     ██║   ██║██║   ██║╚════██║ 
   ╚██████╗╚██████╔╝╚██████╔╝███████║
    ╚═════╝ ╚═════╝  ╚═════╝ ╚══════╝"#,
    long_about = None
)]
struct Args {
    /// 翻訳したいファイルのパス
    file_path: String,
    /// 翻訳先言語（例: 日本語, 英語, 中国語 など）
    #[arg(long, default_value = "日本語")]
    lang: String,
    /// 翻訳結果の出力先ファイル名
    #[arg(long)]
    output: Option<String>,
}

fn main() {
    dotenv().ok();

    let args = Args::parse();

    let api_key = match env::var("API_KEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("API_KEYが.envに設定されていません。");
            process::exit(1);
        }
    };

    let file_content = match fs::read_to_string(&args.file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("ファイルの読み込みに失敗しました: {}", e);
            process::exit(1);
        }
    };

    // スピナー表示
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::with_template("{spinner} {msg}")
            .unwrap(),
    );
    pb.set_message("AIの接続待ち...");

    // スピナーを一定時間回す（接続待ち）
    for _ in 0..30 {
        pb.tick();
        thread::sleep(Duration::from_millis(10));
    }

    let prompt = format!(
        "次のテキストを{}に翻訳してください。応答は翻訳後の文章だけで大丈夫です。:\n\n{}",
        args.lang,
        file_content
    );

    let client = Client::new();
    let url = "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash-thinking-exp:generateContent?key=".to_owned() + &api_key;

    let req_body = json!({
        "contents": [
            {
                "parts": [
                    { "text": prompt }
                ]
            }
        ]
    });

    pb.set_message("AIで翻訳生成中...");

    // APIリクエスト中もスピナーを回す
    let handle = std::thread::spawn({
        let pb = pb.clone();
        move || {
            for _ in 0..60 {
                pb.tick();
                thread::sleep(Duration::from_millis(30));
            }
        }
    });

    let resp = client
        .post(&url)
        .json(&req_body)
        .send();

    // スレッド終了待ち
    let _ = handle.join();

    pb.set_message("完了！");

    let resp = match resp {
        Ok(r) => r,
        Err(e) => {
            pb.finish_and_clear();
            eprintln!("APIリクエストに失敗しました: {}", e);
            process::exit(1);
        }
    };

    if !resp.status().is_success() {
        pb.finish_and_clear();
        eprintln!("APIエラー: {}", resp.status());
        if let Ok(text) = resp.text() {
            eprintln!("レスポンス内容: {}", text);
        }
        process::exit(1);
    }

    let resp_json: serde_json::Value = match resp.json() {
        Ok(j) => j,
        Err(e) => {
            pb.finish_and_clear();
            eprintln!("APIレスポンスのパースに失敗しました: {}", e);
            process::exit(1);
        }
    };

    // Geminiのレスポンスから翻訳結果を抽出
    let translated = resp_json["candidates"][0]["content"]["parts"][0]["text"].as_str().unwrap_or("翻訳結果の取得に失敗しました。");

    pb.finish_with_message("完了！");

    // --output指定時のみファイル保存
    if let Some(ref output_path) = args.output {
        if let Err(e) = fs::write(output_path, translated) {
            eprintln!("翻訳結果の保存に失敗しました: {}", e);
        } else {
            println!("翻訳結果を{}に保存しました。", output_path);
        }
    }

    println!("{}", translated);
}
