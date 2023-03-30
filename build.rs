//! build.rs

use imxrt_rt::{Family, RuntimeBuilder};

fn main() {
    // The iMXRT1064EVK has 64 Mbit QSPI flash, 64 / 8 = 8 MB
    RuntimeBuilder::from_flexspi(Family::Imxrt1064, 8 * 1024 * 1024)
        .build()
        .unwrap();
}
