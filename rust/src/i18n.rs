//! i18n — Internationalization support
//! Supported languages: en-us, zh-cn, zh-tw

use std::sync::atomic::{AtomicU8, Ordering};

/// Display language
#[derive(Clone, Copy, Debug, PartialEq, Eq, clap::ValueEnum)]
pub enum Lang {
    /// English (United States)
    #[value(name = "en-us")]
    EnUs,
    /// 简体中文
    #[value(name = "zh-cn")]
    ZhCn,
    /// 繁體中文
    #[value(name = "zh-tw")]
    ZhTw,
}

static LANG: AtomicU8 = AtomicU8::new(0);

pub fn set_lang(lang: Lang) {
    LANG.store(lang as u8, Ordering::Relaxed);
}

pub fn get_lang() -> Lang {
    match LANG.load(Ordering::Relaxed) {
        1 => Lang::ZhCn,
        2 => Lang::ZhTw,
        _ => Lang::EnUs,
    }
}

/// Look up a translated string by key. Falls back to en-us.
pub fn t(key: &str) -> &'static str {
    match get_lang() {
        Lang::EnUs => t_en_us(key),
        Lang::ZhCn => t_zh_cn(key),
        Lang::ZhTw => t_zh_tw(key),
    }
}

// ─── English (en-us) ───────────────────────────────────────

fn t_en_us(key: &str) -> &'static str {
    match key {
        // -- CLI help --
        "description" => "Network Load Monitor \u{2014} nload-like TUI tool for Windows/Linux/macOS",
        "help_interval" => "Refresh interval in milliseconds",
        "help_average" => "Average window in seconds",
        "help_device" => "Default device name (partial match)",
        "help_debug_info" => "Print debug info about network interfaces and exit",
        "help_emoji" => "Enable emoji decorations in TUI and output",
        "help_unicode" => "Use Unicode block characters for graph (\u{2588}\u{2593}\u{2591}\u{00b7} instead of #|..)",
        "help_unit" => "Display unit: bit (default) or byte",
        "help_bar_style" => "Bar style for header/label/help: fill (default), color, plain",
        "help_in_color" => "Incoming (download) graph color, hex RGB (e.g. 0x00d7ff). Default: cyan",
        "help_out_color" => "Outgoing (upload) graph color, hex RGB (e.g. 0xffaf00). Default: gold",
        "help_max" => "Fixed graph Y-axis max (e.g. 100M, 1G, 500K). Default: auto-scale",
        "help_no_graph" => "Hide traffic graphs, show only statistics",
        "help_hide_separator" => "Hide separator line (the row of equals signs between header and panels)",
        "help_no_color" => "Disable all TUI colors (monochrome mode). Press 'c' to toggle at runtime",
        "help_npcap" => "[Windows only] Use Npcap to capture loopback traffic (recommended)\nRequires Npcap installed: https://npcap.com/#download",
        "help_lang" => "Display language: en-us (default), zh-cn, zh-tw",
        // -- TUI --
        "device" => "Device",
        "device_emoji" => "\u{1f5a7} Device",
        "incoming" => "Incoming",
        "incoming_emoji" => "\u{2b07}\u{fe0f}\u{1f4e5} Incoming",
        "outgoing" => "Outgoing",
        "outgoing_emoji" => "\u{2b06}\u{fe0f}\u{1f4e4} Outgoing",
        "stat_curr" => "Curr",
        "stat_avg" => "Avg",
        "stat_min" => "Min",
        "stat_max" => "Max",
        "stat_ttl" => "Ttl",
        "stat_curr_emoji" => "\u{26a1} Curr",
        "stat_avg_emoji" => "\u{1f4ca}  Avg",
        "stat_min_emoji" => "\u{1f4cf}  Min",
        "stat_max_emoji" => "\u{1f680}  Max",
        "stat_ttl_emoji" => "\u{1f4e6}  Ttl",
        "help_bar" => " \u{2190}/\u{2192} Switch Device | q Quit",
        "help_bar_emoji" => " \u{2b05}\u{fe0f}/\u{27a1}\u{fe0f} Switch Device | \u{1f6aa} q Quit",
        "help_bar_win" => " \u{2190}/\u{2192} Switch Device | q Quit | Loopback: --npcap",
        "help_bar_win_emoji" => " \u{2b05}\u{fe0f}/\u{27a1}\u{fe0f} Switch Device | \u{1f6aa} q Quit | \u{1f4a1} Loopback: --npcap",
        "terminal_too_small" => "Terminal too small!",
        "terminal_too_small_emoji" => "\u{1f62d} Terminal too small! \u{1f4cc}",
        "loopback_warning" => " \u{26a0} Loopback: use --npcap (npcap.com)",
        _ => "",
    }
}

