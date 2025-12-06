use anyhow::{Context, Result};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use spinners::{Spinner, Spinners};
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tracing::{error, info};

// OpenAI API configuration
const OPENAI_API_URL: &str = "https://api.openai.com/v1/chat/completions";
const DEFAULT_MODEL: &str = "gpt-4.1";
const MAX_TOKENS: u32 = 4096;

// OpenAI Chat Completions API structures
#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    max_tokens: u32,
    temperature: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
    usage: Option<Usage>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
    #[allow(dead_code)]
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(Debug, Deserialize)]
struct ApiError {
    error: ApiErrorDetail,
}

#[derive(Debug, Deserialize)]
struct ApiErrorDetail {
    message: String,
    #[serde(rename = "type")]
    #[allow(dead_code)]
    error_type: Option<String>,
}

struct Runeo {
    client: reqwest::Client,
    api_key: String,
    model: String,
    conversation_history: Vec<Message>,
}

impl Runeo {
    fn new(api_key: String, model: Option<String>) -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .build()
            .context("HTTP client yaratishda xatolik")?;

        let system_message = Message {
            role: "system".to_string(),
            content: "Sen Runeo - O'zbek tilida gaplashadigan aqlli terminal yordamchisi. \
                     Foydalanuvchilarga dasturlash, texnologiya va boshqa savollarda yordam berasan. \
                     Javoblaringni qisqa, aniq va foydali qilib ber."
                .to_string(),
        };

        Ok(Self {
            client,
            api_key,
            model: model.unwrap_or_else(|| DEFAULT_MODEL.to_string()),
            conversation_history: vec![system_message],
        })
    }

    async fn send_message(&mut self, user_input: &str) -> Result<String> {
        // Add user message to history
        self.conversation_history.push(Message {
            role: "user".to_string(),
            content: user_input.to_string(),
        });

        let request = ChatRequest {
            model: self.model.clone(),
            messages: self.conversation_history.clone(),
            max_tokens: MAX_TOKENS,
            temperature: 0.7,
        };

        let response = self
            .client
            .post(OPENAI_API_URL)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("API so'rov yuborishda xatolik")?;

        let status = response.status();

        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            if let Ok(api_error) = serde_json::from_str::<ApiError>(&error_text) {
                anyhow::bail!("OpenAI API xatosi: {}", api_error.error.message);
            }
            anyhow::bail!("API xatosi ({}): {}", status, error_text);
        }

        let chat_response: ChatResponse = response
            .json()
            .await
            .context("API javobini o'qishda xatolik")?;

        let assistant_message = chat_response
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .unwrap_or_else(|| "Javob olishda xatolik yuz berdi".to_string());

        // Add assistant response to history
        self.conversation_history.push(Message {
            role: "assistant".to_string(),
            content: assistant_message.clone(),
        });

        // Log token usage if available
        if let Some(usage) = chat_response.usage {
            info!(
                "Token ishlatildi: {} (prompt: {}, completion: {})",
                usage.total_tokens, usage.prompt_tokens, usage.completion_tokens
            );
        }

        Ok(assistant_message)
    }

    fn clear_history(&mut self) {
        // Keep only the system message
        self.conversation_history.truncate(1);
        println!("{}", "Suhbat tarixi tozalandi.".yellow());
    }

    fn print_welcome(&self) {
        println!("{}", "═".repeat(60).bright_blue());
        println!(
            "{}",
            "       RUNEO - Aqlli Terminal Yordamchisi".bright_green().bold()
        );
        println!(
            "{}",
            format!("              Model: {}", self.model).bright_cyan()
        );
        println!("{}", "═".repeat(60).bright_blue());
        println!();
        println!("{}", "Buyruqlar:".yellow().bold());
        println!("  {}  - Suhbat tarixini tozalash", "/clear".cyan());
        println!("  {}   - Chiqish", "/exit".cyan());
        println!("  {}   - Yordam", "/help".cyan());
        println!();
    }

    fn print_help(&self) {
        println!();
        println!("{}", "Yordam:".yellow().bold());
        println!("  - Savolingizni yozing va Enter bosing");
        println!("  - {} - Yangi suhbat boshlash", "/clear".cyan());
        println!("  - {} - Dasturdan chiqish", "/exit".cyan());
        println!("  - {} - Modelni ko'rish", "/model".cyan());
        println!();
    }
}

fn get_api_key() -> Result<String> {
    std::env::var("OPENAI_API_KEY")
        .or_else(|_| std::env::var("TOKEN"))
        .context(
            "OPENAI_API_KEY environment variable topilmadi.\n\
             .env faylida OPENAI_API_KEY=your-api-key qo'shing yoki\n\
             export OPENAI_API_KEY=your-api-key buyrug'ini ishlating.",
        )
}

fn get_model() -> Option<String> {
    std::env::var("OPENAI_MODEL").ok()
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .with_target(false)
        .init();

    // Get API key
    let api_key = get_api_key()?;
    let model = get_model();

    // Initialize Runeo
    let mut runeo = Runeo::new(api_key, model)?;

    // Setup graceful shutdown
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        println!("\n\n{}", "Xayr! Runeo bilan yana ko'rishguncha!".bright_green());
        r.store(false, Ordering::SeqCst);
        std::process::exit(0);
    })
    .context("Ctrl+C handler o'rnatishda xatolik")?;

    // Clear screen and print welcome
    print!("\x1B[2J\x1B[1;1H");
    runeo.print_welcome();

    // Main loop
    while running.load(Ordering::SeqCst) {
        print!("{} ", "runeo=>".bright_green().bold());
        io::stdout().flush()?;

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            continue;
        }

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        // Handle commands
        match input.to_lowercase().as_str() {
            "/exit" | "/quit" | "/q" => {
                println!("\n{}", "Xayr! Runeo bilan yana ko'rishguncha!".bright_green());
                break;
            }
            "/clear" | "/c" => {
                runeo.clear_history();
                continue;
            }
            "/help" | "/h" => {
                runeo.print_help();
                continue;
            }
            "/model" => {
                println!("Joriy model: {}", runeo.model.bright_cyan());
                continue;
            }
            _ => {}
        }

        // Show spinner while waiting for response
        let mut sp = Spinner::new(Spinners::Dots12, "O'ylamoqda...".into());

        match runeo.send_message(input).await {
            Ok(response) => {
                sp.stop_with_newline();
                println!();
                println!("{}", response);
                println!();
            }
            Err(e) => {
                sp.stop_with_newline();
                error!("Xatolik: {}", e);
                println!("{} {}", "Xatolik:".red().bold(), e);
                println!();
            }
        }
    }

    Ok(())
}
