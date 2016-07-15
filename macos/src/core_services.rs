use core_foundation::{CFStringRef,create_string_ref};


#[link(name = "CoreServices", kind = "framework")]
extern {
    fn LSSetDefaultHandlerForURLScheme(scheme: CFStringRef, bundle_id:CFStringRef);
}

pub fn register_default_scheme_handler(scheme: &str, bundle_id: &str) {
    let scheme = create_string_ref(scheme);
    let bundle_id = create_string_ref(bundle_id);
    unsafe { LSSetDefaultHandlerForURLScheme(scheme, bundle_id); }
}
