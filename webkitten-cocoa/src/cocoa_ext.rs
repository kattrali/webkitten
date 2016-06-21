pub mod foundation {
    use cocoa::base::{class,id,nil};
    use cocoa::foundation::{NSString,NSUInteger,NSInteger};

    pub unsafe fn NSURL(url: &str) -> id {
        let url_str = NSString::alloc(nil).init_str(url);
        msg_send![class("NSURL"), URLWithString:url_str]
    }

    pub unsafe fn NSURLRequest(url: &str) -> id {
        msg_send![class("NSURLRequest"), requestWithURL:NSURL(url)]
    }

    pub trait NSNumber {

        unsafe fn integer_value(self) -> NSInteger;
    }

    impl NSNumber for id {

        unsafe fn integer_value(self) -> NSInteger {
            msg_send![self, integerValue]
        }
    }

    pub trait NSDictionary {

        unsafe fn object_for_key(self, key: &str) -> id;
    }

    impl NSDictionary for id {

        unsafe fn object_for_key(self, key: &str) -> id {
            let key_str = NSString::alloc(nil).init_str(key);
            msg_send![self, objectForKey:key_str]
        }
    }

    pub trait NSArray {

        unsafe fn object_at_index(self, index: NSUInteger) -> id;
        unsafe fn get(self, index: NSUInteger) -> Option<id>;
        unsafe fn count(self) -> NSUInteger;
    }

    impl NSArray for id {

        unsafe fn object_at_index(self, index: NSUInteger) -> id {
            msg_send![self, objectAtIndex:index]
        }

        unsafe fn get(self, index: NSUInteger) -> Option<id> {
            if self.count() > index {
                Some(self.object_at_index(index))
            } else {
                None
            }
        }

        unsafe fn count(self) -> NSUInteger {
            msg_send![self, count]
        }
    }

    pub trait NSNotification {

        unsafe fn object(self) -> id;
        unsafe fn user_info(self) -> id;
    }

    impl NSNotification for id {

        unsafe fn object(self) -> id {
            msg_send![self, object]
        }

        unsafe fn user_info(self) -> id {
            msg_send![self, userInfo]
        }
    }
}

pub mod core_graphics {
    use core_graphics::geometry::{CGRect,CGPoint,CGSize};

    pub fn CGRectZero() -> CGRect {
        CGRect {
            origin: CGPoint { x: 0., y: 0. },
            size: CGSize { width: 0., height: 0. }
        }
    }
}

pub mod appkit {
    use cocoa::base::{class,id,nil,NO,YES};
    use core_graphics::base::CGFloat;

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

    pub trait NSLayoutConstraint {

        unsafe fn new(view1: id, attr1: NSLayoutAttribute, relation: NSLayoutRelation,
                      view2: id, attr2: NSLayoutAttribute, multiplier: CGFloat,
                      constant: CGFloat) -> id {
            msg_send![class("NSLayoutConstraint"), constraintWithItem:view1
                                                            attribute:attr1
                                                            relatedBy:relation
                                                               toItem:view2
                                                            attribute:attr2
                                                           multiplier:multiplier
                                                             constant:constant]
        }

        unsafe fn bind(view1: id, attr1: NSLayoutAttribute, view2: id, attr2: NSLayoutAttribute) -> id {
            <id as NSLayoutConstraint>::new(view1, attr1, NSLayoutRelation::Equal, view2, attr2, 1 as CGFloat, 0 as CGFloat)
        }
    }

    impl NSLayoutConstraint for id {}

    pub trait NSControl {

        unsafe fn string_value(self) -> id;
    }

    impl NSControl for id {

        unsafe fn string_value(self) -> id {
            msg_send![self, stringValue]
        }
    }

    pub trait NSConstraintBasedLayoutInstallingConstraints {

        unsafe fn add_constraint(self, constraint: id);
        unsafe fn set_height(self, height: CGFloat);
    }

    impl NSConstraintBasedLayoutInstallingConstraints for id {

        unsafe fn add_constraint(self, constraint: id) {
            msg_send![self, addConstraint:constraint];
        }

        unsafe fn set_height(self, height: CGFloat) {
            self.add_constraint(<id as NSLayoutConstraint>::new(self,
                NSLayoutAttribute::Height, NSLayoutRelation::Equal, nil,
                NSLayoutAttribute::NotAnAttribute, 1 as CGFloat, height))
        }
    }

    pub trait NSView {

        unsafe fn new() -> id {
            msg_send![class("NSView"), new]
        }

        unsafe fn subviews(self) -> id;
        unsafe fn add_subview(self, view: id);
        unsafe fn set_hidden(self, hidden: bool);
        unsafe fn remove_from_superview(self);
    }

    impl NSView for id {

        unsafe fn subviews(self) -> id {
            msg_send![self, subviews]
        }

        unsafe fn add_subview(self, view: id) {
            msg_send![self, addSubview:view];
        }

        unsafe fn set_hidden(self, hidden: bool) {
            let value = if hidden { YES } else { NO };
            msg_send![self, setHidden:value];
        }

        unsafe fn remove_from_superview(self) {
            msg_send![self, removeFromSuperview];
        }
    }

    pub trait NSTextField {

        unsafe fn new() -> id;
        unsafe fn disable_translates_autoresizing_mask_into_constraints(self);
        unsafe fn set_delegate(self, delegate: id);
    }

    impl NSTextField for id {

        unsafe fn new() -> id {
            msg_send![class("NSTextField"), new]
        }

        unsafe fn disable_translates_autoresizing_mask_into_constraints(self) {
            msg_send![self, setTranslatesAutoresizingMaskIntoConstraints:NO];
        }

        unsafe fn set_delegate(self, delegate: id) {
            msg_send![self, setDelegate:delegate];
        }
    }
}

