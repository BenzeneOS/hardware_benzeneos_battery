// Copyright (C) 2025 Amaan Qureshi <contact@amaanq.com>
// SPDX-License-Identifier: Apache-2.0

use std::sync::Mutex;

use binder::{
   BinderFeatures,
   ExceptionCode,
   Interface,
   Result,
   Status,
   Strong,
};
use log::{error, info};
use vendor_benzeneos_battery::aidl::vendor::benzeneos::battery::IBattery::{
   BnBattery,
   IBattery,
   CalibrationMode::CalibrationMode,
   CalibrationState::CalibrationState,
   ChargingPolicy::ChargingPolicy,
   ChargingStage::ChargingStage,
   ChargingStatus::ChargingStatus,
   ChargingType::ChargingType,
   DefenderType::DefenderType,
   DockDefendStatus::DockDefendStatus,
   Feature::Feature,
   HealthAlgo::HealthAlgo,
   HealthStats::HealthStats,
   HealthStatus::HealthStatus,
};
// Feature is re-exported from sysfs module for get_property_sysfs

use crate::sysfs::{
   self,
   paths,
};

const DEFAULT_STOP: i32 = 80;
const DEFAULT_START: i32 = 70;

fn sysfs_err(e: sysfs::Error, ctx: &str) -> Status {
   let msg = format!("{ctx}: {e}");
   error!("{msg}");
   Status::new_service_specific_error_str(1, Some(&msg))
}

fn bad_arg(msg: &str) -> Status {
   Status::new_exception_str(ExceptionCode::ILLEGAL_ARGUMENT, Some(msg))
}

fn unsupported(msg: &str) -> Status {
   Status::new_exception_str(ExceptionCode::UNSUPPORTED_OPERATION, Some(msg))
}

struct Limits {
   stop:  i32,
   start: i32,
}

pub struct BatteryService {
   limits: Mutex<Limits>,
}

impl Interface for BatteryService {}

impl BatteryService {
   pub fn new() -> Self {
      info!("Creating BatteryService");
      Self {
         limits: Mutex::new(Limits {
            stop:  DEFAULT_STOP,
            start: DEFAULT_START,
         }),
      }
   }

   fn apply_levels(&self, stop: i32, start: i32) -> Result<()> {
      // Kernel requires: new_stop > current_start AND new_start < current_stop
      // Also: kernel rejects writes if value == current value
      let current_start = paths::CHARGE_START_LEVEL.read_int_or(0);
      let current_stop = paths::CHARGE_STOP_LEVEL.read_int_or(100);

      // Lower start first if new_stop <= current_start
      let prep_start = if stop <= current_start && start != current_start {
         // Use target start or minimum, whichever allows the stop write
         let prep = start.min(stop - 1);
         if prep != current_start && paths::CHARGE_START_LEVEL.exists() {
            paths::CHARGE_START_LEVEL
               .write_int(prep)
               .map_err(|e| sysfs_err(e, "write start (prep)"))?;
         }
         Some(prep)
      } else {
         None
      };

      if stop != current_stop && paths::CHARGE_STOP_LEVEL.exists() {
         paths::CHARGE_STOP_LEVEL
            .write_int(stop)
            .map_err(|e| sysfs_err(e, "write stop"))?;
      }

      // Only write start if different from what we already wrote (or didn't write)
      let already_set = prep_start == Some(start);
      if !already_set && start != current_start && paths::CHARGE_START_LEVEL.exists() {
         paths::CHARGE_START_LEVEL
            .write_int(start)
            .map_err(|e| sysfs_err(e, "write start"))?;
      }

      info!("Set charge levels: {stop}/{start}");
      Ok(())
   }

   fn parse_health_stats(&self, algo: i32) -> Option<HealthStats> {
      let content = paths::HEALTH_INDEX_STATS.read_string().ok()?;
      for line in content.lines() {
         let (a, rest) = line.split_once(':')?;
         if a.trim().parse::<i32>().ok()? != algo {
            continue;
         }
         let v = rest
            .split(|c: char| c == ',' || c.is_whitespace())
            .filter_map(|s| s.parse().ok())
            .collect::<Vec<_>>();

         if v.len() >= 10 {
            return Some(HealthStats {
               algo,
               healthIndex: v[0],
               capacityFcc: v[1],
               capacityRaw: v[2],
               capacityDesign: v[3],
               impedanceRaw: v[4],
               impedanceAvg: v[5],
               impedanceDesign: v[6],
               cycleCount: v[7],
               cycleCountDesign: v[8],
               tempBucket: v[9],
            });
         }
      }
      None
   }
}

