use std::time::Duration;
use std::thread;
use std::env;
use chrono::Local;

// --- 二重起動防止 ---
#[cfg(target_os = "windows")]
fn check_single_instance() {
    use windows::Win32::System::Threading::CreateMutexW;
    use windows::Win32::Foundation::{GetLastError, ERROR_ALREADY_EXISTS};
    use windows::core::w;

    unsafe {
        let handle = match CreateMutexW(None, false, w!("Local\\MyVMSTAT-single-instance-mutex")) {
            Ok(h) => h,
            Err(e) => {
                eprintln!("Error: Failed to create named mutex: {:?}", e);
                std::process::exit(1);
            }
        };
        if handle.is_invalid() {
            eprintln!("Error: Failed to create named mutex (invalid handle).");
            std::process::exit(1);
        }
        if GetLastError() == ERROR_ALREADY_EXISTS {
            eprintln!("Error: Another instance of {} is already running.", env!("CARGO_PKG_NAME"));
            std::process::exit(1);
        }
        let _ = handle;
    }
}

#[cfg(not(target_os = "windows"))]
fn check_single_instance() {}

// --- データ構造 ---
#[derive(Debug, Clone, Default)]
pub struct VmstatData {
    pub r: u64,
    pub b: u64,
    pub swpd: u64,
    pub free: u64,
    pub buff: u64,
    pub cache: u64,
    pub intr: u64,
    pub ctxt: u64,
    pub cpu_us: f64,
    pub cpu_sy: f64,
    pub cpu_id: f64,
    pub cpu_wa: f64,
}

pub trait TelemetryProvider {
    fn get_data(&mut self) -> VmstatData;
    fn get_delta(&mut self, current: &VmstatData, duration_secs: f64) -> VmstatData;
}

// --- Linux プロバイダ ---
#[cfg(target_os = "linux")]
#[derive(Default)]
struct CpuTicks {
    user: u64,
    nice: u64,
    system: u64,
    idle: u64,
    iowait: u64,
    irq: u64,
    softirq: u64,
    steal: u64,
}

#[cfg(target_os = "linux")]
pub struct LinuxProvider {
    prev_ticks: Option<CpuTicks>,
    prev_intr: Option<u64>,
    prev_ctxt: Option<u64>,
}

#[cfg(target_os = "linux")]
impl LinuxProvider {
    pub fn new() -> Self {
        Self {
            prev_ticks: None,
            prev_intr: None,
            prev_ctxt: None,
        }
    }
}

