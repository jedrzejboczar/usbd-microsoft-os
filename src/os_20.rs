use usb_device::descriptor::capability_type;

use crate::windows_version::WindowsVersion;

/// Zero indicates that alternative enumeration is not supported
pub const ALT_ENUM_CODE_NOT_SUPPORTED: u8 = 0;

/// Microsoft OS 2.0 descriptor wIndex values
#[repr(u16)]
#[derive(Clone, Copy)]
pub enum DescriptorIndex {
    /// MS OS 2.0 retrieve descriptor request
    Descriptor = 0x07,
    /// MS OS 2.0 set alternate enumeration command
    SetAltEnumeration = 0x08,
}

impl DescriptorIndex {
    /// Get little-endian bytes as used in the descriptor
    pub const fn bytes(&self) -> [u8; 2] {
        (*self as u16).to_le_bytes()
    }
}

/// Microsoft OS 2.0 descriptor types
#[repr(u16)]
#[derive(Clone, Copy)]
#[allow(missing_docs)]
pub enum DescriptorType {
    SetHeaderDescriptor = 0x00,
    SubsetHeaderConfiguration = 0x01,
    SubsetHeaderFunction = 0x02,
    FeatureCompatbleId = 0x03,
    FeatureRegProperty = 0x04,
    FeatureMinResumeTime = 0x05,
    FeatureModelId = 0x06,
    FeatureCcgpDevice = 0x07,
    FeatureVendorRevision = 0x08,
}

impl DescriptorType {
    /// Get little-endian bytes as used in the descriptor
    pub const fn bytes(&self) -> [u8; 2] {
        (*self as u16).to_le_bytes()
    }
}

/// Registry Property type
#[repr(u16)]
#[derive(Clone, Copy)]
pub enum PropertyDataType {
    /// A NULL-terminated Unicode String (REG_SZ)
    RegSz = 1,
    /// A NULL-terminated Unicode String that includes environment variables (REG_EXPAND_SZ)
    RegExpandSz = 2,
    /// Free-form binary (REG_BINARY)
    RegBinary = 3,
    /// A little-endian 32-bit integer (REG_DWORD_LITTLE_ENDIAN)
    RegDwordLittleEndian = 4,
    /// A big-endian 32-bit integer (REG_DWORD_BIG_ENDIAN)
    RegDwordBigEndian = 5,
    /// A NULL-terminated Unicode string that contains a symbolic link (REG_LINK)
    RegLink = 6,
    /// Multiple NULL-terminated Unicode strings (REG_MULTI_SZ)
    RegMutliSz = 7,
}

impl PropertyDataType {
    /// Get little-endian bytes as used in the descriptor
    pub const fn bytes(&self) -> [u8; 2] {
        (*self as u16).to_le_bytes()
    }
}

/// Platform BOS capability info set
pub struct Capabilities {
    /// Capability information for each MS OS 2.0 descriptor set
    pub infos: &'static [CapabilityInfo],
}

/// Contains information about a unique Microsoft OS 2.0 descriptor set
pub struct CapabilityInfo {
    /// MS OS 2.0 descriptor set for this capability
    pub descriptors: &'static DescriptorSet,
    /// bAltEnumCode, non-zero value indicates that device may return non-default USB descriptors
    pub alt_enum_cmd: u8,
}

/// MS OS 2.0 descriptor set
pub struct DescriptorSet {
    /// Minimum Windows version for which descriptor set applies
    pub version: WindowsVersion,
    /// Features that apply to the whole device regardless of its configuration
    pub features: &'static [FeatureDescriptor],
    /// Configuration subsets
    pub configurations: &'static [ConfigurationSubset],
}

/// MS OS 2.0 configuration subset
pub struct ConfigurationSubset {
    /// bConfigurationValue
    pub configuration: u8,
    /// Features that apply to this USB device configuration
    pub features: &'static [FeatureDescriptor],
    /// Subsets for specific device functions
    pub functions: &'static [FunctionSubset],
}

/// MS OS 2.0 function subset
///
/// Only used for composite devices or single-function devices that use Usbccgp.sys as client driver.
pub struct FunctionSubset {
    /// Interface number for the first interface of the function to which this subset applies
    pub first_interface: u8,
    /// Features that apply to to specific USB function (group of interfaces) within a configuration
    pub features: &'static [FeatureDescriptor],
}

