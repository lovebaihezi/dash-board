use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::tools::BitType;

// #[derive(Debug)]
// pub enum CpuInfoField {
//     processor(String),
// }
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct CpuInfo {
    processor: Option<u32>,
    vendor_id: Option<String>,
    cpu_family: Option<u32>,
    model: Option<u32>,
    model_name: Option<String>,
    stepping: Option<u32>,
    microcode: Option<u32>,
    cpu_mhz: Option<f32>,
    cache_size: Option<BitType>,
    physical_id: Option<u32>,
    siblings: Option<u32>,
    core_id: Option<u32>,
    cpu_cores: Option<u32>,
    apicid: Option<u32>,
    initial_apicid: Option<u32>,
    fpu: Option<bool>,
    fpu_exception: Option<bool>,
    cpuid_level: Option<u32>,
    wp: Option<bool>,
    flags: Option<Vec<String>>,
    vmx_flags: Option<Vec<String>>,
    bugs: Option<Vec<String>>,
    bogomips: Option<f64>,
    clflush_size: Option<u32>,
    cache_alignment: Option<u32>,
    address_sizes: Option<String>,
    power_management: Option<String>,
}

#[inline]
fn read_cpu_info(cpu_info_file_path: &str) -> std::io::Result<String> {
    std::fs::read_to_string(cpu_info_file_path)
}

#[inline]
fn all_core_info_collect(cpu_info_string: String) -> Vec<String> {
    cpu_info_string
        .split("\n\n")
        .map(|s| s.to_string())
        .collect()
}

#[inline]
fn each_core_info_struct(core_info: String) -> CpuInfo {
    let core_detail = core_info
        .split('\n')
        .map(|s| -> Option<(String, String)> {
            let mut name_value_iter = s.split(':').into_iter().map(|s| s.trim().to_string());
            let name = name_value_iter.next()?;
            let value = name_value_iter.next()?;
            Some((name, value))
        })
        .flatten()
        .collect::<Vec<(String, String)>>();
    let mut cpu_info: CpuInfo = CpuInfo::default();
    for (name, value) in core_detail {
        let match_with = name.to_ascii_lowercase().replace(" ", "_");
        match match_with.as_str() {
            // TODO: simplify this! (macro rule?)
            // u32
            "model" => cpu_info.model = Some(value.parse::<u32>().unwrap()),
            "apicid" => cpu_info.apicid = Some(value.parse::<u32>().unwrap()),
            "core_id" => cpu_info.core_id = Some(value.parse::<u32>().unwrap()),
            "stepping" => cpu_info.stepping = Some(value.parse::<u32>().unwrap()),
            "siblings" => cpu_info.siblings = Some(value.parse::<u32>().unwrap()),
            "processor" => cpu_info.processor = Some(value.parse::<u32>().unwrap()),
            "cpu_cores" => cpu_info.cpu_cores = Some(value.parse::<u32>().unwrap()),
            "cpu_family" => cpu_info.cpu_family = Some(value.parse::<u32>().unwrap()),
            "physical_id" => cpu_info.physical_id = Some(value.parse::<u32>().unwrap()),
            "cpuid_level" => cpu_info.cpuid_level = Some(value.parse::<u32>().unwrap()),
            "clflush_size" => cpu_info.clflush_size = Some(value.parse::<u32>().unwrap()),
            "initial_apicid" => cpu_info.initial_apicid = Some(value.parse::<u32>().unwrap()),
            "cache_alignment" => cpu_info.cache_alignment = Some(value.parse::<u32>().unwrap()),
            // hex number
            "microcode" => {
                cpu_info.microcode =
                    Some(u32::from_str_radix(value.trim_start_matches("0x"), 16).unwrap())
            }
            // String
            "vendor_id" => cpu_info.vendor_id = Some(value),
            "model_name" => cpu_info.model_name = Some(value),
            "address_sizes" => cpu_info.address_sizes = Some(value),
            // BitInfo
            "cache_size" => cpu_info.cache_size = BitType::new(value.as_str()),
            // f32
            "cpu_mhz" => cpu_info.cpu_mhz = Some(value.parse::<f32>().unwrap()),
            // f64
            "bogomips" => cpu_info.bogomips = Some(value.parse::<f64>().unwrap()),
            // bool
            "fpu" => cpu_info.fpu = Some(value == "yes"),
            "fpu_exception" => cpu_info.fpu_exception = Some(value == "yes"),
            "wp" => cpu_info.wp = Some(value == "yes"),
            // Vec String
            "flags" => cpu_info.flags = Some(value.split(' ').map(|s| s.to_string()).collect()),
            "vmx_flags" => {
                cpu_info.vmx_flags = Some(value.split(' ').map(|s| s.to_string()).collect())
            }
            "bugs" => cpu_info.bugs = Some(value.split(' ').map(|s| s.to_string()).collect()),
            // _ => print!("[warn] !{} not support yet!", &name),
            "power_management" => cpu_info.power_management = Some(value),
            _ => warn!("{} not support yet!", &name),
        }
    }
    cpu_info
}

pub fn cpu_info() -> std::io::Result<Vec<CpuInfo>> {
    let cpu_info_origin = read_cpu_info("/proc/cpuinfo")?;
    let split_with_each_core_info = all_core_info_collect(cpu_info_origin);
    Ok(split_with_each_core_info
        .iter()
        .filter(|s| !s.is_empty())
        .map(|s| -> CpuInfo { each_core_info_struct(s.clone()) })
        .collect())
}

#[test]
fn read_cpu_info_test() {
    let cpu_info_file_path = "/proc/cpuinfo";
    let result = read_cpu_info(cpu_info_file_path).unwrap();
    assert!(!result.is_empty());
}
