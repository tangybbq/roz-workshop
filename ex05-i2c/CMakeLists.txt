# Copyright (c) 2026 Linaro, LTD
# SPDX-License-Identifier: Apache-2.0

cmake_minimum_required(VERSION 3.20.0)

find_package(Zephyr REQUIRED HINTS $ENV{ZEPHYR_BASE})

project(ex05_i2c)
rust_cargo_application()
