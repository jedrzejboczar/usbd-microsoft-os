//! Microsoft OS USB descriptors for [`usb-device`]

#![no_std]
#![deny(missing_docs)]

// Include std when running tests
#[cfg(test)]
#[macro_use]
extern crate std;

/// Re-export of utf16_lit for constructing utf16 literals in compile time
pub extern crate utf16_lit;

/// Microsoft OS 2.0 Descriptors
pub mod os_20;

/// Windows NTDDI version definitions
pub mod windows_version;

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
