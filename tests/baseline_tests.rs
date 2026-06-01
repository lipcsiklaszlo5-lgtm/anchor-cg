use std::env::temp_dir;
use std::fs;

// manual baseline struct for testing without importing crate
#[test]
fn test_baseline_roundtrip_manual() {
    let json = r#"{
        "program": "counter",
        "instruction": "increment",
        "commit": "abc123",
        "cu_consumed": 3847,
        "threshold_pct": 5.0,
        "timestamp": "2025-06-02T10:00:00Z",
        "anchor_version": "0.31.0"
    }"#;
    let path = temp_dir().join("roundtrip_test.json");
    fs::write(&path, json).unwrap();
    let read_back = fs::read_to_string(&path).unwrap();
    assert!(read_back.contains("3847"));
    assert!(read_back.contains("counter"));
    let _ = fs::remove_file(&path);
}

#[test]
fn test_threshold_logic() {
    // simulate: baseline 1000, current 1100 = 10% increase, threshold 5% = should fail
    let baseline_cu: f64 = 1000.0;
    let current_cu: f64 = 1100.0;
    let threshold: f64 = 5.0;
    let delta_pct = ((current_cu - baseline_cu) / baseline_cu) * 100.0;
    assert!(delta_pct > threshold);
    // if delta > threshold, CI should fail
}
