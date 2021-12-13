// Copyright (c) 2021 MASSA LABS <info@massa.net>

use crate::{start_controller, ExecutionConfig};
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_start_send_command_stop() {
    let (mut command_sender, _event_receiver, manager) = start_controller(ExecutionConfig {}, 2)
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
async fn test_start_stop() {
    // Not sending any commands here, to make sure stopping does not require it.
    let (mut _command_sender, _event_receiver, manager) = start_controller(ExecutionConfig {}, 2)
        .await
        .expect("Failed to start execution.");
    manager.stop().await.expect("Failed to stop execution.");
}