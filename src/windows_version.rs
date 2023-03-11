/// Windows version as defined in "NTDDI version constants" in sdkddkver.h from Windows SDK 10.0.22000.0
///
/// NTDDI version constants as defined in sdkddkver.h from Windows SDK 10.0.22000.0. Note that minimum
/// Windows version allowed for the MS OS 2.0 descriptor set is Windows 8.1 Preview (NTDDI_WINBLUE).
///
/// See also:
/// https://learn.microsoft.com/en-us/windows/win32/winprog/using-the-windows-headers?redirectedfrom=MSDN#macros-for-conditional-declarations
/// https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/wdm/nf-wdm-rtlisntddiversionavailable#parameters
#[repr(u32)]
#[derive(Clone, Copy)]
#[allow(missing_docs)]
pub enum WindowsVersion {
    Win4 = 0x04000000,
    Win2k = 0x05000000,
    // Windows 2000
    Win2KSp1 = 0x05000100,
    Win2KSp2 = 0x05000200,
    Win2KSp3 = 0x05000300,
    Win2KSp4 = 0x05000400,
    // Windows XP
    WinXp = 0x05010000,
    WinXpSp1 = 0x05010100,
    WinXpSp2 = 0x05010200,
    WinXpSp3 = 0x05010300,
    WinXpSp4 = 0x05010400,
    // Windows Server 2003
    WS03 = 0x05020000,
    WS03Sp1 = 0x05020100,
    WS03Sp2 = 0x05020200,
    WS03Sp3 = 0x05020300,
    WS03Sp4 = 0x05020400,
    // Windows Vista
    Win6 = 0x06000000,
    Win6Sp1 = 0x06000100,
    Win6Sp2 = 0x06000200,
    Win6Sp3 = 0x06000300,
    Win6Sp4 = 0x06000400,
    // Windows 7
    Win7 = 0x06010000,
    // Windows 8
    Win8 = 0x06020000,
    /// Windows 8.1
    WinBlue = 0x06030000,
    /// Windows 10 1507
    Win10 = 0x0A000000,
    /// Windows 10 1511
    Win10Th2 = 0x0A000001,
    /// Windows 10 1607
    Win10Rs1 = 0x0A000002,
    /// Windows 10 1703
    Win10Rs2 = 0x0A000003,
    /// Windows 10 1709
    Win10Rs3 = 0x0A000004,
    /// Windows 10 1803
    Win10Rs4 = 0x0A000005,
    /// Windows 10 1809
    Win10Rs5 = 0x0A000006,
    /// Windows 10 1903
    Win1019h1 = 0x0A000007,
    /// Windows 10 2004
    Win10Vb = 0x0A000008,
    Win10Mn = 0x0A000009,
    Win10Fe = 0x0A00000A,
    /// Windows 10 21H2
    Win10Co = 0x0A00000B,
}

#[allow(missing_docs)]
impl WindowsVersion {
    /// Minimal version that can be used in Microsoft OS 2.0 Descriptors
    pub const MINIMAL: Self = Self::WinBlue;

    // Windows Vista
    pub const VISTA: Self = Self::Win6;
    pub const VISTASP1: Self = Self::Win6Sp1;
    pub const VISTASP2: Self = Self::Win6Sp2;
    pub const VISTASP3: Self = Self::Win6Sp3;
    pub const VISTASP4: Self = Self::Win6Sp4;
    pub const LONGHORN: Self = Self::VISTA;

    // Windows Server 2008
    pub const WS08: Self = Self::Win6Sp1;
    pub const WS08SP2: Self = Self::Win6Sp2;
    pub const WS08SP3: Self = Self::Win6Sp3;
    pub const WS08SP4: Self = Self::Win6Sp4;

    pub const WINTHRESHOLD: Self = Self::Win10;

    pub(crate) const fn bytes(&self) -> [u8; 4] {
        self.check_minimal();
        (*self as u32).to_le_bytes()
    }

    pub(crate) const fn check_minimal(&self) {
        if (*self as u32) < (Self::MINIMAL as u32) {
            panic!("Minimal version allowed in Microsoft OS 2.0 Descriptors is Self::MINIMAL (WinBlue = Windows 8.1)");
        }
    }
}
