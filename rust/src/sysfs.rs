// Copyright (C) 2025 Amaan Qureshi <contact@amaanq.com>
// SPDX-License-Identifier: Apache-2.0

use std::{
    fs,
    io::{self, ErrorKind},
    path::Path,
};

#[derive(Debug)]
pub enum Error {
    NotFound { path: String },
    Io { path: String, source: io::Error },
    Parse { path: String, content: String },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound { path } => write!(f, "sysfs path not found: {path}"),
            Self::Io { path, source } => write!(f, "I/O error on {path}: {source}"),
            Self::Parse { path, content } => write!(f, "parse error: '{content}' from {path}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),
            _ => None,
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Copy)]
pub struct SysfsPath {
    pub primary: &'static str,
    pub alternate: Option<&'static str>,
}

impl SysfsPath {
    pub const fn new(primary: &'static str) -> Self {
        Self {
            primary,
            alternate: None,
        }
    }

    pub const fn with_alt(primary: &'static str, alternate: &'static str) -> Self {
        Self {
            primary,
            alternate: Some(alternate),
        }
    }

    pub fn resolve(&self) -> Option<&'static str> {
        if Path::new(self.primary).exists() {
            Some(self.primary)
        } else {
            self.alternate.filter(|p| Path::new(p).exists())
        }
    }

    pub fn exists(&self) -> bool {
        self.resolve().is_some()
    }

    pub fn read_string(&self) -> Result<String> {
        self.resolve()
            .ok_or_else(|| Error::NotFound {
                path: self.primary.into(),
            })
            .and_then(read_string)
    }

    pub fn read_int(&self) -> Result<i32> {
        self.resolve()
            .ok_or_else(|| Error::NotFound {
                path: self.primary.into(),
            })
            .and_then(read_int)
    }

    pub fn write_string(&self, value: &str) -> Result<()> {
        self.resolve()
            .ok_or_else(|| Error::NotFound {
                path: self.primary.into(),
            })
            .and_then(|p| write_string(p, value))
    }

    pub fn write_int(&self, value: i32) -> Result<()> {
        self.write_string(&value.to_string())
    }

    pub fn read_int_or(&self, default: i32) -> i32 {
        self.read_int().unwrap_or(default)
    }
}

pub fn read_string(path: &str) -> Result<String> {
    fs::read_to_string(path)
        .map(|s| s.trim().to_string())
        .map_err(|e| match e.kind() {
            ErrorKind::NotFound => Error::NotFound { path: path.into() },
            _ => Error::Io {
                path: path.into(),
                source: e,
            },
        })
}

pub fn read_int(path: &str) -> Result<i32> {
    let content = read_string(path)?;
    content.parse().map_err(|_| Error::Parse {
        path: path.into(),
        content,
    })
}

pub fn write_string(path: &str, value: &str) -> Result<()> {
    fs::write(path, value).map_err(|e| match e.kind() {
        ErrorKind::NotFound => Error::NotFound { path: path.into() },
        _ => Error::Io {
            path: path.into(),
            source: e,
        },
    })
}

pub mod paths {
    use super::SysfsPath;

    pub const CHARGING_POLICY: SysfsPath =
        SysfsPath::new("/sys/class/power_supply/battery/charging_policy");

    // Charge limit control nodes
    pub const CHARGE_STOP_LEVEL: SysfsPath =
        SysfsPath::new("/sys/devices/platform/google,charger/charge_stop_level");
    pub const CHARGE_START_LEVEL: SysfsPath =
        SysfsPath::new("/sys/devices/platform/google,charger/charge_start_level");

    pub const BD_CLEAR: SysfsPath = SysfsPath::with_alt(
        "/sys/devices/platform/google,charger/bd_clear",
        "/sys/devices/platform/soc/soc:google,charger/bd_clear",
    );
    pub const BD_TRICKLE_RESET_SEC: SysfsPath =
        SysfsPath::new("/sys/class/power_supply/battery/bd_trickle_reset_sec");

