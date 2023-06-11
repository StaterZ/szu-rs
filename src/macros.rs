#[macro_export]
macro_rules! replace_macro_arg {
	($in:tt, $( $out:tt )* ) => {
		$( $out )*
	};
}

#[macro_export]
macro_rules! tag_enum_helper {
	($value_name:ident :: $n:ident ( $( $p:ty ),* ) ) => {
		panic!("bad tag conversion")
	};
	($value_name:ident :: $n:ident) => {
		$value_name :: $n
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

		impl From<$value_name> for $tag_name {
			fn from(value: $value_name) -> Self {
				match value {
					$( $value_name :: $n $( ( $( replace_macro_arg!($p, _) ),* ) )? => $tag_name :: $n ),+
				}
			}
		}
		
		impl From<$tag_name> for $value_name {
			fn from(value: $tag_name) -> Self {
				match value {
					$( $tag_name :: $n => tag_enum_helper!($value_name :: $n $( ( $( $p ),* ) )?) ),+
				}
			}
		}
	};
}

#[macro_export]
macro_rules! ternary {
	($cond:expr => $trueExpr:expr, $falseExpr:expr) => {
		if $cond { $trueExpr } else { $falseExpr }
	}
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
