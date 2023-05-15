//! Microsoft OS USB descriptors for usb-device
//!
//! Implementation of Microsoft OS USB descriptors for [usb-device](https://crates.io/crates/usb-device).
//! Currently only the new [Microsoft OS 2.0 Descriptors](https://learn.microsoft.com/en-us/windows-hardware/drivers/usbcon/microsoft-os-2-0-descriptors-specification)
//! standard is supported. Version 1.0 may be added in the future if needed.
//!
//! This crate provides class `MsOsUsbClass` that is responsible for sending MS OS USB descriptors
//! and appropriate BOS capabilities. It is meant to be configured using `const` structures that
//! describe the descriptors, and `const fn` methods that generate raw descriptor data, e.g. for WinUSB:
//!
//! ```rust
//! use usbd_microsoft_os::{os_20, MsOsUsbClass, WindowsVersion, utf16_lit, utf16_null_le_bytes};
//!
//! const DESCRIPTOR_SET: os_20::DescriptorSet = os_20::DescriptorSet {
//!     version: WindowsVersion::MINIMAL,
//!     features: &[],
//!     configurations: &[
//!         os_20::ConfigurationSubset {
//!             configuration: 0,
//!             features: &[],
//!             functions: &[
//!                 os_20::FunctionSubset {
//!                     first_interface: 3,
//!                     features: &[
//!                         os_20::FeatureDescriptor::CompatibleId {
//!                             id: b"WINUSB\0\0",
//!                             sub_id: b"\0\0\0\0\0\0\0\0",
//!                         },
//!                         os_20::FeatureDescriptor::RegistryProperty {
//!                             data_type: os_20::PropertyDataType::RegMutliSz,
//!                             name: &utf16_lit::utf16_null!("DeviceInterfaceGUIDs"),
//!                             data: &utf16_null_le_bytes!("{6b09aac4-333f-4467-9e23-f88b9e9d95f7}\0"),
//!                         },
//!                     ]
//!                 }
//!             ]
//!         }
//!     ],
//! };
//!
//! const CAPABILITIES: os_20::Capabilities = os_20::Capabilities {
//!     infos: &[
//!         os_20::CapabilityInfo {
//!             descriptors: &DESCRIPTOR_SET,
//!             alt_enum_cmd: os_20::ALT_ENUM_CODE_NOT_SUPPORTED,
//!         }
//!     ],
//! };
//!
//! const DESCRIPTOR_SET_BYTES: [u8; DESCRIPTOR_SET.size()] = DESCRIPTOR_SET.descriptor();
//! const CAPABILITIES_BYTES: [u8; CAPABILITIES.data_len()] = CAPABILITIES.descriptor_data();
//!
//! pub const fn class() -> MsOsUsbClass {
//!     MsOsUsbClass {
//!         os_20_capabilities_data: &CAPABILITIES_BYTES,
//!         os_20_descriptor_sets: &[&DESCRIPTOR_SET_BYTES],
//!     }
//! }
//! ```
//!
//! Check test cases to see more examples from the specification.

#![no_std]
#![deny(missing_docs)]

// Include std when running tests
#[cfg(test)]
#[macro_use]
extern crate std;

/// Re-export of utf16_lit for constructing utf16 literals in compile time
pub extern crate utf16_lit;

/// USB class definition
pub mod class;
/// Microsoft OS 2.0 Descriptors
pub mod os_20;
/// Windows NTDDI version definitions
pub mod windows_version;

pub use class::MsOsUsbClass;
pub use windows_version::WindowsVersion;

/// Generate UTF-16 string using [`utf16_lit::utf16_null`] and get it as little-endian bytes array
///
/// This is useful for constructing registry property values:
/// ```
/// use usbd_microsoft_os::{os_20::{FeatureDescriptor, PropertyDataType}, utf16_null_le_bytes};
/// const FEAT: FeatureDescriptor = FeatureDescriptor::RegistryProperty {
///     data_type: PropertyDataType::RegMutliSz,
///     name: &utf16_lit::utf16_null!("DeviceInterfaceGUIDs"),
///     data: &utf16_null_le_bytes!("{897d7b90-5aae-43e5-9c36-aa0f2fdbafc9}\0"),
/// };
/// ```
#[macro_export]
macro_rules! utf16_null_le_bytes {
    ($string:literal) => {
        {
            const UTF16: &[u16] = $crate::utf16_lit::utf16_null!($string).as_slice();
            const BYTES: [u8; 2 * UTF16.len()] = {
                let mut buffer = [0u8; 2 * UTF16.len()];
                let mut i = 0;
                while i < UTF16.len() {
                    let value = UTF16[i].to_le_bytes();
                    buffer[2 * i] = value[0];
                    buffer[2 * i + 1] = value[1];
                    i += 1;
                }
                buffer
            };
            BYTES
        }
    };
}

/// Generate UTF-16 string using [`utf16_lit::utf16`] and get it as little-endian bytes array
///
/// Usually it is better to use [`utf16_null_le_bytes`].
#[macro_export]
macro_rules! utf16_le_bytes {
    ($string:literal) => {
        {
            const UTF16: &[u16] = $crate::utf16_lit::utf16!($string).as_slice();
            const BYTES: [u8; 2 * UTF16.len()] = {
                let mut buffer = [0u8; 2 * UTF16.len()];
                let mut i = 0;
                while i < UTF16.len() {
                    let value = UTF16[i].to_le_bytes();
                    buffer[2 * i] = value[0];
                    buffer[2 * i + 1] = value[1];
                    i += 1;
                }
                buffer
            };
            BYTES
        }
    };
}
