//! winload — Network Load Monitor
//! 仿 Linux nload 的终端网络流量监控工具 (Rust 版)
//!
//! 用法:
//!     winload              # 监控所有活跃网卡
//!     winload -t 200       # 设置刷新间隔 200ms
//!     winload -d "Wi-Fi"   # 指定默认设备
//!
//! 快捷键:
//!     ←/→ 或 ↑/↓   切换网卡
//!     q / Esc       退出

mod collector;
mod graph;
mod stats;
mod ui;

use std::io;
use std::time::{Duration, Instant};

use clap::Parser;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};

use collector::{Collector, DeviceInfo};
use stats::StatisticsEngine;

// ─── 单位枚举 ─────────────────────────────────────────────

/// 显示单位
#[derive(Clone, Copy, Debug, PartialEq, Eq, clap::ValueEnum)]
pub enum Unit {
    /// 以 Bit/s 显示速率 (默认)
    Bit,
    /// 以 Byte/s 显示速率
    Byte,
}

/// 状态栏/帮助栏样式
#[derive(Clone, Copy, Debug, PartialEq, Eq, clap::ValueEnum)]
pub enum BarStyle {
    /// 背景色铺满整行 (默认)
    Fill,
    /// 背景色仅在文字上
    Color,
    /// 无背景色，纯文字着色
    Plain,
}

/// 解析人类可读的流量值，如 "100M" → 100*1024*1024 bytes/s
pub fn parse_max_value(s: &str) -> Result<f64, String> {
    let s = s.trim();
    if s.is_empty() {
        return Err("empty value".to_string());
    }
    let (num_str, multiplier) = if let Some(n) = s.strip_suffix('G').or_else(|| s.strip_suffix('g')) {
        (n, 1024.0 * 1024.0 * 1024.0)
    } else if let Some(n) = s.strip_suffix('M').or_else(|| s.strip_suffix('m')) {
        (n, 1024.0 * 1024.0)
    } else if let Some(n) = s.strip_suffix('K').or_else(|| s.strip_suffix('k')) {
        (n, 1024.0)
    } else {
        (s, 1.0)
    };
    let num: f64 = num_str.parse().map_err(|e| format!("invalid number: {e}"))?;
    Ok(num * multiplier)
}

/// 解析十六进制颜色码，支持 0xRRGGBB 或 RRGGBB 格式
pub fn parse_hex_color(s: &str) -> Result<ratatui::style::Color, String> {
    let hex = s.trim().strip_prefix("0x").or_else(|| s.trim().strip_prefix("0X")).unwrap_or(s.trim());
    if hex.len() != 6 {
        return Err(format!("expected 6 hex digits (e.g. 0x3399ff), got: {s}"));
    }
    let r = u8::from_str_radix(&hex[0..2], 16).map_err(|e| format!("bad red: {e}"))?;
    let g = u8::from_str_radix(&hex[2..4], 16).map_err(|e| format!("bad green: {e}"))?;
    let b = u8::from_str_radix(&hex[4..6], 16).map_err(|e| format!("bad blue: {e}"))?;
    Ok(ratatui::style::Color::Rgb(r, g, b))
}

// ─── CLI 参数 ──────────────────────────────────────────────

/// Network Load Monitor — nload-like TUI tool
#[derive(Parser)]
#[command(name = "winload", version, about)]
struct Args {
    /// Refresh interval in milliseconds
    #[arg(short = 't', long = "interval", default_value = "500")]
    interval: u64,

    /// Average window in seconds
    #[arg(short = 'a', long = "average", default_value = "300")]
    average: u64,

    /// Default device name (partial match)
    #[arg(short = 'd', long = "device")]
    device: Option<String>,

    /// Print debug info about network interfaces and exit
    #[arg(long = "debug-info")]
    debug_info: bool,

    /// Enable emoji decorations in TUI and output
    #[arg(short = 'e', long = "emoji")]
    emoji: bool,

    /// Use Unicode block characters for graph (█▓░· instead of #|..)
    #[arg(short = 'U', long = "unicode")]
    unicode: bool,

    /// Display unit: bit (default) or byte
    #[arg(short = 'u', long = "unit", value_enum, default_value = "bit")]
    unit: Unit,

    /// Bar style for header/label/help: fill (default), color, plain
    #[arg(short = 'b', long = "bar-style", value_enum, default_value = "fill")]
    bar_style: BarStyle,

    /// Incoming (download) graph color, hex RGB (e.g. 0x00d7ff). Default: cyan
    #[arg(long = "in-color", value_parser = parse_hex_color)]
    in_color: Option<ratatui::style::Color>,

