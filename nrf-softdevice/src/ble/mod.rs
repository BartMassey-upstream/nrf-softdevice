//! Bluetooth Low Energy

mod connection;
mod gatt_traits;
mod types;

pub use connection::*;
pub use gatt_traits::*;
pub use types::*;

mod common;
mod gap;

#[cfg(feature = "ble-central")]
pub mod central;

#[cfg(feature = "ble-peripheral")]
pub mod peripheral;

#[cfg(feature = "ble-gatt-client")]
pub mod gatt_client;

#[cfg(feature = "ble-gatt-server")]
pub mod gatt_server;

#[cfg(feature = "ble-l2cap")]
pub mod l2cap;

use core::mem;

use crate::fmt::*;
use crate::{raw, RawError, Softdevice};

pub(crate) unsafe fn on_evt(ble_evt: *const raw::ble_evt_t) {
    defmt::trace!("ble evt {:u32}", (*ble_evt).header.evt_id as u32);
    match (*ble_evt).header.evt_id as u32 {
        raw::BLE_EVT_BASE..=raw::BLE_EVT_LAST => common::on_evt(ble_evt),
        raw::BLE_GAP_EVT_BASE..=raw::BLE_GAP_EVT_LAST => gap::on_evt(ble_evt),
        #[cfg(feature = "ble-gatt-client")]
        raw::BLE_GATTC_EVT_BASE..=raw::BLE_GATTC_EVT_LAST => gatt_client::on_evt(ble_evt),
        #[cfg(feature = "ble-gatt-server")]
        raw::BLE_GATTS_EVT_BASE..=raw::BLE_GATTS_EVT_LAST => gatt_server::on_evt(ble_evt),
        #[cfg(feature = "ble-l2cap")]
        raw::BLE_L2CAP_EVT_BASE..=raw::BLE_L2CAP_EVT_LAST => l2cap::on_evt(ble_evt),
        _ => {}
    }
}

pub fn get_address(_sd: &Softdevice) -> Address {
    unsafe {
        let mut addr: raw::ble_gap_addr_t = mem::zeroed();
        let ret = raw::sd_ble_gap_addr_get(&mut addr);
        unwrap!(RawError::convert(ret), "sd_ble_gap_addr_get");
        Address::from_raw(addr)
    }
}

pub fn set_address(_sd: &Softdevice, addr: &Address) {
    unsafe {
        let addr = addr.into_raw();
        let ret = raw::sd_ble_gap_addr_set(&addr);
        unwrap!(RawError::convert(ret), "sd_ble_gap_addr_set");
    }
}
