use crate::report::knowledge_base::PossibleMissingAccountVerificationFinding;

use super::knowledge_base::{
    Finding,
    MissingInitIfNeededFinding,
    WrongSpaceAssignmentFinding,
    MissingRequiredInstructionArgumentFinding,
    PossibleDivisionByZeroFinding,
    Severity
};
use super::line_counter::*;

use chrono::Local;

/// ANSI COLORS
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";

const RED: &str = "\x1b[31m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";
const CYAN: &str = "\x1b[36m";
const MAGENTA: &str = "\x1b[35m";
const GREEN: &str = "\x1b[32m";

fn sev_color(sev: Severity) -> &'static str {
    match sev {
        Severity::High => RED,
        Severity::Medium => YELLOW,
        Severity::Low => BLUE,
    }
}

fn sev_icon(sev: Severity) -> &'static str {
    match sev {
        Severity::High => "⚠",
        Severity::Medium => "⚠",
        Severity::Low => "⚠",
    }
}

fn finding_severity(f: &Finding) -> Severity {
    match f {
        Finding::MissingInitIfNeeded(x) => x.rule.severity,
        Finding::WrongSpaceAssignment(x) => x.rule.severity,
        Finding::MissingRequiredInstructionArgument(x) => x.rule.severity,
        Finding::PossibleDivisionByZero(x) => x.rule.severity,
        Finding::PossibleMissingAccountVerification(x) => x.rule.severity
    }
}

#[derive(Default)]
pub struct Report {
    pub findings: Vec<Finding>,
    pub file_info: Option<FileInfo>,
    pub file_path: Option<String>,
}

impl Report {
    pub fn add(&mut self, f: Finding) {
        self.findings.push(f);
    }

    //for implementing line_counter
    pub fn load_file_info<P: AsRef<std::path::Path>>(&mut self, path: P) {
        let path_str = path.as_ref().to_string_lossy().to_string();
        match analyze_file(&path) {
            Ok(info) => {
                self.file_info = Some(info);
                self.file_path = Some(path_str);
            }
            Err(e) => {
                eprintln!("Warning: failed to analyze file stats: {}", e);
            }
        }
    }

    pub fn print(&self) {
        println!("\n{MAGENTA}{BOLD}══════════════════════════════════════════════════════════════");
        println!("        SOLANA STATIC ANALYZER — SECURITY REPORT");
        println!("══════════════════════════════════════════════════════════════{RESET}\n");

        println!("{DIM}Generated at {}{RESET}\n", Local::now().format("%Y-%m-%d %H:%M:%S"));

        if let (Some(info), Some(path)) = (&self.file_info, &self.file_path) {
            println!(" File Analyzed: {}", path);
            println!("   ├─ Lines of Code : {}", info.lines_of_code);
            println!("   ├─ Comments      : {}", info.comments_count);
            println!("   ├─ Blank Lines   : {}", info.blank_spaces_count);
            println!("   └─ Total Lines   : {}\n",
                info.lines_of_code + info.comments_count + info.blank_spaces_count
            );
        } else {
            println!("File metadata: <unavailable>\n");
        }

        println!("\n{BLUE}{BOLD}══════════════════════════════════════════════════════════════");
        println!("        FINDINGS");
        println!("══════════════════════════════════════════════════════════════{RESET}\n");

        if self.findings.is_empty() {
            println!("{GREEN}{BOLD} No vulnerabilities found!{RESET}\n");
            return;
        }

        self.print_summary();
        self.print_findings();
        
        println!("{MAGENTA}{BOLD}══════════════════════════════════════════════════════════════");
        println!("                       END OF REPORT");
        println!("══════════════════════════════════════════════════════════════{RESET}\n");
    }

    fn print_summary(&self) {
        let mut high = 0;
        let mut medium = 0;
        let mut low = 0;

        for f in &self.findings {
            let sev = match f {
                Finding::MissingInitIfNeeded(x) => x.rule.severity,
                Finding::WrongSpaceAssignment(x) => x.rule.severity,
                Finding::MissingRequiredInstructionArgument(x) => x.rule.severity,
                Finding::PossibleDivisionByZero(x) => x.rule.severity,
                Finding::PossibleMissingAccountVerification(x) => x.rule.severity
            };

            match sev {
                Severity::High => high += 1,
                Severity::Medium => medium += 1,
                Severity::Low => low += 1,
            }
        }

        println!("{CYAN}{BOLD}Summary:{RESET}");
        println!("{CYAN}──────────────────────────────────────────────{RESET}");
        println!(
            "  {}{} High Severity Issues{RESET}",
            RED, high
        );
        println!(
            "  {}{} Medium Severity Issues{RESET}",
            YELLOW, medium
        );
        println!(
            "  {}{} Low Severity Issues{RESET}",
            BLUE, low
        );
        println!("{CYAN}──────────────────────────────────────────────{RESET}\n");
    }

