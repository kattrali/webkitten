use std::ops::BitOr;

use objc::runtime::{Class,YES,NO,BOOL,Sel};
use foundation::{NSString,NSMutableArray,NSArray,NSInteger,NSUInteger,NSRect,
                 NSPoint};
use core_graphics::CGFloat;

use super::{Id,ObjCClass,nil};


#[link(name = "AppKit", kind = "framework")]
extern {}

#[repr(isize)]
pub enum NSApplicationActivationPolicy {
    Regular    =  0,
    Accessory  =  1,
    Prohibited =  2,
    ERROR      = -1
}

pub enum NSLayoutAttribute {
    Left          = 1,
    Right         = 2,
    Top           = 3,
    Bottom        = 4,
    Leading       = 5,
    Trailing      = 6,
    Width         = 7,
    Height        = 8,
    CenterX       = 9,
    CenterY       = 10,
    Baseline      = 11,
    // LastBaseline  = 11,
    FirstBaseline = 12,
    NotAnAttribute = 0
}

pub enum NSLayoutRelation {
    LessThanOrEqual   = -1,
    Equal              = 0,
    GreaterThanOrEqual = 1,
}

pub enum NSEventModifierFlags {
    AlphaShift = 1 << 16,
    Shift      = 1 << 17,
    Control    = 1 << 18,
    Alternate  = 1 << 19,
    Command    = 1 << 20,
    Numeric    = 1 << 21,
    Help       = 1 << 22,
    Function   = 1 << 23,
}

pub enum NSBackingStoreType {
    Retained    = 0,
    Nonretained = 1,
    Buffered    = 2
}

pub enum NSWindowMask {
    Borderless             = 0,
    Titled                 = 1 << 0,
    Closable               = 1 << 1,
    Miniaturizable         = 1 << 2,
    Resizable              = 1 << 3,
    TexturedBackground     = 1 << 8,
    UnifiedTitleAndToolbar = 1 << 12,
    FullScreen             = 1 << 14,
    FullSizeContentView    = 1 << 15
}

impl BitOr for NSWindowMask {
    type Output = NSUInteger;

    fn bitor(self, rhs: NSWindowMask) -> NSUInteger {
        self as NSUInteger | rhs as NSUInteger
    }
}

impl_objc_class!(NSApplication);
impl_objc_class!(NSControl);
impl_objc_class!(NSEvent);
impl_objc_class!(NSLayoutConstraint);
impl_objc_class!(NSMenu);
impl_objc_class!(NSMenuItem);
impl_objc_class!(NSPasteboard);
impl_objc_class!(NSResponder);
impl_objc_class!(NSTextField);
impl_objc_class!(NSView);
impl_objc_class!(NSWindow);

pub fn nsapp() -> NSApplication {
    NSApplication::shared_app()
}

impl NSApplication {

    pub fn shared_app() -> Self {
        NSApplication {
            ptr: unsafe { msg_send![class!("NSApplication"), sharedApplication] }
        }
    }

    pub fn set_activation_policy(&self, policy: NSApplicationActivationPolicy) {
        unsafe { msg_send![self.ptr, setActivationPolicy:policy] }
    }

    pub fn activate(&self, ignoring_other_apps: bool) {
        let value = if ignoring_other_apps { YES } else { NO };
        unsafe { msg_send![self.ptr, activateIgnoringOtherApps:value] }
    }

    pub fn set_delegate<T: ObjCClass>(&self, delegate: &T) {
        unsafe { msg_send![self.ptr, setDelegate:delegate.ptr()] }
    }

    pub fn set_main_menu(&self, menu: NSMenu) {
        unsafe { msg_send![self.ptr, setMainMenu:menu.ptr()] }
    }

    pub fn set_windows_menu(&self, menu: &NSMenu) {
        unsafe { msg_send![self.ptr, setWindowsMenu:menu.ptr()] }
    }

    pub fn main_menu(&self) -> Option<NSMenu> {
        NSMenu::from_ptr(unsafe { msg_send![self.ptr, mainMenu] })
    }

    pub fn windows(&self) -> NSArray {
        NSArray::from_ptr(unsafe { msg_send![self.ptr, windows] }).unwrap()
    }

    pub fn window_by_number(&self, window_number: NSInteger) -> Option<NSWindow> {
        NSWindow::from_ptr(unsafe {
            msg_send![self.ptr, windowWithWindowNumber: window_number]
        })
    }

    pub fn window_by_index(&self, window_index: NSUInteger) -> Option<NSWindow> {
        self.windows().get::<NSWindow>(window_index)
    }

    pub fn run(&self) {
        unsafe { msg_send![self.ptr, run] }
    }
}

impl NSControl {

    pub fn text(&self) -> Option<NSString> {
        NSString::from_ptr(unsafe { msg_send![self.ptr, stringValue] })
    }

