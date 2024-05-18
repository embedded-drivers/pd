#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

pub mod regs {
    #[doc = "Table 6.48 “Alert Data Object (ADO)”"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ado(pub u32);
    impl Ado {
        #[doc = "Extended alert type"]
        #[inline(always)]
        pub const fn extended_type(&self) -> super::vals::AdoExtendedType {
            let val = (self.0 >> 0usize) & 0x0f;
            super::vals::AdoExtendedType::from_bits(val as u8)
        }
        #[doc = "Extended alert type"]
        #[inline(always)]
        pub fn set_extended_type(&mut self, val: super::vals::AdoExtendedType) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
        }
        #[doc = "Hot swappable batterie number, 4 to 7"]
        #[inline(always)]
        pub const fn hot_swappable_batteryies(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Hot swappable batterie number, 4 to 7"]
        #[inline(always)]
        pub fn set_hot_swappable_batteryies(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Battery number, 0 to 3"]
        #[inline(always)]
        pub const fn fixed_batteries(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "Battery number, 0 to 3"]
        #[inline(always)]
        pub fn set_fixed_batteries(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "Battery status change"]
        #[inline(always)]
        pub const fn battery_status_change(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Battery status change"]
        #[inline(always)]
        pub fn set_battery_status_change(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Over current protection"]
        #[inline(always)]
        pub const fn ocp(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Over current protection"]
        #[inline(always)]
        pub fn set_ocp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Over temperature protection"]
        #[inline(always)]
        pub const fn otp(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Over temperature protection"]
        #[inline(always)]
        pub fn set_otp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Operating condition change"]
        #[inline(always)]
        pub const fn operating_condition_change(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Operating condition change"]
        #[inline(always)]
        pub fn set_operating_condition_change(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Source input change"]
        #[inline(always)]
        pub const fn source_input_change(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Source input change"]
        #[inline(always)]
        pub fn set_source_input_change(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Over voltage protection"]
        #[inline(always)]
        pub const fn ovp(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Over voltage protection"]
        #[inline(always)]
        pub fn set_ovp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Extended altert event"]
        #[inline(always)]
        pub const fn extended(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Extended altert event"]
        #[inline(always)]
        pub fn set_extended(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ado {
        #[inline(always)]
        fn default() -> Ado {
            Ado(0)
        }
    }
    #[doc = "Augmented Power Data Object."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apdo(pub u32);
    impl Apdo {
        #[doc = "Augmented PDO type."]
        #[inline(always)]
        pub const fn apdo_type(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "Augmented PDO type."]
        #[inline(always)]
        pub fn set_apdo_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[doc = "Data object type"]
        #[inline(always)]
        pub const fn pdo_type(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[doc = "Data object type"]
        #[inline(always)]
        pub fn set_pdo_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for Apdo {
        #[inline(always)]
        fn default() -> Apdo {
            Apdo(0)
        }
    }
    #[doc = "Table 6.28 “AVS Request Data Object”"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AvsRdo(pub u32);
    impl AvsRdo {
        #[doc = "Operating current in 50mA units"]
        #[inline(always)]
        pub const fn current_50ma(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Operating current in 50mA units"]
        #[inline(always)]
        pub fn set_current_50ma(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Output voltage in 25mV units (LSB 2bits are 0)"]
        #[inline(always)]
        pub const fn voltage_25mv(&self) -> u16 {
            let val = (self.0 >> 9usize) & 0x0fff;
            val as u16
        }
        #[doc = "Output voltage in 25mV units (LSB 2bits are 0)"]
        #[inline(always)]
        pub fn set_voltage_25mv(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 9usize)) | (((val as u32) & 0x0fff) << 9usize);
        }
        #[doc = "EPR mode capable"]
        #[inline(always)]
        pub const fn epr_mode_capable(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "EPR mode capable"]
        #[inline(always)]
        pub fn set_epr_mode_capable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Unchunked extended messages supported"]
        #[inline(always)]
        pub const fn unchunked_extended_messages_supported(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Unchunked extended messages supported"]
        #[inline(always)]
        pub fn set_unchunked_extended_messages_supported(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "No USB suspend"]
        #[inline(always)]
        pub const fn no_usb_suspend(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "No USB suspend"]
        #[inline(always)]
        pub fn set_no_usb_suspend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "USB communication capable"]
        #[inline(always)]
        pub const fn usb_communication_capable(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "USB communication capable"]
        #[inline(always)]
        pub fn set_usb_communication_capable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Capability mismatch"]
        #[inline(always)]
        pub const fn capability_mismatch(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Capability mismatch"]
        #[inline(always)]
        pub fn set_capability_mismatch(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Object position"]
        #[inline(always)]
        pub const fn object_position(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[doc = "Object position"]
        #[inline(always)]
        pub fn set_object_position(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
    }
    impl Default for AvsRdo {
        #[inline(always)]
        fn default() -> AvsRdo {
            AvsRdo(0)
        }
    }
    #[doc = "Battery supply PDO."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BatteryPdo(pub u32);
    impl BatteryPdo {
        #[doc = "Maximum power in 250mW units."]
        #[inline(always)]
        pub const fn max_power_250mw(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Maximum power in 250mW units."]
        #[inline(always)]
        pub fn set_max_power_250mw(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Minimum voltage in 50mV units."]
        #[inline(always)]
        pub const fn min_voltage_50mv(&self) -> u16 {
            let val = (self.0 >> 10usize) & 0x03ff;
            val as u16
        }
        #[doc = "Minimum voltage in 50mV units."]
        #[inline(always)]
        pub fn set_min_voltage_50mv(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 10usize)) | (((val as u32) & 0x03ff) << 10usize);
        }
        #[doc = "Maximum voltage in 50mV units."]
        #[inline(always)]
        pub const fn max_voltage_50mv(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x03ff;
            val as u16
        }
        #[doc = "Maximum voltage in 50mV units."]
        #[inline(always)]
        pub fn set_max_voltage_50mv(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 20usize)) | (((val as u32) & 0x03ff) << 20usize);
        }
        #[doc = "Data object type"]
        #[inline(always)]
        pub const fn pdo_type(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[doc = "Data object type"]
        #[inline(always)]
        pub fn set_pdo_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for BatteryPdo {
        #[inline(always)]
        fn default() -> BatteryPdo {
            BatteryPdo(0)
        }
    }
    #[doc = "Table 6.24 “Battery Request Data Object”"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BatteryRdo(pub u32);
    impl BatteryRdo {
        #[doc = "Minimum/Maximum operating power in 250mW units"]
        #[inline(always)]
        pub const fn minmax_power_250mw(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Minimum/Maximum operating power in 250mW units"]
        #[inline(always)]
        pub fn set_minmax_power_250mw(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Operating power in 250mW units"]
        #[inline(always)]
        pub const fn power_250mw(&self) -> u16 {
            let val = (self.0 >> 10usize) & 0x03ff;
            val as u16
        }
        #[doc = "Operating power in 250mW units"]
        #[inline(always)]
        pub fn set_power_250mw(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 10usize)) | (((val as u32) & 0x03ff) << 10usize);
        }
        #[doc = "EPR mode capable"]
        #[inline(always)]
        pub const fn epr_mode_capable(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "EPR mode capable"]
        #[inline(always)]
        pub fn set_epr_mode_capable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Unchunked extended messages supported"]
        #[inline(always)]
        pub const fn unchunked_extended_messages_supported(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Unchunked extended messages supported"]
        #[inline(always)]
        pub fn set_unchunked_extended_messages_supported(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "No USB suspend"]
        #[inline(always)]
        pub const fn no_usb_suspend(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "No USB suspend"]
        #[inline(always)]
        pub fn set_no_usb_suspend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "USB communication capable"]
        #[inline(always)]
        pub const fn usb_communication_capable(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "USB communication capable"]
        #[inline(always)]
        pub fn set_usb_communication_capable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Capability mismatch"]
        #[inline(always)]
        pub const fn capability_mismatch(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Capability mismatch"]
        #[inline(always)]
        pub fn set_capability_mismatch(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Give back flag"]
        #[inline(always)]
        pub const fn give_back(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Give back flag"]
        #[inline(always)]
        pub fn set_give_back(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Object position"]
        #[inline(always)]
        pub const fn object_position(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[doc = "Object position"]
        #[inline(always)]
        pub fn set_object_position(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
    }
    impl Default for BatteryRdo {
        #[inline(always)]
        fn default() -> BatteryRdo {
            BatteryRdo(0)
        }
    }
    #[doc = "Table 6.47 “Battery Status Data Object (BSDO)”"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bsdo(pub u32);
    impl Bsdo {
        #[doc = "Invalid"]
        #[inline(always)]
        pub const fn invalid(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Invalid"]
        #[inline(always)]
        pub fn set_invalid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Battery present"]
        #[inline(always)]
        pub const fn battery_present(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Battery present"]
        #[inline(always)]
        pub fn set_battery_present(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Charging status"]
        #[inline(always)]
        pub const fn charging_status(&self) -> super::vals::BsdoBatteryChargingStatus {
            let val = (self.0 >> 10usize) & 0x03;
            super::vals::BsdoBatteryChargingStatus::from_bits(val as u8)
        }
        #[doc = "Charging status"]
        #[inline(always)]
        pub fn set_charging_status(&mut self, val: super::vals::BsdoBatteryChargingStatus) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
        }
        #[doc = "Battery capacity"]
        #[inline(always)]
        pub const fn battery_capacity(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Battery capacity"]
        #[inline(always)]
        pub fn set_battery_capacity(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Bsdo {
        #[inline(always)]
        fn default() -> Bsdo {
            Bsdo(0)
        }
    }
    #[doc = "Extended Power Range(EPR) Adjustable Voltage Supply."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EprAvsApdo(pub u32);
    impl EprAvsApdo {
        #[doc = "PDP in 1W units."]
        #[inline(always)]
        pub const fn power_1w(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "PDP in 1W units."]
        #[inline(always)]
        pub fn set_power_1w(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Minimum voltage in 100mV units."]
        #[inline(always)]
        pub const fn min_voltage_100mv(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Minimum voltage in 100mV units."]
        #[inline(always)]
        pub fn set_min_voltage_100mv(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Maximum voltage in 100mV units."]
        #[inline(always)]
        pub const fn max_voltage_100mv(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0xff;
            val as u8
        }
        #[doc = "Maximum voltage in 100mV units."]
        #[inline(always)]
        pub fn set_max_voltage_100mv(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 17usize)) | (((val as u32) & 0xff) << 17usize);
        }
        #[doc = "Peak current."]
        #[inline(always)]
        pub const fn peak_current(&self) -> super::vals::PeakCurrent {
            let val = (self.0 >> 26usize) & 0x03;
            super::vals::PeakCurrent::from_bits(val as u8)
        }
        #[doc = "Peak current."]
        #[inline(always)]
        pub fn set_peak_current(&mut self, val: super::vals::PeakCurrent) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
        }
        #[doc = "Augmented PDO type."]
        #[inline(always)]
        pub const fn apdo_type(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "Augmented PDO type."]
        #[inline(always)]
        pub fn set_apdo_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[doc = "Data object type"]
        #[inline(always)]
        pub const fn pdo_type(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[doc = "Data object type"]
        #[inline(always)]
        pub fn set_pdo_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for EprAvsApdo {
        #[inline(always)]
        fn default() -> EprAvsApdo {
            EprAvsApdo(0)
        }
    }
    #[doc = "Table 6.51 “EPR Mode Data Object (EPRMDO)”"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eprmod(pub u32);
    impl Eprmod {
        #[doc = "EPR Mode Data"]
        #[inline(always)]
        pub const fn data(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "EPR Mode Data"]
        #[inline(always)]
        pub fn set_data(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Action"]
        #[inline(always)]
        pub const fn action(&self) -> super::vals::EprmodAction {
            let val = (self.0 >> 24usize) & 0xff;
            super::vals::EprmodAction::from_bits(val as u8)
        }
        #[doc = "Action"]
        #[inline(always)]
        pub fn set_action(&mut self, val: super::vals::EprmodAction) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val.to_bits() as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Eprmod {
        #[inline(always)]
        fn default() -> Eprmod {
            Eprmod(0)
        }
    }
    #[doc = "Extended message header."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ExtendedHeader(pub u16);
    impl ExtendedHeader {
        #[doc = "Data size."]
        #[inline(always)]
        pub const fn data_size(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Data size."]
        #[inline(always)]
        pub fn set_data_size(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
        }
        #[doc = "Request chunk."]
        #[inline(always)]
        pub const fn request_chunk(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Request chunk."]
        #[inline(always)]
        pub fn set_request_chunk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "Chunk number."]
        #[inline(always)]
        pub const fn chunk_number(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x0f;
            val as u8
        }
        #[doc = "Chunk number."]
        #[inline(always)]
        pub fn set_chunk_number(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u16) & 0x0f) << 11usize);
        }
        #[doc = "Chunked message."]
        #[inline(always)]
        pub const fn chunked(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Chunked message."]
        #[inline(always)]
        pub fn set_chunked(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for ExtendedHeader {
        #[inline(always)]
        fn default() -> ExtendedHeader {
            ExtendedHeader(0)
        }
    }
    #[doc = "Fixed supply PDO."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FixedPdo(pub u32);
    impl FixedPdo {
        #[doc = "Maximum current in 10mA units."]
        #[inline(always)]
        pub const fn max_current_10ma(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Maximum current in 10mA units."]
        #[inline(always)]
        pub fn set_max_current_10ma(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Voltage in 50mV units."]
        #[inline(always)]
        pub const fn voltage_50mv(&self) -> u16 {
            let val = (self.0 >> 10usize) & 0x03ff;
            val as u16
        }
        #[doc = "Voltage in 50mV units."]
        #[inline(always)]
        pub fn set_voltage_50mv(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 10usize)) | (((val as u32) & 0x03ff) << 10usize);
        }
        #[doc = "Peak current."]
        #[inline(always)]
        pub const fn peak_current(&self) -> super::vals::PeakCurrent {
            let val = (self.0 >> 20usize) & 0x03;
            super::vals::PeakCurrent::from_bits(val as u8)
        }
        #[doc = "Peak current."]
        #[inline(always)]
        pub fn set_peak_current(&mut self, val: super::vals::PeakCurrent) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
        }
        #[doc = "Extended power range mode supported."]
        #[inline(always)]
        pub const fn epr_mode_supported(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Extended power range mode supported."]
        #[inline(always)]
        pub fn set_epr_mode_supported(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Unchunked extended messages supported."]
        #[inline(always)]
        pub const fn unchunked_extended_messages_supported(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Unchunked extended messages supported."]
        #[inline(always)]
        pub fn set_unchunked_extended_messages_supported(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Dual role data."]
        #[inline(always)]
        pub const fn dual_role_data(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Dual role data."]
        #[inline(always)]
        pub fn set_dual_role_data(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "USB communication capable."]
        #[inline(always)]
        pub const fn usb_communication_capable(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "USB communication capable."]
        #[inline(always)]
        pub fn set_usb_communication_capable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Unconstrained power."]
        #[inline(always)]
        pub const fn unconstrained_power(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Unconstrained power."]
        #[inline(always)]
        pub fn set_unconstrained_power(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "USB Suspend supported."]
        #[inline(always)]
        pub const fn usb_suspend_supported(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "USB Suspend supported."]
        #[inline(always)]
        pub fn set_usb_suspend_supported(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Dual role power."]
        #[inline(always)]
        pub const fn dual_role_power(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Dual role power."]
        #[inline(always)]
        pub fn set_dual_role_power(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Data object type"]
        #[inline(always)]
        pub const fn pdo_type(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[doc = "Data object type"]
        #[inline(always)]
        pub fn set_pdo_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for FixedPdo {
        #[inline(always)]
        fn default() -> FixedPdo {
            FixedPdo(0)
        }
    }
    #[doc = "Fixed supply PDO for sink."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FixedPdoSink(pub u32);
    impl FixedPdoSink {
        #[doc = "Operating current in 10mA units."]
        #[inline(always)]
        pub const fn current_10ma(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Operating current in 10mA units."]
        #[inline(always)]
        pub fn set_current_10ma(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Voltage in 50mV units."]
        #[inline(always)]
        pub const fn voltage_50mv(&self) -> u16 {
            let val = (self.0 >> 10usize) & 0x03ff;
            val as u16
        }
        #[doc = "Voltage in 50mV units."]
        #[inline(always)]
        pub fn set_voltage_50mv(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 10usize)) | (((val as u32) & 0x03ff) << 10usize);
        }
        #[doc = "Fast Role Swap required USB Type-C® Current"]
        #[inline(always)]
        pub const fn fast_swap_current(&self) -> u8 {
            let val = (self.0 >> 23usize) & 0x03;
            val as u8
        }
        #[doc = "Fast Role Swap required USB Type-C® Current"]
        #[inline(always)]
        pub fn set_fast_swap_current(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 23usize)) | (((val as u32) & 0x03) << 23usize);
        }
        #[doc = "Dual role data."]
        #[inline(always)]
        pub const fn dual_role_data(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Dual role data."]
        #[inline(always)]
        pub fn set_dual_role_data(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "USB communication capable."]
        #[inline(always)]
        pub const fn usb_communication_capable(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "USB communication capable."]
        #[inline(always)]
        pub fn set_usb_communication_capable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Unconstrained power."]
        #[inline(always)]
        pub const fn unconstrained_power(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Unconstrained power."]
        #[inline(always)]
        pub fn set_unconstrained_power(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "High capability."]
        #[inline(always)]
        pub const fn high_capability(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "High capability."]
        #[inline(always)]
        pub fn set_high_capability(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Dual role power."]
        #[inline(always)]
        pub const fn dual_role_power(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Dual role power."]
        #[inline(always)]
        pub fn set_dual_role_power(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Data object type"]
        #[inline(always)]
        pub const fn pdo_type(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[doc = "Data object type"]
        #[inline(always)]
        pub fn set_pdo_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for FixedPdoSink {
        #[inline(always)]
        fn default() -> FixedPdoSink {
            FixedPdoSink(0)
        }
    }
    #[doc = "Table 6.22 “Fixed and Variable Request Data Object”"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FixedRdo(pub u32);
    impl FixedRdo {
        #[doc = "Minimum/Maximum operating current in 10mA units"]
        #[inline(always)]
        pub const fn minmax_current_10ma(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Minimum/Maximum operating current in 10mA units"]
        #[inline(always)]
        pub fn set_minmax_current_10ma(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Operating current in 10mA units"]
        #[inline(always)]
        pub const fn current_10ma(&self) -> u16 {
            let val = (self.0 >> 10usize) & 0x03ff;
            val as u16
        }
        #[doc = "Operating current in 10mA units"]
        #[inline(always)]
        pub fn set_current_10ma(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 10usize)) | (((val as u32) & 0x03ff) << 10usize);
        }
        #[doc = "EPR mode capable"]
        #[inline(always)]
        pub const fn epr_mode_capable(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "EPR mode capable"]
        #[inline(always)]
        pub fn set_epr_mode_capable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Unchunked extended messages supported"]
        #[inline(always)]
        pub const fn unchunked_extended_messages_supported(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Unchunked extended messages supported"]
        #[inline(always)]
        pub fn set_unchunked_extended_messages_supported(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "No USB suspend"]
        #[inline(always)]
        pub const fn no_usb_suspend(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "No USB suspend"]
        #[inline(always)]
        pub fn set_no_usb_suspend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "USB communication capable"]
        #[inline(always)]
        pub const fn usb_communication_capable(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "USB communication capable"]
        #[inline(always)]
        pub fn set_usb_communication_capable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Capability mismatch"]
        #[inline(always)]
        pub const fn capability_mismatch(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Capability mismatch"]
        #[inline(always)]
        pub fn set_capability_mismatch(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Give back flag"]
        #[inline(always)]
        pub const fn give_back(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Give back flag"]
        #[inline(always)]
        pub fn set_give_back(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Object position"]
        #[inline(always)]
        pub const fn object_position(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[doc = "Object position"]
        #[inline(always)]
        pub fn set_object_position(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
    }
    impl Default for FixedRdo {
        #[inline(always)]
        fn default() -> FixedRdo {
            FixedRdo(0)
        }
    }
    #[doc = "Message Header."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Header(pub u16);
    impl Header {
        #[doc = "Message type."]
        #[inline(always)]
        pub const fn control_message_type(&self) -> super::vals::ControlMessageType {
            let val = (self.0 >> 0usize) & 0x1f;
            super::vals::ControlMessageType::from_bits(val as u8)
        }
        #[doc = "Message type."]
        #[inline(always)]
        pub fn set_control_message_type(&mut self, val: super::vals::ControlMessageType) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u16) & 0x1f) << 0usize);
        }
        #[doc = "Message type."]
        #[inline(always)]
        pub const fn data_message_type(&self) -> super::vals::DataMessageType {
            let val = (self.0 >> 0usize) & 0x1f;
            super::vals::DataMessageType::from_bits(val as u8)
        }
        #[doc = "Message type."]
        #[inline(always)]
        pub fn set_data_message_type(&mut self, val: super::vals::DataMessageType) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u16) & 0x1f) << 0usize);
        }
        #[doc = "Extended message type."]
        #[inline(always)]
        pub const fn extended_message_type(&self) -> super::vals::ExtendedMessageType {
            let val = (self.0 >> 0usize) & 0x1f;
            super::vals::ExtendedMessageType::from_bits(val as u8)
        }
        #[doc = "Extended message type."]
        #[inline(always)]
        pub fn set_extended_message_type(&mut self, val: super::vals::ExtendedMessageType) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u16) & 0x1f) << 0usize);
        }
        #[doc = "Port data role."]
        #[inline(always)]
        pub const fn port_data_role(&self) -> super::vals::PortDataRole {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::PortDataRole::from_bits(val as u8)
        }
        #[doc = "Port data role."]
        #[inline(always)]
        pub fn set_port_data_role(&mut self, val: super::vals::PortDataRole) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u16) & 0x01) << 5usize);
        }
        #[doc = "Specification revision."]
        #[inline(always)]
        pub const fn spec_revision(&self) -> super::vals::SpecRevision {
            let val = (self.0 >> 6usize) & 0x03;
            super::vals::SpecRevision::from_bits(val as u8)
        }
        #[doc = "Specification revision."]
        #[inline(always)]
        pub fn set_spec_revision(&mut self, val: super::vals::SpecRevision) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
        }
        #[doc = "Port power role."]
        #[inline(always)]
        pub const fn port_power_role(&self) -> super::vals::PortPowerRole {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::PortPowerRole::from_bits(val as u8)
        }
        #[doc = "Port power role."]
        #[inline(always)]
        pub fn set_port_power_role(&mut self, val: super::vals::PortPowerRole) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
        }
        #[doc = "Message ID."]
        #[inline(always)]
        pub const fn message_id(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x07;
            val as u8
        }
        #[doc = "Message ID."]
        #[inline(always)]
        pub fn set_message_id(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u16) & 0x07) << 9usize);
        }
        #[doc = "Number of data objects."]
        #[inline(always)]
        pub const fn number_of_data_objects(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "Number of data objects."]
        #[inline(always)]
        pub fn set_number_of_data_objects(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u16) & 0x07) << 12usize);
        }
        #[doc = "Extended message."]
        #[inline(always)]
        pub const fn extended(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Extended message."]
        #[inline(always)]
        pub fn set_extended(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Header {
        #[inline(always)]
        fn default() -> Header {
            Header(0)
        }
    }
    #[doc = "Power Data Object."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdo(pub u32);
    impl Pdo {
        #[doc = "Data object type"]
        #[inline(always)]
        pub const fn pdo_type(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[doc = "Data object type"]
        #[inline(always)]
        pub fn set_pdo_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for Pdo {
        #[inline(always)]
        fn default() -> Pdo {
            Pdo(0)
        }
    }
    #[doc = "Table 6.26 “PPS Request Data Object”"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PpsRdo(pub u32);
    impl PpsRdo {
        #[doc = "Operating current in 50mA units"]
        #[inline(always)]
        pub const fn current_50ma(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Operating current in 50mA units"]
        #[inline(always)]
        pub fn set_current_50ma(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Output voltage in 20mV units"]
        #[inline(always)]
        pub const fn voltage_20mv(&self) -> u16 {
            let val = (self.0 >> 9usize) & 0x0fff;
            val as u16
        }
        #[doc = "Output voltage in 20mV units"]
        #[inline(always)]
        pub fn set_voltage_20mv(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 9usize)) | (((val as u32) & 0x0fff) << 9usize);
        }
        #[doc = "EPR mode capable"]
        #[inline(always)]
        pub const fn epr_mode_capable(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "EPR mode capable"]
        #[inline(always)]
        pub fn set_epr_mode_capable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Unchunked extended messages supported"]
        #[inline(always)]
        pub const fn unchunked_extended_messages_supported(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Unchunked extended messages supported"]
        #[inline(always)]
        pub fn set_unchunked_extended_messages_supported(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "No USB suspend"]
        #[inline(always)]
        pub const fn no_usb_suspend(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "No USB suspend"]
        #[inline(always)]
        pub fn set_no_usb_suspend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "USB communication capable"]
        #[inline(always)]
        pub const fn usb_communication_capable(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "USB communication capable"]
        #[inline(always)]
        pub fn set_usb_communication_capable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Capability mismatch"]
        #[inline(always)]
        pub const fn capability_mismatch(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Capability mismatch"]
        #[inline(always)]
        pub fn set_capability_mismatch(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Object position"]
        #[inline(always)]
        pub const fn object_position(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[doc = "Object position"]
        #[inline(always)]
        pub fn set_object_position(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
    }
    impl Default for PpsRdo {
        #[inline(always)]
        fn default() -> PpsRdo {
            PpsRdo(0)
        }
    }
    #[doc = "Request Data Object."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rdo(pub u32);
    impl Rdo {
        #[doc = "EPR mode capable"]
        #[inline(always)]
        pub const fn epr_mode_capable(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "EPR mode capable"]
        #[inline(always)]
        pub fn set_epr_mode_capable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Unchunked extended messages supported"]
        #[inline(always)]
        pub const fn unchunked_extended_messages_supported(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Unchunked extended messages supported"]
        #[inline(always)]
        pub fn set_unchunked_extended_messages_supported(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "No USB suspend"]
        #[inline(always)]
        pub const fn no_usb_suspend(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "No USB suspend"]
        #[inline(always)]
        pub fn set_no_usb_suspend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "USB communication capable"]
        #[inline(always)]
        pub const fn usb_communication_capable(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "USB communication capable"]
        #[inline(always)]
        pub fn set_usb_communication_capable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Capability mismatch"]
        #[inline(always)]
        pub const fn capability_mismatch(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Capability mismatch"]
        #[inline(always)]
        pub fn set_capability_mismatch(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Object position"]
        #[inline(always)]
        pub const fn object_position(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[doc = "Object position"]
        #[inline(always)]
        pub fn set_object_position(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
    }
    impl Default for Rdo {
        #[inline(always)]
        fn default() -> Rdo {
            Rdo(0)
        }
    }
    #[doc = "Standard Power Range (SPR) Adjustable Voltage Supply."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SprAvsApdo(pub u32);
    impl SprAvsApdo {
        #[doc = "Maximum current in 10mA units for 15V-20V."]
        #[inline(always)]
        pub const fn max_current_for_20v_10ma(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Maximum current in 10mA units for 15V-20V."]
        #[inline(always)]
        pub fn set_max_current_for_20v_10ma(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Maximum current in 10mA units for 9V-15V."]
        #[inline(always)]
        pub const fn max_current_for_15v_10ma(&self) -> u16 {
            let val = (self.0 >> 10usize) & 0x03ff;
            val as u16
        }
        #[doc = "Maximum current in 10mA units for 9V-15V."]
        #[inline(always)]
        pub fn set_max_current_for_15v_10ma(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 10usize)) | (((val as u32) & 0x03ff) << 10usize);
        }
        #[doc = "Peak current."]
        #[inline(always)]
        pub const fn peak_current(&self) -> super::vals::PeakCurrent {
            let val = (self.0 >> 26usize) & 0x03;
            super::vals::PeakCurrent::from_bits(val as u8)
        }
        #[doc = "Peak current."]
        #[inline(always)]
        pub fn set_peak_current(&mut self, val: super::vals::PeakCurrent) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
        }
        #[doc = "Augmented PDO type."]
        #[inline(always)]
        pub const fn apdo_type(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "Augmented PDO type."]
        #[inline(always)]
        pub fn set_apdo_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[doc = "Data object type"]
        #[inline(always)]
        pub const fn pdo_type(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[doc = "Data object type"]
        #[inline(always)]
        pub fn set_pdo_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for SprAvsApdo {
        #[inline(always)]
        fn default() -> SprAvsApdo {
            SprAvsApdo(0)
        }
    }
    #[doc = "SPR Programmable Power Supply. (PPS)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SprPpsApdo(pub u32);
    impl SprPpsApdo {
        #[doc = "Maximum current in 50mA units."]
        #[inline(always)]
        pub const fn max_current_50ma(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Maximum current in 50mA units."]
        #[inline(always)]
        pub fn set_max_current_50ma(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Minimum voltage in 100mV units."]
        #[inline(always)]
        pub const fn min_voltage_100mv(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Minimum voltage in 100mV units."]
        #[inline(always)]
        pub fn set_min_voltage_100mv(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Maximum voltage in 100mV units."]
        #[inline(always)]
        pub const fn max_voltage_100mv(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0xff;
            val as u8
        }
        #[doc = "Maximum voltage in 100mV units."]
        #[inline(always)]
        pub fn set_max_voltage_100mv(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 17usize)) | (((val as u32) & 0xff) << 17usize);
        }
        #[doc = "PPS power limited."]
        #[inline(always)]
        pub const fn pps_power_limited(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "PPS power limited."]
        #[inline(always)]
        pub fn set_pps_power_limited(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Augmented PDO type."]
        #[inline(always)]
        pub const fn apdo_type(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "Augmented PDO type."]
        #[inline(always)]
        pub fn set_apdo_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[doc = "Data object type"]
        #[inline(always)]
        pub const fn pdo_type(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[doc = "Data object type"]
        #[inline(always)]
        pub fn set_pdo_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for SprPpsApdo {
        #[inline(always)]
        fn default() -> SprPpsApdo {
            SprPpsApdo(0)
        }
    }
    #[doc = "Structured VDM Header"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct StructuredVdmHeader(pub u32);
    impl StructuredVdmHeader {
        #[doc = "Command"]
        #[inline(always)]
        pub const fn command(&self) -> super::vals::VdmCommandType {
            let val = (self.0 >> 0usize) & 0x1f;
            super::vals::VdmCommandType::from_bits(val as u8)
        }
        #[doc = "Command"]
        #[inline(always)]
        pub fn set_command(&mut self, val: super::vals::VdmCommandType) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
        }
        #[doc = "Command type"]
        #[inline(always)]
        pub const fn command_type(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "Command type"]
        #[inline(always)]
        pub fn set_command_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "Object position"]
        #[inline(always)]
        pub const fn object_position(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "Object position"]
        #[inline(always)]
        pub fn set_object_position(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "Minor version"]
        #[inline(always)]
        pub const fn minor_version(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x03;
            val as u8
        }
        #[doc = "Minor version"]
        #[inline(always)]
        pub fn set_minor_version(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u32) & 0x03) << 11usize);
        }
        #[doc = "Major version"]
        #[inline(always)]
        pub const fn major_version(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x03;
            val as u8
        }
        #[doc = "Major version"]
        #[inline(always)]
        pub fn set_major_version(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
        }
        #[doc = "VDM Type"]
        #[inline(always)]
        pub const fn type_(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "VDM Type"]
        #[inline(always)]
        pub fn set_type_(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "USB Vendor ID"]
        #[inline(always)]
        pub const fn vid(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "USB Vendor ID"]
        #[inline(always)]
        pub fn set_vid(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for StructuredVdmHeader {
        #[inline(always)]
        fn default() -> StructuredVdmHeader {
            StructuredVdmHeader(0)
        }
    }
    #[doc = "Variable supply PDO. (non-battery)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VariablePdo(pub u32);
    impl VariablePdo {
        #[doc = "Maximum current in 10mA units."]
        #[inline(always)]
        pub const fn max_current_10ma(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Maximum current in 10mA units."]
        #[inline(always)]
        pub fn set_max_current_10ma(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Minimum voltage in 50mV units."]
        #[inline(always)]
        pub const fn min_voltage_50mv(&self) -> u16 {
            let val = (self.0 >> 10usize) & 0x03ff;
            val as u16
        }
        #[doc = "Minimum voltage in 50mV units."]
        #[inline(always)]
        pub fn set_min_voltage_50mv(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 10usize)) | (((val as u32) & 0x03ff) << 10usize);
        }
        #[doc = "Maximum voltage in 50mV units."]
        #[inline(always)]
        pub const fn max_voltage_50mv(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x03ff;
            val as u16
        }
        #[doc = "Maximum voltage in 50mV units."]
        #[inline(always)]
        pub fn set_max_voltage_50mv(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 20usize)) | (((val as u32) & 0x03ff) << 20usize);
        }
        #[doc = "Data object type"]
        #[inline(always)]
        pub const fn pdo_type(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[doc = "Data object type"]
        #[inline(always)]
        pub fn set_pdo_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for VariablePdo {
        #[inline(always)]
        fn default() -> VariablePdo {
            VariablePdo(0)
        }
    }
    #[doc = "VDM Header"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VdmHeader(pub u32);
    impl VdmHeader {
        #[doc = "VDM Type"]
        #[inline(always)]
        pub const fn type_(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "VDM Type"]
        #[inline(always)]
        pub fn set_type_(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "USB Vendor ID"]
        #[inline(always)]
        pub const fn vid(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "USB Vendor ID"]
        #[inline(always)]
        pub fn set_vid(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for VdmHeader {
        #[inline(always)]
        fn default() -> VdmHeader {
            VdmHeader(0)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum AdoExtendedType {
        _RESERVED_0 = 0x0,
        DFP_POWER_STATE_CHANGE = 0x01,
        UFP_POWER_BUTTON_PRESS = 0x02,
        UFP_POWER_BUTTON_RELEASE = 0x03,
        UFP_WAKE = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        _RESERVED_8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl AdoExtendedType {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> AdoExtendedType {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for AdoExtendedType {
        #[inline(always)]
        fn from(val: u8) -> AdoExtendedType {
            AdoExtendedType::from_bits(val)
        }
    }
    impl From<AdoExtendedType> for u8 {
        #[inline(always)]
        fn from(val: AdoExtendedType) -> u8 {
            AdoExtendedType::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum ApdoType {
        #[doc = "Standard Power Range, Programmable Power Supply"]
        SPR_PPS = 0x0,
        #[doc = "Extended Power Range, Adjustable Voltage Supply"]
        EPR_AVS = 0x01,
        #[doc = "Standard Power Range, Adjustable Voltage Supply"]
        SPR_AVS = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl ApdoType {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ApdoType {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for ApdoType {
        #[inline(always)]
        fn from(val: u8) -> ApdoType {
            ApdoType::from_bits(val)
        }
    }
    impl From<ApdoType> for u8 {
        #[inline(always)]
        fn from(val: ApdoType) -> u8 {
            ApdoType::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum BsdoBatteryChargingStatus {
        CHARGING = 0x0,
        DISCHARGING = 0x01,
        IDLE = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl BsdoBatteryChargingStatus {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> BsdoBatteryChargingStatus {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for BsdoBatteryChargingStatus {
        #[inline(always)]
        fn from(val: u8) -> BsdoBatteryChargingStatus {
            BsdoBatteryChargingStatus::from_bits(val)
        }
    }
    impl From<BsdoBatteryChargingStatus> for u8 {
        #[inline(always)]
        fn from(val: BsdoBatteryChargingStatus) -> u8 {
            BsdoBatteryChargingStatus::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum ControlMessageType {
        #[doc = "GoodCRC"]
        GOODCRC = 0x0,
        #[doc = "GotoMin"]
        GOTOMIN = 0x01,
        #[doc = "Accept"]
        ACCEPT = 0x02,
        #[doc = "Reject"]
        REJECT = 0x03,
        #[doc = "Ping"]
        PING = 0x04,
        #[doc = "PS_RDY"]
        PS_RDY = 0x05,
        #[doc = "Get_Source_Cap"]
        GET_SOURCE_CAP = 0x06,
        #[doc = "Get_Sink_Cap"]
        GET_SINK_CAP = 0x07,
        #[doc = "DR_Swap"]
        DR_SWAP = 0x08,
        #[doc = "PR_Swap"]
        PR_SWAP = 0x09,
        #[doc = "VCONN_Swap"]
        VCONN_SWAP = 0x0a,
        #[doc = "Wait"]
        WAIT = 0x0b,
        #[doc = "Soft_Reset"]
        SOFT_RESET = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
        #[doc = "Not_Supported"]
        NOT_SUPPORTED = 0x10,
        #[doc = "Get_Source_Cap_Extended"]
        GET_SOURCE_CAP_EXTENDED = 0x11,
        #[doc = "Get_Status"]
        GET_STATUS = 0x12,
        #[doc = "FR_Swap"]
        FR_SWAP = 0x13,
        #[doc = "Get_PPS_Status"]
        GET_PPS_STATUS = 0x14,
        #[doc = "Get_Country_Codes"]
        GET_COUNTRY_CODES = 0x15,
        #[doc = "Get_Sink_Cap_Extended"]
        GET_SINK_CAP_EXTENDED = 0x16,
        #[doc = "Get_Source_Info"]
        GET_SOURCE_INFO = 0x17,
        #[doc = "Get_Revision"]
        GET_REVISION = 0x18,
        _RESERVED_19 = 0x19,
        _RESERVED_1a = 0x1a,
        _RESERVED_1b = 0x1b,
        _RESERVED_1c = 0x1c,
        _RESERVED_1d = 0x1d,
        _RESERVED_1e = 0x1e,
        _RESERVED_1f = 0x1f,
    }
    impl ControlMessageType {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ControlMessageType {
            unsafe { core::mem::transmute(val & 0x1f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for ControlMessageType {
        #[inline(always)]
        fn from(val: u8) -> ControlMessageType {
            ControlMessageType::from_bits(val)
        }
    }
    impl From<ControlMessageType> for u8 {
        #[inline(always)]
        fn from(val: ControlMessageType) -> u8 {
            ControlMessageType::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum DataMessageType {
        _RESERVED_0 = 0x0,
        #[doc = "Source_Capabilities"]
        SOURCE_CAPABILITIES = 0x01,
        #[doc = "Request"]
        REQUEST = 0x02,
        #[doc = "BIST"]
        BIST = 0x03,
        #[doc = "Sink_Capabilities"]
        SINK_CAPABILITIES = 0x04,
        #[doc = "Battery_Status"]
        BATTERY_STATUS = 0x05,
        #[doc = "Alert"]
        ALERT = 0x06,
        #[doc = "Get_Country_Info"]
        GET_COUNTRY_INFO = 0x07,
        #[doc = "Enter_USB"]
        ENTER_USB = 0x08,
        #[doc = "EPR_Request"]
        EPR_REQUEST = 0x09,
        #[doc = "EPR_Mode"]
        EPR_MODE = 0x0a,
        #[doc = "Source_Info"]
        SOURCE_INFO = 0x0b,
        #[doc = "Revision"]
        REVISION = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        #[doc = "Vendor_Defined"]
        VENDOR_DEFINED = 0x0f,
        _RESERVED_10 = 0x10,
        _RESERVED_11 = 0x11,
        _RESERVED_12 = 0x12,
        _RESERVED_13 = 0x13,
        _RESERVED_14 = 0x14,
        _RESERVED_15 = 0x15,
        _RESERVED_16 = 0x16,
        _RESERVED_17 = 0x17,
        _RESERVED_18 = 0x18,
        _RESERVED_19 = 0x19,
        _RESERVED_1a = 0x1a,
        _RESERVED_1b = 0x1b,
        _RESERVED_1c = 0x1c,
        _RESERVED_1d = 0x1d,
        _RESERVED_1e = 0x1e,
        _RESERVED_1f = 0x1f,
    }
    impl DataMessageType {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> DataMessageType {
            unsafe { core::mem::transmute(val & 0x1f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for DataMessageType {
        #[inline(always)]
        fn from(val: u8) -> DataMessageType {
            DataMessageType::from_bits(val)
        }
    }
    impl From<DataMessageType> for u8 {
        #[inline(always)]
        fn from(val: DataMessageType) -> u8 {
            DataMessageType::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct EprmodAction(pub u8);
    impl EprmodAction {
        #[doc = "Enter (Sink Only)"]
        pub const ENTER: Self = Self(0x01);
        #[doc = "Enter Acknowledged (Source Only)"]
        pub const ENTER_ACKNOWLEDGED: Self = Self(0x02);
        #[doc = "Enter Succeeded (Source Only)"]
        pub const ENTER_SUCCEEDED: Self = Self(0x03);
        #[doc = "Enter Failed (Source Only)"]
        pub const ENTER_FAILED: Self = Self(0x04);
        #[doc = "Exit (Sink or Source)"]
        pub const EXIT: Self = Self(0x05);
    }
    impl EprmodAction {
        pub const fn from_bits(val: u8) -> EprmodAction {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl From<u8> for EprmodAction {
        #[inline(always)]
        fn from(val: u8) -> EprmodAction {
            EprmodAction::from_bits(val)
        }
    }
    impl From<EprmodAction> for u8 {
        #[inline(always)]
        fn from(val: EprmodAction) -> u8 {
            EprmodAction::to_bits(val)
        }
    }
    #[doc = "Table 6.54 “Extended Message Types”"]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum ExtendedMessageType {
        _RESERVED_0 = 0x0,
        SOURCE_CAPABILITIES_EXTENDED = 0x01,
        STATUS = 0x02,
        GET_BATTERY_CAP = 0x03,
        GET_BATTERY_STATUS = 0x04,
        BATTERY_CAPABILITIES = 0x05,
        GET_MANUFACTURER_INFO = 0x06,
        MANUFACTURER_INFO = 0x07,
        SECURITY_REQUEST = 0x08,
        SECURITY_RESPONSE = 0x09,
        FIRMWARE_UPDATE_REQUEST = 0x0a,
        FIRMWARE_UPDATE_RESPONSE = 0x0b,
        PPS_STATUS = 0x0c,
        COUNTRY_INFO = 0x0d,
        COUNTRY_CODES = 0x0e,
        SINK_CAPABILITIES_EXTENDED = 0x0f,
        EXTENDED_CONTROL = 0x10,
        EPR_SOURCE_CAPABILITIES = 0x11,
        EPR_SINK_CAPABILITIES = 0x12,
        _RESERVED_13 = 0x13,
        _RESERVED_14 = 0x14,
        _RESERVED_15 = 0x15,
        _RESERVED_16 = 0x16,
        _RESERVED_17 = 0x17,
        _RESERVED_18 = 0x18,
        _RESERVED_19 = 0x19,
        _RESERVED_1a = 0x1a,
        _RESERVED_1b = 0x1b,
        _RESERVED_1c = 0x1c,
        _RESERVED_1d = 0x1d,
        _RESERVED_1e = 0x1e,
        _RESERVED_1f = 0x1f,
    }
    impl ExtendedMessageType {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ExtendedMessageType {
            unsafe { core::mem::transmute(val & 0x1f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for ExtendedMessageType {
        #[inline(always)]
        fn from(val: u8) -> ExtendedMessageType {
            ExtendedMessageType::from_bits(val)
        }
    }
    impl From<ExtendedMessageType> for u8 {
        #[inline(always)]
        fn from(val: ExtendedMessageType) -> u8 {
            ExtendedMessageType::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum PdoType {
        #[doc = "Fixed supply"]
        FIXED = 0x0,
        #[doc = "Battery"]
        BATTERY = 0x01,
        #[doc = "Variable supply"]
        VARIABLE = 0x02,
        #[doc = "Augmented PDO"]
        AUGMENTED = 0x03,
    }
    impl PdoType {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PdoType {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PdoType {
        #[inline(always)]
        fn from(val: u8) -> PdoType {
            PdoType::from_bits(val)
        }
    }
    impl From<PdoType> for u8 {
        #[inline(always)]
        fn from(val: PdoType) -> u8 {
            PdoType::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum PeakCurrent {
        DEFAULT = 0x0,
        LOWOVERLOAD = 0x01,
        MEDIUMOVERLOAD = 0x02,
        HIGHOVERLOAD = 0x03,
    }
    impl PeakCurrent {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PeakCurrent {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PeakCurrent {
        #[inline(always)]
        fn from(val: u8) -> PeakCurrent {
            PeakCurrent::from_bits(val)
        }
    }
    impl From<PeakCurrent> for u8 {
        #[inline(always)]
        fn from(val: PeakCurrent) -> u8 {
            PeakCurrent::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum PortDataRole {
        #[doc = "UFP"]
        UFP = 0x0,
        #[doc = "DFP"]
        DFP = 0x01,
    }
    impl PortDataRole {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PortDataRole {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PortDataRole {
        #[inline(always)]
        fn from(val: u8) -> PortDataRole {
            PortDataRole::from_bits(val)
        }
    }
    impl From<PortDataRole> for u8 {
        #[inline(always)]
        fn from(val: PortDataRole) -> u8 {
            PortDataRole::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum PortPowerRole {
        #[doc = "Sink"]
        SINK = 0x0,
        #[doc = "Source"]
        SOURCE = 0x01,
    }
    impl PortPowerRole {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PortPowerRole {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PortPowerRole {
        #[inline(always)]
        fn from(val: u8) -> PortPowerRole {
            PortPowerRole::from_bits(val)
        }
    }
    impl From<PortPowerRole> for u8 {
        #[inline(always)]
        fn from(val: PortPowerRole) -> u8 {
            PortPowerRole::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum SpecRevision {
        #[doc = "Revision 1.0"]
        R1_0 = 0x0,
        #[doc = "Revision 2.0"]
        R2_0 = 0x01,
        #[doc = "Revision 3.0"]
        R3_0 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl SpecRevision {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> SpecRevision {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for SpecRevision {
        #[inline(always)]
        fn from(val: u8) -> SpecRevision {
            SpecRevision::from_bits(val)
        }
    }
    impl From<SpecRevision> for u8 {
        #[inline(always)]
        fn from(val: SpecRevision) -> u8 {
            SpecRevision::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum VdmCommandType {
        _RESERVED_0 = 0x0,
        DISCOVER_IDENTITY = 0x01,
        DISCOVER_SVIDS = 0x02,
        DISCOVER_MODES = 0x03,
        ENTER_MODAL = 0x04,
        EXIT_MODAL = 0x05,
        ATTENTION = 0x06,
        _RESERVED_7 = 0x07,
        _RESERVED_8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
        _RESERVED_10 = 0x10,
        _RESERVED_11 = 0x11,
        _RESERVED_12 = 0x12,
        _RESERVED_13 = 0x13,
        _RESERVED_14 = 0x14,
        _RESERVED_15 = 0x15,
        _RESERVED_16 = 0x16,
        _RESERVED_17 = 0x17,
        _RESERVED_18 = 0x18,
        _RESERVED_19 = 0x19,
        _RESERVED_1a = 0x1a,
        _RESERVED_1b = 0x1b,
        _RESERVED_1c = 0x1c,
        _RESERVED_1d = 0x1d,
        _RESERVED_1e = 0x1e,
        _RESERVED_1f = 0x1f,
    }
    impl VdmCommandType {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> VdmCommandType {
            unsafe { core::mem::transmute(val & 0x1f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for VdmCommandType {
        #[inline(always)]
        fn from(val: u8) -> VdmCommandType {
            VdmCommandType::from_bits(val)
        }
    }
    impl From<VdmCommandType> for u8 {
        #[inline(always)]
        fn from(val: VdmCommandType) -> u8 {
            VdmCommandType::to_bits(val)
        }
    }
}
