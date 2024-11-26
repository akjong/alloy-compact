use alloy::{
    consensus::TxEnvelope,
    eips::eip2718::Encodable2718,
    network::{Ethereum, EthereumWallet, TransactionBuilder, TransactionBuilderError},
    primitives::{Bytes, ChainId},
    rpc::types::TransactionRequest,
};

pub fn envelope_to_raw_tx(tx: &TxEnvelope) -> Bytes {
    let mut encoded = Vec::new();
    tx.encode_2718(&mut encoded);
    encoded.into()
}

pub async fn tx_req_to_envelope(
    tx_req: TransactionRequest,
    chain_id: ChainId,
    wallet: EthereumWallet,
) -> Result<TxEnvelope, TransactionBuilderError<Ethereum>> {
    <TransactionRequest as TransactionBuilder<Ethereum>>::with_chain_id(tx_req, chain_id)
        .build(&wallet)
        .await
}
