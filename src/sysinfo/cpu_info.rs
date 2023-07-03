use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

const BAY_TRAIL_CPUS: [&str; 46] = [
    "A1020", "E3805", "E3815", "E3825", "E3826", "E3827", "E3845", "J1750", "J1800", "J1850",
    "J1900", "J2850", "J2900", "N2805", "N2806", "N2807", "N2808", "N2810", "N2815", "N2820",
    "N2830", "N2840", "N2910", "N2920", "N2930", "N2940", "N3510", "N3520", "N3530", "N3540",
    "Z3735D", "Z3735E", "Z3735F", "Z3735G", "Z3736F", "Z3736G", "Z3740", "Z3740D", "Z3745",
    "Z3745D", "Z3770", "Z3770D", "Z3775", "Z3775D", "Z3785", "Z3795",
];

#[derive(Debug)]
pub struct CpuInfo {
    cpu_info: procfs::CpuInfo,
}

impl CpuInfo {
    pub fn new() -> Result<Self, procfs::ProcError> {
        Ok(Self {
            cpu_info: procfs::CpuInfo::new()?,
        })
    }

    pub fn flags(&self) -> Option<Vec<String>> {
        self.cpu_info
            .flags(0)
            .map(|flags| flags.iter().map(ToString::to_string).collect())
    }

    pub fn is_bay_trail(&self) -> bool {
        (0..self.cpu_info.num_cores())
            .filter_map(|core| self.cpu_info.model_name(core))
            .any(|model_name| {
                BAY_TRAIL_CPUS
                    .iter()
                    .any(|bay_trail_model| model_name.contains(bay_trail_model))
            })
    }
}

impl Serialize for CpuInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("CpuInfo", 3)?;

        if let Some(flags) = self.flags() {
            state.serialize_field("flags", &flags)?;
        }

        state.serialize_field("is_bay_trail", &self.is_bay_trail())?;
        state.end()
    }
}
