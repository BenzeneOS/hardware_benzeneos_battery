# Copyright (C) 2025 Amaan Qureshi <contact@amaanq.com>
# SPDX-License-Identifier: Apache-2.0

PRODUCT_PACKAGES += vendor.benzeneos.battery-service

DEVICE_PRODUCT_COMPATIBILITY_MATRIX_FILE += \
    hardware/benzeneos/battery/compatibility_matrix.xml

# SELinux policy
BOARD_VENDOR_SEPOLICY_DIRS += hardware/benzeneos/battery/sepolicy
