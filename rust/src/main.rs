// Copyright (C) 2025 Amaan Qureshi <contact@amaanq.com>
// SPDX-License-Identifier: Apache-2.0

//! Benzene Battery HAL service.

mod service;
mod sysfs;

use log::{error, info};

fn main() {
    android_logger::init_once(
        android_logger::Config::default()
            .with_tag("benzene_battery")
            .with_max_level(log::LevelFilter::Info),
    );

    info!("Starting Benzene Battery HAL");

    binder::ProcessState::set_thread_pool_max_thread_count(1);
    binder::ProcessState::start_thread_pool();

    if let Err(e) = service::register() {
        error!("Failed to register: {e:?}");
        std::process::exit(1);
    }

    binder::ProcessState::join_thread_pool();
}
