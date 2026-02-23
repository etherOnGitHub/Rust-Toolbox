fn main() -> anyhow::Result<()> {
    let cfg = rtb_config::AppConfig::default();
    let msg = rtb_core::application::build_banner(&cfg.environment);
    let state = rtb_storage::adapters::in_memory::bootstrap();

    println!("{msg} | storage={state}");
    Ok(())
}
