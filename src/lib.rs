pub mod tests;

use chrono::naive::NaiveDateTime;
use chrono::{Datelike, Timelike};
use colored::Colorize;
use flate2::read;
use lazy_static::lazy_static;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::path::Path;

lazy_static! {
    // it is assumed that the log is written in the current century
    static ref CENTURY: i32 = (chrono::offset::Local::now().year() / 100) * 100;
}

pub struct DateTimeHolder {
    start: u64,
    end: u64,
}

impl DateTimeHolder {
    pub fn new(start: Option<&String>, end: Option<&String>) -> DateTimeHolder {
        let start = match normalized_command_line_date(start) {
            Some(Ok(value)) => value,
            Some(Err(e)) => {
                eprintln!("Start-Date: {}", e.bold().red());
                ::std::process::exit(1);
            }
            None => 0,
        };
        let end = match normalized_command_line_date(end) {
            Some(Ok(value)) => value,
            Some(Err(e)) => {
                eprintln!("End-Date{}", e.bold().red());
                ::std::process::exit(1);
            }
            None => u64::MAX,
        };

        DateTimeHolder { start, end }
    }

    pub fn validate(&self) -> bool {
        if self.start > self.end {
            return false;
        }
        true
    }
}

/// Main entry point
#[inline(never)]
pub fn process_file(
    start_end_date: &DateTimeHolder,
    file_name: Option<&str>,
    debug: u8,
    fast: bool,
    output: &mut impl Write,
    input: &mut impl Read,
) {
    // read from stdin, or file (gzip)
    #[allow(clippy::unnecessary_unwrap)]
    let mut buf_reader: Box<dyn BufRead> = if file_name.is_none() {
        Box::new(BufReader::with_capacity(262_144, input))
    } else {
        let file_name = file_name.unwrap();
        let file = File::open(file_name);
        if let Ok(file) = file {
            let path = Path::new(file_name);
            if path.extension() == Some(OsStr::new("gz")) {
                Box::new(BufReader::with_capacity(
                    262_144,
                    read::GzDecoder::new(file),
                ))
            } else {
                Box::new(BufReader::with_capacity(262_144, file))
            }
        } else {
            eprintln!("Could not open file {}", file_name.bold().red());
            return;
        }
    };

    let mut buf: Vec<u8> = Vec::with_capacity(4096);
    let mut bw = BufWriter::with_capacity(262_144, output);
    while let Ok(bytes_read) = buf_reader.read_until(0x0A_u8, &mut buf) {
        if bytes_read == 0 {
            break;
        }

        if buf.len() < 19 {
            if debug > 1 {
                eprintln!("{}{}", "Line to short: ".bright_red(), unsafe {
                    std::str::from_utf8_unchecked(&buf)
                });
            }
            buf.clear();
            continue;
        }

        //let log_datetime = normalized_datetime_naive(&buf);
        let log_datetime = if fast {normalized_datetime(&buf)} else {normalized_datetime_naive(&buf)};
        if let Some(log_datetime) = log_datetime {
            if (log_datetime >= start_end_date.start) & (log_datetime <= start_end_date.end) {
                // BufWriter.write_all() gives UTF-8 errors on windows
                // let retval = output.write_all(&buf);
                let retval = bw.write_all(&buf);
                match retval {
                    Ok(_) => (),
                    Err(err) => {
                        eprintln!("{:?}", err);
                        ::std::process::exit(1);
                    }
                }
            }
        }
        buf.clear();
    }
}

