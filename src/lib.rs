// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use gl_core::preludes::*;

pub struct Std {}

impl Std {
	pub fn new() -> Env {
		let env: Env = Env::new();
		env
	}
}
