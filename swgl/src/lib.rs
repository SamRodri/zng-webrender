/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#![crate_name = "zng_swgl"]
#![crate_type = "lib"]

extern crate gleam;

mod swgl_fns;

pub use crate::swgl_fns::*;
