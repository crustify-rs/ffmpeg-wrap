//! Wrappers for `libavutil/csp.c`.

use core::ptr::{NonNull, addr_of, addr_of_mut};

use ffibox::{CVal, CValued};

use crate::ffi;
use crate::rational::{AVRationalMut, AVRationalRef};

ffibox::define_ctype!(
    /// Wraps: AVLumaCoefficients
    ///
    /// ABI-compatible inline luma coefficients used by colorspace conversion.
    /// The structure owns no resources.
    AVLumaCoefficients,
    AVLumaCoefficientsRef,
    AVLumaCoefficientsMut,
    ffi::AVLumaCoefficients
);

// SAFETY: all three fields are inline `AVRational` values with no teardown.
unsafe impl CValued for AVLumaCoefficients {
    unsafe fn c_dispose(_this: NonNull<Self>) {}
}

impl AVLumaCoefficients {
    /// Creates zero-initialized coefficients in owned inline storage.
    #[must_use]
    pub fn new() -> CVal<Self> {
        CVal::new(Self::zeroed())
    }
}

macro_rules! rational_field {
    ($(#[$meta:meta])* $field:ident, $field_mut:ident) => {
        impl<'a> AVLumaCoefficientsRef<'a> {
            $(#[$meta])*
            #[must_use]
            pub fn $field(&self) -> AVRationalRef<'a> {
                // SAFETY: the projected field is an initialized inline
                // rational that lives for the parent handle's lifetime.
                unsafe {
                    AVRationalRef::from_ptr(addr_of!((*self.as_ptr()).$field).cast_mut())
                }
                .expect("an inline field is non-null")
            }
        }

        impl AVLumaCoefficientsMut<'_> {
            #[doc = concat!("Exclusively borrows [`", stringify!($field), "`](AVLumaCoefficientsRef::", stringify!($field), ").")]
            #[must_use]
            pub fn $field_mut(&mut self) -> AVRationalMut<'_> {
                // SAFETY: the exclusive parent handle supplies write
                // provenance to the initialized inline field for this reborrow.
                unsafe {
                    AVRationalMut::from_ptr(addr_of_mut!((*self.as_mut_ptr()).$field))
                }
                .expect("an inline field is non-null")
            }
        }
    };
}

rational_field!(
    /// Field: AVLumaCoefficients.cb
    cb,
    cb_mut
);
rational_field!(
    /// Field: AVLumaCoefficients.cg
    cg,
    cg_mut
);
rational_field!(
    /// Field: AVLumaCoefficients.cr
    cr,
    cr_mut
);

#[cfg(test)]
mod tests {
    use core::mem::{align_of, size_of};

    use super::*;

    #[test]
    fn layout_and_rational_fields_match_c() {
        assert_eq!(
            size_of::<AVLumaCoefficients>(),
            size_of::<ffi::AVLumaCoefficients>()
        );
        assert_eq!(
            align_of::<AVLumaCoefficients>(),
            align_of::<ffi::AVLumaCoefficients>()
        );

        let mut coefficients = AVLumaCoefficients::new();
        let mut view = coefficients.as_mut();
        view.cr_mut().set_num(299);
        view.cg_mut().set_num(587);
        view.cb_mut().set_num(114);
        view.cr_mut().set_den(1_000);
        view.cg_mut().set_den(1_000);
        view.cb_mut().set_den(1_000);

        let view = coefficients.as_ref();
        assert_eq!((view.cr().num(), view.cr().den()), (299, 1_000));
        assert_eq!((view.cg().num(), view.cg().den()), (587, 1_000));
        assert_eq!((view.cb().num(), view.cb().den()), (114, 1_000));
    }
}

ffibox::define_ctype!(
    /// Wraps: AVCIExy
    ///
    /// ABI-compatible CIE 1931 chromaticity coordinates.
    AVCIExy,
    AVCIExyRef,
    AVCIExyMut,
    ffi::AVCIExy
);

// SAFETY: AVCIExy contains two by-value AVRational pairs and owns no resource,
// so disposing an initialized inline value is always a no-op.
unsafe impl CValued for AVCIExy {
    unsafe fn c_dispose(_this: NonNull<Self>) {}
}

impl AVCIExy {
    /// Creates owned inline chromaticity coordinates.
    #[must_use]
    pub fn new(x: AVRationalRef<'_>, y: AVRationalRef<'_>) -> CVal<Self> {
        CVal::new(Self(ffibox::CType::new(ffi::AVCIExy {
            x: x.copy_ffi(),
            y: y.copy_ffi(),
        })))
    }
}

