use std::process::ExitCode;

#[repr(u8)]
#[allow(dead_code)]
pub enum IcingaCode {
    Ok = 0,
    Warning = 1,
    Critical = 2,
    Unknown = 3
}

impl From<IcingaCode> for ExitCode {
    fn from(value: IcingaCode) -> Self {
        ExitCode::from(value as u8)
    }
}