impl IBattery for BatteryService {
   fn setChargingPolicy(&self, policy: ChargingPolicy) -> Result<()> {
      let val = match policy {
         ChargingPolicy::DEFAULT => 1,
         ChargingPolicy::LONGLIFE | ChargingPolicy::CUSTOM => 2,
         ChargingPolicy::ADAPTIVE => 3,
         _ => return Err(bad_arg("invalid policy")),
      };
      if !paths::CHARGING_POLICY.exists() {
         return Ok(());
      }
      paths::CHARGING_POLICY
         .write_int(val)
         .map_err(|e| sysfs_err(e, "write policy"))?;
      if policy == ChargingPolicy::CUSTOM {
         let l = self.limits.lock().unwrap();
         self.apply_levels(l.stop, l.start)?;
      }
      Ok(())
   }

   fn getChargingPolicy(&self) -> Result<ChargingPolicy> {
      Ok(ChargingPolicy(paths::CHARGING_POLICY.read_int_or(1)))
   }

   fn setChargeLimit(&self, stop: i32, start: i32) -> Result<()> {
      if !(50..=100).contains(&stop) {
         return Err(bad_arg("stop must be 50-100"));
      }
      if stop - start < 5 {
         return Err(bad_arg("gap must be >= 5"));
      }
      {
         let mut l = self.limits.lock().unwrap();
         l.stop = stop;
         l.start = start;
      }
      if paths::CHARGING_POLICY.read_int_or(1) == 2 {
         self.apply_levels(stop, start)?;
      }
      Ok(())
   }

   fn getChargeLimit(&self) -> Result<Vec<i32>> {
      let l = self.limits.lock().unwrap();
      Ok(vec![
         paths::CHARGE_STOP_LEVEL.read_int_or(l.stop),
         paths::CHARGE_START_LEVEL.read_int_or(l.start),
      ])
   }

   fn setEnable(&self, feature: Feature, enabled: bool) -> Result<()> {
      match feature {
         Feature::DOCK_DEFEND if paths::DD_SETTINGS.exists() => {
            paths::DD_SETTINGS
               .write_string(if enabled { "B2" } else { "1M" })
               .map_err(|e| sysfs_err(e, "dock defend"))
         },
         _ => Err(unsupported("feature not controllable")),
      }
   }

   fn clearBatteryDefenders(&self, kind: DefenderType) -> Result<()> {
      let clear_temp = || {
         paths::BD_CLEAR
            .write_string("B2")
            .map_err(|e| sysfs_err(e, "clear temp"))
      };
      let clear_trickle = || {
         paths::BD_TRICKLE_RESET_SEC
            .write_int(0)
            .map_err(|e| sysfs_err(e, "clear trickle"))
      };
      let clear_dwell = || {
         paths::BD_TRICKLE_RATE
            .write_int(0)
            .map_err(|e| sysfs_err(e, "clear dwell"))
      };
      let clear_dock = || -> Result<()> {
         if paths::DD_STATE.read_int_or(0) == 1 && paths::DD_SETTINGS.exists() {
            paths::DD_SETTINGS
               .write_string("02")
               .map_err(|e| sysfs_err(e, "clear dock"))?;
         }
         Ok(())
      };
      match kind {
         DefenderType::ALL => {
            clear_temp()?;
            clear_trickle()?;
            clear_dwell()?;
            clear_dock()?;
         },
         DefenderType::TEMP => clear_temp()?,
         DefenderType::TRICKLE => clear_trickle()?,
         DefenderType::DWELL => clear_dwell()?,
         DefenderType::DOCK => clear_dock()?,
         _ => {},
      }
      Ok(())
   }

   fn getStringProperty(&self, feature: Feature, prop: i32) -> Result<String> {
      if prop >= 51 {
         return Err(bad_arg("property out of range"));
      }
      match sysfs::get_property_sysfs(feature, prop) {
         Some(path) if std::path::Path::new(path).exists() => {
            sysfs::read_string(path).map_err(|e| sysfs_err(e, "getStringProperty"))
         }
         _ => Ok(String::new()),
      }
   }

   fn setStringProperty(&self, feature: Feature, prop: i32, value: &str) -> Result<()> {
      if prop >= 51 {
         return Err(bad_arg("property out of range"));
      }
      match sysfs::get_property_sysfs(feature, prop) {
         Some(path) if std::path::Path::new(path).exists() => {
            sysfs::write_string(path, value).map_err(|e| sysfs_err(e, "setStringProperty"))
         }
         _ => Ok(()), // Silently succeed if path not configured (matches Google behavior)
      }
   }

