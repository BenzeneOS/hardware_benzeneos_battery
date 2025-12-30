/*
 * Copyright (C) 2025 Amaan Qureshi <contact@amaanq.com>
 * SPDX-License-Identifier: Apache-2.0
 *
 * Benzene Battery HAL
 */

package vendor.benzeneos.battery;

@VintfStability
interface IBattery {
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
        UNKNOWN = -1,
    }

    @Backing(type="int")
    enum ChargingType {
        UNKNOWN = -1,
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
        DISABLED = -2,
        ERROR = -1,
        INACTIVE = 0,
        ACTIVE = 1,
    }

    // ============ Parcelables ============

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

    // ============ Methods ============

    void setChargingPolicy(ChargingPolicy policy);
    ChargingPolicy getChargingPolicy();

    void setChargeLimit(int stopLevel, int startLevel);
    int[] getChargeLimit();

    void setEnable(Feature feature, boolean enabled);
    void clearBatteryDefenders(DefenderType type);

    // ============ Property Access ============

    String getStringProperty(Feature feature, int prop);
    void setStringProperty(Feature feature, int prop, String value);

    // ============ Charging Info ============

    ChargingStatus getChargingStatus();
    ChargingType getChargingType();
    int getChargingSpeed();

    // ============ Adaptive Charging ============

    void setChargingDeadline(int deadline);
    ChargingStage getChargingStageAndDeadline();

    // ============ Health ============

    int getHealthIndex();
    HealthStatus getHealthStatus();
    int getHealthCapacityIndex();
    int getHealthImpedanceIndex();
    HealthStats getHealthStats(HealthAlgo algo);
    void setHealthAlwaysOn(int value);

    // ============ Calibration ============

    void scheduleCalibration(CalibrationMode mode);
    CalibrationState getCalibrationState();

    // ============ Dock Defend ============

    DockDefendStatus getDockDefendStatus();

    // ============ Unsupported ============

    int getAdapterId();
}
