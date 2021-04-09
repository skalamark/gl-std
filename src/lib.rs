// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use gl_core::preludes::*;

mod preludes;

pub struct Std {}

impl Std {
	pub fn new() -> Env {
		let mut env: Env = Env::new();
		preludes::load(&mut env);
		env
	}
}
