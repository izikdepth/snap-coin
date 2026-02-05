use snap_coin::{build_block, crypto::keys::Private, full_node::{accept_block, create_full_node}};
use flexi_logger::Logger;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = Logger::try_with_str("debug").ok();

    let (blockchain, node_state) = create_full_node("./node-devnet", false);


    loop {
        let transactions = vec![];

        let mut some_block = build_block(&*blockchain, &transactions, Private::new_random().to_public()).await?;
        #[allow(deprecated)]
        some_block.compute_pow()?;

        let mut another_block = build_block(&*blockchain, &transactions, Private::new_random().to_public()).await?;
        #[allow(deprecated)]
        another_block.compute_pow()?;   

        accept_block(&blockchain, &node_state, some_block).await?;
        println!("Last block: {}", blockchain.block_store().get_last_block_hash().dump_base36());

        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    }
}