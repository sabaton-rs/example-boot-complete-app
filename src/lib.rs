use anyhow::Result;
use libcore::bootloader::message::{BootloaderMessageAB,BootloaderControl,BootloaderMessage};
use std::{
    convert::TryFrom,
    ffi::{CStr, CString},
    fmt::Display,
    io::Write,
    mem::MaybeUninit,
    os::unix::prelude::FileExt,
};
use libcore::{
    mount::early_partitions::{ensure_mount_device_is_created, MISC_PARTITION_NAME},
    uevent::create_and_bind_netlink_socket,
};
  /// Read the contents of the MISC partition and create a BootloaderMessageAB structure from it
  pub fn create_from_misc_partition() -> Result<BootloaderMessageAB, std::io::Error> {
    let mut nl_socket = create_and_bind_netlink_socket()?;

    let misc_partition_name = CString::new(MISC_PARTITION_NAME)?;
    ensure_mount_device_is_created(&misc_partition_name, &mut nl_socket)?;

    let misc_partition_handle = std::fs::OpenOptions::new()
        .read(true)
        .open(MISC_PARTITION_NAME)?;

    // create an uninitialized BootloaderMessageAB structure
    let mut bootloader_message_ab: MaybeUninit<BootloaderMessageAB> = MaybeUninit::uninit();
    let as_ptr = bootloader_message_ab.as_mut_ptr() as *mut u8;
    let slice = unsafe {
        std::slice::from_raw_parts_mut(
            as_ptr as *mut u8,
            std::mem::size_of::<BootloaderMessageAB>(),
        )
    };
    assert_eq!(slice.len(), 4096);

    misc_partition_handle.read_exact_at(slice, 0)?;
    unsafe { Ok(bootloader_message_ab.assume_init()) }
}

pub fn set_successful_boot()->Result<()> {
    //let bytes_slice = include_bytes!("./testdata/bolomessage.dat");
    //let bolo_message_ab: &BootloaderMessageAB = bytes_slice.as_slice().try_into().unwrap();
    let bolo_message_ab=create_from_misc_partition()?;
    let ctrl = bolo_message_ab.get_bootloader_control().unwrap();

    assert_eq!(ctrl.nb_slot(), 2);
    assert_eq!(ctrl.recovery_tries_remaining(), 0);


    let mut copy = bolo_message_ab.clone();
    let original_as_slice = copy.as_slice();
    

    for s in ctrl.slot_info.as_ref().iter() {
        println!("SlotMetadata before settig successful boot:{:?}", s.to_string());
    }
    let control = copy.get_bootloader_control_mut().unwrap();

    let current_suffix = control.slot_suffix().unwrap();
    println!("Current slot is {:?}", current_suffix);
    let mut active_slot=0;
    match  current_suffix{
        a=>active_slot=0,
        b=>active_slot=1,
        _=>active_slot=0,
        
    }
    control.slot_info[active_slot].set_successful_boot(1);
    for s in control.slot_info.as_ref().iter() {
        println!("SlotMetadata after settig successful boot::{:?}", s.to_string());
    }


    let slice = copy.as_slice();
    assert_eq!(slice.len(), 4096);
    Ok(())
}