#[cfg(target_os = "linux")]
impl Default for LinuxProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(target_os = "linux")]
impl TelemetryProvider for LinuxProvider {
    fn get_data(&mut self) -> VmstatData {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        // proc stat パース
        let mut ticks = CpuTicks::default();
        let mut intr = 0;
        let mut ctxt = 0;
        let mut r = 0;
        let mut b = 0;

        if let Ok(file) = File::open("/proc/stat") {
            let reader = BufReader::new(file);
            for line in reader.lines().flatten() {
                if line.starts_with("cpu ") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 9 {
                        ticks.user = parts[1].parse().unwrap_or(0);
                        ticks.nice = parts[2].parse().unwrap_or(0);
                        ticks.system = parts[3].parse().unwrap_or(0);
                        ticks.idle = parts[4].parse().unwrap_or(0);
                        ticks.iowait = parts[5].parse().unwrap_or(0);
                        ticks.irq = parts[6].parse().unwrap_or(0);
                        ticks.softirq = parts[7].parse().unwrap_or(0);
                        ticks.steal = parts[8].parse().unwrap_or(0);
                    }
                } else if line.starts_with("intr ") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        intr = parts[1].parse().unwrap_or(0);
                    }
                } else if line.starts_with("ctxt ") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        ctxt = parts[1].parse().unwrap_or(0);
                    }
                } else if line.starts_with("procs_running ") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        r = parts[1].parse().unwrap_or(0);
                    }
                } else if line.starts_with("procs_blocked ") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        b = parts[1].parse().unwrap_or(0);
                    }
                }
            }
        }

        // meminfo パース
        let mut free = 0;
        let mut buff = 0;
        let mut cache = 0;
        let mut swap_total = 0;
        let mut swap_free = 0;

        if let Ok(file) = File::open("/proc/meminfo") {
            let reader = BufReader::new(file);
            for line in reader.lines().flatten() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let key = parts[0];
                    let val: u64 = parts[1].parse().unwrap_or(0);
                    match key {
                        "MemFree:" => free = val / 1024,
                        "Buffers:" => buff = val / 1024,
                        "Cached:" => cache = val / 1024,
                        "SwapTotal:" => swap_total = val / 1024,
                        "SwapFree:" => swap_free = val / 1024,
                        _ => {}
                    }
                }
            }
        }
        let swpd = swap_total.saturating_sub(swap_free);

        let mut data = VmstatData {
            r,
            b,
            swpd,
            free,
            buff,
            cache,
            intr,
            ctxt,
            ..Default::default()
        };

        if let Some(ref prev) = self.prev_ticks {
            let total_prev = prev.user + prev.nice + prev.system + prev.idle + prev.iowait + prev.irq + prev.softirq + prev.steal;
            let total_curr = ticks.user + ticks.nice + ticks.system + ticks.idle + ticks.iowait + ticks.irq + ticks.softirq + ticks.steal;
            let total_diff = total_curr.saturating_sub(total_prev) as f64;

            if total_diff > 0.0 {
                let user_diff = ticks.user.saturating_sub(prev.user) + ticks.nice.saturating_sub(prev.nice);
                let sys_diff = ticks.system.saturating_sub(prev.system) + ticks.irq.saturating_sub(prev.irq) + ticks.softirq.saturating_sub(prev.softirq);
                let idle_diff = ticks.idle.saturating_sub(prev.idle);
                let iowait_diff = ticks.iowait.saturating_sub(prev.iowait);

                data.cpu_us = (user_diff as f64 / total_diff) * 100.0;
                data.cpu_sy = (sys_diff as f64 / total_diff) * 100.0;
                data.cpu_id = (idle_diff as f64 / total_diff) * 100.0;
                data.cpu_wa = (iowait_diff as f64 / total_diff) * 100.0;
            }
        }

        self.prev_ticks = Some(ticks);
        self.prev_intr = Some(intr);
        self.prev_ctxt = Some(ctxt);

        data
    }

    fn get_delta(&mut self, current: &VmstatData, duration_secs: f64) -> VmstatData {
        let mut delta = current.clone();
        if let Some(prev) = self.prev_intr {
            delta.intr = ((current.intr.saturating_sub(prev)) as f64 / duration_secs) as u64;
        } else {
            delta.intr = 0;
        }
        if let Some(prev) = self.prev_ctxt {
            delta.ctxt = ((current.ctxt.saturating_sub(prev)) as f64 / duration_secs) as u64;
        } else {
            delta.ctxt = 0;
        }
        delta
    }
}

// --- Sysinfo プロバイダ ---
pub struct SysinfoProvider {
    sys: sysinfo::System,
}

impl SysinfoProvider {
    pub fn new() -> Self {
        let mut sys = sysinfo::System::new_all();
        sys.refresh_all();
        Self { sys }
    }
}

impl Default for SysinfoProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl TelemetryProvider for SysinfoProvider {
    fn get_data(&mut self) -> VmstatData {
        use sysinfo::ProcessStatus;
        
        self.sys.refresh_all();

        let mut r = 0;
        let mut b = 0;
        for process in self.sys.processes().values() {
            match process.status() {
                ProcessStatus::Run => r += 1,
                ProcessStatus::UninterruptibleDiskSleep => b += 1,
                _ => {}
            }
        }

        let swpd = (self.sys.total_swap().saturating_sub(self.sys.free_swap())) / 1024 / 1024;
        let free = self.sys.free_memory() / 1024 / 1024;
        let buff = 0;
        let cache = 0;
        let intr = 0;
        let ctxt = 0;

        let cpu_usage = self.sys.global_cpu_usage() as f64;
        let cpu_us = cpu_usage;
        let cpu_id = 100.0 - cpu_usage;
        let cpu_sy = 0.0;
        let cpu_wa = 0.0;

        VmstatData {
            r,
            b,
            swpd,
            free,
            buff,
            cache,
            intr,
            ctxt,
            cpu_us,
            cpu_sy,
            cpu_id,
            cpu_wa,
        }
    }

