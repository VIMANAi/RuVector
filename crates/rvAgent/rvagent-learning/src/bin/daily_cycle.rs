//! Daily Learning Cycle CLI
//!
//! Run a single learning cycle or start the scheduler.
//!
//! Usage:
//!   cargo run -p rvagent-learning --bin daily_cycle -- [OPTIONS]
//!
//! Options:
//!   --once       Run a single cycle and exit
//!   --status     Show current state and exit
//!   --scan-dir   Directory to scan (default: current directory)
//!   --dry-run    Don't submit to π.ruv.io

use rvagent_learning::{
    DailyLearningLoop, SchedulerConfig,
    discovery::{CodebaseScanner, PatternAnalyzer, DiscoveryLog},
    goap::{GoapPlanner, LearningGoal, LearningWorldState},
    integration::PiRuvIoClient,
};
use std::env;
use std::time::Instant;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("rvagent_learning=info".parse()?)
        )
        .init();

    let args: Vec<String> = env::args().collect();

    let once = args.iter().any(|a| a == "--once");
    let status_only = args.iter().any(|a| a == "--status");
    let dry_run = args.iter().any(|a| a == "--dry-run");

    let scan_dir = args.iter()
        .position(|a| a == "--scan-dir")
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str())
        .unwrap_or(".");

    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║          RuVector Daily Learning Loop (ADR-115)              ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║  GOAP-based discovery with Gemini 2.5 Flash reasoning        ║");
    println!("║  Submits discoveries to π.ruv.io cloud brain                 ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    if status_only {
        show_status().await?;
        return Ok(());
    }

    if once {
        run_single_cycle(scan_dir, dry_run).await?;
    } else {
        run_scheduler(scan_dir).await?;
    }

    Ok(())
}

async fn show_status() -> anyhow::Result<()> {
    println!("📊 System Status");
    println!("────────────────────────────────────────");

    // Check π.ruv.io connection
    let pi_client = PiRuvIoClient::default_client();
    let connected = pi_client.check_connection().await;
    println!("π.ruv.io connection: {}", if connected { "✅ Connected" } else { "❌ Disconnected" });

    // Check Gemini API key
    let gemini_available = env::var("GOOGLE_API_KEY").is_ok() || env::var("GEMINI_API_KEY").is_ok();
    println!("Gemini API key: {}", if gemini_available { "✅ Available" } else { "⚠️  Not set (set GOOGLE_API_KEY)" });

    // Show current state
    let state = LearningWorldState::default();
    println!();
    println!("📈 Default State");
    println!("────────────────────────────────────────");
    println!("Patterns discovered: {}", state.patterns_discovered);
    println!("Pending submission: {}", state.patterns_pending_submission);
    println!("Memory utilization: {:.1}%", state.memory_utilization * 100.0);
    println!("Consolidation due: {}", state.consolidation_due);

    Ok(())
}

async fn run_single_cycle(scan_dir: &str, dry_run: bool) -> anyhow::Result<()> {
    println!("🔄 Running single learning cycle...");
    println!("   Scan directory: {}", scan_dir);
    println!("   Dry run: {}", dry_run);
    println!();

    let start = Instant::now();

    // Phase 1: Scan codebase
    println!("📂 Phase 1: Scanning codebase...");
    let scanner = CodebaseScanner::new(scan_dir);
    let files = scanner.scan().await?;
    println!("   Found {} files to analyze", files.len());

    // Phase 2: Analyze patterns
    println!("🔍 Phase 2: Analyzing patterns...");
    let analyzer = PatternAnalyzer::new();
    let file_contents: Vec<(String, String)> = files
        .into_iter()
        .map(|f| (f.path.to_string_lossy().to_string(), f.content))
        .collect();
    let discoveries = analyzer.analyze_files(&file_contents);
    println!("   Discovered {} patterns", discoveries.len());

    // Show discoveries
    if !discoveries.is_empty() {
        println!();
        println!("📋 Discoveries");
        println!("────────────────────────────────────────");
        for (i, d) in discoveries.iter().take(10).enumerate() {
            println!("{}. [{}] {}", i + 1, format!("{:?}", d.category), d.title);
            println!("   Quality: {:.2} | Files: {:?}", d.quality.composite, d.source_files);
            println!("   Method: {}", d.method_attribution());
        }
        if discoveries.len() > 10 {
            println!("   ... and {} more", discoveries.len() - 10);
        }
    }

    // Phase 3: GOAP Planning
    println!();
    println!("🧠 Phase 3: GOAP Planning...");
    let planner = GoapPlanner::new();
    let mut state = LearningWorldState::default();
    state.patterns_discovered = discoveries.len();

    let goal = LearningGoal::SubmitToCloudBrain { min_quality: 0.7 };
    let plan = planner.plan(&state, &goal)?;

    println!("   Plan: {} actions, cost: {:.1}", plan.actions.len(), plan.estimated_cost);
    for action in &plan.actions {
        println!("   - {} (cost: {:.1})", action.action, action.cost);
    }

    // Phase 4: Submit to π.ruv.io (if not dry run)
    if !dry_run && !discoveries.is_empty() {
        println!();
        println!("☁️  Phase 4: Submitting to π.ruv.io...");
        let pi_client = PiRuvIoClient::default_client();

        if pi_client.check_connection().await {
            let high_quality: Vec<&DiscoveryLog> = discoveries
                .iter()
                .filter(|d| d.quality.composite >= 0.5)
                .collect();

            println!("   {} high-quality discoveries to submit", high_quality.len());

            for discovery in high_quality.iter().take(3) {
                match pi_client.submit(discovery).await {
                    Ok(response) => {
                        if response.success {
                            println!("   ✅ Submitted: {} -> {}",
                                discovery.title,
                                response.memory_id.unwrap_or_default());
                        } else {
                            println!("   ❌ Rejected: {}", response.error.unwrap_or_default());
                        }
                    }
                    Err(e) => {
                        println!("   ⚠️  Failed: {}", e);
                    }
                }
            }
        } else {
            println!("   ⚠️  π.ruv.io not connected, skipping submission");
        }
    } else if dry_run {
        println!();
        println!("☁️  Phase 4: Skipped (dry run mode)");
    }

    let duration = start.elapsed();
    println!();
    println!("════════════════════════════════════════");
    println!("✅ Cycle complete in {:.2}s", duration.as_secs_f64());
    println!("   Patterns found: {}", discoveries.len());
    println!("════════════════════════════════════════");

    Ok(())
}

async fn run_scheduler(scan_dir: &str) -> anyhow::Result<()> {
    println!("🕐 Starting scheduled learning loop...");
    println!("   Press Ctrl+C to stop");
    println!();

    let mut config = SchedulerConfig::default();
    config.scan.root_directory = scan_dir.to_string();

    let mut learning_loop = DailyLearningLoop::new(config).await?;

    // Run first cycle immediately
    println!("Running initial cycle...");
    let result = learning_loop.run_cycle().await?;
    println!("Initial cycle: {} discoveries, {} submitted",
        result.discoveries_found, result.discoveries_submitted);

    // Start scheduled loop
    learning_loop.start().await?;

    Ok(())
}
