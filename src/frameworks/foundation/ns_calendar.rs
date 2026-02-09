/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `NSCalendar`.

use crate::frameworks::core_foundation::time::CFAbsoluteTimeGetGregorianDate;
use crate::frameworks::foundation::{ns_date_components, NSTimeInterval, NSUInteger};
use crate::objc::{
    id, msg, msg_class, nil, objc_classes, release, retain, ClassExports, HostObject, NSZonePtr,
};
use crate::Environment;

#[derive(Default)]
pub struct State {
    current_calendar: Option<id>,
}
impl State {
    fn get(env: &mut Environment) -> &mut State {
        &mut env.framework_state.foundation.ns_calendar
    }
}

struct NSCalendarHostObject {
    time_zone: id,
    locale: id,
}
impl HostObject for NSCalendarHostObject {}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation NSCalendar: NSObject

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::new(NSCalendarHostObject {
        time_zone: nil,
        locale: nil,
    });
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

+ (id)currentCalendar {
    if let Some(existing) = State::get(env).current_calendar {
        existing
    } else {
        let new: id = msg![env; this alloc];
        let new: id = msg![env; new init];
        State::get(env).current_calendar = Some(new);
        new
    }
}

+ (id)calendarWithIdentifier:(id)_identifier {
    msg![env; this currentCalendar]
}

- (id)init {
    let tz: id = msg_class![env; NSTimeZone localTimeZone];
    let loc: id = msg_class![env; NSLocale currentLocale];
    retain(env, tz);
    retain(env, loc);
    let host_object = env.objc.borrow_mut::<NSCalendarHostObject>(this);
    host_object.time_zone = tz;
    host_object.locale = loc;
    this
}

- (())dealloc {
    let (time_zone, locale) = {
        let host_object = env.objc.borrow_mut::<NSCalendarHostObject>(this);
        (host_object.time_zone, host_object.locale)
    };
    release(env, time_zone);
    release(env, locale);
    env.objc.dealloc_object(this, &mut env.mem)
}

// NSCopying implementation
- (id)copyWithZone:(NSZonePtr)_zone {
    retain(env, this)
}

- (id)timeZone {
    env.objc.borrow::<NSCalendarHostObject>(this).time_zone
}

- (())setTimeZone:(id)tz { // NSTimeZone *
    retain(env, tz);
    let old_tz = {
        let host_object = env.objc.borrow_mut::<NSCalendarHostObject>(this);
        let old = host_object.time_zone;
        host_object.time_zone = tz;
        old
    };
    release(env, old_tz);
}

- (id)locale {
    env.objc.borrow::<NSCalendarHostObject>(this).locale
}

- (())setLocale:(id)locale { // NSLocale *
    retain(env, locale);
    let old_locale = {
        let host_object = env.objc.borrow_mut::<NSCalendarHostObject>(this);
        let old = host_object.locale;
        host_object.locale = locale;
        old
    };
    release(env, old_locale);
}

- (id)components:(NSUInteger)_unit_flags
       fromDate:(id)date { // NSDate*
    let ti: NSTimeInterval = msg![env; date timeIntervalSinceReferenceDate];
    let greg = CFAbsoluteTimeGetGregorianDate(env, ti, nil);
    ns_date_components::from_parts(
        env,
        greg.year,
        greg.month as i32,
        greg.day as i32,
        greg.hours as i32,
        greg.minutes as i32,
        greg.seconds as i32,
    )
}

@end

};
