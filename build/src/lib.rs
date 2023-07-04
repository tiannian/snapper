use anyhow::Result;
use snapper_solc::Solc;

async fn _build() -> Result<()> {
    // Compile code
    let builder = Solc::new().await?;
    builder.compile().await?;
    // Generate lib from abi
    Ok(())
}
