use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Baseline {
    pub program: String,
    pub instruction: String,
    pub commit: String,
    pub cu_consumed: u64,
    pub threshold_pct: f64,
    pub timestamp: String,
    pub anchor_version: String,
}

// saves the baseline, creates dir if it doesnt exist
// TODO: maybe atomic write here eventually
pub fn save_baseline(path: &Path, baseline: &Baseline) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(baseline)?;
    fs::write(path, json)?;
    Ok(())
}

pub fn load_baseline(path: &Path) -> Result<Baseline> {
    let json = fs::read_to_string(path)?;
    let baseline: Baseline = serde_json::from_str(&json)?;
    Ok(baseline)
}

// stub - real litesvm integration comes next
// currently just returns a fixed value so we can test the CLI flow
pub fn get_current_cu(_program: &str, _instruction: &str) -> u64 {
    3847
}

// compares current CU against saved baseline
// returns false if delta exceeds threshold (CI should fail)
pub fn compare_baseline(baseline: &Baseline) -> Result<bool> {
    let current_cu = get_current_cu(&baseline.program, &baseline.instruction);
    let baseline_cu = baseline.cu_consumed as f64;
    let current = current_cu as f64;
    let delta_pct = ((current - baseline_cu) / baseline_cu) * 100.0;
    
    println!("Baseline CU: {}", baseline_cu as u64);
    println!("Current CU:  {}", current_cu);
    println!("Delta:       {:.2}%", delta_pct);
    println!("Threshold:   {:.2}%", baseline.threshold_pct);
    
    if delta_pct > baseline.threshold_pct {
        println!("FAIL: CU increase exceeds threshold!");
        Ok(false)
    } else {
        println!("OK: within threshold");
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::temp_dir;

    #[test]
    fn test_save_load_roundtrip() {
        let b = Baseline {
            program: "test".into(),
            instruction: "test".into(),
            commit: "abc123".into(),
            cu_consumed: 3847,
            threshold_pct: 5.0,
            timestamp: "2025-06-02T10:00:00Z".into(),
            anchor_version: "0.31.0".into(),
        };
        let path = temp_dir().join("test_baseline.json");
        save_baseline(&path, &b).unwrap();
        let loaded = load_baseline(&path).unwrap();
        assert_eq!(loaded.cu_consumed, 3847);
        assert_eq!(loaded.program, "test");
        // cleanup
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn test_compare_detects_regression() {
        let b = Baseline {
            program: "test".into(),
            instruction: "test".into(),
            commit: "abc".into(),
            cu_consumed: 1000,  // way below stub value of 3847
            threshold_pct: 5.0,
            timestamp: "now".into(),
            anchor_version: "0.31".into(),
        };
        // this should definitely fail
        let result = compare_baseline(&b).unwrap();
        assert!(!result);
    }
}
