# i.MX RT1064 Rust Template

A very basic application targeted at the i.MX RT1064.
The application resembles [the HAL example discussed in the imxrt-rs book](https://imxrt-rs.github.io/book/ecosystem_walkthrough/hal.html), ported to [NXP's i.MX RT1064 EVK](https://www.nxp.com/design/development-boards/i-mx-evaluation-and-development-boards/i-mx-rt1064-evaluation-kit:MIMXRT1064-EVK).

## Prerequisites

Set up your toolchain as described [here](https://imxrt-rs.github.io/book/toolchain.html).
To build the application, no additional prerequisites are required.

To follow the flashing procedure described below, an installation of [this fork of openOCD is required](https://github.com/5inf/openocd).
Make sure to get yourself the i.MX RT1064 flashdriver from [this repository](https://github.com/sysprogs/flash_drivers).

## Building

Building is a matter of

```bash
cargo build
```

## Flashing

As of 2023-03-31, I did not get [`cargo embed`](https://probe.rs/docs/tools/cargo-embed/) to successfullly program the target.
For this reason, I switched to openOCD instead.

In the following, let `$firmware` and `$openocd_dir` denote the binary file build above and openOCD's install directory respectively.

Flashing is a matter of

```bash
$openocd_dir/bin/imxrt-openocd -f $openocd_dir/share/openocd/scripts/interface/cmsis-dap.cfg -f $openocd_dir/share/openocd/scripts/target/imxrt.cfg -c "program $firmware 0x0 verify"
```
