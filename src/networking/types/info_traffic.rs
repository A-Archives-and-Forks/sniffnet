//! Module defining the `ReportInfo` struct, useful to format the output report file and
//! to keep track of statistics about the sniffed traffic.

use std::collections::{HashMap, HashSet};

use indexmap::IndexMap;

use crate::networking::types::address_port_pair::AddressPortPair;
use crate::networking::types::data_info::DataInfo;
use crate::networking::types::host::Host;
use crate::networking::types::info_address_port_pair::InfoAddressPortPair;
use crate::AppProtocol;

/// Struct to be shared between the threads in charge of parsing packets and update reports.
pub struct InfoTraffic {
    /// Total amount of filtered bytes received.
    pub tot_received_bytes: u128,
    /// Total amount of filtered bytes sent.
    pub tot_sent_bytes: u128,
    /// Total amount of filtered packets received.
    pub tot_received_packets: u128,
    /// Total amount of filtered packets sent.
    pub tot_sent_packets: u128,
    /// Total packets including those not filtered
    pub all_packets: u128,
    /// Total bytes including those not filtered
    pub all_bytes: u128,
    /// Number of dropped packets
    pub dropped_packets: u32,
    /// Map of the filtered traffic
    pub map: IndexMap<AddressPortPair, InfoAddressPortPair>,
    /// Set with the addresses of the last time interval
    pub addresses_last_interval: HashSet<usize>,
    /// Collection of the favorite hosts
    pub favorite_hosts: HashSet<Host>,
    /// Collection of favorite hosts that exchanged data in the last interval
    pub favorites_last_interval: HashSet<Host>,
    /// Map of the application layer protocols with their data info
    pub app_protocols: HashMap<AppProtocol, DataInfo>,
    /// Map of the hosts with their data info
    pub hosts: HashMap<Host, (DataInfo, bool)>,
}

impl InfoTraffic {
    /// Constructs a new `InfoTraffic` element.
    pub fn new() -> Self {
        InfoTraffic {
            tot_received_bytes: 0,
            tot_sent_bytes: 0,
            tot_received_packets: 0,
            tot_sent_packets: 0,
            all_packets: 0,
            all_bytes: 0,
            dropped_packets: 0,
            map: IndexMap::new(),
            addresses_last_interval: HashSet::new(),
            favorite_hosts: HashSet::new(),
            favorites_last_interval: HashSet::new(),
            app_protocols: HashMap::new(),
            hosts: HashMap::new(),
        }
    }
}
