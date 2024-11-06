use anchor_lang::prelude::*;

#[error_code]
pub enum MarketplaceError {
    #[msg("way to big")]
    WayToBig,
}
