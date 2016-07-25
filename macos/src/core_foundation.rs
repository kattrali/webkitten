use libc;


#[repr(C)]
pub struct __CFString(libc::c_void);
pub type CFStringRef = *const __CFString;

type CFAllocatorRef = *const libc::c_void;
type CFIndex = libc::c_long;
type CFStringEncoding = u32;

#[link(name = "CoreFoundation", kind = "framework")]
extern {
	static kCFAllocatorDefault: CFAllocatorRef;
	static kCFAllocatorNull: CFAllocatorRef;
    fn CFStringCreateWithBytes(alloc: CFAllocatorRef,
                               bytes: *const u8,
                               numBytes: CFIndex,
                               encoding: CFStringEncoding,
                               isExternalRepresentation: u8,
                               contentsDeallocator: CFAllocatorRef)
                               -> CFStringRef;
}

pub fn create_string_ref(content: &str) -> CFStringRef {
	static STRING_ENCODING_UTF8: CFStringEncoding = 0x08000100;
    unsafe {
		CFStringCreateWithBytes(kCFAllocatorDefault,
                                content.as_ptr(),
                                content.len() as CFIndex,
                                STRING_ENCODING_UTF8,
                                false as u8,
                                kCFAllocatorNull)
    }
}
