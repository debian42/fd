use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fd::process_file;
use fd::DateTimeHolder;
use fd::normalized_datetime_naive;
use fd::normalized_datetime;
use std::env;
use std::io::Cursor;

// microsoft malloc
use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn benchmark_date_time_holder_new(c: &mut Criterion) {
    c.bench_function("benchmark_DateTimeHolder_new", |b| {
        b.iter(|| {
            black_box({
                let start_end_date: DateTimeHolder =
                    DateTimeHolder::new(Some(&"1.1.23 0:0:0".to_string()), None);

                if !start_end_date.validate() {
                    eprintln!("{}", "End-Date must be greater then Start-Date");
                    ::std::process::exit(1);
                }
            })
        })
    });
}

fn bench_normalized_datetime_carmen_error(c: &mut Criterion) {
    let log_line = r#"20230729111544;edeyl6;;TfcWebserviceProvider;1950;E;0;0 Nr: 30004 Message: E_UnknownCommonKeyValueType: ErrorCount=0;WorstError=-1;SubsystemID=0;TextDBID=0;LocationNr=0;FileName=//users//XXXXXXXXXXX//user1//data//projects//XXXXXXXXXXX//ccb_source//ccr_apps//src//servicesimpl//miscreadservices//impl//CCGetObjectDescriptionList2ServiceImplementation.cpp;LineNumber=1081;ErrorNo=30004;Text=Unbekannter Value Type 'Competitor';TextID=0;Recommendation=0;Level=5;OutputChannel=0;ExceptionClassName=E_UnknownCommonKeyValueType;ProcessId=0;ThreadId=0;ChannelId=0; 0  /users/XXXXXXXXXXX/user1/data/projects/XXXXXXXXXXX/tfc_source/tfc_apps/src/tfcwebserviceprovider/tfcrpc.cpp 398 TfcRpc 0;lknsrv2w_-v2e99tp55kgi"#.to_string().into_bytes();
    
    c.bench_function("bench_normalized_datetime_carmen_error", |b| {
        b.iter(|| {
            black_box({
                normalized_datetime(&log_line);
            })
        })
    });
}

fn bench_normalized_datetime_carmen(c: &mut Criterion) {
    let log_line = r#"30.12.22 00:22:52 H     0 FILE /users/cloud/user1/data/projects/carmen-224/tfc_source/tfc/src/tfctools/TMLogFile.cpp:1595 [TMLogFile] PID: 3825 Der Prozess 3825 auf der Maschine dcplnx22049230 mit PPID=1 wechselt das Logfile von ../var/TfcWebserviceProvider_prot_4.log zu ../var/TfcWebserviceProvider_prot_5.log."#.to_string().into_bytes();
    
    c.bench_function("bench_normalized_datetime_carmen", |b| {
        b.iter(|| {
            black_box({
                normalized_datetime(&log_line);
            })
        })
    });
}

fn bench_normalized_datetime_yoda(c: &mut Criterion) {
    let log_line = r#"2023-01-26 09:32:28,828 INFO  [null,d7256a35f724f75f9083233230373335393931] [de.telekom.crm.rest.service.base.impl.ServiceStateContainerFilter] (default task-24) START SERVICE [/EmailVerificationResult/v1/business-partner/email/verification-result] Header: Accept:[application/json, application/*+json] Accept-Encoding:[gzip] Authorization:[Bearer "}"#.to_string().into_bytes();
    
    c.bench_function("bench_normalized_datetime_yoda", |b| {
        b.iter(|| {
            black_box({
                normalized_datetime(&log_line);
            })
        })
    });
}

fn bench_normalized_datetime_naive_carmen(c: &mut Criterion) {
    let log_line = r"30.12.22 00:22:52 H     0 FILE /users/cloud/user1/data/projects/carmen-224/tfc_source/tfc/src/tfctools/TMLogFile.cpp:1595 [TMLogFile] PID: 3825 Der Prozess 3825 auf der Maschine XYZ mit PPID=1 wechselt das Logfile von ../var/TfcWebserviceProvider_prot_4.log zu ../var/TfcWebserviceProvider_prot_5.log.".as_bytes();
    let array = <&[u8; 19]>::try_from(&log_line[..19]);
    let v = match array {
        Ok(v) => v,
        Err(e) => {println!("{e:?}"); &[2u8;19]}
    };
    c.bench_function("bench_normalized_datetime_naive_carmen", |b| {
        b.iter(|| {
            black_box({
                normalized_datetime_naive(v);
            })
        })
    });
}

