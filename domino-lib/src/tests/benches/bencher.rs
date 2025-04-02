use std::{fmt, time::Duration};

pub struct FormattedDuration(pub Duration);

impl fmt::Display for FormattedDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let nanos = self.0.as_nanos();

        if nanos < 1_000 {
            write!(f, "{}ns", nanos)
        } else if nanos < 1_000_000 {
            write!(f, "{:.2}µs", nanos as f64 / 1_000.0)
        } else if nanos < 1_000_000_000 {
            write!(f, "{:.2}ms", nanos as f64 / 1_000_000.0)
        } else if nanos < 60_000_000_000 {
            write!(f, "{:.2}s", nanos as f64 / 1_000_000_000.0)
        } else {
            let seconds = self.0.as_secs();
            let minutes = seconds / 60;
            let seconds = seconds % 60;

            if minutes < 60 {
                write!(f, "{}m {}s", minutes, seconds)
            } else {
                let hours = minutes / 60;
                let minutes = minutes % 60;
                write!(f, "{}h {}m {}s", hours, minutes, seconds)
            }
        }
    }
}

pub fn format_duration(duration: Duration) -> FormattedDuration {
    FormattedDuration(duration)
}