/// MS OS 2.0 feature descriptor
pub enum FeatureDescriptor {
    /// Define a compatible device ID
    CompatibleId {
        /// Compatible ID String
        id: &'static [u8; 8],
        /// Sub-compatible ID String
        sub_id: &'static [u8; 8],
    },
    /// Adds per-device/function registry values used by USB stack or device’s function driver
    RegistryProperty {
        /// Type of registry property
        data_type: PropertyDataType,
        /// Name of registry property
        name: &'static [u16],
        /// Property data
        data: &'static [u8],
    },
    /// Indicate to the Windows USB driver stack the minimum times related to suspend
    ResumeTime {
        /// Number of milliseconds the device requires to recover from port resume (valid 0..=10)
        recovery: u8,
        /// Number of milliseconds the device requires resume signaling to be asserted (valid 1..=20)
        signaling: u8,
    },
    /// Used to uniquely identify the physical device.
    ModelId {
        /// 128-bit number that uniquely identifies a physical device.
        ///
        /// Refer to IETF RFC 4122 for details on generation of a UUID.
        id: &'static [u8; 16],
    },
    /// Indicates that the device should be always treated as a composite device by Windows
    CcgpDevice,
    /// Indicates the revision of registry property and other MSOS descriptor
    VendorRevision {
        /// Revision number associated with the descriptor set
        ///
        /// Modify it every time you add/modify a registry property or other MSOS descriptor.
        /// Shell set to greater than or equal to 1.
        revision: u16,
    },
}

/// Implement slice_total_len `const fn` for a type using Self::total_len()
macro_rules! impl_slice_total_len {
    () => {
        const fn slice_total_len(items: &[Self]) -> u16 {
            let mut size = 0;
            let mut i = 0;
            while i < items.len() {
                size += items[i].total_len();
                i += 1;
            }
            size
        }
    };
}

/// Copy range from `src` to `dst` slice. This is like [`slice::copy_from_slice`] but works in `const fn`.
macro_rules! slice_assign {
    // Rust won't allow expr before `..`, it must be followed by one of `=>`, `,` or `;`,
    // so use `buf[2, 4]` as it seems to look the best.
    ($dst:path[$dst_start:expr, $dst_end:expr] = $src:path[$src_start:expr, $src_end:expr]) => {
        {
            let dst_start = $dst_start;
            let dst_end = $dst_end;
            let src_start = $src_start;
            let src_end = $src_end;

            #[allow(unused_comparisons)]
            if dst_end > $dst.len() || dst_start > dst_end  {
                panic!("Incorrect destination (lhs) range");
            }
            #[allow(unused_comparisons)]
            if src_end > $src.len() || src_start > src_end  {
                panic!("Incorrect source (rhs) range");
            }
            if src_end - src_start != dst_end - dst_start {
                panic!("Source and destination must have the same length");
            }

            let len = dst_end - dst_start;
            let mut i = 0;
            while i < len {
                $dst[dst_start + i] = $src[src_start + i];
                i += 1;
            }
        }
    };
}

/// Start writing a descriptor by filling first 4 bytes with wLength and wDescriptorType
macro_rules! descriptor_start {
    ($buf:ident, $pos:ident, [$len:expr, $desc_type:expr]) => {
        {
            let length = $len.to_le_bytes();
            let descriptor_type = $desc_type.bytes();
            slice_assign!($buf[$pos, $pos + 2] = length[0, 2]);
            slice_assign!($buf[$pos + 2, $pos + 4] = descriptor_type[0, 2]);
            $pos += 4;
        }
    };
}

/// Write feature descriptor to buffer and advance position
macro_rules! feature_descriptor {
    ($buf:ident, $pos:ident, $feature:expr) => {
        {
            descriptor_start!($buf, $pos, [$feature.total_len(), $feature.descriptor_type()]);

            match &$feature {
                FeatureDescriptor::CompatibleId { id, sub_id } => {
                    slice_assign!($buf[$pos, $pos + 8] = id[0, 8]);
                    slice_assign!($buf[$pos + 8, $pos + 16] = sub_id[0, 8]);
                    $pos += 16;
                },
                FeatureDescriptor::RegistryProperty { data_type, name, data } => {
                    let dtype = (*data_type as u16).to_le_bytes();
                    let name_len = (2 * name.len() as u16).to_le_bytes();
                    let data_len = (data.len() as u16).to_le_bytes();

                    slice_assign!($buf[$pos, $pos + 2] = dtype[0, 2]);
                    slice_assign!($buf[$pos + 2, $pos + 4] = name_len[0, 2]);
                    $pos += 4;

                    // PropertyName
                    let mut i = 0;
                    while i < name.len() {
                        $buf[$pos] = name[i].to_le_bytes()[0];
                        $buf[$pos + 1] = name[i].to_le_bytes()[1];
                        $pos += 2;
                        i += 1;
                    }

                    // wPropertyDataLength
                    slice_assign!($buf[$pos, $pos + 2] = data_len[0, 2]);
                    $pos += 2;

                    // PropertyData
                    slice_assign!($buf[$pos, $pos + data.len()] = data[0, data.len()]);
                    $pos += data.len();
                },
                FeatureDescriptor::ResumeTime { recovery, signaling } => {
                    $buf[$pos] = *recovery;
                    $buf[$pos + 1] = *signaling;
                    $pos += 2;
                },
                FeatureDescriptor::ModelId { id } => {
                    slice_assign!($buf[$pos, $pos + 16] = id[0, 16]);
                    $pos += 16;
                },
                FeatureDescriptor::CcgpDevice => {},
                FeatureDescriptor::VendorRevision { revision } => {
                    let revision = revision.to_le_bytes();
                    slice_assign!($buf[$pos, $pos + 2] = revision[0, 2]);
                    $pos += 2;
                },
            }
        }
    };
}

