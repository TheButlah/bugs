// This one works
#[safer_ffi::derive_ReprC]
#[ReprC::opaque]
pub struct NotGated {
    bar: String,
}

// This one doesn't work
#[cfg_attr(feature = "c_api", safer_ffi::derive_ReprC, ReprC::opaque)]
pub struct Gated {
    bar: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foobar() {
        let not_gated = NotGated {
            bar: "yeet".to_string(),
        };
        let gated = Gated {
            bar: "yeet".to_string(),
        };
        // Intellisense works for `not_gated` but fails on `gated`, go ahead and try it
        assert_eq!(gated.bar, "yeet");
        assert_eq!(not_gated.bar, "yeet");
    }
}
