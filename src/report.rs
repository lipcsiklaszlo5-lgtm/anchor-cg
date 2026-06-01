use crate::baseline::Baseline;

pub fn print_summary(b: &Baseline) {
    println!("Program:      {}", b.program);
    println!("Instruction:  {}", b.instruction);
    println!("CU consumed:  {}", b.cu_consumed);
    println!("Threshold:    {}%", b.threshold_pct);
    println!("Timestamp:    {}", b.timestamp);
    println!("Anchor ver:   {}", b.anchor_version);
    println!("Commit:       {}", b.commit);
}

pub fn write_json(path: &std::path::Path, b: &Baseline) -> anyhow::Result<()> {
    let json = serde_json::to_string_pretty(b)?;
    std::fs::write(path, json)?;
    Ok(())
}
