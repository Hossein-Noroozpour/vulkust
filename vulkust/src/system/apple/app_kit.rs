use super::NSUInteger;

#[link(name = "AppKit", kind = "framework")]
extern "C" {
    // pub static NSImageHintCTM: Id;
}

bitflags! {
    pub struct NSTrackingAreaOptions: NSUInteger {
        /* Type of tracking area. You must specify one or more type from this list in the NSTrackingAreaOptions argument of -initWithRect:options:owner:userInfo: */
        const NS_TRACKING_MOUSE_ENTERED_AND_EXITED     = 0x01;  // owner receives mouseEntered when mouse enters area, and mouseExited when mouse leaves area
        const NS_TRACKING_MOUSE_MOVED                  = 0x02;  // owner receives mouseMoved while mouse is within area.  Note that mouseMoved events do not contain userInfo
        const NS_TRACKING_CURSOR_UPDATE                = 0x04;  // owner receives cursorUpdate when mouse enters area.  Cursor is set appropriately when mouse exits area
        /* When tracking area is active. You must specify one of the following in the NSTrackingAreaOptions argument of -initWithRect:options:owner:userInfo: */
        const NS_TRACKING_ACTIVE_WHEN_FIRST_RESPONDER  = 0x10;  // owner receives mouseEntered/Exited, mouseMoved, or cursorUpdate when view is first responder
        const NS_TRACKING_ACTIVE_IN_KEY_WINDOW         = 0x20;  // owner receives mouseEntered/Exited, mouseMoved, or cursorUpdate when view is in key window
        const NS_TRACKING_ACTIVE_IN_ACTIVE_APP         = 0x40;  // owner receives mouseEntered/Exited, mouseMoved, or cursorUpdate when app is active
        const NS_TRACKING_ACTIVE_ALWAYS                = 0x80;  // owner receives mouseEntered/Exited or mouseMoved regardless of activation.  Not supported for NSTrackingCursorUpdate.
        /* Behavior of tracking area.  These values are used in NSTrackingAreaOptions.  You may specify any number of the following in the NSTrackingAreaOptions argument of -initWithRect:options:owner:userInfo: */
        const NS_TRACKING_ASSUME_INSIDE                = 0x100; // If set, generate mouseExited event when mouse leaves area (same as assumeInside argument in deprecated addTrackingRect:owner:userData:assumeInside:)
        const NS_TRACKING_IN_VISIBLE_RECT              = 0x200; // If set, tracking occurs in visibleRect of view and rect is ignored
        const NS_TRACKING_ENABLED_DURING_MOUSE_DRAG    = 0x400; // If set, mouseEntered events will be generated as mouse is dragged.  If not set, mouseEntered events will be generated as mouse is moved, and on mouseUp after a drag.  mouseExited events are paired with mouseEntered events so their delivery is affected indirectly.  That is, if a mouseEntered event is generated and the mouse subsequently moves out of the trackingArea, a mouseExited event will be generated whether the mouse is being moved or dragged, independent of this flag.
    }
}