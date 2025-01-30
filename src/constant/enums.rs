#[derive(Debug, Clone, Copy)]
pub enum INTERVALS {
    Min,
    Hour,
    Day,
    Week,
    Month,
    Quarter,
    Year,
}

pub const fn get_value(key: INTERVALS) -> &'static str {
    match key {
        INTERVALS::Min => "5min",
        INTERVALS::Hour => "hour",
        INTERVALS::Day => "day",
        INTERVALS::Week => "week",
        INTERVALS::Month => "month",
        INTERVALS::Quarter => "quarter",
        INTERVALS::Year => "year",
    }
}
