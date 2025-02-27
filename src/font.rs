use std::fmt::Error;
use std::path::Path;
use std::{ffi::CString, ptr::NonNull};
use stereokit_sys::{_font_t, default_id_font, font_create, font_find};
use ustr::ustr;

use crate::lifecycle::StereoKitContext;

pub struct Font {
	pub(crate) font: NonNull<_font_t>,
}

impl Font {
	pub fn from_file(_sk: &impl StereoKitContext, file: impl AsRef<Path>) -> Option<Self> {
		let file_path = ustr(file.as_ref().as_os_str().to_str()?);

		Some(Font {
			font: NonNull::new(unsafe { stereokit_sys::font_create(file_path.as_char_ptr()) })?,
		})
	}
	pub fn default(_sk: &impl StereoKitContext) -> Self {
		let default_id = ustr("default/font");

		Font {
			font: NonNull::new(unsafe { font_find(default_id.as_char_ptr()) }).unwrap(),
		}
	}
}