impl<'a> AVCIExyRef<'a> {
    /// Field: AVCIExy.x
    #[must_use]
    pub fn x(&self) -> AVRationalRef<'a> {
        // SAFETY: raw-place projection locates the initialized inline rational
        // without forming a reference. It lives for the coordinate handle's
        // lifetime and is only exposed through a shared handle.
        unsafe {
            AVRationalRef::from_ptr(addr_of!((*self.as_ptr()).x).cast_mut())
                .expect("an embedded field is non-null")
        }
    }

    /// Field: AVCIExy.y
    #[must_use]
    pub fn y(&self) -> AVRationalRef<'a> {
        // SAFETY: as `x`, for the second initialized inline rational.
        unsafe {
            AVRationalRef::from_ptr(addr_of!((*self.as_ptr()).y).cast_mut())
                .expect("an embedded field is non-null")
        }
    }
}

impl AVCIExyMut<'_> {
    /// Exclusively borrows the x coordinate.
    #[must_use]
    pub fn x_mut(&mut self) -> AVRationalMut<'_> {
        // SAFETY: the exclusive parent handle supplies write provenance and
        // the result is bound to its mutable borrow.
        unsafe {
            AVRationalMut::from_ptr(addr_of_mut!((*self.as_mut_ptr()).x))
                .expect("an embedded field is non-null")
        }
    }

    /// Exclusively borrows the y coordinate.
    #[must_use]
    pub fn y_mut(&mut self) -> AVRationalMut<'_> {
        // SAFETY: as `x_mut`, for the second inline rational.
        unsafe {
            AVRationalMut::from_ptr(addr_of_mut!((*self.as_mut_ptr()).y))
                .expect("an embedded field is non-null")
        }
    }
}

#[cfg(test)]
mod avciexy_tests {
    use core::mem::{align_of, size_of};

    use crate::rational::AVRational;

    use super::*;

    #[test]
    fn layout_and_nested_handles_match_c() {
        assert_eq!(size_of::<AVCIExy>(), size_of::<ffi::AVCIExy>());
        assert_eq!(align_of::<AVCIExy>(), align_of::<ffi::AVCIExy>());

        let x = AVRational::new(3, 10);
        let y = AVRational::new(4, 10);
        let mut xy = AVCIExy::new(x.as_ref(), y.as_ref());
        assert_eq!(xy.as_ref().x().num(), 3);
        assert_eq!(xy.as_ref().y().den(), 10);

        xy.as_mut().x_mut().set_num(5);
        xy.as_mut().y_mut().set_den(12);
        assert_eq!(xy.as_ref().x().num(), 5);
        assert_eq!(xy.as_ref().y().den(), 12);
    }
}

ffibox::define_ctype!(
    /// Wraps: AVPrimaryCoefficients
    ///
    /// ABI-compatible CIE 1931 red, green, and blue primary locations. The
    /// structure contains only inline chromaticity coordinates.
    AVPrimaryCoefficients,
    AVPrimaryCoefficientsRef,
    AVPrimaryCoefficientsMut,
    ffi::AVPrimaryCoefficients
);

// SAFETY: all three fields are initialized inline `AVCIExy` values with no
// teardown operation or separately owned resource.
unsafe impl CValued for AVPrimaryCoefficients {
    unsafe fn c_dispose(_this: NonNull<Self>) {}
}

impl AVPrimaryCoefficients {
    /// Creates zero-initialized primary coordinates in owned inline storage.
    #[must_use]
    pub fn new() -> CVal<Self> {
        CVal::new(Self::zeroed())
    }
}

macro_rules! primary_field {
    ($(#[$meta:meta])* $field:ident, $field_mut:ident) => {
        impl<'a> AVPrimaryCoefficientsRef<'a> {
            $(#[$meta])*
            #[must_use]
            pub fn $field(&self) -> AVCIExyRef<'a> {
                // SAFETY: raw-place projection locates an initialized inline
                // coordinate that lives for the parent handle's lifetime.
                unsafe {
                    AVCIExyRef::from_ptr(addr_of!((*self.as_ptr()).$field).cast_mut())
                        .expect("an embedded field is non-null")
                }
            }
        }

        impl AVPrimaryCoefficientsMut<'_> {
            #[doc = concat!("Exclusively borrows [`", stringify!($field), "`](AVPrimaryCoefficientsRef::", stringify!($field), ").")]
            #[must_use]
            pub fn $field_mut(&mut self) -> AVCIExyMut<'_> {
                // SAFETY: the exclusive parent handle supplies write
                // provenance to this initialized inline coordinate for the
                // returned reborrow.
                unsafe {
                    AVCIExyMut::from_ptr(addr_of_mut!((*self.as_mut_ptr()).$field))
                        .expect("an embedded field is non-null")
                }
            }
        }
    };
}

primary_field!(
    /// Field: AVPrimaryCoefficients.r
    r,
    r_mut
);
primary_field!(
    /// Field: AVPrimaryCoefficients.g
    g,
    g_mut
);
primary_field!(
    /// Field: AVPrimaryCoefficients.b
    b,
    b_mut
);

#[cfg(test)]
mod primary_coefficients_tests {
    use core::mem::{align_of, size_of};

    use super::*;

