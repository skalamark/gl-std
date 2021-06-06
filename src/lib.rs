// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use gl_core::preludes::*;

mod preludes;

pub struct Std {}

impl Std {
	pub fn new() -> Env {
		let mut env: Env = Env::new();
		let mut env_std = HashMap::new();

		preludes::load(&mut env);

		env.set("std", Object::ModuleRust(format!("std"), env_std));
		env
	}

	fn std(env: &mut HashMap<String, Object>) {}
}