   fn getChargingStatus(&self) -> Result<ChargingStatus> {
      Ok(ChargingStatus(paths::CHARGING_STATUS.read_int_or(-1)))
   }

   fn getChargingType(&self) -> Result<ChargingType> {
      Ok(ChargingType(paths::CHARGING_TYPE.read_int_or(-1)))
   }

   fn getChargingSpeed(&self) -> Result<i32> {
      Ok(paths::CHARGING_SPEED.read_int_or(0))
   }

   fn setChargingDeadline(&self, deadline: i32) -> Result<()> {
      paths::CHARGE_DEADLINE
         .write_int(deadline)
         .map_err(|e| sysfs_err(e, "write deadline"))
   }

   fn getChargingStageAndDeadline(&self) -> Result<ChargingStage> {
      Ok(ChargingStage {
         stage:    paths::CHARGE_STAGE.read_string().unwrap_or_default(),
         deadline: paths::CHARGE_DEADLINE.read_int_or(0),
      })
   }

   fn getHealthIndex(&self) -> Result<i32> {
      Ok(paths::HEALTH_INDEX.read_int_or(100))
   }

   fn getHealthStatus(&self) -> Result<HealthStatus> {
      Ok(HealthStatus(paths::HEALTH_STATUS.read_int_or(0)))
   }

   fn getHealthCapacityIndex(&self) -> Result<i32> {
      paths::HEALTH_CAPACITY_INDEX
         .read_int()
         .map_err(|e| sysfs_err(e, "capacity index"))
   }

   fn getHealthImpedanceIndex(&self) -> Result<i32> {
      paths::HEALTH_IMPEDANCE_INDEX
         .read_int()
         .map_err(|e| sysfs_err(e, "impedance index"))
   }

   fn getHealthStats(&self, algo: HealthAlgo) -> Result<HealthStats> {
      let algo_int = match algo {
         HealthAlgo::GOOGLE => 1,
         HealthAlgo::MAXIM => 2,
         _ => 0,
      };
      Ok(self.parse_health_stats(algo_int).unwrap_or_default())
   }

   fn setHealthAlwaysOn(&self, value: i32) -> Result<()> {
      paths::CHARGE_LIMIT
         .write_int(value)
         .map_err(|e| sysfs_err(e, "write charge_limit"))
   }

   fn scheduleCalibration(&self, mode: CalibrationMode) -> Result<()> {
      if !paths::HEALTH_SET_CAL_MODE.exists() {
         return Ok(());
      }
      let v = match mode {
         CalibrationMode::DISABLED => 0,
         CalibrationMode::ENABLED => 1,
         CalibrationMode::FORCED => 2,
         _ => return Err(bad_arg("invalid mode")),
      };
      paths::HEALTH_SET_CAL_MODE
         .write_int(v)
         .map_err(|e| sysfs_err(e, "calibration"))
   }

   fn getCalibrationState(&self) -> Result<CalibrationState> {
      Ok(CalibrationState(paths::HEALTH_GET_CAL_STATE.read_int_or(0)))
   }

   fn getDockDefendStatus(&self) -> Result<DockDefendStatus> {
      if !paths::DD_STATE.exists() {
         return Ok(DockDefendStatus::DISABLED);
      }
      let state = paths::DD_STATE.read_int_or(-1);
      let settings = paths::DD_SETTINGS.read_int_or(-1);
      Ok(match (state, settings) {
         (-1, _) => DockDefendStatus::DISABLED,
         (_, -1) => DockDefendStatus::ERROR,
         (1, 1) => DockDefendStatus::ACTIVE,
         _ => DockDefendStatus::INACTIVE,
      })
   }

   fn getAdapterId(&self) -> Result<i32> {
      Err(unsupported("getAdapterId not supported"))
   }
}

pub fn register() -> Result<Strong<dyn IBattery>> {
   let svc = BatteryService::new();
   let binder = BnBattery::new_binder(svc, BinderFeatures::default());
   binder::add_service("vendor.benzeneos.battery.IBattery/default", binder.as_binder())
      .map_err(|_| Status::new_exception_str(ExceptionCode::SERVICE_SPECIFIC, Some("register failed")))?;
   info!("Registered vendor.benzeneos.battery.IBattery/default");
   Ok(binder)
}