    pub const BD_TRICKLE_RATE: SysfsPath =
        SysfsPath::new("/sys/class/power_supply/battery/bd_trickle_rate");

    pub const DD_STATE: SysfsPath = SysfsPath::new("/sys/devices/platform/google,charger/dd_state");

    pub const DD_SETTINGS: SysfsPath =
        SysfsPath::new("/sys/devices/platform/google,charger/dd_settings");

    pub const CHARGING_STATUS: SysfsPath = SysfsPath::with_alt(
        "/sys/devices/platform/google,charger/charging_status",
        "/sys/devices/platform/soc/soc:google,charger/charging_status",
    );
    pub const CHARGING_TYPE: SysfsPath = SysfsPath::with_alt(
        "/sys/devices/platform/google,charger/charging_type",
        "/sys/devices/platform/soc/soc:google,charger/charging_type",
    );
    pub const CHARGING_SPEED: SysfsPath =
        SysfsPath::new("/sys/class/power_supply/battery/charging_speed");

    pub const CHARGE_DEADLINE: SysfsPath =
        SysfsPath::new("/sys/class/power_supply/battery/charge_deadline");
    pub const CHARGE_STAGE: SysfsPath =
        SysfsPath::new("/sys/class/power_supply/battery/charge_stage");
    pub const CHARGE_LIMIT: SysfsPath =
        SysfsPath::new("/sys/class/power_supply/battery/charge_limit");

    pub const HEALTH_INDEX: SysfsPath =
        SysfsPath::new("/sys/class/power_supply/battery/health_index");
    pub const HEALTH_STATUS: SysfsPath =
        SysfsPath::new("/sys/class/power_supply/battery/health_status");
    pub const HEALTH_CAPACITY_INDEX: SysfsPath =
        SysfsPath::new("/sys/class/power_supply/battery/health_capacity_index");
    pub const HEALTH_IMPEDANCE_INDEX: SysfsPath =
        SysfsPath::new("/sys/class/power_supply/battery/health_impedance_index");
    pub const HEALTH_INDEX_STATS: SysfsPath =
        SysfsPath::new("/sys/class/power_supply/battery/health_index_stats");
    pub const HEALTH_SET_CAL_MODE: SysfsPath =
        SysfsPath::new("/sys/class/power_supply/battery/health_set_cal_mode");
    pub const HEALTH_GET_CAL_STATE: SysfsPath =
        SysfsPath::new("/sys/class/power_supply/battery/health_get_cal_state");
}

use vendor_benzeneos_battery::aidl::vendor::benzeneos::battery::IBattery::Feature::Feature;

/// Property IDs used with getStringProperty/setStringProperty.
/// These are per-feature and based on reverse engineering of Google's HAL.
pub mod property {
    // Common properties across features
    pub const ENABLE: i32 = 0;
    pub const DRY_RUN: i32 = 1;
    pub const STATE: i32 = 18;
    pub const PROFILE: i32 = 32;

    // CHARGE_DEADLINE properties
    pub const DEADLINE_DRYRUN: i32 = 1;
    pub const HEALTH_SAFETY_MARGIN: i32 = 12;

    // TRICKLE_DEFEND properties
    pub const TRICKLE_RATE: i32 = 3;
    pub const TRICKLE_CNT: i32 = 8;
    pub const TRICKLE_RESET_SEC: i32 = 12;
    pub const TRICKLE_RECHARGE_SOC: i32 = 15;
    pub const TRICKLE_VERSION: i32 = 33;
    pub const TRICKLE_CNT_THR: i32 = 50;

    // WIRELESS properties
    pub const MITIGATE_THRESHOLD: i32 = 5;

    // CPM properties
    pub const DC_CTL: i32 = 2;
    pub const THERMAL_DC_FAN_ALARM: i32 = 19;
    pub const THERMAL_MDIS_FAN_ALARM: i32 = 20;

    // AACR properties
    pub const CYCLE_GRACE: i32 = 8;
    pub const CYCLE_MAX: i32 = 21;
    pub const MIN_CAPACITY_RATE: i32 = 24;
    pub const CLIFF_CAPACITY_RATE: i32 = 27;

