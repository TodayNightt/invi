#[macro_export]
macro_rules! get {
    (@parse {$val:ident} [$ty:ident] $key:ident . $($tt:tt)+) => {
        match $val.as_object().and_then(|obj| obj.get(stringify!($key))) {
            Some(val) => $crate::get!(@parse {val} [$ty] $($tt)+),
            None => None,
        }
    };

    (@parse {$val:ident} [$ty:ident] $num:literal . $($tt:tt)+) => {
        match $val.as_array().and_then(|arr| arr.get($num)) {
            Some(val) => $crate::get!(@parse {val} [$ty] $($tt)+),
            None => None,
        }
    };

    // Final object key access
   (@parse {$val:ident} [$ty:ident] $key:ident) => {
        match $val.as_object().and_then(|obj| obj.get(stringify!($key))) {
            Some(val) => $crate::get!(@cast $ty val),
            None => None,
        }
    };

    // Final array index access
    (@parse {$val:ident} [$ty:ident] $num:literal) => {
        match $val.as_array().and_then(|arr| arr.get($num)) {
            Some(val) => $crate::get!(@cast $ty val),
            None => None,
        }
    };

    (@cast number $val:ident) => {
        $val.as_number()
    };

    (@cast string $val:ident) => {
        $val.as_string()
    };

    (@cast value $val:ident) => {
        Some($val.clone())
    };

    (@cast array $val:ident) => {
        $val.as_array().cloned()
    };

    (@cast object $val:ident) => {
        $val.as_object().cloned()
    };


    // Entrypoint
    ($val:ident, $ty:ident, $($tt:tt)+) => {
        {
            let val = $val.as_value();
            $crate::get!(@parse {val} [$ty] $($tt)*)
        }
    };

    ($val:ident, $($tt:tt)+) => {
        {
            let val = $val.as_value();
            $crate::get!(@parse {val} [value] $($tt)*)
        }
    };
}