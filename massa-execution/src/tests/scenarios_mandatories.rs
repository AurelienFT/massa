// Copyright (c) 2021 MASSA LABS <info@massa.net>

use std::str::FromStr;

use crate::{start_controller, ExecutionSettings, SCELedger, SCELedgerEntry};
use massa_models::{address::AddressHashMap, Address, Amount, Slot};
use massa_signature::{derive_public_key, generate_random_private_key};
use serial_test::serial;
use tempfile::NamedTempFile;

/// generate a named temporary initial ledger file
pub fn generate_ledger_initial_file(values: &AddressHashMap<Amount>) -> NamedTempFile {
    use std::io::prelude::*;
    let file_named = NamedTempFile::new().expect("cannot create temp file");
    serde_json::to_writer_pretty(file_named.as_file(), &values)
        .expect("unable to write initial ledger file");
    file_named
        .as_file()
        .seek(std::io::SeekFrom::Start(0))
        .expect("could not seek file");
    file_named
}

pub fn get_random_address() -> Address {
    let priv_key = generate_random_private_key();
    let pub_key = derive_public_key(&priv_key);
    Address::from_public_key(&pub_key).unwrap()
}

fn get_sample_settings() -> (NamedTempFile, ExecutionSettings) {
    let initial_file = generate_ledger_initial_file(
        &vec![
            (get_random_address(), Amount::from_str("14785.22").unwrap()),
            (get_random_address(), Amount::from_str("4778.1").unwrap()),
        ]
        .into_iter()
        .collect(),
    );
    let res = ExecutionSettings {
        initial_sce_ledger_path: initial_file.path().into(),
    };
    (initial_file, res)
}

fn get_sample_ledger() -> SCELedger {
    SCELedger(
        vec![
            (
                get_random_address(),
                SCELedgerEntry {
                    balance: Amount::from_str("129").unwrap(),
                    opt_module: None,
                    data: vec![
                        (
                            massa_hash::hash::Hash::from("key_testA".as_bytes()),
                            "test1_data".into(),
                        ),
                        (
                            massa_hash::hash::Hash::from("key_testB".as_bytes()),
                            "test2_data".into(),
                        ),
                        (
                            massa_hash::hash::Hash::from("key_testC".as_bytes()),
                            "test3_data".into(),
                        ),
                    ]
                    .into_iter()
                    .collect(),
                },
            ),
            (
                get_random_address(),
                SCELedgerEntry {
                    balance: Amount::from_str("878").unwrap(),
                    opt_module: Some("bytecodebytecode".into()),
                    data: vec![
                        (
                            massa_hash::hash::Hash::from("key_testD".as_bytes()),
                            "test4_data".into(),
                        ),
                        (
                            massa_hash::hash::Hash::from("key_testE".as_bytes()),
                            "test5_data".into(),
                        ),
                    ]
                    .into_iter()
                    .collect(),
                },
            ),
        ]
        .into_iter()
        .collect(),
    )
}

#[tokio::test]
#[serial]
async fn test_execution_basic() {
    let (_config_file_keepalive, settings) = get_sample_settings();
    assert!(start_controller(settings, 2, None).await.is_ok());
}

#[tokio::test]
#[serial]
async fn test_execution_shutdown() {
    let (_config_file_keepalive, settings) = get_sample_settings();
    let (_command_sender, _event_receiver, manager) = start_controller(settings, 2, None)
        .await
        .expect("Failed to start execution.");
    manager.stop().await.expect("Failed to stop execution.");
}

#[tokio::test]
#[serial]
async fn test_sending_command() {
    let (_config_file_keepalive, settings) = get_sample_settings();
    let (command_sender, _event_receiver, manager) = start_controller(settings, 2, None)
        .await
        .expect("Failed to start execution.");
    command_sender
        .update_blockclique(Default::default(), Default::default())
        .await
        .expect("Failed to send command");
    manager.stop().await.expect("Failed to stop execution.");
}

#[tokio::test]
#[serial]
async fn test_execution_with_bootstrap() {
    let bootstrap_state = crate::BootstrapExecutionState {
        final_slot: Slot::new(12, 5),
        final_ledger: get_sample_ledger(),
    };
    let (_config_file_keepalive, settings) = get_sample_settings();
    let (command_sender, _event_receiver, manager) =
        start_controller(settings, 2, Some(bootstrap_state))
            .await
            .expect("Failed to start execution.");
    command_sender
        .update_blockclique(Default::default(), Default::default())
        .await
        .expect("Failed to send command");
    manager.stop().await.expect("Failed to stop execution.");
}