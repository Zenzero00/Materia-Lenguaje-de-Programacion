use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum Status {
    #[serde(rename = "Por hacer")]
    Todo,
    #[serde(rename = "En progreso")]
    InProgress,
    #[serde(rename = "Terminada")]
    Done,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Todo => write!(f, "Por hacer"),
            Status::InProgress => write!(f, "En progreso"),
            Status::Done => write!(f, "Terminada"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: i32,
    pub description: String,
    pub status: Status,
    pub created_at: u64,
    pub updated_at: u64,
    pub completed_at: Option<u64>,
}

impl Task {
    pub fn now_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }

    pub fn format_time(ts: Option<u64>) -> String {
        if let Some(sec) = ts {
            let time = UNIX_EPOCH + std::time::Duration::from_secs(sec);
            let datetime: chrono_like::SimpleDateTime = time.into();
            datetime.to_string()
        } else {
            "---".into()
        }
    }
}

mod chrono_like {
    use std::time::{SystemTime, UNIX_EPOCH};
    use std::fmt;

    pub struct SimpleDateTime {
        pub year: i32,
        pub month: u32,
        pub day: u32,
        pub hour: u32,
        pub minute: u32,
    }

    impl From<SystemTime> for SimpleDateTime {
        fn from(st: SystemTime) -> Self {
            use std::time::Duration;
            let since_epoch = st.duration_since(UNIX_EPOCH).unwrap_or(Duration::ZERO);
            let days = since_epoch.as_secs() / 86400;
            let year = 1970 + (days / 365) as i32;
            let day_of_year = (days % 365) as u32;
            let month = 1 + (day_of_year / 30);
            let day = 1 + (day_of_year % 30);
            let secs_today = since_epoch.as_secs() % 86400;
            let hour = (secs_today / 3600) as u32;
            let minute = ((secs_today % 3600) / 60) as u32;
            Self {
                year,
                month,
                day,
                hour,
                minute,
            }
        }
    }

    impl fmt::Display for SimpleDateTime {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{:02}/{:02}/{} {:02}:{:02}",
                self.day, self.month, self.year, self.hour, self.minute
            )
        }
    }
}