    fn get_delta(&mut self, current: &VmstatData, _duration_secs: f64) -> VmstatData {
        current.clone()
    }
}

// --- ファクトリ ---
#[cfg(target_os = "linux")]
fn get_provider() -> Box<dyn TelemetryProvider> {
    Box::new(LinuxProvider::new())
}

#[cfg(not(target_os = "linux"))]
fn get_provider() -> Box<dyn TelemetryProvider> {
    Box::new(SysinfoProvider::new())
}

// --- 表示フォーマット ---
fn format_val(val: u64, width: usize, color_code: &str) -> String {
    let s = format!("{:>width$}", val, width = width);
    if val == 0 {
        format!("\x1b[90m{}\x1b[0m", s)
    } else if !color_code.is_empty() {
        format!("{}{}\x1b[0m", color_code, s)
    } else {
        s
    }
}

fn format_cpu(val: f64, width: usize, color_code: &str) -> String {
    let round_val = val.round() as u64;
    let s = format!("{:>width$}", round_val, width = width);
    if round_val == 0 {
        format!("\x1b[90m{}\x1b[0m", s)
    } else if !color_code.is_empty() {
        format!("{}{}\x1b[0m", color_code, s)
    } else {
        s
    }
}

fn print_row(data: &VmstatData) {
    let r_str = format_val(data.r, 2, "");
    let b_str = format_val(data.b, 2, "");
    
    let swpd_color = if data.swpd > 128 { "\x1b[1;31m" } else { "" };
    let swpd_str = format_val(data.swpd, 6, swpd_color);
    
    let free_color = if data.free < 512 {
        "\x1b[1;31m"
    } else if data.free < 1536 {
        "\x1b[1;33m"
    } else {
        ""
    };
    let free_str = format_val(data.free, 6, free_color);
    let buff_str = format_val(data.buff, 6, "");
    let cache_str = format_val(data.cache, 5, "");
    
    let in_str = format_val(data.intr, 4, "\x1b[32m");
    let cs_color = if data.ctxt > 2000 { "\x1b[1;33m" } else { "\x1b[32m" };
    let cs_str = format_val(data.ctxt, 5, cs_color);
    
    let us_color = if data.cpu_us > 80.0 { "\x1b[1;31m" } else if data.cpu_us > 40.0 { "\x1b[1;33m" } else { "" };
    let us_str = format_cpu(data.cpu_us, 2, us_color);
    
    let sy_color = if data.cpu_sy > 40.0 { "\x1b[1;31m" } else if data.cpu_sy > 20.0 { "\x1b[1;33m" } else { "" };
    let sy_str = format_cpu(data.cpu_sy, 2, sy_color);
    
    let id_str = format_cpu(data.cpu_id, 2, "");
    
    let wa_color = if data.cpu_wa > 15.0 { "\x1b[1;31m" } else { "" };
    let wa_str = format_cpu(data.cpu_wa, 2, wa_color);
    
    let time_str = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let time_colored = format!("\x1b[34m{}\x1b[0m", time_str);

    // `{r} {b}   {swpd}   {free}   {buff}  {cache}   {in}    {cs} {us} {sy} {id} {wa}      {timestamp}`
    println!(
        "{} {}   {}   {}   {}  {}   {}    {} {} {} {} {}      {}",
        r_str, b_str, swpd_str, free_str, buff_str, cache_str, in_str, cs_str, us_str, sy_str, id_str, wa_str, time_colored
    );
}