    // HEALTH properties
    pub const ALGO: i32 = 2;
    pub const TREND_POINTS: i32 = 23;
    pub const LOW_BOUNDARY: i32 = 24;
    pub const CSI_STATS: i32 = 25;

    // FW_UPDATE properties
    pub const UPDATE_FIRMWARE: i32 = 26;

    // CHARGE_LIMIT properties
    pub const CHARGE_TO_LIMIT: i32 = 5;
    pub const FORCE_FCR_UPDATE: i32 = 39;
    pub const BYPASS_FCN_DELTA: i32 = 41;
    pub const BYPASS_CYCLE_DELTA: i32 = 42;
    pub const BYPASS_MODE: i32 = 43;

    // AAFV properties
    pub const APPLY_MAX: i32 = 28;
    pub const MAX_OFFSET: i32 = 29;
    pub const CLIFF_CYCLE: i32 = 30;
    pub const CLIFF_OFFSET: i32 = 31;
    pub const AAFV_CONFIG: i32 = 38;

    // AACT properties
    pub const CV_LIMITS: i32 = 34;
    pub const TEMP_LIMITS: i32 = 35;
    pub const CHG_ECC: i32 = 36;

    // AACP properties
    pub const VERSION: i32 = 33;
    pub const OPT_OUT: i32 = 37;
    pub const OPT_OUT_CUTOFF: i32 = 44;
}

