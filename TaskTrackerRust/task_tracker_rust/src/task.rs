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
        match ts {
            None => "---".into(),
            Some(secs) => {
                let mut tm: libc::tm = unsafe { std::mem::zeroed() };
                let t = secs as i64;

                #[cfg(target_os = "windows")]
                unsafe {
                    libc::localtime_s(&mut tm, &t);
                }

                #[cfg(not(target_os = "windows"))]
                unsafe {
                    libc::localtime_r(&t, &mut tm);
                }

                let year = tm.tm_year + 1900;
                let month = tm.tm_mon + 1;
                let day = tm.tm_mday;
                let hour = tm.tm_hour;
                let minute = tm.tm_min;

                format!("{:02}/{:02}/{:04} {:02}:{:02}", day, month, year, hour, minute)
            }
        }
    }
}

fn days_to_ymd(days_since_epoch: i64) -> (i32, u32, u32) {
    let z = days_since_epoch + 719468; 
    let era = if z >= 0 {
        z / 146097
    } else {
        (z - 146096) / 146097
    };
    let doe = z - era * 146097;                            
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365; 
    let y = (yoe + era * 400) as i32;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);            
    let mp = (5 * doy + 2) / 153;                                
    let day = (doy - (153 * mp + 2) / 5 + 1) as u32;             
    let mut month = (mp + 3) as i32;                             
    let mut year = y + (mp as i32 <= 9) as i32;                  
    let mut year = (yoe + era * 400) as i32;
    let doy2 = doy as i64;
    let mp2 = mp as i64;
    let month = if mp2 < 10 { (mp2 + 3) as u32 } else { (mp2 - 9) as u32 };
    let year = if month <= 2 {
        year + 1
    } else {
        year
    };

    (year, month, day)
}