// disgusting but ~3x faster 
// 2023-01-24 13:57:31   yoda               19
// 24.12.22 00:02:05
// 20230729111238
//#[inline(always)]
pub fn normalized_datetime(buf: &[u8]) -> Option<u64> 
{    
    if (buf.len() > 14) && ( 
       (buf[0] >= 48) & (buf[0] <= 57) & (buf[1] >= 48) & (buf[1] <= 57) &
       (buf[3] >= 48) & (buf[3] <= 57) & (buf[6] >= 48) & (buf[6] <= 57) &
       (buf[9] >= 48) & (buf[9] <= 57) & (buf[12] >= 48) & (buf[12] <= 57)) {
        // quick check passed 
    } else  {
        return None;
    }

    let y2 = (((buf[0] as i32) - 48) * 1000) + (((buf[1] as i32) - 48) * 100);
    if y2 < 0 {
        return None;
    }    

    // yoda format
    if  (buf.len() >= 19) && ((buf[4] == 45) & (buf[7] == 45) & (buf[10] == 32) & (buf[13] == 58) & (buf[16] == 58)) 
    {
        let t2 = (buf[2] >= 48) & (buf[2] <= 57);
        let t5 = (buf[5] >= 48) & (buf[5] <= 57);
        let t8 = (buf[8] >= 48) & (buf[8] <= 57);
        let t11 = (buf[11] >= 48) & (buf[11] <= 57);
        let t14 = (buf[14] >= 48) & (buf[14] <= 57);
        let t15 = (buf[15] >= 48) & (buf[15] <= 57);
        let t17 = (buf[17] >= 48) & (buf[17] <= 57);
        let t18 = (buf[18] >= 48) & (buf[18] <= 57);
        let yoda_found =t2 & t5 & t8 & t11 & t14 & t15 & t17 & t18;
        if yoda_found {
            let y1 = ((buf[2] as i16) - 48) * 10 + ((buf[3] as i16) - 48);
            let year = (y1 as i32) + y2;
            let month = ((buf[5] as i16) - 48) * 10 + ((buf[6] as i16)- 48);
            let day = ((buf[8] as i16) - 48) * 10 + ((buf[9] as i16)- 48);
            let hour = ((buf[11] as i16) - 48) * 10 + ((buf[12] as i16) - 48);
            let minute = ((buf[14] as i16) - 48) * 10 + ((buf[15] as i16) - 48);
            let second = ((buf[17] as i16) - 48) * 10 + ((buf[18] as i16)- 48);

            return calc_u64(second, minute, hour, day, month, year);
        }
    }
    
    // 20230729111238 Carmen-Error
    if (buf[2] >= 48) & (buf[2] <= 57) & 
    (buf[4] >= 48) & (buf[4] <= 57) & 
    (buf[5] >= 48) & (buf[5] <= 57) & 
    (buf[7] >= 48) & (buf[7] <= 57) & 
    (buf[8] >= 48) & (buf[8] <= 57) & 
    (buf[10] >= 48) & (buf[10] <= 57) &
    (buf[11] >= 48) & (buf[11] <= 57) &
    (buf[13] >= 48) & (buf[13] <= 57) 
    {
        // parse as carmen error Carmen-Error
        let y1 = ((buf[2] as i16) - 48) * 10 + ((buf[3] as i16) - 48);
        let year = (y1 as i32) + y2;
        let month = ((buf[4] as i16) - 48) * 10 + ((buf[5] as i16) - 48);
        let day = ((buf[6] as i16) - 48) * 10 + ((buf[7] as i16) - 48);
        let hour = ((buf[8] as i16) - 48) * 10 + ((buf[9] as i16) - 48);
        let minute = ((buf[10] as i16) - 48) * 10 + ((buf[11] as i16) - 48);
        let second = ((buf[12] as i16) - 48) * 10 + ((buf[13] as i16) - 48);

        return calc_u64(second, minute, hour, day, month, year);
    }
    
    // 24.12.22 00:02:05 carmen normal 
    if (buf.len() >= 17) && ((buf[2] == 46) & (buf[5] == 46) & (buf[8] == 32) & (buf[11] == 58) & (buf[14] == 58)) 
    {
        let t3 = (buf[4] >= 48) & (buf[5] <= 57);
        let t5 = (buf[7] >= 48) & (buf[7] <= 57);
        let t7 = (buf[10] >= 48) & (buf[10] <= 57);
        let t9 = (buf[13] >= 48) & (buf[13] <= 57);
        let t10 = (buf[15] >= 48) & (buf[15] <= 57);
        let t11 = (buf[16] >= 48) & (buf[16] <= 57);
        let carmen_found =  t3 & t5 & t7 & t9 & t10 & t11;
        
        if carmen_found {
            let year  = (((((buf[6] as i16) - 48) * 10) + ((buf[7] as i16) - 48)) as i32) + *CENTURY;
            let month = ((buf[3] as i16) - 48) * 10 + ((buf[4] as i16) - 48);
            let day = ((buf[0] as i16) - 48) * 10 + ((buf[1] as i16) - 48);
            let hour = ((buf[9] as i16) - 48) * 10 + ((buf[10] as i16) - 48);
            let minute = ((buf[12] as i16) - 48) * 10 + ((buf[13] as i16) - 48);
            let second = ((buf[15] as i16) - 48) * 10 + ((buf[16] as i16) - 48);
            return calc_u64(second, minute, hour, day, month, year);
        }
    }

   None
}

