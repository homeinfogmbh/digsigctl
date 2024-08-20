use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CpuInfo {
    is_bay_trail: bool,
    model_name: Option<String>,
}

impl CpuInfo {
    #[must_use]
    pub const fn is_bay_trail(&self) -> bool {
        self.is_bay_trail
    }

    #[must_use]
    pub fn model_name(&self) -> Option<&str> {
        self.model_name.as_deref()
    }
}

#[cfg(target_family = "windows")]
mod windows {
    use super::CpuInfo;

    impl CpuInfo {
        #[allow(clippy::unnecessary_wraps)]
        pub const fn read() -> Result<Self, std::io::Error> {
            Ok(Self {
                is_bay_trail: false,
                model_name: None,
            })
        }
    }
}

#[cfg(target_family = "unix")]
mod unix {
    use super::CpuInfo;

    const BAY_TRAIL_CPUS: [&str; 46] = [
        "A1020", "E3805", "E3815", "E3825", "E3826", "E3827", "E3845", "J1750", "J1800", "J1850",
        "J1900", "J2850", "J2900", "N2805", "N2806", "N2807", "N2808", "N2810", "N2815", "N2820",
        "N2830", "N2840", "N2910", "N2920", "N2930", "N2940", "N3510", "N3520", "N3530", "N3540",
        "Z3735D", "Z3735E", "Z3735F", "Z3735G", "Z3736F", "Z3736G", "Z3740", "Z3740D", "Z3745",
        "Z3745D", "Z3770", "Z3770D", "Z3775", "Z3775D", "Z3785", "Z3795",
    ];

    impl CpuInfo {
        pub fn read() -> Result<Self, std::io::Error> {
            proc_cpuinfo::CpuInfo::read().map(Self::from)
        }
    }

    impl From<proc_cpuinfo::CpuInfo> for CpuInfo {
        fn from(cpu_info: proc_cpuinfo::CpuInfo) -> Self {
            Self {
                is_bay_trail: is_bay_trail(&cpu_info),
                model_name: model_name(&cpu_info),
            }
        }
    }

    fn is_bay_trail(cpu_info: &proc_cpuinfo::CpuInfo) -> bool {
        cpu_info
            .cpus()
            .filter_map(|cpu| cpu.model_name().map(ToString::to_string))
            .any(|model_name| {
                BAY_TRAIL_CPUS
                    .iter()
                    .any(|bay_trail_model| model_name.contains(bay_trail_model))
            })
    }

    fn model_name(cpu_info: &proc_cpuinfo::CpuInfo) -> Option<String> {
        cpu_info
            .cpus()
            .next()
            .and_then(|cpu| cpu.model_name().map(ToString::to_string))
    }
}
