//! Pure Rust representation of the USB Power Delivery 3.2 specification.
//!
//! - Messages
//!   - Header
//!   - ExtendedHeader
//! - DataObjects
//!   - PDO: Power Data Object
//!     - APDO: Augmented Power Data Object
//!   - RDO: Request Data Object
//!   - VDO: Vendor Defined Data Object
//!   - BDO: BIST Data Object
//!   - BSDO: Battery Status Data Object
//!   - ADO: Alert Data Object

#[allow(non_camel_case_types)]
mod usbpd_v3;

pub use usbpd_v3::regs::*;
pub use usbpd_v3::vals;
