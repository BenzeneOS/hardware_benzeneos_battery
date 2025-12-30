# Copyright (C) 2025 Amaan Qureshi <contact@amaanq.com>
# SPDX-License-Identifier: Apache-2.0

PRODUCT_PACKAGES += vendor.benzeneos.battery-service

DEVICE_PRODUCT_COMPATIBILITY_MATRIX_FILE += \
    hardware/benzeneos/battery/compatibility_matrix.xml

# SELinux policy
BOARD_VENDOR_SEPOLICY_DIRS += hardware/benzeneos/battery/sepolicy

# Pixel 8+ runs SystemUI as systemui_app; only granted where the device defines the type.
ifneq (,$(shell grep -ls '^type systemui_app\b' vendor/google_devices/$(TARGET_PRODUCT)/sepolicy/system_ext/public/types.te 2>/dev/null))
BOARD_VENDOR_SEPOLICY_DIRS += hardware/benzeneos/battery/sepolicy_systemui
endif
