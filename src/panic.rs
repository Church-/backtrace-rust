use std::panic::PanicInfo;

use crate::sender;
use crate::Report;
use crate::SubmissionTarget;

pub fn register_error_handler<T>(url: &str, token: &str, user_handler: T)
where
    T: Fn(&mut Report, &PanicInfo) -> () + Send + Sync + 'static,
{
    let submission_target = SubmissionTarget {
        token: String::from(token),
        url: String::from(url),
    };

    std::panic::set_hook(Box::new(move |panic_info| {
        let mut r = Report {
            ..Default::default()
        };

        user_handler(&mut r, panic_info);

        let bt = backtrace::Backtrace::new();

        sender::submit(&submission_target, &mut r, bt);
    }));
}
