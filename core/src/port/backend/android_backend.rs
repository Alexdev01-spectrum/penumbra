/*
    SPDX-License-Identifier: AGPL-3.0-or-later
    SPDX-FileCopyrightText: 2026 Alexdev01-spectrum
*/

use std::time::Duration;

use crate::error::{ConnectionError, Error, Result};
use crate::port::{ConnectionType, MtkPort, MIN_TIMEOUT};

/// Android builds supply their USB transport from the Android UsbManager/JNI layer.
/// This placeholder keeps the generic Penumbra core buildable without pulling in a
/// desktop USB enumeration backend. The actual Android transport is implemented by
/// the consuming application and passed directly to `DeviceBuilder<P>`.
pub struct AndroidPort {
    conn_type: ConnectionType,
    timeout: Duration,
}

impl AndroidPort {
    pub const fn new(conn_type: ConnectionType) -> Self {
        Self { conn_type, timeout: MIN_TIMEOUT }
    }
}

fn unsupported() -> Error {
    Error::Connection(ConnectionError::OpenFailed(
        "Android backend requires the application's Android USB transport".to_string(),
    ))
}

impl MtkPort for AndroidPort {
    fn open(&mut self) -> Result<()> { Err(unsupported()) }
    fn close(&mut self) -> Result<()> { Ok(()) }
    fn reenumerate(&mut self, _vid: u16, _pid: u16) -> Result<()> { Err(unsupported()) }
    fn read_exact(&mut self, _buf: &mut [u8]) -> Result<usize> { Err(unsupported()) }
    fn write_all(&mut self, _buf: &[u8]) -> Result<()> { Err(unsupported()) }
    fn flush(&mut self) -> Result<()> { Ok(()) }
    fn get_baudrate(&self) -> u32 { 0 }
    fn get_port_name(&self) -> String { "Android USB transport".to_string() }
    fn set_timeout(&mut self, timeout: Duration) -> Result<()> { self.timeout = timeout; Ok(()) }
    fn get_timeout(&self) -> Duration { self.timeout }
    fn connection_type(&self) -> ConnectionType { self.conn_type }
    fn set_connection_type(&mut self, connection_type: ConnectionType) -> Result<()> { self.conn_type = connection_type; Ok(()) }
    fn ctrl_out(&mut self, _request_type: u8, _request: u8, _value: u16, _index: u16, _data: &[u8]) -> Result<()> { Err(unsupported()) }
    fn ctrl_in(&mut self, _request_type: u8, _request: u8, _value: u16, _index: u16, _len: usize) -> Result<Vec<u8>> { Err(unsupported()) }
}
