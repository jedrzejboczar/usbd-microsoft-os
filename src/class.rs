use usb_device::class_prelude::*;

use crate::os_20::{Capabilities, DescriptorIndex};

/// USB class responsible for handling MS OS descriptor requests
///
/// This class will report Microsoft OS 2.0 descriptor set as well as related BOS capabilities.
///
/// For performance reasons all the descriptors should be statically generated arrays. Use
/// [`crate::os_20::DescriptorSet::descriptor`] and
/// [`crate::os_20::Capabilities::descriptor_data`] const functions to generate the descriptors.
pub struct MsOsUsbClass {
    /// Capabilities data obtained from [`crate::os_20::Capabilities::descriptor_data`]
    pub os_20_capabilities_data: &'static [u8],
    /// Data for each descriptor obtained from [`crate::os_20::DescriptorSet::descriptor`]
    pub os_20_descriptor_sets: &'static [&'static [u8]],
}

impl<B: UsbBus> UsbClass<B> for MsOsUsbClass {
    fn get_bos_descriptors(&self, writer: &mut BosWriter) -> usb_device::Result<()> {
        writer.capability(Capabilities::CAPABILITY_TYPE, self.os_20_capabilities_data)
    }

    fn control_in(&mut self, xfer: ControlIn<B>) {
        let req = xfer.request();

        // MS OS 2.0 get descriptors request
        if req.request_type == control::RequestType::Vendor
            && req.recipient == control::Recipient::Device
            // && req.value == 0x00 // ignore just in case
            && req.index == DescriptorIndex::Descriptor as u16
        {
            let descriptor_set = Capabilities::vendor_code_to_descriptor_set(req.request)
                .and_then(|i| self.os_20_descriptor_sets.get(i).copied());

            if let Some(set) = descriptor_set {
                xfer.accept_with_static(set).ok();
            } else {
                xfer.reject().ok();
            }
        }
    }

    fn control_out(&mut self, xfer: ControlOut<B>) {
        let req = xfer.request();

        // MS OS 2.0 set alternate enumeration command
        if req.request_type == control::RequestType::Vendor
            && req.recipient == control::Recipient::Device
            && req.index == DescriptorIndex::SetAltEnumeration as u16
        {
            let _alt_enum_code = req.value.to_le_bytes()[1];
            // FIXME: not supported yet
            xfer.reject().ok();
        }
    }
}
