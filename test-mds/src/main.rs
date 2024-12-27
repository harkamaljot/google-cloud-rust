use gcp_sdk_auth::{credentials::mds_credential, token::TokenProvider};  // Assuming the path is correct, replace with actual package and module name
use tokio::runtime::Runtime;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rt = Runtime::new()?;
    let mut token_provider = mds_credential::MDSAccessTokenProvider {
        token_endpoint: mds_credential::METADATA_ROOT.clone(),
    };
    println!("{:?}", rt.block_on(token_provider.get_token())?);
    Ok(())
}
