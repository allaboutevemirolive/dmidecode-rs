use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum BiosParseError {
    BiosVendor,
    BiosVersion,
    BiosReleaseDate,
    BiosRevision,
    FirmwareRevision,
    SystemManufacturer,
    SystemProductName,
    SystemVersion,
    SystemSerialNumber,
    SystemUuid,
    SystemSkuNumber,
    SystemFamily,
    BaseboardManufacturer,
    BaseboardProductName,
    BaseboardVersion,
    BaseboardSerialNumber,
    BaseboardAssetTag,
    ChassisManufacturer,
    ChassisType,
    ChassisVersion,
    ChassisSerialNumber,
    ChassisAssetTag,
    ProcessorFamily,
    ProcessorManufacturer,
    ProcessorVersion,
    ProcessorFrequency,
}

impl Error for BiosParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl Display for BiosParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Here we can match and turn each arm into a human readable statement.
        // We have other variants to add so we will wait before doing so.
        write!(f, "{:?}", &self)
    }
}