    fn print_findings(&self) {

        let mut sorted_findings = self.findings.clone();

        sorted_findings.sort_by(|a, b| {
            finding_severity(a).cmp(&finding_severity(b))
        });

        for f in &sorted_findings {
            match f {
                Finding::MissingInitIfNeeded(x) => self.print_missing_init_if_needed(x),
                Finding::WrongSpaceAssignment(x) => self.print_wrong_space_assignment(x),
                Finding::MissingRequiredInstructionArgument(x) => {
                    self.print_missing_instruction_arg(x)
                }
                Finding::PossibleDivisionByZero(x) => self.print_division_by_zero(x),
                Finding::PossibleMissingAccountVerification(x) => self.print_missing_account_verification(x)
            }
        }
    }

    fn header(rule_code: &str, title: &str, severity: Severity) {
        println!("{BOLD}{MAGENTA}────────────────────────────────────────────────────────{RESET}");
        println!(
            "{BOLD}{} {}  {}{}{}{RESET}",
            sev_icon(severity),
            rule_code,
            sev_color(severity),
            title,
            RESET
        );
        println!("{MAGENTA}────────────────────────────────────────────────────────{RESET}");
    }

    fn print_missing_init_if_needed(&self, x: &MissingInitIfNeededFinding) {
        Self::header(x.rule.code, x.rule.title, x.rule.severity);

        println!("  {BOLD}Account:{RESET} {}", x.account);
        println!("  {BOLD}Context:{RESET} {}", x.context);
        println!("  {BOLD}Line:{RESET} {}", x.line);

        println!("\n  {BOLD}Description:{RESET}");
        println!("    {}", x.rule.description);

        println!("\n  {BOLD}Recommendation:{RESET}");
        println!("    {}", x.rule.recommendation);

        if let Some(links) = x.rule.additional_links {
            println!("\n  {BOLD}Resources:{RESET}");
            for link in links.split('\n') {
                println!("    🔗 {}", link);
            }
        }

        println!();
    }

    fn print_wrong_space_assignment(&self, x: &WrongSpaceAssignmentFinding) {
        Self::header(x.rule.code, x.rule.title, x.rule.severity);

        println!("  {BOLD}Location Line:{RESET} {}", x.line);
        println!("  {BOLD}Account:{RESET} {}", x.account);
        println!("  {BOLD}Expected:{RESET} {}", x.expected);
        println!("  {BOLD}Actual:{RESET} {}", x.actual);

        println!("\n  {BOLD}Description:{RESET}");
        println!("    {}", x.rule.description);

        println!("\n  {BOLD}Recommendation:{RESET}");
        println!("    {}", x.rule.recommendation);
        println!();
    }

    fn print_missing_instruction_arg(&self, x: &MissingRequiredInstructionArgumentFinding) {
        Self::header(x.rule.code, x.rule.title, x.rule.severity);

        println!("  {BOLD}Function:{RESET} {}", x.fn_name);
        println!("  {BOLD}Missing Argument:{RESET} {}: {}", x.required_arg_name, x.required_arg_type);
        println!("  {BOLD}Line:{RESET} {}", x.line);

        println!("\n  {BOLD}Description:{RESET}");
        println!("    {}", x.rule.description);

        println!("\n  {BOLD}Recommendation:{RESET}");
        println!("    {}", x.rule.recommendation);
        println!();
    }

    fn print_division_by_zero(&self, x: &PossibleDivisionByZeroFinding) {
        Self::header(x.rule.code, x.rule.title, x.rule.severity);

        println!("  {BOLD}Function:{RESET} {}", x.fn_name);
        println!("  {BOLD}Divisor Variable:{RESET} {}", x.divisor);
        println!("  {BOLD}Line:{RESET} {}", x.line);

        println!("\n  {BOLD}Description:{RESET}");
        println!("    {}", x.rule.description);

        println!("\n  {BOLD}Recommendation:{RESET}");
        println!("    {}", x.rule.recommendation);

        if let Some(links) = x.rule.additional_links {
            println!("\n  {BOLD}Resources:{RESET}");
            for link in links.split('\n') {
                println!("    🔗 {}", link);
            }
        }

        println!();
    }

    fn print_missing_account_verification(&self, x: &PossibleMissingAccountVerificationFinding) {
        Self::header(x.rule.code, x.rule.title, x.rule.severity);

        println!("  {BOLD}Account:{RESET} {}", x.account_name);
        println!("  {BOLD}Type:{RESET} {}", x.field_type);
        println!("  {BOLD}Line:{RESET} {}", x.line);

        println!("\n  {BOLD}Description:{RESET}");
        println!("    {}", x.rule.description);

        println!("\n  {BOLD}Recommendation:{RESET}");
        println!("    {}", x.rule.recommendation);

        if let Some(links) = x.rule.additional_links {
            println!("\n  {BOLD}Resources:{RESET}");
            for link in links.split('\n') {
                println!("    🔗 {}", link);
            }
        }

        println!();
    }
}
