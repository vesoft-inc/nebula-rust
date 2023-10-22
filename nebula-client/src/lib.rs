//
pub mod v3;

//
pub trait Version {}

pub struct VersionV1;
impl Version for VersionV1 {}

pub struct VersionV2;
impl Version for VersionV2 {}

pub struct VersionV3;
impl Version for VersionV3 {}
