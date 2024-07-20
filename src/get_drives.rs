use bitvec::prelude::*;
use std::{
    collections::HashSet,
    error::Error,
    fmt::{Debug, Display, Formatter},
};
use windows::Win32::{Foundation::GetLastError, Storage::FileSystem::GetLogicalDrives};

const INVALID_DRIVE_LETTER_BITMASK: u32 = 0b11111100_00000000_00000000_00000000;

pub enum GetLogicalDrivesError {
    TooManyDrivesError,
    ApiError(u32),
}

impl Display for GetLogicalDrivesError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Debug for GetLogicalDrivesError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GetLogicalDrivesError::TooManyDrivesError => write!(f, "TooManyDrives"),
            GetLogicalDrivesError::ApiError(code) => write!(f, "ApiError({code})"),
        }
    }
}

impl Error for GetLogicalDrivesError {}

/// https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getlogicaldrives
pub fn get_drives() -> Result<HashSet<char>, GetLogicalDrivesError> {
    let drives_bitmap = unsafe { GetLogicalDrives() };

    // If the function fails, the return value is zero. To get extended error information, call GetLastError.
    if drives_bitmap == 0 {
        let err = unsafe { GetLastError() };
        Err(GetLogicalDrivesError::ApiError(err.0))
    } else if drives_bitmap & INVALID_DRIVE_LETTER_BITMASK != 0 {
        Err(GetLogicalDrivesError::TooManyDrivesError)
    } else {
        Ok(drives_bitmap
            .view_bits::<Lsb0>()
            .iter()
            .zip('A'..='Z')
            .filter_map(|(bit, drive_letter)| {
                // a bit derefs into a bool
                if *bit {
                    Some(drive_letter)
                } else {
                    None
                }
            })
            .collect())
    }
}
