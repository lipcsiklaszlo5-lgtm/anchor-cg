use anyhow::Result;
use std::io::Write;
use std::path::Path;

use crate::baseline;

pub fn recalibrate(program: &str, reason: &str) -> Result<()> {
    let current_cu = baseline::get_current_cu(program, "dummy");
    let path_str = format!("baselines/{}_dummy.json", program);
    let path = Path::new(&path_str);

    let (old_cu, is_new) = match baseline::load_baseline(path) {
        Ok(existing) => (existing.cu_consumed, false),
        Err(_) => (0, true),
    };

    let now = chrono::Utc::now().to_rfc3339();

    let new_baseline = baseline::Baseline {
        program: program.to_string(),
        instruction: "dummy".to_string(),
        commit: "unknown".to_string(),
        cu_consumed: current_cu,
        threshold_pct: 5.0,
        timestamp: now.clone(),
        anchor_version: "0.31.0".to_string(),
    };

    baseline::save_baseline(path, &new_baseline)?;

    // append to CALIBRATION_LOG.md
    let log_entry = if is_new {
        format!("{} | initial calibration | reason: {} | CU: {}\n", now, reason, current_cu)
    } else {
        format!("{} | {} | old: {} CU -> new: {} CU\n", now, reason, old_cu, current_cu)
    };

    std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("CALIBRATION_LOG.md")?
        .write_all(log_entry.as_bytes())?;

    println!("Recalibrated. CU: {} (was: {})", current_cu, if is_new { 0 } else { old_cu });
    Ok(())
}
