//! Module defining the `ConfigDevice` struct, which allows to save and reload
//! the application default configuration.

use std::sync::{Arc, Mutex};

use pcap::Device;
use serde::{Deserialize, Serialize};

use crate::networking::types::my_device::MyDevice;

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigDevice {
    pub device_name: String,
}

impl Default for ConfigDevice {
    fn default() -> Self {
        Self {
            device_name: Device::lookup().unwrap().unwrap().name,
        }
    }
}

impl ConfigDevice {
    pub fn load() -> Self {
        if let Ok(device) = confy::load::<ConfigDevice>("sniffnet", "device") {
            device
        } else {
            confy::store("sniffnet", "device", ConfigDevice::default()).unwrap_or(());
            ConfigDevice::default()
        }
    }

    pub fn store(self) {
        confy::store("sniffnet", "device", self).unwrap_or(());
    }

    pub fn to_my_device(&self) -> MyDevice {
        for device in Device::list().unwrap() {
            if device.name.eq(&self.device_name) {
                return MyDevice {
                    name: device.name,
                    desc: device.desc,
                    addresses: Arc::new(Mutex::new(device.addresses)),
                };
            }
        }
        let standard_device = Device::lookup().unwrap().unwrap();
        MyDevice {
            name: standard_device.name,
            desc: standard_device.desc,
            addresses: Arc::new(Mutex::new(standard_device.addresses)),
        }
    }
}
