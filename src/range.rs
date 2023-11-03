// Dates are in YYYY/MM/DD format.
pub enum Range {
    AllTime,
    In(String),
    FromTo( (String, String) )
}

pub fn process_range(range: Range) -> (Option<String>, Option<String>, Option<String>) {
    let mut from: Option<String> = None;
    let mut until: Option<String> = None;
    let mut _in: Option<String> = None;
    match range {
        Range::AllTime => {
            from = None;
            until = None;
            _in = None;
        },
        Range::In(range) => {
            _in = Some(range);
        }
        Range::FromTo(range) => {
            from = Some(range.0);
            until = Some(range.1);
        },
    };
    (from, until, _in)
}