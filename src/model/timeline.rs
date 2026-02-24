use chrono::NaiveDateTime;

/// Controls what scale the timeline displays.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimelineScale {
    Hours,
    Days,
    Weeks,
    Months,
}

/// Manages the visible viewport of the timeline.
#[derive(Debug, Clone)]
pub struct TimelineViewport {
    /// The leftmost visible datetime.
    pub start: NaiveDateTime,
    /// The rightmost visible datetime.
    pub end: NaiveDateTime,
    /// Current display scale.
    pub scale: TimelineScale,
    /// Pixels per day (controls zoom level).
    pub pixels_per_day: f32,
    /// Pixels per hour (derived from pixels_per_day).
    pub pixels_per_hour: f32,
}

impl TimelineViewport {
    pub fn new(start: NaiveDateTime, end: NaiveDateTime) -> Self {
        let pixels_per_day = 18.0;
        Self {
            start,
            end,
            scale: TimelineScale::Weeks,
            pixels_per_day,
            pixels_per_hour: pixels_per_day / 24.0,
        }
    }

    /// Convert a datetime to an x-pixel offset from the viewport start.
    pub fn datetime_to_x(&self, dt: NaiveDateTime) -> f32 {
        match self.scale {
            TimelineScale::Hours => {
                let hours = (dt - self.start).num_seconds() as f32 / 3600.0;
                hours * self.pixels_per_hour
            }
            _ => {
                let total_seconds = (dt - self.start).num_seconds() as f32;
                let days = total_seconds / 86400.0; // 86400 seconds in a day
                days * self.pixels_per_day
            }
        }
    }

    /// Legacy method for compatibility - delegates to datetime_to_x.
    pub fn date_to_x(&self, date: NaiveDateTime) -> f32 {
        self.datetime_to_x(date)
    }

    /// Convert an x-pixel offset to a datetime (inverse of datetime_to_x).
    pub fn x_to_datetime(&self, x: f32) -> NaiveDateTime {
        match self.scale {
            TimelineScale::Hours => {
                let hours = x / self.pixels_per_hour;
                self.start + chrono::Duration::seconds((hours * 3600.0) as i64)
            }
            _ => {
                let days = x / self.pixels_per_day;
                self.start + chrono::Duration::seconds((days * 86400.0) as i64)
            }
        }
    }

    /// Total width in pixels for the visible range.
    pub fn total_width(&self) -> f32 {
        self.datetime_to_x(self.end)
    }

    /// Zoom in (increase pixels per day), auto-switching scale if needed.
    pub fn zoom_in(&mut self) {
        self.pixels_per_day = (self.pixels_per_day * 1.2).min(80.0);
        self.pixels_per_hour = self.pixels_per_day / 24.0;

        // Auto-switch to Hours scale when zoomed in enough
        if self.pixels_per_day > 40.0 && self.scale == TimelineScale::Days {
            self.scale = TimelineScale::Hours;
        }
    }

    /// Zoom out (decrease pixels per day), auto-switching scale if needed.
    pub fn zoom_out(&mut self) {
        self.pixels_per_day = (self.pixels_per_day / 1.2).max(2.0);
        self.pixels_per_hour = self.pixels_per_day / 24.0;

        // Auto-switch away from Hours scale when zoomed out
        if self.pixels_per_day < 30.0 && self.scale == TimelineScale::Hours {
            self.scale = TimelineScale::Days;
        }
    }

}
