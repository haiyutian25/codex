#![allow(clippy::expect_used)]
#![allow(clippy::upper_case_acronyms)]

// This file is copied from https://github.com/wezterm/wezterm (MIT license).
// Copyright (c) 2018-Present Wez Furlong
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use anyhow::Error;
use anyhow::ensure;
use filedescriptor::FileDescriptor;
use lazy_static::lazy_static;
use shared_library::shared_library;
use std::mem;
use std::os::windows::io::AsRawHandle;
use std::path::Path;
use winapi::shared::minwindef::DWORD;
use winapi::shared::ntdef::NTSTATUS;
use winapi::shared::ntstatus::STATUS_SUCCESS;
use winapi::shared::winerror::HRESULT;
use winapi::shared::winerror::S_OK;
use winapi::um::handleapi::*;
use winapi::um::wincon::COORD;
use winapi::um::winnt::HANDLE;
use winapi::um::winnt::OSVERSIONINFOW;

pub type HPCON = HANDLE;

pub const PSEUDOCONSOLE_RESIZE_QUIRK: DWORD = 0x2;
#[allow(dead_code)]
pub const PSEUDOCONSOLE_PASSTHROUGH_MODE: DWORD = 0x8;

// https://learn.microsoft.com/en-gb/windows/console/createpseudoconsole
// https://learn.microsoft.com/en-gb/windows/release-health/release-information
const MIN_CONPTY_BUILD: u32 = 17_763;

shared_library!(ConPtyFuncs,
    pub fn CreatePseudoConsole(
        size: COORD,
        hInput: HANDLE,
        hOutput: HANDLE,
        flags: DWORD,
        hpc: *mut HPCON
    ) -> HRESULT,
    pub fn ResizePseudoConsole(hpc: HPCON, size: COORD) -> HRESULT,
    pub fn ClosePseudoConsole(hpc: HPCON),
);

shared_library!(Ntdll,
    pub fn RtlGetVersion(
        version_info: *mut OSVERSIONINFOW
    ) -> NTSTATUS,
);

lazy_static! {
    static ref CONPTY: ConPtyFuncs = ConPtyFuncs::open(Path::new("kernel32.dll")).expect(
        "this system does not support conpty.  Windows 10 October 2018 or newer is required",
    );
}

pub fn conpty_supported() -> bool {
    windows_build_number().is_some_and(|build| build >= MIN_CONPTY_BUILD)
}

fn windows_build_number() -> Option<u32> {
    let ntdll = Ntdll::open(Path::new("ntdll.dll")).ok()?;
    let mut info: OSVERSIONINFOW = unsafe { mem::zeroed() };
    info.dwOSVersionInfoSize = mem::size_of::<OSVERSIONINFOW>() as u32;
    let status = unsafe { (ntdll.RtlGetVersion)(&mut info) };
    if status == STATUS_SUCCESS {
        Some(info.dwBuildNumber)
    } else {
        None
    }
}

pub struct PsuedoCon {
    con: HPCON,
    // CreatePseudoConsole borrows these pipe handles for the lifetime of the
    // pseudoconsole, so we must keep owning them until ClosePseudoConsole.
    _input: FileDescriptor,
    _output: FileDescriptor,
}

unsafe impl Send for PsuedoCon {}
unsafe impl Sync for PsuedoCon {}

impl Drop for PsuedoCon {
    fn drop(&mut self) {
        unsafe { (CONPTY.ClosePseudoConsole)(self.con) };
    }
}

impl PsuedoCon {
    pub fn raw_handle(&self) -> HPCON {
        self.con
    }

    pub fn new(size: COORD, input: FileDescriptor, output: FileDescriptor) -> Result<Self, Error> {
        let mut con: HPCON = INVALID_HANDLE_VALUE;
        let result = unsafe {
            (CONPTY.CreatePseudoConsole)(
                size,
                input.as_raw_handle() as _,
                output.as_raw_handle() as _,
                PSEUDOCONSOLE_RESIZE_QUIRK,
                &mut con,
            )
        };
        ensure!(
            result == S_OK,
            "failed to create psuedo console: HRESULT {result}"
        );
        Ok(Self {
            con,
            _input: input,
            _output: output,
        })
    }

    pub fn resize(&self, size: COORD) -> Result<(), Error> {
        let result = unsafe { (CONPTY.ResizePseudoConsole)(self.con, size) };
        ensure!(
            result == S_OK,
            "failed to resize console to {}x{}: HRESULT: {}",
            size.X,
            size.Y,
            result
        );
        Ok(())
    }

}

#[cfg(test)]
mod tests {
    use super::MIN_CONPTY_BUILD;
    use super::windows_build_number;

    #[test]
    fn windows_build_number_returns_value() {
        // We can't stably check the version of the GH workers, but we can
        // at least check that this.
        let version = windows_build_number().unwrap();
        assert!(version > MIN_CONPTY_BUILD);
    }
}