impl ConfigurationSubset {
    /// Get total size of descriptor
    pub const fn size(&self) -> usize {
        self.total_len() as usize
    }

    const HEADER_SIZE: u16 = 8;

    const fn total_len(&self) -> u16 {
        Self::HEADER_SIZE
            + FeatureDescriptor::slice_total_len(self.features)
            + FunctionSubset::slice_total_len(self.functions)
    }

    impl_slice_total_len!();
}

impl FunctionSubset {
    /// Get total size of descriptor
    pub const fn size(&self) -> usize {
        self.total_len() as usize
    }

    const HEADER_SIZE: u16 = 8;

    impl_slice_total_len!();

    const fn total_len(&self) -> u16 {
        Self::HEADER_SIZE + FeatureDescriptor::slice_total_len(self.features)
    }
}

impl FeatureDescriptor {
    /// Get total size of descriptor
    pub const fn size(&self) -> usize {
        self.total_len() as usize
    }

    const fn total_len(&self) -> u16 {
        match self {
            Self::CompatibleId { .. } => 2 + 2 + 8 + 8,
            Self::RegistryProperty { name, data, .. } => {
                2 + 2 + 2 + 2 + 2 + (2 * name.len() + data.len()) as u16
            },
            Self::ResumeTime { .. } => 2 + 2 + 1 + 1,
            Self::ModelId { .. } => 2 + 2 + 16,
            Self::CcgpDevice => 2 + 2,
            Self::VendorRevision { .. } => 2 + 2 + 2,
        }
    }

    impl_slice_total_len!();

    const fn descriptor_type(&self) -> DescriptorType {
        match self {
            Self::CompatibleId { .. } => DescriptorType::FeatureCompatbleId,
            Self::RegistryProperty { .. } => DescriptorType::FeatureRegProperty,
            Self::ResumeTime { .. } => DescriptorType::FeatureMinResumeTime,
            Self::ModelId { .. } => DescriptorType::FeatureModelId,
            Self::CcgpDevice => DescriptorType::FeatureCcgpDevice,
            Self::VendorRevision { .. } => DescriptorType::FeatureVendorRevision,
        }
    }

    #[allow(dead_code)] // used in tests
    const fn descriptor<const N: usize>(&self) -> [u8; N] {
        let mut buf = [0u8; N];
        let mut pos = 0;
        feature_descriptor!(buf, pos, self);
        let _ = pos; // avoid warning `pos is never read`
        buf
    }
}

impl DescriptorSet {
    const HEADER_SIZE: u16 = 10;

    const fn total_len(&self) -> u16 {
        Self::HEADER_SIZE
            + FeatureDescriptor::slice_total_len(self.features)
            + ConfigurationSubset::slice_total_len(self.configurations)
    }

    /// Get total size of descriptor
    pub const fn size(&self) -> usize {
        self.total_len() as usize
    }

    /// Get descriptor array in compile time
    ///
    /// Array length must be passed as generic parameter because rust does not allow
    /// using `self` to retrive this value automatically. Use [`Self::size`] method
    /// to get the correct value for the descriptor array length.
    pub const fn descriptor<const N: usize>(&self) -> [u8; N] {
        let mut buf = [0; N];
        let mut pos = 0;

        // Descriptor set header
        descriptor_start!(buf, pos, [Self::HEADER_SIZE, DescriptorType::SetHeaderDescriptor]);
        let total_len = self.total_len().to_le_bytes();
        let ver = self.version.bytes();
        slice_assign!(buf[4, 8] = ver[0, 4]);
        slice_assign!(buf[8, 10] = total_len[0, 2]);
        pos += 6;

        // Device-level feature descriptors
        let mut f = 0;
        while f < self.features.len() {
            feature_descriptor!(buf, pos, self.features[f]);
            f += 1;
        }

        // Configuration subsets
        let mut c = 0;
        while c < self.configurations.len() {
            let config = &self.configurations[c];

            // Configuration subset header
            descriptor_start!(buf, pos, [ConfigurationSubset::HEADER_SIZE, DescriptorType::SubsetHeaderConfiguration]);
            buf[pos] = config.configuration;  // bConfigurationValue
            buf[pos + 1] = 0; // bReserved
            let total_len = config.total_len().to_le_bytes();
            slice_assign!(buf[pos + 2, pos + 4] = total_len[0, 2]);
            pos += 4;

            // Configuration-level feature descriptors
            let mut f = 0;
            while f < config.features.len() {
                feature_descriptor!(buf, pos, config.features[f]);
                f += 1;
            }

            // Function subsets
            let mut fun = 0;
            while fun < config.functions.len() {
                let function = &config.functions[fun];

                // Function subset header
                descriptor_start!(buf, pos, [FunctionSubset::HEADER_SIZE, DescriptorType::SubsetHeaderFunction]);
                buf[pos] = function.first_interface;  // bFirstInterface
                buf[pos + 1] = 0; // bReserved
                let total_len = function.total_len().to_le_bytes();
                slice_assign!(buf[pos + 2, pos + 4] = total_len[0, 2]);
                pos += 4;

                // Function-level feature descriptors
                let mut f = 0;
                while f < function.features.len() {
                    feature_descriptor!(buf, pos, function.features[f]);
                    f += 1;
                }

                fun += 1;
            }

            c += 1;
        }

        buf
    }
}