    /// Outgoing (upload) graph color, hex RGB (e.g. 0xffaf00). Default: gold
    #[arg(long = "out-color", value_parser = parse_hex_color)]
    out_color: Option<ratatui::style::Color>,

    /// Fixed graph Y-axis max (e.g. 100M, 1G, 500K). Default: auto-scale
    #[arg(short = 'm', long = "max", value_parser = parse_max_value)]
    max: Option<f64>,

    /// Hide traffic graphs, show only statistics
    #[arg(short = 'n', long = "no-graph")]
    no_graph: bool,
}

// ─── App 状态 ──────────────────────────────────────────────

/// 单个网卡的视图状态
pub struct DeviceView {
    pub info: DeviceInfo,
    pub engine: StatisticsEngine,
}

/// 应用主状态
pub struct App {
    pub views: Vec<DeviceView>,
    pub current_idx: usize,
    pub emoji: bool,
    pub unicode: bool,
    pub unit: Unit,
    pub bar_style: BarStyle,
    pub in_color: ratatui::style::Color,
    pub out_color: ratatui::style::Color,
    pub fixed_max: Option<f64>,
    pub no_graph: bool,
    collector: Collector,
}

impl App {
    fn new(args: &Args) -> Self {
        let collector = Collector::new();
        let devices = collector.devices();

        let views: Vec<DeviceView> = devices
            .into_iter()
            .map(|info| DeviceView {
                info,
                engine: StatisticsEngine::new(args.interval, args.average),
            })
            .collect();

        // 如果指定了默认设备，定位到对应索引
        let mut current_idx = 0;
        if let Some(ref name) = args.device {
            let lower = name.to_lowercase();
            if let Some(idx) = views
                .iter()
                .position(|v| v.info.name.to_lowercase().contains(&lower))
            {
                current_idx = idx;
            }
        }

        Self {
            views,
            current_idx,
            emoji: args.emoji,
            unicode: args.unicode,
            unit: args.unit,
            bar_style: args.bar_style,
            in_color: args.in_color.unwrap_or(ratatui::style::Color::Rgb(0x00, 0xd7, 0xff)),
            out_color: args.out_color.unwrap_or(ratatui::style::Color::Rgb(0xff, 0xaf, 0x00)),
            fixed_max: args.max,
            no_graph: args.no_graph,
            collector,
        }
    }

    pub fn current_view(&self) -> Option<&DeviceView> {
        self.views.get(self.current_idx)
    }

    fn update(&mut self) {
        let snapshots = self.collector.collect();
        for view in &mut self.views {
            if let Some(snap) = snapshots.get(&view.info.name) {
                view.engine.update(snap.clone());
            }
        }
    }

    fn next_device(&mut self) {
        if !self.views.is_empty() {
            self.current_idx = (self.current_idx + 1) % self.views.len();
        }
    }

    fn prev_device(&mut self) {
        if !self.views.is_empty() {
            self.current_idx = (self.current_idx + self.views.len() - 1) % self.views.len();
        }
    }
}

// ─── 主循环 ────────────────────────────────────────────────

fn run(terminal: &mut ratatui::DefaultTerminal, args: Args) -> io::Result<()> {
    let mut app = App::new(&args);
    let tick_rate = Duration::from_millis(args.interval);
    let mut last_tick = Instant::now();

    // 初始采集
    app.update();

    loop {
        terminal.draw(|frame| ui::draw(frame, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_default();

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                // Windows 下 crossterm 会产生 Press + Release，只处理 Press
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                            return Ok(());
                        }
                        KeyCode::Char('c')
                            if key.modifiers.contains(KeyModifiers::CONTROL) =>
                        {
                            return Ok(());
                        }
                        KeyCode::Right | KeyCode::Down | KeyCode::Tab | KeyCode::Enter => {
                            app.next_device();
                        }
                        KeyCode::Left | KeyCode::Up => {
                            app.prev_device();
                        }
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.update();
            last_tick = Instant::now();
        }
    }
}

// ─── 入口 ──────────────────────────────────────────────────

fn main() -> io::Result<()> {
    let args = Args::parse();

    // 如果传入 --debug-info，打印接口信息后退出
    if args.debug_info {
        let collector = Collector::new();
        if args.emoji {
            println!("\n🔍🌐 Network Interfaces Debug Info 🖧✨");
        }
        collector.print_debug_info();
        if args.emoji {
            println!("🏁 Done! Happy debugging! 🎉🐛");
        }
        return Ok(());
    }
    let mut terminal = ratatui::init();
    let result = run(&mut terminal, args);
    ratatui::restore();
    result
}
