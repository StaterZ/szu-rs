#![allow(dead_code)]

#[derive(Debug)]
pub enum EnumTagConversionError {
	VariantHasValue,
}

#[macro_export]
macro_rules! replace_macro_arg {
	($in:tt, $( $out:tt )* ) => {
		$( $out )*
	};
}

#[macro_export]
macro_rules! tag_enum_variant_converter {
	($value_name:ident :: $n:ident ( $( $p:ty ),* ) ) => {
		Err($crate::macros::EnumTagConversionError::VariantHasValue)
	};
	($value_name:ident :: $n:ident) => {
		Ok($value_name :: $n)
	};
}

#[macro_export]
macro_rules! tag_enum {
	(
		$vis:vis enum (
			$( # $value_meta:tt )? $value_name:ident ,
			$( # $tag_meta:tt )? $tag_name:ident $(,)?
		) {
			$( $n:ident $( ( $( $p:ty ),* ) )? ),+ $(,)?
		}
	) => {
		$( # $value_meta )?
		$vis enum $value_name {
			$( $n $( ( $( $p ),* ) )? ),+
		}

		$( # $tag_meta )?
		$vis enum $tag_name {
			$( $n ),+
		}

		impl From<&$value_name> for $tag_name {
			fn from(value: &$value_name) -> Self {
				match value {
					$( $value_name :: $n $( ( $( $crate::replace_macro_arg!($p, _) ),* ) )? => $tag_name :: $n ),+
				}
			}
		}
		
		impl TryFrom<$tag_name> for $value_name {
			type Error = $crate::macros::EnumTagConversionError;

			fn try_from(value: $tag_name) -> Result<Self, Self::Error> {
				match value {
					$( $tag_name :: $n => $crate::tag_enum_variant_converter!($value_name :: $n $( ( $( $p ),* ) )?) ),+
				}
			}
		}
	};
}

#[macro_export]
macro_rules! extract_variant {
	($e:expr, $p:pat => $result:expr) => {
		match $e {
			$p => Some($result),
			_ => None,
		}
	};
}

#[macro_export]
macro_rules! set_chain {
	($lhs:expr, $rhs:expr) => {
		{
			$lhs = $rhs;
			$lhs
		}
	}
}

#[macro_export]
macro_rules! unwrap_or_return (
	($val:expr $(, $ret_val:expr)?) => {
		match $val {
			Some(val) => val,
			None => return $($ret_val)?,
		}
	};
);

//using this instead of a const func cuz they too unstable with trait bounds
#[macro_export]
macro_rules! int_div_round {
	($lhs:expr, $rhs:expr) => {
		{
			let (lhs, rhs) = ($lhs, $rhs);
			(lhs + (rhs / 2)) / rhs
		}
	}
}

#[macro_export]
macro_rules! int_div_ceil {
	($lhs:expr, $rhs:expr) => {
		{
			let (lhs, rhs) = ($lhs, $rhs);
			(lhs + (rhs - 1)) / rhs
		}
	};
}

#[macro_export]
macro_rules! flush_print {
	($($all:tt)*) => {
		print!($($all)*);
		_ = std::io::stdout().flush();
	};
}
