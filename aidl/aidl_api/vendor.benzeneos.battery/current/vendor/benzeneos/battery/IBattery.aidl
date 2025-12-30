/*
 * Copyright (C) 2025 Amaan Qureshi <contact@amaanq.com>
 * SPDX-License-Identifier: Apache-2.0
 *
 * Benzene Battery HAL
 */
///////////////////////////////////////////////////////////////////////////////
// THIS FILE IS IMMUTABLE. DO NOT EDIT IN ANY CASE.                          //
///////////////////////////////////////////////////////////////////////////////

// This file is a snapshot of an AIDL file. Do not edit it manually. There are
// two cases:
// 1). this is a frozen version file - do not edit this in any case.
// 2). this is a 'current' file. If you make a backwards compatible change to
//     the interface (from the latest frozen version), the build system will
//     prompt you to update this file with `m <name>-update-api`.
//
// You must not make a backward incompatible change to any AIDL file built
// with the aidl_interface module type with versions property set. The module
// type is used to build AIDL files in a way that they can be used across
// independently updatable components of the system. If a device is shipped
// with such a backward incompatible change, it has a high risk of breaking
// later when a module using the interface is updated, e.g., Mainline modules.

package vendor.benzeneos.battery;
@VintfStability
interface IBattery {
  void setChargingPolicy(vendor.benzeneos.battery.IBattery.ChargingPolicy policy);
  vendor.benzeneos.battery.IBattery.ChargingPolicy getChargingPolicy();
  void setChargeLimit(int stopLevel, int startLevel);
  int[] getChargeLimit();
  void setEnable(vendor.benzeneos.battery.IBattery.Feature feature, boolean enabled);
  void clearBatteryDefenders(vendor.benzeneos.battery.IBattery.DefenderType type);
  String getStringProperty(vendor.benzeneos.battery.IBattery.Feature feature, int prop);
  void setStringProperty(vendor.benzeneos.battery.IBattery.Feature feature, int prop, String value);
  vendor.benzeneos.battery.IBattery.ChargingStatus getChargingStatus();
  vendor.benzeneos.battery.IBattery.ChargingType getChargingType();
  int getChargingSpeed();
  void setChargingDeadline(int deadline);
  vendor.benzeneos.battery.IBattery.ChargingStage getChargingStageAndDeadline();
  int getHealthIndex();
  vendor.benzeneos.battery.IBattery.HealthStatus getHealthStatus();
  int getHealthCapacityIndex();
  int getHealthImpedanceIndex();
  vendor.benzeneos.battery.IBattery.HealthStats getHealthStats(vendor.benzeneos.battery.IBattery.HealthAlgo algo);
  void setHealthAlwaysOn(int value);
  void scheduleCalibration(vendor.benzeneos.battery.IBattery.CalibrationMode mode);
  vendor.benzeneos.battery.IBattery.CalibrationState getCalibrationState();
  vendor.benzeneos.battery.IBattery.DockDefendStatus getDockDefendStatus();
  int getAdapterId();
  @Backing(type="int")
  enum ChargingPolicy {
    DEFAULT = 1,
    LONGLIFE = 2,
    ADAPTIVE = 3,
    CUSTOM = 4,
  }
  @Backing(type="int")
  enum DefenderType {
    ALL = 0,
    TEMP = 1,
    TRICKLE = 2,
    DWELL = 3,
    DOCK = 4,
  }
  @Backing(type="int")
  enum Feature {
    CHARGE_DEADLINE = 0,
    BATTERY_DEFENDER = 1,
    TRICKLE_DEFEND = 2,
    CSI = 3,
    WIRELESS = 4,
    CPM = 5,
    AACR = 6,
    DOCK_DEFEND = 7,
    HEALTH = 8,
    CSI_STATS = 9,
    FW_UPDATE = 11,
    CHARGE_LIMIT = 12,
    FG_CYCLE = 13,
    AAFV = 14,
    AACT = 15,
    AACP = 16,
    WLC_FW = 17,
    TX_FW = 18,
    QI22 = 19,
    RELAXATION = 20,
    AACC = 21,
  }
  @Backing(type="int")
  enum ChargingStatus {
    UNKNOWN = (-1) /* -1 */,
  }
  @Backing(type="int")
  enum ChargingType {
    UNKNOWN = (-1) /* -1 */,
  }
  @Backing(type="int")
  enum HealthAlgo {
    UNKNOWN = 0,
    GOOGLE = 1,
    MAXIM = 2,
  }
  @Backing(type="int")
  enum HealthStatus {
    UNKNOWN = 0,
  }
  @Backing(type="int")
  enum CalibrationMode {
    DISABLED = 0,
    ENABLED = 1,
    FORCED = 2,
  }
  @Backing(type="int")
  enum CalibrationState {
    NOT_CALIBRATING = 0,
  }
  @Backing(type="int")
  enum DockDefendStatus {
    DISABLED = (-2) /* -2 */,
    ERROR = (-1) /* -1 */,
    INACTIVE = 0,
    ACTIVE = 1,
  }
  parcelable ChargingStage {
    String stage;
    int deadline;
  }
  parcelable HealthStats {
    int algo;
    int healthIndex;
    int capacityFcc;
    int capacityRaw;
    int capacityDesign;
    int impedanceRaw;
    int impedanceAvg;
    int impedanceDesign;
    int cycleCount;
    int cycleCountDesign;
    int tempBucket;
  }
}