    pub fn set_text(&self, text: &str) {
        unsafe {
            msg_send![self.ptr, setStringValue:NSString::from(text).ptr()];
        }
    }

    pub fn set_font(&self, family: &str, size: isize) {
        let name = NSString::from(family);
        let size = size as CGFloat;
        unsafe {
            let font: Id = msg_send![class!("NSFont"), fontWithName:name
                                                               size:size];
            if font != nil {
                msg_send![self.ptr, setFont:font];
            }
        }
    }
}

impl NSEvent {

    pub fn modifier_flags(&self) -> NSUInteger {
        unsafe { msg_send![self.ptr, modifierFlags] }
    }
}

impl NSLayoutConstraint {

    pub fn new(view1: &NSView, attr1: NSLayoutAttribute, relation: NSLayoutRelation,
               view2: &NSView, attr2: NSLayoutAttribute, multiplier: CGFloat,
               constant: CGFloat) -> Self {
        let ptr = unsafe {
            msg_send![class!("NSLayoutConstraint"), constraintWithItem:view1.ptr()
                                                             attribute:attr1
                                                             relatedBy:relation
                                                                toItem:view2.ptr()
                                                             attribute:attr2
                                                            multiplier:multiplier
                                                              constant:constant]
        };
        NSLayoutConstraint { ptr: ptr }
    }

    /// Stick two views together with a constant of 0
    pub fn bind(view1: &NSView, attr1: NSLayoutAttribute,
                view2: &NSView, attr2: NSLayoutAttribute) -> Self {
        NSLayoutConstraint::new(view1, attr1, NSLayoutRelation::Equal,
                                view2, attr2, 1 as CGFloat, 0 as CGFloat)
    }
}

impl NSMenu {

    pub fn new(title: &str) -> Self {
        let title = NSString::from(title);
        NSMenu {
            ptr: unsafe {
                let menu: Id = msg_send![class!("NSMenu"), alloc];
                let menu: Id = msg_send![menu, initWithTitle:title.ptr()];
                menu
            }
        }
    }

    pub fn add_item(&self, item: NSMenuItem) {
        unsafe { msg_send![self.ptr, addItem:item.ptr()] }
    }

    pub fn set_autoenables_items(&self, enables: bool) {
        let value = if enables { YES } else { NO };
        unsafe { msg_send![self.ptr, setAutoenablesItems:value] }
    }
}

impl NSMenuItem {

    pub fn blank() -> Self {
        NSMenuItem { ptr: unsafe { msg_send![class!("NSMenuItem"), new] } }
    }

    pub fn separator() -> Self {
        NSMenuItem { ptr: unsafe { msg_send![class!("NSMenuItem"), separatorItem] } }
    }

    pub fn new(title: &str, action: Sel, key_equivalent: &str) -> Self {
        let title = NSString::from(title);
        let key = NSString::from(key_equivalent);
        let ptr = unsafe {
            let item: Id = msg_send![class!("NSMenuItem"), alloc];
            let item: Id = msg_send![item, initWithTitle:title
                                                  action:action
                                           keyEquivalent:key];
            item
        };
        NSMenuItem { ptr: ptr }
    }

    pub fn set_submenu(&self, menu: NSMenu) {
        unsafe { msg_send![self.ptr, setSubmenu:menu.ptr()] }
    }

    pub fn set_target<T: ObjCClass>(&self, target: &T) {
        unsafe { msg_send![self.ptr, setTarget:target.ptr()] }
    }

    pub fn set_key_equivalent_modifier_mask(&self, mask: NSUInteger) {
        unsafe { msg_send![self.ptr, setKeyEquivalentModifierMask:mask] }
    }
}

impl NSPasteboard {

    pub fn general() -> Self {
        NSPasteboard {
            ptr: unsafe { msg_send![class!("NSPasteboard"), generalPasteboard] }
        }
    }

    pub fn copy(&self, text: &str) {
        const NSPASTEBOARD_TYPE_STRING: &'static str = "public.utf8-plain-text";
        let data = NSString::from(text);
        let data_type = NSString::from(NSPASTEBOARD_TYPE_STRING);
        let data_types = NSMutableArray::new();
        data_types.push(NSString::from(NSPASTEBOARD_TYPE_STRING));
        unsafe {
            msg_send![self.ptr, declareTypes:data_types.ptr() owner:nil];
            msg_send![self.ptr, setString:data.ptr() forType:data_type.ptr()];
        }
    }
}

impl NSTextField {

    pub fn new() -> Self {
        NSTextField { ptr: unsafe { msg_send![class!("NSTextField"), new] } }
    }

    pub fn set_delegate<T: ObjCClass>(&self, delegate: &T) {
        unsafe { msg_send![self.ptr, setDelegate:delegate.ptr()] }
    }
}

impl NSResponder {

    pub fn become_first_responder(&self) {
        unsafe { msg_send![self.ptr, becomeFirstResponder] }
    }
}