/// Get sysfs path for a feature/property combination.
/// Based on reverse engineering of vendor.google.google_battery-service.
pub fn get_property_sysfs(feature: Feature, prop: i32) -> Option<&'static str> {
    use property::*;

    match feature {
        Feature::CHARGE_DEADLINE => match prop {
            DEADLINE_DRYRUN => Some("/sys/class/power_supply/battery/charge_deadline_dryrun"),
            HEALTH_SAFETY_MARGIN => Some("/sys/class/power_supply/battery/health_safety_margin"),
            _ => None,
        },
        Feature::TRICKLE_DEFEND => match prop {
            ENABLE => Some("/sys/class/power_supply/battery/bd_trickle_enable"),
            DRY_RUN => Some("/sys/class/power_supply/battery/bd_trickle_dry_run"),
            TRICKLE_RATE => Some("/sys/class/power_supply/battery/bd_trickle_rate"),
            TRICKLE_CNT => Some("/sys/class/power_supply/battery/bd_trickle_cnt"),
            TRICKLE_RESET_SEC => Some("/sys/class/power_supply/battery/bd_trickle_reset_sec"),
            TRICKLE_RECHARGE_SOC => Some("/sys/class/power_supply/battery/bd_trickle_recharge_soc"),
            TRICKLE_VERSION => Some("/sys/class/power_supply/battery/bd_trickle_version"),
            TRICKLE_CNT_THR => Some("/sys/class/power_supply/battery/bd_trickle_cnt_thr"),
            _ => None,
        },
        Feature::WIRELESS => match prop {
            MITIGATE_THRESHOLD => Some("/sys/class/power_supply/wireless/device/mitigate_threshold"),
            _ => None,
        },
        Feature::CPM => match prop {
            DC_CTL => Some("/sys/devices/platform/google,cpm/dc_ctl"),
            THERMAL_DC_FAN_ALARM => Some("/sys/devices/platform/google,charger/thermal_dc_fan_alarm"),
            THERMAL_MDIS_FAN_ALARM => Some("/sys/devices/platform/google,cpm/thermal_mdis_fan_alarm"),
            _ => None,
        },
        Feature::AACR => match prop {
            CYCLE_GRACE => Some("/sys/class/power_supply/battery/aacr_cycle_grace"),
            STATE => Some("/sys/class/power_supply/battery/aacr_state"),
            CYCLE_MAX => Some("/sys/class/power_supply/battery/aacr_cycle_max"),
            MIN_CAPACITY_RATE => Some("/sys/class/power_supply/battery/aacr_min_capacity_rate"),
            CLIFF_CAPACITY_RATE => Some("/sys/class/power_supply/battery/aacr_cliff_capacity_rate"),
            PROFILE => Some("/sys/class/power_supply/battery/aacr_profile"),
            _ => None,
        },
        Feature::HEALTH => match prop {
            ALGO => Some("/sys/class/power_supply/battery/health_algo"),
            TREND_POINTS => Some("/sys/class/power_supply/battery/health_set_trend_points"),
            LOW_BOUNDARY => Some("/sys/class/power_supply/battery/health_set_low_boundary"),
            _ => None,
        },
        Feature::CSI_STATS => match prop {
            CSI_STATS => Some("/sys/class/power_supply/battery/csi_stats"),
            _ => None,
        },
        Feature::FW_UPDATE => match prop {
            ENABLE => Some("/sys/devices/platform/maxim,max77779fwu/enable_update"),
            UPDATE_FIRMWARE => Some("/sys/devices/platform/maxim,max77779fwu/update_firmware"),
            _ => None,
        },
        Feature::CHARGE_LIMIT => match prop {
            CHARGE_TO_LIMIT => Some("/sys/class/power_supply/battery/charge_to_limit"),
            FORCE_FCR_UPDATE => Some("/sys/class/power_supply/battery/force_fcr_update_ops"),
            BYPASS_FCN_DELTA => Some("/sys/class/power_supply/maxfg/bypass_chargelimit_fcn_delta"),
            BYPASS_CYCLE_DELTA => Some("/sys/class/power_supply/maxfg/bypass_chargelimit_cycle_delta"),
            BYPASS_MODE => Some("/sys/class/power_supply/maxfg/bypass_chargelimit_mode"),
            _ => None,
        },
        Feature::FG_CYCLE => match prop {
            ENABLE => Some("/sys/class/power_supply/maxfg/fix_cycle_count"),
            _ => None,
        },
        Feature::AAFV => match prop {
            STATE => Some("/sys/class/power_supply/battery/aafv_state"),
            APPLY_MAX => Some("/sys/class/power_supply/battery/aafv_apply_max"),
            MAX_OFFSET => Some("/sys/class/power_supply/battery/aafv_max_offset"),
            CLIFF_CYCLE => Some("/sys/class/power_supply/battery/aafv_cliff_cycle"),
            CLIFF_OFFSET => Some("/sys/class/power_supply/battery/aafv_cliff_offset"),
            PROFILE => Some("/sys/class/power_supply/battery/aafv_profile"),
            AAFV_CONFIG => Some("/sys/class/power_supply/maxfg/aafv_config"),
            _ => None,
        },
        Feature::AACT => match prop {
            STATE => Some("/sys/class/power_supply/battery/aact_state"),
            PROFILE => Some("/sys/class/power_supply/battery/aact_profile"),
            CV_LIMITS => Some("/sys/class/power_supply/battery/aact_cv_limits"),
            TEMP_LIMITS => Some("/sys/class/power_supply/battery/aact_temp_limits"),
            CHG_ECC => Some("/sys/class/power_supply/battery/aact_chg_ecc"),
            _ => None,
        },
        Feature::AACP => match prop {
            VERSION => Some("/sys/class/power_supply/battery/aacp_version"),
            OPT_OUT => Some("/sys/class/power_supply/battery/aacp_opt_out"),
            OPT_OUT_CUTOFF => Some("/sys/class/power_supply/battery/aacp_opt_out_cutoff_cycles"),
            _ => None,
        },
        Feature::WLC_FW => match prop {
            ENABLE => Some("/sys/class/power_supply/wireless/device/rx_fwupdate"),
            UPDATE_FIRMWARE => Some("/sys/class/power_supply/wireless/device/rx_vertag"),
            _ => None,
        },
        Feature::QI22 => match prop {
            ENABLE => Some("/sys/class/power_supply/wireless/device/qi22_en_gpio"),
            _ => None,
        },
        Feature::AACC => match prop {
            PROFILE => Some("/sys/class/power_supply/battery/aacc_chg_profile"),
            _ => None,
        },
        _ => None,
    }
}
