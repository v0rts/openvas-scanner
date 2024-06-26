// SPDX-FileCopyrightText: 2024 Greenbone AG
//
// SPDX-License-Identifier: GPL-2.0-or-later WITH x11vnc-openssl-exception

mod frame_forgery;
mod packet_forgery;
mod raw_ip_utils;
use nasl_builtin_utils::{Context, NaslVars, Register};

pub struct RawIp;

impl nasl_builtin_utils::NaslFunctionExecuter for RawIp {
    fn nasl_fn_execute(
        &self,
        name: &str,
        register: &Register,
        context: &Context,
    ) -> Option<nasl_builtin_utils::NaslResult> {
        frame_forgery::lookup(name)
            .map(|x| x(register, context))
            .or_else(|| packet_forgery::lookup(name).map(|x| x(register, context)))
    }

    fn nasl_fn_defined(&self, name: &str) -> bool {
        frame_forgery::lookup(name)
            .or_else(|| packet_forgery::lookup(name))
            .is_some()
    }
}

impl nasl_builtin_utils::NaslVarDefiner for RawIp {
    fn nasl_var_define(&self) -> NaslVars {
        let mut raw_ip_vars = packet_forgery::expose_vars();
        raw_ip_vars.extend(frame_forgery::expose_vars());
        raw_ip_vars
    }
}
