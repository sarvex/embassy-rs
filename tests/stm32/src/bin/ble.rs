#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

// required-features: ble

#[path = "../example_common.rs"]
mod example_common;
use embassy_executor::Spawner;
use embassy_stm32::ipcc::{Config, Ipcc};
use embassy_stm32::tl_mbox::TlMbox;
use embassy_time::{Duration, Timer};
use example_common::*;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(config());
    info!("Hello World!");

    let config = Config::default();
    let mut ipcc = Ipcc::new(p.IPCC, config);

    let mbox = TlMbox::init(&mut ipcc);

    loop {
        let wireless_fw_info = mbox.wireless_fw_info();
        match wireless_fw_info {
            None => error!("not yet initialized"),
            Some(fw_info) => {
                let version_major = fw_info.version_major();
                let version_minor = fw_info.version_minor();
                let subversion = fw_info.subversion();

                let sram2a_size = fw_info.sram2a_size();
                let sram2b_size = fw_info.sram2b_size();

                info!(
                    "version {}.{}.{} - SRAM2a {} - SRAM2b {}",
                    version_major, version_minor, subversion, sram2a_size, sram2b_size
                );

                break;
            }
        }

        Timer::after(Duration::from_millis(500)).await;
    }

    info!("Test OK");
    cortex_m::asm::bkpt();
}