// ─── Simplified Chinese (zh-cn) ────────────────────────────

fn t_zh_cn(key: &str) -> &'static str {
    match key {
        // -- CLI help --
        "description" => "网络负载监控工具 \u{2014} 仿 Linux nload 的终端网络流量监控工具",
        "help_interval" => "刷新间隔（毫秒）",
        "help_average" => "平均值计算窗口（秒）",
        "help_device" => "默认网卡名称（支持部分匹配）",
        "help_debug_info" => "打印网卡调试信息并退出",
        "help_emoji" => "在 TUI 和输出中启用 emoji 装饰",
        "help_unicode" => "使用 Unicode 块字符绘制图形（\u{2588}\u{2593}\u{2591}\u{00b7} 代替 #|..）",
        "help_unit" => "显示单位：bit（默认）或 byte",
        "help_bar_style" => "状态栏/帮助栏样式：fill（默认），color，plain",
        "help_in_color" => "入站（下载）图形颜色，十六进制 RGB（如 0x00d7ff）。默认：青色",
        "help_out_color" => "出站（上传）图形颜色，十六进制 RGB（如 0xffaf00）。默认：金色",
        "help_max" => "固定图形 Y 轴最大值（如 100M、1G、500K）。默认：自动缩放",
        "help_no_graph" => "隐藏流量图形，仅显示统计信息",
        "help_hide_separator" => "隐藏分隔线（标题和面板之间的等号行）",
        "help_no_color" => "禁用所有 TUI 颜色（单色模式）。运行时按 'c' 切换",
        "help_npcap" => "[仅 Windows] 使用 Npcap 捕获回环流量（推荐）\n需要安装 Npcap：https://npcap.com/#download",
        "help_lang" => "显示语言：en-us（默认），zh-cn，zh-tw",
        // -- TUI --
        "device" => "设备",
        "device_emoji" => "\u{1f5a7} 设备",
        "incoming" => "入站",
        "incoming_emoji" => "\u{2b07}\u{fe0f}\u{1f4e5} 入站",
        "outgoing" => "出站",
        "outgoing_emoji" => "\u{2b06}\u{fe0f}\u{1f4e4} 出站",
        "stat_curr" => "当前",
        "stat_avg" => "平均",
        "stat_min" => "最小",
        "stat_max" => "最大",
        "stat_ttl" => "总计",
        "stat_curr_emoji" => "\u{26a1} 当前",
        "stat_avg_emoji" => "\u{1f4ca} 平均",
        "stat_min_emoji" => "\u{1f4cf} 最小",
        "stat_max_emoji" => "\u{1f680} 最大",
        "stat_ttl_emoji" => "\u{1f4e6} 总计",
        "help_bar" => " \u{2190}/\u{2192} 切换设备 | q 退出",
        "help_bar_emoji" => " \u{2b05}\u{fe0f}/\u{27a1}\u{fe0f} 切换设备 | \u{1f6aa} q 退出",
        "help_bar_win" => " \u{2190}/\u{2192} 切换设备 | q 退出 | 回环: --npcap",
        "help_bar_win_emoji" => " \u{2b05}\u{fe0f}/\u{27a1}\u{fe0f} 切换设备 | \u{1f6aa} q 退出 | \u{1f4a1} 回环: --npcap",
        "terminal_too_small" => "终端窗口太小！",
        "terminal_too_small_emoji" => "\u{1f62d} 终端窗口太小！\u{1f4cc}",
        "loopback_warning" => " \u{26a0} 回环设备：请使用 --npcap (npcap.com)",
        _ => t_en_us(key),
    }
}