    #[test]
    fn layout_and_nested_coordinate_access_match_c() {
        assert_eq!(
            size_of::<AVPrimaryCoefficients>(),
            size_of::<ffi::AVPrimaryCoefficients>()
        );
        assert_eq!(
            align_of::<AVPrimaryCoefficients>(),
            align_of::<ffi::AVPrimaryCoefficients>()
        );

        let mut primaries = AVPrimaryCoefficients::new();
        let mut view = primaries.as_mut();
        view.r_mut().x_mut().set_num(64);
        view.g_mut().y_mut().set_num(60);
        view.b_mut().x_mut().set_den(100);

        let view = primaries.as_ref();
        assert_eq!(view.r().x().num(), 64);
        assert_eq!(view.g().y().num(), 60);
        assert_eq!(view.b().x().den(), 100);
    }
}

ffibox::define_ctype!(
    /// Wraps: AVColorPrimariesDesc
    ///
    /// ABI-compatible complete color-gamut description. Both the white point
    /// and primary coordinates are stored inline and own no resources.
    AVColorPrimariesDesc,
    AVColorPrimariesDescRef,
    AVColorPrimariesDescMut,
    ffi::AVColorPrimariesDesc
);

// SAFETY: both fields are initialized inline coordinate structures with no
// teardown operation or separately owned resource.
unsafe impl CValued for AVColorPrimariesDesc {
    unsafe fn c_dispose(_this: NonNull<Self>) {}
}

impl AVColorPrimariesDesc {
    /// Creates a zero-initialized gamut description in owned inline storage.
    #[must_use]
    pub fn new() -> CVal<Self> {
        CVal::new(Self::zeroed())
    }
}

impl<'a> AVColorPrimariesDescRef<'a> {
    /// Field: AVColorPrimariesDesc.wp
    ///
    /// Borrows the inline white-point coordinates.
    #[must_use]
    pub fn wp(&self) -> AVCIExyRef<'a> {
        // SAFETY: raw-place projection locates the initialized inline white
        // point without forming a reference. It lives for the parent handle's
        // lifetime and is exposed through a shared handle only.
        unsafe { AVCIExyRef::from_ptr(addr_of!((*self.as_ptr()).wp).cast_mut()) }
            .expect("an inline field is non-null")
    }

    /// Field: AVColorPrimariesDesc.prim
    ///
    /// Borrows the inline red, green, and blue primary coordinates.
    #[must_use]
    pub fn prim(&self) -> AVPrimaryCoefficientsRef<'a> {
        // SAFETY: raw-place projection locates the initialized inline primary
        // coordinates without forming a reference. They live for the parent
        // handle's lifetime and are exposed through a shared handle only.
        unsafe { AVPrimaryCoefficientsRef::from_ptr(addr_of!((*self.as_ptr()).prim).cast_mut()) }
            .expect("an inline field is non-null")
    }
}

impl AVColorPrimariesDescMut<'_> {
    /// Exclusively borrows the inline white-point coordinates.
    #[must_use]
    pub fn wp_mut(&mut self) -> AVCIExyMut<'_> {
        // SAFETY: the exclusive parent handle supplies write provenance to
        // this initialized inline field for the returned reborrow.
        unsafe { AVCIExyMut::from_ptr(addr_of_mut!((*self.as_mut_ptr()).wp)) }
            .expect("an inline field is non-null")
    }

    /// Exclusively borrows the inline primary coordinates.
    #[must_use]
    pub fn prim_mut(&mut self) -> AVPrimaryCoefficientsMut<'_> {
        // SAFETY: the exclusive parent handle supplies write provenance to
        // this initialized inline field for the returned reborrow.
        unsafe { AVPrimaryCoefficientsMut::from_ptr(addr_of_mut!((*self.as_mut_ptr()).prim)) }
            .expect("an inline field is non-null")
    }
}

#[cfg(test)]
mod color_primaries_desc_tests {
    use core::mem::{align_of, size_of};

    use super::*;

    #[test]
    fn layout_and_nested_fields_match_c() {
        assert_eq!(
            size_of::<AVColorPrimariesDesc>(),
            size_of::<ffi::AVColorPrimariesDesc>()
        );
        assert_eq!(
            align_of::<AVColorPrimariesDesc>(),
            align_of::<ffi::AVColorPrimariesDesc>()
        );

        let mut description = AVColorPrimariesDesc::new();
        let mut view = description.as_mut();
        view.wp_mut().x_mut().set_num(31_270);
        view.wp_mut().y_mut().set_den(100_000);
        view.prim_mut().r_mut().x_mut().set_num(64);
        view.prim_mut().g_mut().y_mut().set_num(60);
        view.prim_mut().b_mut().x_mut().set_den(100);

        let view = description.as_ref();
        assert_eq!(view.wp().x().num(), 31_270);
        assert_eq!(view.wp().y().den(), 100_000);
        assert_eq!(view.prim().r().x().num(), 64);
        assert_eq!(view.prim().g().y().num(), 60);
        assert_eq!(view.prim().b().x().den(), 100);
    }
}
