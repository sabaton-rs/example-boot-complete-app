use anyhow::Result;
use sabaton_hal::bootloader::BootControl;
use libcore::bootloader::bootcontrol::BootControlImpl;

pub fn set_successful_boot()->Result<()> {
    let mut bootctrl = BootControlImpl::create()?;
    bootctrl.set_boot_successful()?;
    Ok(())
}