fn bench_normalized_datetime_naive_carmen_err(c: &mut Criterion) {
    let log_line = r"20230729111544;edeyl6;;TfcWebserviceProvider;1950;E;0;0 Nr: 30004 Message: E_UnknownCommonKeyValueType: ErrorCount=0;WorstError=-1;SubsystemID=0;TextDBID=0;LocationNr=0;FileName=//users//XXXXXXXXXXX//user1//data//projects//XXXXXXXXXXX//ccb_source//ccr_apps//src//servicesimpl//miscreadservices//impl//CCGetObjectDescriptionList2ServiceImplementation.cpp;LineNumber=1081;ErrorNo=30004;Text=Unbekannter Value Type 'Competitor';TextID=0;Recommendation=0;Level=5;OutputChannel=0;ExceptionClassName=E_UnknownCommonKeyValueType;ProcessId=0;ThreadId=0;ChannelId=0; 0  /users/XXXXXXXXXXX/user1/data/projects/XXXXXXXXXXX/tfc_source/tfc_apps/src/tfcwebserviceprovider/tfcrpc.cpp 398 TfcRpc 0;lknsrv2w_-v2e99tp55kgi".as_bytes();
    let array = <&[u8; 19]>::try_from(&log_line[..19]);
    let v = match array {
        Ok(v) => v,
        Err(e) => {println!("{e:?}"); &[2u8;19]}
    };
    c.bench_function("bench_normalized_datetime_naive_carmen_err", |b| {
        b.iter(|| {
            black_box({
                normalized_datetime_naive(v);
            })
        })
    });
}

fn parse_benchmark_server_local_log(c: &mut Criterion) {
    let filename = if env::consts::OS == "windows" {
        r".\misc\server-local.log"
    } else {
        r"./misc/server-local.log"
    };
    let mut out: Vec<u8> = Vec::with_capacity(2_000_000);
    c.bench_function("server-local.log", |b| {
        b.iter(|| {
            black_box({
                out.clear();
                let start_end_date: DateTimeHolder =
                    DateTimeHolder::new(Some(&"1.1.23 0:0:0".to_string()), None);
                process_file(
                    &start_end_date,
                    Some(filename),
                    0,
                    true, 
                    false,
                    &mut out,
                    &mut std::io::stdin(),
                );
            })
        })
    });
}

fn parse_benchmark_server_local_log_replace(c: &mut Criterion) {
    let filename = if env::consts::OS == "windows" {
        r".\misc\server-local.log"
    } else {
        r"./misc/server-local.log"
    };
    let mut out: Vec<u8> = Vec::with_capacity(2_000_000);
    c.bench_function("server-local.log replace", |b| {
        b.iter(|| {
            black_box({
                out.clear();
                let start_end_date: DateTimeHolder =
                    DateTimeHolder::new(Some(&"1.1.23 0:0:0".to_string()), None);
                process_file(
                    &start_end_date,
                    Some(filename),
                    0,
                    true, 
                    true,
                    &mut out,
                    &mut std::io::stdin(),
                );
            })
        })
    });
}


fn benchmark_line_carmen(c: &mut Criterion) {
    let log_line = black_box(r#"30.12.22 02:30:57 M     0 FILE /users/cloud/user1/data/projects/carmen-224/tfc_source/tfc_apps/src/tfcwebserviceprovider/tfcrpc.cpp:615 [CRequestSOAP] PID: 3825 ServiceCall http://de.de.de/services/ERP/IntangibleAsset/SIMReadServices/getSIMInfo7.getSIMInfo7 CorrelationId: a63b1b3d-59bb-4851-8c98-c655"#.to_string().into_bytes());
    let mut data = Cursor::new(log_line);
    let mut out: Vec<u8> = Vec::with_capacity(100_000);
    c.bench_function("benchmark_line_carmen", |b| {
        b.iter(|| {
            black_box({
                out.clear();
                let start_end_date: DateTimeHolder =
                    DateTimeHolder::new(Some(&"1.1.23 0:0:0".to_string()), None);
                process_file(&start_end_date, None, 0,true, false, &mut out, &mut data);
            })
        })
    });
}

fn benchmark_line_yoda(c: &mut Criterion) {
    let log_line = black_box(r#"2023-01-24 13:57:31,828 INFO  [null,d7256a35f724f75f9083233230373335393931] [de.telekom.crm.rest.service.base.impl.ServiceStateContainerFilter] (default task-24) START SERVICE [/EmailVerificationResult/v1/business-partner/email/verification-result] Header: Accept:[application/json, application/*+json] Accept-Encoding:[gzip] Authorization:[Bearer yyyyyyyyyyyyyyyyyyyyyyyyy] Content-Length:[292] Content-Type:[application/json] Environment:[prod] "#.to_string().into_bytes());
    let mut data = Cursor::new(log_line);
    let mut out: Vec<u8> = Vec::with_capacity(100_000);
    c.bench_function("benchmark_line_yoda", |b| {
        b.iter(|| {
            black_box({
                out.clear();
                let start_end_date: DateTimeHolder =
                    DateTimeHolder::new(Some(&"1.1.23 0:0:0".to_string()), None);
                process_file(&start_end_date, None, 0,true, false, &mut out, &mut data);
            })
        })
    });
}

//
//  Config
//
fn custom_config() -> Criterion {
    let criterion = Criterion::default();
    criterion
        .sample_size(40)
        .warm_up_time(std::time::Duration::from_secs(2))
        .measurement_time(std::time::Duration::from_secs(5))
}

criterion_group! {
       name = benches;
       config = custom_config();
       targets = benchmark_date_time_holder_new, bench_normalized_datetime_yoda,
       bench_normalized_datetime_carmen, bench_normalized_datetime_carmen_error, bench_normalized_datetime_naive_carmen_err,
       parse_benchmark_server_local_log, parse_benchmark_server_local_log_replace, benchmark_line_carmen, benchmark_line_yoda, bench_normalized_datetime_naive_carmen
}
criterion_main!(benches);