impl NSView {

    pub fn new() -> Self {
        NSView { ptr: unsafe { msg_send![class!("NSView"), new] } }
    }

    pub fn add_constraint(&self, constraint: NSLayoutConstraint) {
        unsafe { msg_send![self.ptr, addConstraint:constraint.ptr()] }
    }

    pub fn disable_translates_autoresizing_mask_into_constraints(&self) {
        unsafe {
            msg_send![self.ptr, setTranslatesAutoresizingMaskIntoConstraints:NO];
        }
    }

    pub fn key_down(&self, event: NSEvent) {
        unsafe { msg_send![self.ptr, keyDown:event.ptr()] }
    }

    pub fn set_height(&self, height: CGFloat) {
        self.add_constraint(NSLayoutConstraint::new(self,
            NSLayoutAttribute::Height, NSLayoutRelation::Equal, &NSView::nil(),
            NSLayoutAttribute::NotAnAttribute, 1 as CGFloat, height));
    }

    pub fn set_width(&self, width: CGFloat) {
        self.add_constraint(NSLayoutConstraint::new(self,
            NSLayoutAttribute::Width, NSLayoutRelation::Equal, &NSView::nil(),
            NSLayoutAttribute::NotAnAttribute, 1 as CGFloat, width));
    }

    pub fn subviews(&self) -> Option<NSArray> {
        NSArray::from_ptr(unsafe { msg_send![self.ptr, subviews] })
    }

    pub fn superview(&self) -> Option<NSView> {
        NSView::from_ptr(unsafe { msg_send![self.ptr, superview] })
    }

    pub fn subview(&self, index: NSUInteger) -> Option<NSView> {
        match self.subviews() {
            Some(views) => views.get::<NSView>(index),
            None => None
        }
    }

    pub fn subview_index(&self) -> Option<NSUInteger> {
        if let Some(superview) = self.superview() {
            if let Some(views) = superview.subviews() {
                return Some(views.index_of(self));
            }
        }
        None
    }

    pub fn add_subview(&self, view: &NSView) {
        unsafe { msg_send![self.ptr, addSubview:view.ptr()] }
    }

    pub fn set_hidden(&self, hidden: bool) {
        let value = if hidden { YES } else { NO };
        unsafe { msg_send![self.ptr, setHidden:value]; }
    }

    pub fn hidden(&self) -> bool {
        let hidden: BOOL = unsafe { msg_send![self.ptr, isHidden] };
        hidden == YES
    }

    pub fn remove_from_superview(&self) {
        unsafe { msg_send![self.ptr, removeFromSuperview] }
    }

    pub fn window(&self) -> Option<NSWindow> {
        NSWindow::from_ptr(unsafe { msg_send![self.ptr, window] })
    }
}

impl NSWindow {

    pub fn new(content_rect: NSRect, mask: NSUInteger,
               backing: NSBackingStoreType, defer: bool) -> Self {
        let value = if defer { YES } else { NO };
        let ptr = unsafe {
            let ptr: Id = msg_send![class!("NSWindow"), alloc];
            let ptr: Id = msg_send![ptr, initWithContentRect:content_rect
                                                   styleMask:mask
                                                     backing:backing
                                                       defer:value];
            ptr
        };
        NSWindow { ptr: ptr }
    }

    pub fn content_view(&self) -> Option<NSView> {
        NSView::from_ptr(unsafe { msg_send![self.ptr, contentView] })
    }

    pub fn close(&self) {
        unsafe { msg_send![self.ptr, close] }
    }

    pub fn center(&self) {
        unsafe { msg_send![self.ptr, center] }
    }

    pub fn cascade_top_left_from_point(&self, point: NSPoint) {
        unsafe { msg_send![self.ptr, cascadeTopLeftFromPoint:point] }
    }

    pub fn number(&self) -> u32 {
        unsafe { msg_send![self.ptr, windowNumber] }
    }

    pub fn is_key_window(&self) -> bool {
        let is_key: BOOL = unsafe { msg_send![self.ptr, isKeyWindow] };
        is_key == YES
    }

    pub fn title(&self) -> Option<NSString> {
        NSString::from_ptr(unsafe { msg_send![self.ptr, title] })
    }

    pub fn set_title(&self, title: &str) {
        unsafe { msg_send![self.ptr, setTitle:NSString::from(title)] }
    }

    pub fn make_key_and_order_front(&self) {
        unsafe { msg_send![self.ptr, makeKeyAndOrderFront:nil] }
    }

    pub fn order_out(&self) {
        unsafe { msg_send![self.ptr, orderOut:nil] }
    }

    pub fn set_frame(&self, rect: NSRect) {
        unsafe { msg_send![self.ptr, setFrame:rect display:YES] }
    }

    pub fn frame(&self) -> NSRect {
        unsafe { msg_send![self.ptr, frame] }
    }
}