impl CapabilityInfo {
    const TOTAL_LEN: u8 = 4 + 2 + 1 + 1;
}

impl Capabilities {
    const HEADER_SIZE: u8 = 4 + Self::CAPABILITY_ID.len() as u8;

    // PlatformCapabilityUUID = D8DD60DF-4589-4CC7-9CD2-659D9E648A9F
    // For encoding rules ("fields" as little-endian) see: https://www.rfc-editor.org/rfc/rfc4122
    const CAPABILITY_ID: [u8; 16] = [
        0xDF, 0x60, 0xDD, 0xD8,
        0x89, 0x45,
        0xC7, 0x4C,
        0x9C,
        0xD2,
        0x65, 0x9D, 0x9E, 0x64, 0x8A, 0x9F,
    ];

    const fn total_len(&self) -> u8 {
        Self::HEADER_SIZE + self.infos.len() as u8 * CapabilityInfo::TOTAL_LEN
    }

    /// Capability type passed to [`usb_device::descriptor::BosWriter`]'s `capability` method
    pub const CAPABILITY_TYPE: u8 = capability_type::PLATFORM;

    /// Size of data as passed to [`usb_device::descriptor::BosWriter`]'s `capability` method
    pub const fn data_len(&self) -> usize {
        (self.total_len() - 3) as usize
    }

    /// Data passed to [`usb_device::descriptor::BosWriter`]'s `capability` method
    pub const fn descriptor_data<const N: usize>(&self) -> [u8; N] {
        let mut buf = [0u8; N];
        let mut pos = 0;

        buf[0] = 0; // bReserved
        slice_assign!(buf[1, 17] = Self::CAPABILITY_ID[0, 16]); // MS_OS_20_Platform_Capability_ID
        pos += 17;

        let mut i = 0;
        while i < self.infos.len() {
            let info = &self.infos[i];
            let version = info.descriptors.version.bytes();
            let total_len = info.descriptors.total_len().to_le_bytes();
            slice_assign!(buf[pos, pos + 4] = version[0, 4]);
            slice_assign!(buf[pos + 4, pos + 6] = total_len[0, 2]);
            buf[pos + 6] = (i + 1) as u8;
            buf[pos + 7] = info.alt_enum_cmd;
            pos += 8;
            i += 1;
        }

        buf
    }
}

#[cfg(test)]
mod tests {
    use crate::utf16_null_le_bytes;

    use super::*;
    use std::{format, println};
    use std::vec::Vec;

    fn diff(data1: &[u8], data2: &[u8]) {
        if data1.len() != data2.len() {
            println!("Data length mismatch: {} vs {}", data1.len(), data2.len());
        }

        println!("    | RESULT     | EXPECTED   |");
        println!("POS | DEC  HEX C | DEC  HEX C |");
        println!("-------------------------------");
        let fmt = |val: Option<&u8>| {
            if let Some(val) = val {
                let ch = if val.is_ascii_graphic() {
                    char::from(*val)
                } else {
                    ' '
                };
                format!("{0:3} 0x{0:02x} {1}", val, ch)
            } else {
                " ".repeat(3 + 1 + 4 + 1 + 1)
            }
        };

        let n = data1.len().max(data2.len());
        for i in 0..n {
            let diff = if data1.get(i) != data2.get(i) {
                "<== DIFF"
            } else {
                ""
            };
            println!("{:3} | {} | {} | {}", i, fmt(data1.get(i)), fmt(data2.get(i)), diff);
        }
    }

    #[test]
    fn registry_property_size() {
        const DESCRIPTOR: FeatureDescriptor = FeatureDescriptor::RegistryProperty {
            data_type: PropertyDataType::RegMutliSz,
            name: &utf16_lit::utf16_null!("DeviceInterfaceGUIDs"),
            data: &unsafe {
                core::mem::transmute::<[u16; 40], [u8; 80]>(
                    utf16_lit::utf16_null!("{897d7b90-5aae-43e5-9c36-aa0f2fdbafc9}\0"),
                )
            },
        };
        assert_eq!(DESCRIPTOR.total_len(), 0x0084);
    }

    const EXAMPLE_SET: DescriptorSet = DescriptorSet {
        version: WindowsVersion::MINIMAL,
        features: &[],
        configurations: &[
            ConfigurationSubset {
                configuration: 0,
                features: &[],
                functions: &[
                    FunctionSubset {
                        first_interface: 1,
                        features: &[
                            FeatureDescriptor::CompatibleId { id: b"WINUSB\0\0", sub_id: b"\0\0\0\0\0\0\0\0" },
                            FeatureDescriptor::RegistryProperty {
                                data_type: PropertyDataType::RegMutliSz,
                                name: &utf16_lit::utf16_null!("DeviceInterfaceGUIDs"),
                                data: &utf16_null_le_bytes!("{897d7b90-5aae-43e5-9c36-aa0f2fdbafc9}\0"),
                            },
                        ]
                    }
                ]
            }
        ],
    };