// ─── Traditional Chinese (zh-tw) ───────────────────────────

fn t_zh_tw(key: &str) -> &'static str {
    match key {
        // -- CLI help --
        "description" => "網路負載監控工具 \u{2014} 仿 Linux nload 的終端網路流量監控工具",
        "help_interval" => "重新整理間隔（毫秒）",
        "help_average" => "平均值計算視窗（秒）",
        "help_device" => "預設網路卡名稱（支援部分匹配）",
        "help_debug_info" => "列印網路卡除錯資訊並退出",
        "help_emoji" => "在 TUI 和輸出中啟用 emoji 裝飾",
        "help_unicode" => "使用 Unicode 區塊字元繪製圖形（\u{2588}\u{2593}\u{2591}\u{00b7} 取代 #|..）",
        "help_unit" => "顯示單位：bit（預設）或 byte",
        "help_bar_style" => "狀態列/說明列樣式：fill（預設），color，plain",
        "help_in_color" => "入站（下載）圖形顏色，十六進位 RGB（如 0x00d7ff）。預設：青色",
        "help_out_color" => "出站（上傳）圖形顏色，十六進位 RGB（如 0xffaf00）。預設：金色",
        "help_max" => "固定圖形 Y 軸最大值（如 100M、1G、500K）。預設：自動縮放",
        "help_no_graph" => "隱藏流量圖形，僅顯示統計資訊",
        "help_hide_separator" => "隱藏分隔線（標題和面板之間的等號行）",
        "help_no_color" => "停用所有 TUI 顏色（單色模式）。執行時按 'c' 切換",
        "help_npcap" => "[僅 Windows] 使用 Npcap 擷取回環流量（建議）\n需要安裝 Npcap：https://npcap.com/#download",
        "help_lang" => "顯示語言：en-us（預設），zh-cn，zh-tw",
        // -- TUI --
        "device" => "裝置",
        "device_emoji" => "\u{1f5a7} 裝置",
        "incoming" => "入站",
        "incoming_emoji" => "\u{2b07}\u{fe0f}\u{1f4e5} 入站",
        "outgoing" => "出站",
        "outgoing_emoji" => "\u{2b06}\u{fe0f}\u{1f4e4} 出站",
        "stat_curr" => "目前",
        "stat_avg" => "平均",
        "stat_min" => "最小",
        "stat_max" => "最大",
        "stat_ttl" => "總計",
        "stat_curr_emoji" => "\u{26a1} 目前",
        "stat_avg_emoji" => "\u{1f4ca} 平均",
        "stat_min_emoji" => "\u{1f4cf} 最小",
        "stat_max_emoji" => "\u{1f680} 最大",
        "stat_ttl_emoji" => "\u{1f4e6} 總計",
        "help_bar" => " \u{2190}/\u{2192} 切換裝置 | q 退出",
        "help_bar_emoji" => " \u{2b05}\u{fe0f}/\u{27a1}\u{fe0f} 切換裝置 | \u{1f6aa} q 退出",
        "help_bar_win" => " \u{2190}/\u{2192} 切換裝置 | q 退出 | 回環: --npcap",
        "help_bar_win_emoji" => " \u{2b05}\u{fe0f}/\u{27a1}\u{fe0f} 切換裝置 | \u{1f6aa} q 退出 | \u{1f4a1} 回環: --npcap",
        "terminal_too_small" => "終端視窗太小！",
        "terminal_too_small_emoji" => "\u{1f62d} 終端視窗太小！\u{1f4cc}",
        "loopback_warning" => " \u{26a0} 回環裝置：請使用 --npcap (npcap.com)",
        _ => t_en_us(key),
    }
}
