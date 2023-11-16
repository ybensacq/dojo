//! This module contains a hooker trait, that is added to katana in order to
//! allow external code to react at some precise moment of katana processing.
//!
use async_trait::async_trait;
use starknet::core::types::BroadcastedInvokeTransaction;
use starknet::accounts::Call;

#[async_trait]
pub trait KatanaHooker {
    /// Runs code right before an invoke transaction
    /// is being added to the pool.
    /// Returns true if the transaction should be included
    /// in the pool, false otherwise.
    ///
    /// # Arguments
    ///
    /// * `transaction` - The invoke transaction to be verified.
    async fn verify_invoke_tx_before_pool(
        &self,
        transaction: BroadcastedInvokeTransaction,
    ) -> bool;

    /// Runs code right before a message to starknet
    /// is being sent via a direct transaction.
    /// As the message is sent to starknet in a transaction
    /// the `Call` of the transaction is being verified.
    ///
    /// # Arguments
    ///
    /// * `call` - The `Call` to inspect, built from the
    /// message.
    async fn verify_message_to_starknet_before_tx(
        &self,
        call: Call,
    ) -> bool;

    /// Runs when Solis attempts to execute an order on Starknet,
    /// but it fails.
    ///
    /// # Arguments
    ///
    /// * `call` - The `Call` of the transaction that has failed.
    ///            Usually the same as `verify_message_to_starknet_before_tx`.
    async fn react_on_starknet_tx_failed(
        &self,
        call: Call,
    );
}