    #[test]
    fn descriptor_set() {
        let utf16le = |s: &str| -> Vec<u8> {
            s.encode_utf16()
                .flat_map(|c| c.to_le_bytes())
                .collect()
        };
        let expected_fields: &[&[u8]] = &[
            // Microsoft OS 2.0 descriptor set header
            &0x000A_u16.to_le_bytes(),	                            // wLength
            &0x0000_u16.to_le_bytes(),	                            // wDescriptorType
            &0x06030000_u32.to_le_bytes(),	                        // dwWindowsVersion
            &0x00B2_u16.to_le_bytes(),	                            // wTotalLength
            // Microsoft OS 2.0 configuration subset header
            &0x0008_u16.to_le_bytes(),	                            // wLength
            &0x0001_u16.to_le_bytes(),	                            // wDescriptorType
            &0x00_u8.to_le_bytes(),	                                // bConfigurationValue
            &0x00_u8.to_le_bytes(),	                                // bReserved
            &0x00A8_u16.to_le_bytes(),	                            // wTotalLength
            // Microsoft OS 2.0 function subset header
            &0x0008_u16.to_le_bytes(),	                            // wLength
            &0x0002_u16.to_le_bytes(),	                            // wDescriptorType
            &0x01_u8.to_le_bytes(),	                                // bFirstInterface
            &0x00_u8.to_le_bytes(),	                                // bReserved
            &0x00A0_u16.to_le_bytes(),	                            // wSubsetLength
            // Microsoft OS 2.0 compatible ID descriptor
            &0x0014_u16.to_le_bytes(),	                            // wLength
            &0x0003_u16.to_le_bytes(),	                            // wDescriptorType
            b"WINUSB\0\0",	                                        // CompatibileID
            b"\0\0\0\0\0\0\0\0",	                                // SubCompatibleID
            // Microsoft OS 2.0 registry property descriptor
            &0x0084_u16.to_le_bytes(),	                            // wLength
            &0x0004_u16.to_le_bytes(),	                            // wDescriptorType
            &0x0007_u16.to_le_bytes(),	                            // wPropertyDataType
            &0x002A_u16.to_le_bytes(),	                            // wPropertyNameLength
            &utf16le("DeviceInterfaceGUIDs\0"),	                    // PropertyName
            &0x0050_u16.to_le_bytes(),	                            // wPropertyDataLength
            &utf16le("{897d7b90-5aae-43e5-9c36-aa0f2fdbafc9}\0\0"), // PropertyData
        ];
        let expected_bytes: Vec<u8> = expected_fields.iter()
            .copied()
            .flatten()
            .copied()
            .collect();
        assert_eq!(expected_bytes.len(), 0x00b2);

        // Constants
        const SIZE: u16 = EXAMPLE_SET.total_len();
        const DESC: [u8; SIZE as usize] = EXAMPLE_SET.descriptor();

        diff(&DESC, expected_bytes.as_slice());
        assert_eq!(&DESC, expected_bytes.as_slice());
        assert_eq!(SIZE, 0xb2);
    }

    #[test]
    fn feature_descriptor_compatible_id() {
        const FEAT: FeatureDescriptor = FeatureDescriptor::CompatibleId {
            id: b"WINUSB\0\0",
            sub_id: b"TESTING\0",
        };
        const DESC: [u8; FEAT.size()] = FEAT.descriptor();
        assert_eq!(DESC, [
            20,
            0,
            3,
            0,
            b'W', b'I', b'N', b'U', b'S', b'B', 0, 0,
            b'T', b'E', b'S', b'T', b'I', b'N', b'G', 0,
        ]);
    }

    #[test]
    fn feature_descriptor_register_property() {
        const FEAT: FeatureDescriptor = FeatureDescriptor::RegistryProperty {
            data_type: PropertyDataType::RegMutliSz,
            name: &utf16_lit::utf16_null!("DeviceInterfaceGUIDs"),
            data: &utf16_null_le_bytes!("{897d7b90-5aae-43e5-9c36-aa0f2fdbafc9}\0"),
        };
        const DESC: [u8; FEAT.size()] = FEAT.descriptor();
        let name_len = 2 * 21;
        let data_len = 2 * 40;
        let len = 10 + name_len + data_len;
        println!("reg property desc length = {0} = 0x{0:02x}", DESC.len());
        assert_eq!(DESC, [
            len, 0,
            4, 0,
            7, 0, // wPropertyDataType
            name_len, 0, // wPropertyNameLength
            b'D', 0, b'e', 0, b'v', 0, b'i', 0, b'c', 0, b'e', 0,
            b'I', 0, b'n', 0, b't', 0, b'e', 0, b'r', 0, b'f', 0, b'a', 0, b'c', 0, b'e', 0,
            b'G', 0, b'U', 0, b'I', 0, b'D', 0, b's', 0,
            0, 0,
            data_len, 0, // wPropertyDataLength
            b'{', 0, b'8', 0, b'9', 0, b'7', 0, b'd', 0, b'7', 0, b'b', 0, b'9', 0, b'0', 0,
            b'-', 0, b'5', 0, b'a', 0, b'a', 0, b'e', 0,
            b'-', 0, b'4', 0, b'3', 0, b'e', 0, b'5', 0,
            b'-', 0, b'9', 0, b'c', 0, b'3', 0, b'6', 0,
            b'-', 0, b'a', 0, b'a', 0, b'0', 0, b'f', 0, b'2', 0, b'f', 0, b'd', 0, b'b', 0, b'a', 0, b'f', 0, b'c', 0, b'9', 0,
            b'}', 0, 0, 0,
            0, 0,
        ]);
    }

