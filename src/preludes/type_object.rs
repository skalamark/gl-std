// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use gl_core::preludes::*;

pub fn type_object(args: Vec<Object>) -> Result<Object, Exception> {
	if args.len() > 0 {
		Ok(Object::String(args[0].typer().to_string()))
	} else {
		Ok(Object::Null)
	}
}
