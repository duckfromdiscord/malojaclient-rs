/// A date range for a maloja operation.
/// Can either be all time, within a time period (e.g. "today", "thisyear", "2024/01/01"), or within two dates.
///
/// Dates are in YYYY/MM/DD format.
#[derive(PartialEq)]
pub enum Range {
    /// All time.
    AllTime,
    /// Within a time period (e.g. "today", "thisyear", "2024/01/01").
    In(String),
    /// Within two dates.
    FromTo((String, String)),
}

#[allow(missing_docs)]
pub fn process_range(range: Range) -> (Option<String>, Option<String>, Option<String>) {
    let mut from: Option<String> = None;
    let mut until: Option<String> = None;
    let mut _in: Option<String> = None;
    match range {
        Range::AllTime => {
            from = None;
            until = None;
            _in = None;
        }
        Range::In(range) => {
            _in = Some(range);
        }
        Range::FromTo(range) => {
            from = Some(range.0);
            until = Some(range.1);
        }
    };
    (from, until, _in)
}
