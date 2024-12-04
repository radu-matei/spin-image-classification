#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod component {
        #[allow(dead_code)]
        pub mod image_classification_lib {
            #[allow(dead_code, clippy::all)]
            pub mod image_classification {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                /// Error returned by image classification components.
                #[derive(Clone)]
                pub enum ClassificationError {
                    ModelError(_rt::String),
                    ImageError(_rt::String),
                    IoError(_rt::String),
                    Unknown(_rt::String),
                    Unclassified,
                }
                impl ::core::fmt::Debug for ClassificationError {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        match self {
                            ClassificationError::ModelError(e) => {
                                f.debug_tuple("ClassificationError::ModelError")
                                    .field(e)
                                    .finish()
                            }
                            ClassificationError::ImageError(e) => {
                                f.debug_tuple("ClassificationError::ImageError")
                                    .field(e)
                                    .finish()
                            }
                            ClassificationError::IoError(e) => {
                                f.debug_tuple("ClassificationError::IoError")
                                    .field(e)
                                    .finish()
                            }
                            ClassificationError::Unknown(e) => {
                                f.debug_tuple("ClassificationError::Unknown")
                                    .field(e)
                                    .finish()
                            }
                            ClassificationError::Unclassified => {
                                f.debug_tuple("ClassificationError::Unclassified").finish()
                            }
                        }
                    }
                }
                impl ::core::fmt::Display for ClassificationError {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        write!(f, "{:?}", self)
                    }
                }
                impl std::error::Error for ClassificationError {}
                /// Type representing an image.
                pub type Image = _rt::Vec<u8>;
                /// Result of an image classification operation representing the
                /// image class and computed probability.
                pub type ClassificationResult = (_rt::String, f32);
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_classify_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg1;
                    let result1 = T::classify(
                        _rt::Vec::from_raw_parts(arg0.cast(), len0, len0),
                    );
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result1 {
                        Ok(e) => {
                            *ptr2.add(0).cast::<u8>() = (0i32) as u8;
                            let (t3_0, t3_1) = e;
                            let vec4 = (t3_0.into_bytes()).into_boxed_slice();
                            let ptr4 = vec4.as_ptr().cast::<u8>();
                            let len4 = vec4.len();
                            ::core::mem::forget(vec4);
                            *ptr2.add(8).cast::<usize>() = len4;
                            *ptr2.add(4).cast::<*mut u8>() = ptr4.cast_mut();
                            *ptr2.add(12).cast::<f32>() = _rt::as_f32(t3_1);
                        }
                        Err(e) => {
                            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
                            match e {
                                ClassificationError::ModelError(e) => {
                                    *ptr2.add(4).cast::<u8>() = (0i32) as u8;
                                    let vec5 = (e.into_bytes()).into_boxed_slice();
                                    let ptr5 = vec5.as_ptr().cast::<u8>();
                                    let len5 = vec5.len();
                                    ::core::mem::forget(vec5);
                                    *ptr2.add(12).cast::<usize>() = len5;
                                    *ptr2.add(8).cast::<*mut u8>() = ptr5.cast_mut();
                                }
                                ClassificationError::ImageError(e) => {
                                    *ptr2.add(4).cast::<u8>() = (1i32) as u8;
                                    let vec6 = (e.into_bytes()).into_boxed_slice();
                                    let ptr6 = vec6.as_ptr().cast::<u8>();
                                    let len6 = vec6.len();
                                    ::core::mem::forget(vec6);
                                    *ptr2.add(12).cast::<usize>() = len6;
                                    *ptr2.add(8).cast::<*mut u8>() = ptr6.cast_mut();
                                }
                                ClassificationError::IoError(e) => {
                                    *ptr2.add(4).cast::<u8>() = (2i32) as u8;
                                    let vec7 = (e.into_bytes()).into_boxed_slice();
                                    let ptr7 = vec7.as_ptr().cast::<u8>();
                                    let len7 = vec7.len();
                                    ::core::mem::forget(vec7);
                                    *ptr2.add(12).cast::<usize>() = len7;
                                    *ptr2.add(8).cast::<*mut u8>() = ptr7.cast_mut();
                                }
                                ClassificationError::Unknown(e) => {
                                    *ptr2.add(4).cast::<u8>() = (3i32) as u8;
                                    let vec8 = (e.into_bytes()).into_boxed_slice();
                                    let ptr8 = vec8.as_ptr().cast::<u8>();
                                    let len8 = vec8.len();
                                    ::core::mem::forget(vec8);
                                    *ptr2.add(12).cast::<usize>() = len8;
                                    *ptr2.add(8).cast::<*mut u8>() = ptr8.cast_mut();
                                }
                                ClassificationError::Unclassified => {
                                    *ptr2.add(4).cast::<u8>() = (4i32) as u8;
                                }
                            }
                        }
                    };
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_classify<T: Guest>(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {
                            let l1 = *arg0.add(4).cast::<*mut u8>();
                            let l2 = *arg0.add(8).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                        }
                        _ => {
                            let l3 = i32::from(*arg0.add(4).cast::<u8>());
                            match l3 {
                                0 => {
                                    let l4 = *arg0.add(8).cast::<*mut u8>();
                                    let l5 = *arg0.add(12).cast::<usize>();
                                    _rt::cabi_dealloc(l4, l5, 1);
                                }
                                1 => {
                                    let l6 = *arg0.add(8).cast::<*mut u8>();
                                    let l7 = *arg0.add(12).cast::<usize>();
                                    _rt::cabi_dealloc(l6, l7, 1);
                                }
                                2 => {
                                    let l8 = *arg0.add(8).cast::<*mut u8>();
                                    let l9 = *arg0.add(12).cast::<usize>();
                                    _rt::cabi_dealloc(l8, l9, 1);
                                }
                                3 => {
                                    let l10 = *arg0.add(8).cast::<*mut u8>();
                                    let l11 = *arg0.add(12).cast::<usize>();
                                    _rt::cabi_dealloc(l10, l11, 1);
                                }
                                _ => {}
                            }
                        }
                    }
                }
                pub trait Guest {
                    /// Default image classification function.
                    fn classify(
                        img: Image,
                    ) -> Result<ClassificationResult, ClassificationError>;
                }
                #[doc(hidden)]
                macro_rules! __export_component_image_classification_lib_image_classification_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "component:image-classification-lib/image-classification#classify"]
                        unsafe extern "C" fn export_classify(arg0 : * mut u8, arg1 :
                        usize,) -> * mut u8 { $($path_to_types)*::
                        _export_classify_cabi::<$ty > (arg0, arg1) } #[export_name =
                        "cabi_post_component:image-classification-lib/image-classification#classify"]
                        unsafe extern "C" fn _post_return_classify(arg0 : * mut u8,) {
                        $($path_to_types)*:: __post_return_classify::<$ty > (arg0) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_component_image_classification_lib_image_classification_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 16]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 16],
                );
            }
        }
    }
}
mod _rt {
    pub use alloc_crate::string::String;
    pub use alloc_crate::vec::Vec;
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub fn as_f32<T: AsF32>(t: T) -> f32 {
        t.as_f32()
    }
    pub trait AsF32 {
        fn as_f32(self) -> f32;
    }
    impl<'a, T: Copy + AsF32> AsF32 for &'a T {
        fn as_f32(self) -> f32 {
            (*self).as_f32()
        }
    }
    impl AsF32 for f32 {
        #[inline]
        fn as_f32(self) -> f32 {
            self as f32
        }
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
    }
    extern crate alloc as alloc_crate;
    pub use alloc_crate::alloc;
}
/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]
macro_rules! __export_classification_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::component::image_classification_lib::image_classification::__export_component_image_classification_lib_image_classification_cabi!($ty
        with_types_in $($path_to_types_root)*::
        exports::component::image_classification_lib::image_classification);
    };
}
#[doc(inline)]
pub(crate) use __export_classification_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.31.0:component:image-classification-lib:classification:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 428] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xa7\x02\x01A\x02\x01\
A\x02\x01B\x09\x01q\x05\x0bmodel-error\x01s\0\x0bimage-error\x01s\0\x08io-error\x01\
s\0\x07unknown\x01s\0\x0cunclassified\0\0\x04\0\x14classification-error\x03\0\0\x01\
p}\x04\0\x05image\x03\0\x02\x01o\x02sv\x04\0\x15classification-result\x03\0\x04\x01\
j\x01\x05\x01\x01\x01@\x01\x03img\x03\0\x06\x04\0\x08classify\x01\x07\x04\x017co\
mponent:image-classification-lib/image-classification\x05\0\x04\x011component:im\
age-classification-lib/classification\x04\0\x0b\x14\x01\0\x0eclassification\x03\0\
\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-component\x070.216.0\x10wit-bi\
ndgen-rust\x060.31.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
