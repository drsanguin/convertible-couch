use rand::{rngs::StdRng, RngCore, SeedableRng};

use super::{
    config_mod_info_id::ConfigModeInfoIdFuzzer,
    device_id::DeviceIdFuzzer,
    gsm_id::GsmIdFuzzer,
    monitor_name::MonitorNameFuzzer,
    position::{FuzzedMonitorPosition, FuzzedMonitorPositionedResolution},
    resolution::FuzzedResolution,
};

#[derive(Clone)]
pub struct FuzzedMonitor {
    pub name: String,
    pub primary: bool,
    pub config_mode_info_id: u32,
    pub device_id: String,
    pub resolution: FuzzedResolution,
    pub position: FuzzedMonitorPosition,
}

pub struct MonitorFuzzer {
    monitor_name_fuzzer: MonitorNameFuzzer,
    device_id_fuzzer: DeviceIdFuzzer,
    config_mode_info_id_fuzzer: ConfigModeInfoIdFuzzer,
    gsm_id_fuzzer: GsmIdFuzzer,
}

impl MonitorFuzzer {
    pub fn new(mut rand: StdRng) -> Self {
        let seed = rand.next_u64();

        Self {
            monitor_name_fuzzer: MonitorNameFuzzer::new(StdRng::seed_from_u64(seed)),
            device_id_fuzzer: DeviceIdFuzzer::new(StdRng::seed_from_u64(seed)),
            config_mode_info_id_fuzzer: ConfigModeInfoIdFuzzer::new(StdRng::seed_from_u64(seed)),
            gsm_id_fuzzer: GsmIdFuzzer::new(StdRng::seed_from_u64(seed)),
        }
    }

    pub fn generate_several(
        &mut self,
        monitors_id_common_part_1: i32,
        monitors_id_common_part_2: &str,
        monitors_id_common_part_3: i32,
        monitors_id_common_part_4: &str,
        has_an_internal_display: bool,
        positioned_resolutions: &Vec<FuzzedMonitorPositionedResolution>,
    ) -> Vec<FuzzedMonitor> {
        let n_monitor = positioned_resolutions.len();

        let names = self.monitor_name_fuzzer.generate_several(n_monitor);
        let config_mode_info_ids = self.config_mode_info_id_fuzzer.generate_several(n_monitor);
        let monitor_id_gsm_parts = self.gsm_id_fuzzer.generate_several(n_monitor);

        (0..n_monitor)
            .map(|monitor_index| {
                let position = positioned_resolutions[monitor_index].position;
                let resolution = positioned_resolutions[monitor_index].resolution;
                let primary = position.is_positioned_at_origin();
                let name = if has_an_internal_display && primary {
                    String::from("")
                } else {
                    names[monitor_index].to_owned()
                };
                let config_mode_info_id = config_mode_info_ids[monitor_index];
                let monitor_id_gsm_part = &monitor_id_gsm_parts[monitor_index];
                let device_id = self.device_id_fuzzer.generate_from_parts(
                    monitor_id_gsm_part,
                    monitors_id_common_part_1,
                    monitors_id_common_part_2,
                    monitors_id_common_part_3,
                    monitors_id_common_part_4,
                    config_mode_info_id,
                );
                FuzzedMonitor {
                    config_mode_info_id,
                    device_id,
                    name,
                    position,
                    primary,
                    resolution,
                }
            })
            .collect()
    }
}
