#![allow(clippy::unwrap_used)]

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

use crate::win::psuedocon::PsuedoCon;
use filedescriptor::FileDescriptor;
use filedescriptor::Pipe;
use portable_pty::PtySize;
use std::mem::ManuallyDrop;
use std::os::windows::io::RawHandle;
use std::ptr;
use winapi::um::wincon::COORD;

fn create_conpty_handles(
    size: PtySize,
) -> anyhow::Result<(PsuedoCon, FileDescriptor, FileDescriptor)> {
    let stdin = Pipe::new()?;
    let stdout = Pipe::new()?;

    let con = PsuedoCon::new(
        COORD {
            X: size.cols as i16,
            Y: size.rows as i16,
        },
        stdin.read,
        stdout.write,
    )?;

    Ok((con, stdin.write, stdout.read))
}

pub struct RawConPty {
    con: PsuedoCon,
    input_write: FileDescriptor,
    output_read: FileDescriptor,
}

impl RawConPty {
    pub fn new(cols: i16, rows: i16) -> anyhow::Result<Self> {
        let (con, input_write, output_read) = create_conpty_handles(PtySize {
            rows: rows as u16,
            cols: cols as u16,
            pixel_width: 0,
            pixel_height: 0,
        })?;
        Ok(Self {
            con,
            input_write,
            output_read,
        })
    }

    pub fn pseudoconsole_handle(&self) -> RawHandle {
        self.con.raw_handle()
    }

    pub fn into_handles(self) -> (PsuedoCon, FileDescriptor, FileDescriptor) {
        let me = ManuallyDrop::new(self);
        unsafe {
            (
                ptr::read(&me.con),
                ptr::read(&me.input_write),
                ptr::read(&me.output_read),
            )
        }
    }
}