//#[inline(never)]
fn calc_u64(second: i16, minute: i16, hour: i16, day: i16, month: i16, year: i32) -> Option<u64> {
    if (0..=59).contains(&second) & (0..=59).contains(&minute) & (0..=24).contains(&hour) & (0..=31).contains(&day) & (0..=12).contains(&month) & (1000..=4000).contains(&year)
    {
        let mut value: u64 = second as u64;
        value += (minute as u64) << 8;
        value += (hour as u64) << 16;
        value += (day as u64) << 24;
        value += (month as u64) << 32;
        value += (year as u64) << 40;
        return Some(value)
    }
    None
}

// 2023-02-01 08:18:12,
// 24.12.22 00:02:05
//#[inline(always)]
pub fn normalized_datetime_naive(buf: &[u8]) -> Option<u64> {
    if buf.len() < 19 {
        return None;
    }
    // 24.12.22 00:10:27     carmen  17
    let carmen_fmt = "%d.%m.%Y %H:%M:%S";
    // 2023-01-26 09:32:28   yoda  19
    let yoda_fmt = "%Y-%m-%d %H:%M:%S";
    // 20230729111238        carmen, error log
    let carmen_fmt_err = "%Y%m%d%H%M%S";

    let line_str = unsafe { std::str::from_utf8_unchecked(buf) };
    let dt = NaiveDateTime::parse_from_str(&line_str[..17], carmen_fmt);
    match dt {
        Ok(d) => Some(normalize_bits(d)),
        Err(_) => {
            let dt = NaiveDateTime::parse_from_str(&line_str[..19], yoda_fmt);
            match dt {
                Ok(d) => Some(normalize_bits(d)),
                Err(_) => {
                    let dt = NaiveDateTime::parse_from_str(&line_str[..14], carmen_fmt_err);
                    match dt {
                        Ok(d) => Some(normalize_bits(d)),
                        Err(_) => None,
                    }
                }
            }
        }
    }
}

/// "%d.%m.%Y %H:%M:%S"
#[inline(never)]
fn normalized_command_line_date(date_time: Option<&String>) -> Option<Result<u64, String>> {
    match date_time {
        Some(time) => {
            let dt = NaiveDateTime::parse_from_str(time, "%d.%m.%Y %H:%M:%S");
            match dt {
                Ok(d) => Some(Ok(normalize_bits(d))),
                Err(_) => Some(Err(format!("couldn't parse dateTime: {time}"))),
            }
        }
        None => None,
    }
}

// YYYYMMDDhhmmss
//#[inline(always)]
fn normalize_bits(d: NaiveDateTime) -> u64 {
    let mut value: u64 = d.second() as u64;
    value += (d.minute() as u64) << 8;
    value += (d.hour() as u64) << 16;
    value += (d.day() as u64) << 24;
    value += (d.month() as u64) << 32;
    if d.year() <= 99 {
        value += ((d.year() + *CENTURY) as u64) << 40;
    } else {
        value += (d.year() as u64) << 40;
    }
    value
}