    #[test]
    fn feature_descriptor_resume_time() {
        const FEAT: FeatureDescriptor = FeatureDescriptor::ResumeTime { recovery: 100, signaling: 200 };
        const DESC: [u8; FEAT.size()] = FEAT.descriptor();
        assert_eq!(DESC, [6, 0, 5, 0, 100, 200]);
    }

    #[test]
    fn feature_descriptor_model_id() {
        const FEAT: FeatureDescriptor = FeatureDescriptor::ModelId {
            id: &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        };
        const DESC: [u8; FEAT.size()] = FEAT.descriptor();
        assert_eq!(DESC, [20, 0, 6, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    }

    #[test]
    fn feature_descriptor_ccgp() {
        const FEAT: FeatureDescriptor = FeatureDescriptor::CcgpDevice;
        const DESC: [u8; FEAT.size()] = FEAT.descriptor();
        assert_eq!(DESC, [4, 0, 7, 0]);
    }

    #[test]
    fn feature_descriptor_vendor_revision() {
        const FEAT: FeatureDescriptor = FeatureDescriptor::VendorRevision { revision: 0x11aa };
        const DESC: [u8; FEAT.size()] = FEAT.descriptor();
        assert_eq!(DESC, [6, 0, 8, 0, 0xaa, 0x11]);
    }

    fn write_descriptor_set(buf: &mut [u8]) -> Result<usize, usb_device::UsbError> {
        const SIZE: usize = EXAMPLE_SET.size();
        const DESC: [u8; SIZE] = EXAMPLE_SET.descriptor();

        if buf.len() < SIZE {
            return Err(usb_device::UsbError::BufferOverflow);
        }

        buf[..SIZE].copy_from_slice(&DESC);

        Ok(SIZE)
    }

    #[test]
    fn descriptor_set_write_overflow() {
        // Default control endpoint size used in usb-device
        let mut buf = [0u8; 128];
        assert!(write_descriptor_set(buf.as_mut_slice()).is_err());
    }

    #[test]
    fn descriptor_set_no_overflow() {
        // Control endpoint size with feature "control-buffer-256" enabled
        let mut buf = [0u8; 256];
        assert_eq!(write_descriptor_set(buf.as_mut_slice()).unwrap(), EXAMPLE_SET.size());
    }

    // From specification:
    // Example: Microsoft OS 2.0 descriptor sets for a registry value
    mod example1 {
        use super::*;

        const REF_DESCRIPTOR_SET: &[u8] = &[
            //
            // Microsoft OS 2.0 Descriptor Set Header
            //
            0x0A, 0x00, // wLength - 10 bytes
            0x00, 0x00, // MSOS20_SET_HEADER_DESCRIPTOR
            0x00, 0x00, 0x03, 0x06, // dwWindowsVersion – 0x06030000 for Windows Blue
            0x48, 0x00, // wTotalLength – 72 bytes
            //
            // Microsoft OS 2.0 Registry Value Feature Descriptor
            //
            0x3E, 0x00, // wLength - 62 bytes
            0x04, 0x00, // wDescriptorType – 4 for Registry Property
            0x04, 0x00, // wPropertyDataType - 4 for REG_DWORD
            0x30, 0x00, // wPropertyNameLength – 48 bytes
            0x53, 0x00, 0x65, 0x00, // Property Name - “SelectiveSuspendEnabled”
            0x6C, 0x00, 0x65, 0x00,
            0x63, 0x00, 0x74, 0x00,
            0x69, 0x00, 0x76, 0x00,
            0x65, 0x00, 0x53, 0x00,
            0x75, 0x00, 0x73, 0x00,
            0x70, 0x00, 0x65, 0x00,
            0x6E, 0x00, 0x64, 0x00,
            0x45, 0x00, 0x6E, 0x00,
            0x61, 0x00, 0x62, 0x00,
            0x6C, 0x00, 0x65, 0x00,
            0x64, 0x00, 0x00, 0x00,
            0x04, 0x00, // wPropertyDataLength – 4 bytes
            0x01, 0x00, 0x00, 0x00 // PropertyData - 0x00000001
        ];

        const REF_CAPABILITIES: &[u8] = &[
            //
            // Microsoft OS 2.0 Platform Capability Descriptor Header
            //
            0x1C, // bLength - 28 bytes
            0x10, // bDescriptorType - 16
            0x05, // bDevCapability – 5 for Platform Capability
            0x00, // bReserved - 0
            0xDF, 0x60, 0xDD, 0xD8, // MS_OS_20_Platform_Capability_ID -
            0x89, 0x45, 0xC7, 0x4C, // {D8DD60DF-4589-4CC7-9CD2-659D9E648A9F}
            0x9C, 0xD2, 0x65, 0x9D, //
            0x9E, 0x64, 0x8A, 0x9F,
            //
            // Descriptor Information Set for Windows 8.1 or later
            //
            0x00, 0x00, 0x03, 0x06, // dwWindowsVersion – 0x06030000 for Windows Blue
            0x48, 0x00, // wLength – size of MS OS 2.0 descriptor set
            0x01, // bMS_VendorCode
            0x00, // bAltEnumCmd – 0 Does not support alternate enum
        ];

        const DESCRIPTOR_SET: DescriptorSet = DescriptorSet {
            version: WindowsVersion::MINIMAL,
            features: &[
                FeatureDescriptor::RegistryProperty {
                    data_type: PropertyDataType::RegDwordLittleEndian,
                    name: &utf16_lit::utf16_null!("SelectiveSuspendEnabled"),
                    data: &[1, 0, 0, 0],
                },
            ],
            configurations: &[],
        };

        const CAPABILITIES: Capabilities = Capabilities {
            infos: &[
                CapabilityInfo {
                    descriptors: &DESCRIPTOR_SET,
                    alt_enum_cmd: ALT_ENUM_CODE_NOT_SUPPORTED,
                }
            ],
        };

        #[test]
        fn bos_capability_descriptor() {
            const SIZE: usize = CAPABILITIES.data_len();
            const DATA: [u8; SIZE] = CAPABILITIES.descriptor_data();
            diff(&DATA, &REF_CAPABILITIES[3..]);
            assert_eq!(DATA, &REF_CAPABILITIES[3..]);
        }


        #[test]
        fn descriptor_set() {
            const SIZE: usize = DESCRIPTOR_SET.total_len() as usize;
            const DATA: [u8; SIZE] = DESCRIPTOR_SET.descriptor();
            diff(&DATA, &REF_DESCRIPTOR_SET);
            assert_eq!(DATA, REF_DESCRIPTOR_SET);
        }
    }

    // From specification:
    // Example: Microsoft OS 2.0 descriptor sets for a registry value based on specific Windows version
    mod example2 {
        use super::*;

        const REF_DESCRIPTOR_SETS: &[&[u8]] = &[
            // Example2_MSOS20DescriptorSetForWindows_Windows81OrLater[0x48]
            &[
                //
                // Microsoft OS 2.0 Descriptor Set Header
                //
                0x0A, 0x00, // wLength - 12 bytes
                0x00, 0x00, // MSOS20_SET_HEADER_DESCRIPTOR
                0x00, 0x00, 0x03, 0x06, // dwWindowsVersion – 0x06030000 for Windows 8.1 Build
                // NOTE: value 0x4A in specification is wrong, comment says 72 which is the actual length
                // 0x4A, 0x00, // wTotalLength – 72 bytes,
                0x48, 0x00,
                //
                // Microsoft OS 2.0 Registry Value Feature Descriptor
                //
                0x3E, 0x00, // wLength - 62 bytes
                0x04, 0x00, // wDescriptorType – 4 for Registry Property
                0x04, 0x00, // wPropertyDataType - 4 for REG_DWORD
                0x30, 0x00, // wPropertyNameLength – 48 bytes
                0x53, 0x00, 0x65, 0x00, // Property Name - “SelectiveSuspendEnabled”
                0x6C, 0x00, 0x65, 0x00,
                0x63, 0x00, 0x74, 0x00,
                0x69, 0x00, 0x76, 0x00,
                0x65, 0x00, 0x53, 0x00,
                0x75, 0x00, 0x73, 0x00,
                0x70, 0x00, 0x65, 0x00,
                0x6E, 0x00, 0x64, 0x00,
                0x45, 0x00, 0x6E, 0x00,
                0x61, 0x00, 0x62, 0x00,
                0x6C, 0x00, 0x65, 0x00,
                0x64, 0x00, 0x00, 0x00,
                0x04, 0x00, // wPropertyDataLength – 4 bytes
                0x00, 0x00, 0x00, 0x00 // PropertyData - 0x00000000
            ],
            // Example2_MSOS20DescriptorSetForFutureWindows[0x4A]
            &[
                //
                // Microsoft OS 2.0 Descriptor Set Header
                //
                0x0A, 0x00, // wLength - 10 bytes
                0x00, 0x00, // MSOS20_SET_HEADER_DESCRIPTOR
                0x00, 0x00, 0x00, 0x0A, // dwWindowsVersion – Windows 10
                // NOTE: value 0x4A in specification is wrong, total length is 72
                // 0x4A, 0x00, // wTotalLength – 74 bytes
                0x48, 0x00,
                //
                // Microsoft OS 2.0 Registry Value Feature Descriptor
                //
                0x3E, 0x00, // wLength- 62 bytes
                0x04, 0x00, // wDescriptorType – 4 for Registry Property
                0x04, 0x00, // wPropertyDataType - 4 for REG_DWORD
                0x30, 0x00, // wPropertyNameLength – 48 bytes
                0x53, 0x00, 0x65, 0x00, // Property Name - “SelectiveSuspendEnabled”
                0x6C, 0x00, 0x65, 0x00,
                0x63, 0x00, 0x74, 0x00,
                0x69, 0x00, 0x76, 0x00,
                0x65, 0x00, 0x53, 0x00,
                0x75, 0x00, 0x73, 0x00,
                0x70, 0x00, 0x65, 0x00,
                0x6E, 0x00, 0x64, 0x00,
                0x45, 0x00, 0x6E, 0x00,
                0x61, 0x00, 0x62, 0x00,
                0x6C, 0x00, 0x65, 0x00,
                0x64, 0x00, 0x00, 0x00,
                0x04, 0x00, // wPropertyDataLength – 4 bytes
                0x01, 0x00, 0x00, 0x00 // PropertyData - 0x00000001
            ],
        ];

        const REF_CAPABILITIES: &[u8] = &[
            //
            // Microsoft OS 2.0 Platform Descriptor Header
            //
            0x24, // bLength - 36 bytes
            0x10, // bDescriptorType - 16
            0x05, // bDevCapability – 5 for Platform Capability
            0x00, // bReserved - 0
            0xDF, 0x60, 0xDD, 0xD8, // MS_OS_20_Platform_Capability_ID -
            0x89, 0x45, 0xC7, 0x4C, // {D8DD60DF-4589-4CC7-9CD2-659D9E648A9F}
            0x9C, 0xD2, 0x65, 0x9D, //
            0x9E, 0x64, 0x8A, 0x9F, //
            //
            // Descriptor Information Set for Windows 8.1 or later
            //
            0x00, 0x00, 0x03, 0x06, // dwWindowsVersion – 0x06030000 for Windows 8.1 Build
            0x48, 0x00, // wLength – size of MS OS 2.0 descriptor set
            0x01, // bMS_VendorCode_
            0x00, // bAltEnumCmd – 0 Does not support alternate enum
            //
            // Descriptor Information Set for future version of Windows or later
            //
            0x00, 0x00, 0x00, 0x0A, // dwWindowsVersion – Windows 10
            0x48, 0x00, // wLength – size of MS OS 2.0 descriptor set
            0x02, // bMS_VendorCode
            0x10 // bAltEnumCmd – non-zero, supports alternate enum
        ];

        const DESCRIPTOR_SETS: &[DescriptorSet] = &[
            DescriptorSet {
                version: WindowsVersion::MINIMAL,
                features: &[
                    FeatureDescriptor::RegistryProperty {
                        data_type: PropertyDataType::RegDwordLittleEndian,
                        name: &utf16_lit::utf16_null!("SelectiveSuspendEnabled"),
                        data: &[0, 0, 0, 0],
                    },
                ],
                configurations: &[],
            },
            DescriptorSet {
                version: WindowsVersion::Win10,
                features: &[
                    FeatureDescriptor::RegistryProperty {
                        data_type: PropertyDataType::RegDwordLittleEndian,
                        name: &utf16_lit::utf16_null!("SelectiveSuspendEnabled"),
                        data: &[1, 0, 0, 0],
                    },
                ],
                configurations: &[],
            }
        ];


        const CAPABILITIES: Capabilities = Capabilities {
            infos: &[
                CapabilityInfo {
                    descriptors: &DESCRIPTOR_SETS[0],
                    alt_enum_cmd: ALT_ENUM_CODE_NOT_SUPPORTED,
                },
                CapabilityInfo {
                    descriptors: &DESCRIPTOR_SETS[1],
                    alt_enum_cmd: 0x10,
                }
            ],
        };

        #[test]
        fn bos_capability_descriptor() {
            const SIZE: usize = CAPABILITIES.data_len();
            const DATA: [u8; SIZE] = CAPABILITIES.descriptor_data();
            diff(&DATA, &REF_CAPABILITIES[3..]);
            assert_eq!(DATA, &REF_CAPABILITIES[3..]);
        }


        #[test]
        fn descriptor_set_0() {
            const SIZE: usize = DESCRIPTOR_SETS[0].total_len() as usize;
            const DATA: [u8; SIZE] = DESCRIPTOR_SETS[0].descriptor();
            diff(&DATA, REF_DESCRIPTOR_SETS[0]);
            assert_eq!(DATA, REF_DESCRIPTOR_SETS[0]);
        }

        #[test]
        fn descriptor_set_1() {
            const SIZE: usize = DESCRIPTOR_SETS[1].total_len() as usize;
            const DATA: [u8; SIZE] = DESCRIPTOR_SETS[1].descriptor();
            diff(&DATA, REF_DESCRIPTOR_SETS[1]);
            assert_eq!(DATA, REF_DESCRIPTOR_SETS[1]);
        }
    }
}
