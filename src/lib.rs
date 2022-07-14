use anyhow::Result;
use sabaton_hal::bootloader::BootControl;
use libcore::bootloader::bootcontrol::BootControlImpl;

pub fn set_successful_boot()->Result<()> {
    let mut bootctrl = BootControlImpl::create()?;
    let active_slot=bootctrl.active_slot()?;
    print!("Current active slot is {}",active_slot);
    bootctrl.set_boot_successful()?;
    Ok(())
}

