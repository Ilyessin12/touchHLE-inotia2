/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `NSDateComponents`.

use super::NSInteger;
use crate::objc::{
    autorelease, id, msg, msg_class, objc_classes, retain, ClassExports, HostObject, NSZonePtr,
};
use crate::Environment;

#[derive(Default)]
struct NSDateComponentsHostObject {
    year: NSInteger,
    month: NSInteger,
    day: NSInteger,
    hour: NSInteger,
    minute: NSInteger,
    second: NSInteger,
}
impl HostObject for NSDateComponentsHostObject {}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation NSDateComponents: NSObject

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::<NSDateComponentsHostObject>::default();
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (id)init {
    this
}

// NSCopying implementation
- (id)copyWithZone:(NSZonePtr)_zone {
    retain(env, this)
}

- (NSInteger)year { env.objc.borrow::<NSDateComponentsHostObject>(this).year }
- (())setYear:(NSInteger)val { env.objc.borrow_mut::<NSDateComponentsHostObject>(this).year = val; }

- (NSInteger)month { env.objc.borrow::<NSDateComponentsHostObject>(this).month }
- (())setMonth:(NSInteger)val { env.objc.borrow_mut::<NSDateComponentsHostObject>(this).month = val; }

- (NSInteger)day { env.objc.borrow::<NSDateComponentsHostObject>(this).day }
- (())setDay:(NSInteger)val { env.objc.borrow_mut::<NSDateComponentsHostObject>(this).day = val; }

- (NSInteger)hour { env.objc.borrow::<NSDateComponentsHostObject>(this).hour }
- (())setHour:(NSInteger)val { env.objc.borrow_mut::<NSDateComponentsHostObject>(this).hour = val; }

- (NSInteger)minute { env.objc.borrow::<NSDateComponentsHostObject>(this).minute }
- (())setMinute:(NSInteger)val { env.objc.borrow_mut::<NSDateComponentsHostObject>(this).minute = val; }

- (NSInteger)second { env.objc.borrow::<NSDateComponentsHostObject>(this).second }
- (())setSecond:(NSInteger)val { env.objc.borrow_mut::<NSDateComponentsHostObject>(this).second = val; }

@end

};

pub fn from_parts(
    env: &mut Environment,
    year: NSInteger,
    month: NSInteger,
    day: NSInteger,
    hour: NSInteger,
    minute: NSInteger,
    second: NSInteger,
) -> id {
    let comps: id = msg_class![env; NSDateComponents alloc];
    let comps: id = msg![env; comps init];
    let host_object = env.objc.borrow_mut::<NSDateComponentsHostObject>(comps);
    host_object.year = year;
    host_object.month = month;
    host_object.day = day;
    host_object.hour = hour;
    host_object.minute = minute;
    host_object.second = second;
    autorelease(env, comps)
}
