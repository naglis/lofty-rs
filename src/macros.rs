#[doc(hidden)]
#[macro_export]
macro_rules! impl_tag {
	($tag:ident, $inner:ident, $tag_type:expr) => {
		#[doc(hidden)]
		pub struct $tag($inner);

		impl Default for $tag {
			fn default() -> Self {
				Self($inner::default())
			}
		}

		impl $tag {
			pub fn new() -> Self {
				Self::default()
			}
			#[allow(clippy::missing_errors_doc)]
			pub fn read_from_path<P>(path: P, tag_type: Option<TagType>) -> Result<Self>
			where
				P: AsRef<Path>,
			{
				Ok(Self($inner::from_path(path, tag_type)?))
			}
		}

		use std::any::Any;

		impl ToAnyTag for $tag {
			fn to_anytag(&self) -> AnyTag<'_> {
				self.into()
			}
		}

		impl ToAny for $tag {
			fn to_any(&self) -> &dyn Any {
				self
			}
			fn to_any_mut(&mut self) -> &mut dyn Any {
				self
			}
		}

		impl AudioTag for $tag {}

		// From wrapper to inner (same type)
		impl From<$tag> for $inner {
			fn from(inp: $tag) -> Self {
				inp.0
			}
		}

		// From inner to wrapper (same type)
		impl From<$inner> for $tag {
			fn from(inp: $inner) -> Self {
				Self(inp)
			}
		}

		// From dyn AudioTag to wrapper (any type)
		impl From<Box<dyn AudioTag>> for $tag {
			fn from(inp: Box<dyn AudioTag>) -> Self {
				let mut inp = inp;
				if let Some(t_refmut) = inp.to_any_mut().downcast_mut::<$tag>() {
					let t = std::mem::replace(t_refmut, $tag::new()); // TODO: can we avoid creating the dummy tag?
					t
				} else {
					let mut t = inp.to_dyn_tag($tag_type);
					let t_refmut = t.to_any_mut().downcast_mut::<$tag>().unwrap();
					let t = std::mem::replace(t_refmut, $tag::new());
					t
				}
			}
		}

		// From dyn AudioTag to inner (any type)
		impl From<Box<dyn AudioTag>> for $inner {
			fn from(inp: Box<dyn AudioTag>) -> Self {
				let t: $tag = inp.into();
				t.into()
			}
		}
	};
}

/// Convert a concrete tag type into another
#[macro_export]
macro_rules! convert {
	($inp:expr, $target_type:ty) => {
		$target_type::from(inp.to_anytag())
	};
}