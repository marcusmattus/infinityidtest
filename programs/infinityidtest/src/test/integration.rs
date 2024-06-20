use anchor_lang::prelude::*;
use anchor_lang::prelude::ProgramError;
use solana_program_test::*;
use solana_sdk::signature::Keypair;
use solana_sdk::transport::TransportError;
use infinityid::{entrypoint::*, id};

#[tokio::test]
async fn test_create_profile() {
    let program = ProgramTest::new(
        "infinityid",
        id(),
        processor!(infinityid::entry),
    );

    let (mut banks_client, payer, recent_blockhash) = program.start().await;

    // Your test implementation here
}
