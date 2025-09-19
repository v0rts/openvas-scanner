// SPDX-FileCopyrightText: 2023 Greenbone AG
//
// SPDX-License-Identifier: GPL-2.0-or-later WITH x11vnc-openssl-exception

#![doc = include_str!("README.md")]
mod commands;
mod connection;
mod response;
mod scanner;

#[cfg(test)]
mod tests;

pub use response::ResultType as OspResultType;
pub use response::ScanResult as OspScanResult;
pub use scanner::Scanner;

#[cfg(test)]
use response::Response as OspResponse;
#[cfg(test)]
use response::Scan as OspScan;
#[cfg(test)]
use response::ScanStatus as OspScanStatus;