fn print_usage() {
    let pkg_name = env!("CARGO_PKG_NAME");
    eprintln!("Usage: {} [delay [count]]", pkg_name);
    eprintln!("       {} -h | --help", pkg_name);
    eprintln!("       {} -v | --version", pkg_name);
}

fn print_help() {
    let pkg_name = env!("CARGO_PKG_NAME");
    println!("{} - A dstat-like colorized vmstat simulation utility", pkg_name);
    println!();
    println!("Usage:");
    println!("  {} [delay [count]]", pkg_name);
    println!("  {} -h | --help", pkg_name);
    println!("  {} -v | --version", pkg_name);
    println!();
    println!("Arguments:");
    println!("  delay     Sampling interval in seconds (default: 1.0)");
    println!("  count     Number of samples to display (default: infinite)");
    println!();
    println!("Options:");
    println!("  -h, --help     Show this help message and exit");
    println!("  -v, --version  Show version information and exit");
    println!();
    println!("Terminal Color Legend & Thresholds:");
    println!("  Grey (0)       Inactive / Silent metrics");
    println!("  Yellow         Warning threshold:");
    println!("                 - Free memory < 1.5 GB");
    println!("                 - CPU User > 40%");
    println!("                 - CPU System > 20%");
    println!("                 - Context switches > 2000/s");
    println!("  Bold Red       Critical threshold:");
    println!("                 - Free memory < 512 MB");
    println!("                 - Swap space usage > 128 MB");
    println!("                 - CPU User > 80%");
    println!("                 - CPU System > 40%");
    println!("                 - CPU I/O Wait > 15%");
    println!("  Green          Standard activity (Interrupts, Default Context switches)");
    println!("  Blue           Timestamps");
}

#[derive(Debug, PartialEq)]
pub enum CliAction {
    Run { delay: f64, count: Option<u64> },
    Help,
    Version,
}

pub fn parse_args(args: &[String]) -> Result<CliAction, String> {
    if args.len() > 3 {
        return Err("Error: Too many arguments.".to_string());
    }

    if args.len() > 1 {
        let first_arg = &args[1];
        if first_arg == "-v" || first_arg == "--version" {
            return Ok(CliAction::Version);
        } else if first_arg == "-h" || first_arg == "--help" {
            return Ok(CliAction::Help);
        } else if first_arg.starts_with('-') {
            return Err(format!("Error: Invalid option '{}'", first_arg));
        }

        let delay = match first_arg.parse::<f64>() {
            Ok(d) if d > 0.0 => d,
            _ => return Err(format!("Error: Invalid delay value '{}'. Must be a positive number.", first_arg)),
        };

        let mut count = None;
        if args.len() > 2 {
            let second_arg = &args[2];
            count = match second_arg.parse::<u64>() {
                Ok(c) => Some(c),
                _ => return Err(format!("Error: Invalid count value '{}'. Must be a positive integer.", second_arg)),
            };
        }

        Ok(CliAction::Run { delay, count })
    } else {
        Ok(CliAction::Run { delay: 1.0, count: None })
    }
}

// --- メイン関数 ---
fn main() {
    // 1. 二重起動防止
    check_single_instance();

    // 2. 引数のパース
    let args: Vec<String> = env::args().collect();
    let action = match parse_args(&args) {
        Ok(act) => act,
        Err(err) => {
            eprintln!("{}", err);
            print_usage();
            std::process::exit(1);
        }
    };

    let (delay, count) = match action {
        CliAction::Version => {
            println!("{} version {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
            return;
        }
        CliAction::Help => {
            print_help();
            return;
        }
        CliAction::Run { delay, count } => (delay, count),
    };



    let mut provider = get_provider();

    // 初期取得とウォームアップ
    let _ = provider.get_data();
    thread::sleep(Duration::from_millis(200));
    let mut current = provider.get_data();

    // ヘッダー表示
    println!("procs -----------memory---------- ---system-- ------cpu----- -----time-----");
    println!(" r  b   swpd   free   buff  cache   in    cs us sy id wa      timestamp");

    let mut loop_count = 0;
    loop {
        let display_data = provider.get_delta(&current, delay);
        print_row(&display_data);

        loop_count += 1;
        if let Some(max_count) = count {
            if loop_count >= max_count {
                break;
            }
        }

        // 待機
        thread::sleep(Duration::from_secs_f64(delay));
        current = provider.get_data();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_val_zero() {
        let result = format_val(0, 4, "\x1b[32m");
        assert_eq!(result, "\x1b[90m   0\x1b[0m");
    }

    #[test]
    fn test_format_val_normal() {
        let result = format_val(123, 4, "");
        assert_eq!(result, " 123");
    }

    #[test]
    fn test_format_val_colored() {
        let result = format_val(123, 4, "\x1b[32m");
        assert_eq!(result, "\x1b[32m 123\x1b[0m");
    }

    #[test]
    fn test_format_cpu_zero() {
        let result = format_cpu(0.0, 3, "\x1b[31m");
        assert_eq!(result, "\x1b[90m  0\x1b[0m");
    }

    #[test]
    fn test_format_cpu_normal() {
        let result = format_cpu(45.6, 3, "");
        assert_eq!(result, " 46");
    }

    #[test]
    fn test_format_cpu_colored() {
        let result = format_cpu(85.2, 3, "\x1b[31m");
        assert_eq!(result, "\x1b[31m 85\x1b[0m");
    }

    #[test]
    fn test_parse_args_empty() {
        let args = vec!["program".to_string()];
        assert_eq!(parse_args(&args), Ok(CliAction::Run { delay: 1.0, count: None }));
    }

    #[test]
    fn test_parse_args_help() {
        let args = vec!["program".to_string(), "-h".to_string()];
        assert_eq!(parse_args(&args), Ok(CliAction::Help));
        let args_long = vec!["program".to_string(), "--help".to_string()];
        assert_eq!(parse_args(&args_long), Ok(CliAction::Help));
    }

    #[test]
    fn test_parse_args_version() {
        let args = vec!["program".to_string(), "-v".to_string()];
        assert_eq!(parse_args(&args), Ok(CliAction::Version));
        let args_long = vec!["program".to_string(), "--version".to_string()];
        assert_eq!(parse_args(&args_long), Ok(CliAction::Version));
    }

    #[test]
    fn test_parse_args_delay_only() {
        let args = vec!["program".to_string(), "2.5".to_string()];
        assert_eq!(parse_args(&args), Ok(CliAction::Run { delay: 2.5, count: None }));
    }

    #[test]
    fn test_parse_args_delay_and_count() {
        let args = vec!["program".to_string(), "2.0".to_string(), "10".to_string()];
        assert_eq!(parse_args(&args), Ok(CliAction::Run { delay: 2.0, count: Some(10) }));
    }

    #[test]
    fn test_parse_args_invalid_option() {
        let args = vec!["program".to_string(), "--invalid".to_string()];
        assert!(parse_args(&args).is_err());
    }

    #[test]
    fn test_parse_args_invalid_delay() {
        let args = vec!["program".to_string(), "abc".to_string()];
        assert!(parse_args(&args).is_err());
        let args_neg = vec!["program".to_string(), "-1.0".to_string()];
        assert!(parse_args(&args_neg).is_err());
    }

    #[test]
    fn test_parse_args_invalid_count() {
        let args = vec!["program".to_string(), "1.0".to_string(), "abc".to_string()];
        assert!(parse_args(&args).is_err());
        let args_float = vec!["program".to_string(), "1.0".to_string(), "5.5".to_string()];
        assert!(parse_args(&args_float).is_err());
    }

    #[test]
    fn test_parse_args_too_many() {
        let args = vec!["program".to_string(), "1.0".to_string(), "5".to_string(), "extra".to_string()];
        assert!(parse_args(&args).is_err());
    }
}

