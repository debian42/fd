#[cfg(test)]
mod test {
    use crate::process_file;
    use crate::DateTimeHolder;
    use crate::normalized_datetime_naive;
    use crate::normalized_datetime;
    use std::io::Cursor;

    #[test]
    fn test_math_century() {
        assert_eq!(1900, (1999 / 100) * 100);
        assert_eq!(2000, (2099 / 100) * 100);
        assert_eq!(2000, (2001 / 100) * 100);
        assert_eq!(2000, (2000 / 100) * 100);
        assert_eq!(2100, (2199 / 100) * 100);
        assert_eq!(3100, (3199 / 100) * 100);
    }

    #[test]
    fn test_date_time_holder() {
        let log_line = r#"2099-12-31 00:00:01,828"#.to_string().into_bytes();
        let log_datetime = normalized_datetime(&log_line);
        let start_end_date: DateTimeHolder = DateTimeHolder::new(None, Some(&"31.12.99 0:0:1".to_string()));
        assert_eq!(log_datetime.unwrap(), start_end_date.end);
    }    

    #[test]
    fn test_normalized_datetime_yoda() {
        let log_line = r#"2023-01-26 09:32:28,828 INFO  [null,d7256a35f724f75f9083233230373335393931] [de.telekom.crm.rest.service.base.impl.ServiceStateContainerFilter] (default task-24) START SERVICE [/EmailVerificationResult/v1/business-partner/email/verification-result] Header: Accept:[application/json, application/*+json] Accept-Encoding:[gzip] Authorization:[Bearer }"#.to_string().into_bytes();
        let log_datetime = normalized_datetime(&log_line);
        assert!(log_datetime.is_some());
        assert_eq!(log_datetime.unwrap(), 2224316754763804);
    } 
    
    #[test]
    fn test_normalized_datetime_a1() {
        let log_line = r#"2023-a1-26 09:32:28,828 INFO  [null,d7256a35f724f75f9083233230373335393931] [de.telekom.crm.rest.service.base.impl.ServiceStateContainerFilter] (default task-24) START SERVICE [/EmailVerificationResult/v1/business-partner/email/verification-result] Header: Accept:[application/json, application/*+json] Accept-Encoding:[gzip] Authorization:[Bearer }"#.to_string().into_bytes();
        let log_datetime = normalized_datetime(&log_line);
        assert!(log_datetime.is_none());
    }  
    
    #[test]
    fn test_normalized_datetime_222() {
        let log_line = r#"2023-221-26 09:32:28,828 INFO  [null,d7256a35f724f75f9083233230373335393931] [de.telekom.crm.rest.service.base.impl.ServiceStateContainerFilter] (default task-24) START SERVICE [/EmailVerificationResult/v1/business-partner/email/verification-result] Header: Accept:[application/json, application/*+json] Accept-Encoding:[gzip] Authorization:[Bearer }"#.to_string().into_bytes();
        let log_datetime = normalized_datetime(&log_line);
        assert!(log_datetime.is_none());
    }

    #[test]
    fn test_normalized_datetime_tilde() {
        let log_line = r#"~123-21-26 09:32:28,828 INFO  [null,d7256a35f724f75f9083233230373335393931] [de.telekom.crm.rest.service.base.impl.ServiceStateContainerFilter] (default task-24) START SERVICE [/EmailVerificationResult/v1/business-partner/email/verification-result] Header: Accept:[application/json, application/*+json] Accept-Encoding:[gzip] Authorization:[Bearer }"#.to_string().into_bytes();
        let log_datetime = normalized_datetime(&log_line);
        assert!(log_datetime.is_none());
    }

    #[test]
    fn test_normalized_datetime_exclamation_mark() {
        let log_line = r#"!!23-21-26 09:32:28,828 INFO  [null,d7256a35f724f75f9083233230373335393931] [de.telekom.crm.rest.service.base.impl.ServiceStateContainerFilter] (default task-24) START SERVICE [/EmailVerificationResult/v1/business-partner/email/verification-result] Header: Accept:[application/json, application/*+json] Accept-Encoding:[gzip] Authorization:[Bearer }"#.to_string().into_bytes();
        let log_datetime = normalized_datetime(&log_line);
        assert!(log_datetime.is_none());
    }

    #[test]
    fn test_normalized_datetime_exclamation_mark_1() {
        let log_line = r#"!!230729111238; INFO  [null,d7256a35f724f75f9083233230373335393931] [de.telekom.crm.rest.service.base.impl.ServiceStateContainerFilter] (default task-24) START SERVICE [/EmailVerificationResult/v1/business-partner/email/verification-result] Header: Accept:[application/json, application/*+json] Accept-Encoding:[gzip] Authorization:[Bearer }"#.to_string().into_bytes();
        let log_datetime = normalized_datetime(&log_line);
        assert!(log_datetime.is_none());
    }

    #[test]
    fn test_normalized_datetime_crap() {
        let log_line = r#"0234087082347882 [de.telekom.crm.rest.service.base.impl.ServiceStateContainerFilter] (default task-24) START SERVICE [/EmailVerificationResult/v1/business-partner/email/verification-result] Header: Accept:[application/json, application/*+json] Accept-Encoding:[gzip] Authorization:[Bearer }"#.to_string().into_bytes();
        let log_datetime = normalized_datetime(&log_line);
        assert!(log_datetime.is_none());
    }

    #[test]
    fn test_normalized_datetime_2023() {
        let log_line = r#"2023.21.26 09:32:28,828 INFO  [null,d7256a35f724f75f9083233230373335393931] [de.telekom.crm.rest.service.base.impl.ServiceStateContainerFilter] (default task-24) START SERVICE [/EmailVerificationResult/v1/business-partner/email/verification-result] Header: Accept:[application/json, application/*+json] Accept-Encoding:[gzip] Authorization:[Bearer }"#.to_string().into_bytes();
        let log_datetime = normalized_datetime(&log_line);
        assert!(log_datetime.is_none());
    }

    #[test]
    fn test_normalized_datetime_99() {
        let log_line = r#"2023-99-26 09:32:28,828 INFO  [null,d7256a35f724f75f9083233230373335393931] [de.telekom.crm.rest.service.base.impl.ServiceStateContainerFilter] (default task-24) START SERVICE [/EmailVerificationResult/v1/business-partner/email/verification-result] Header: Accept:[application/json, application/*+json] Accept-Encoding:[gzip] Authorization:[Bearer }"#.to_string().into_bytes();
        let log_datetime = normalized_datetime(&log_line);
        assert!(log_datetime.is_none());
    }

    #[test]
    fn test_normalized_datetime_yoda_carmen() {
        let log_line = r#"30.12.22 00:22:52 M     0 FILE /users/cloud/user1/data/projects/Software/tfc_source/tfc_apps/src/tfcwebserviceprovider/tfcrpc.cpp:615 [CRequestSOAP] PID: 3825 ServiceCall HTMX:\rwv/services/ERP/IntangibleAsset/SIMReadServices/getSIMInfo7.getSIMInfo7 CorrelationId: a63b1b3d-59bb-4851-8c98-c655"#.to_string().into_bytes();
        let log_datetime = normalized_datetime(&log_line);
        assert!(log_datetime.is_some());
        assert_eq!(log_datetime.unwrap(), 2223264554292788);
    }

    #[test]
    fn test_normalized_datetime_yoda_carmen_error() {
        let log_line = r#"20230729111238;edeyl6;;TfcWebserviceProvider;1950;E;0;0 Nr: 2 Message: E_TechUnexpectedService: ErrorCount=0;WorstError=-1;SubsystemID=0;TextDBID=0;LocationNr=0;FileName=//users//cloud//user1//data//projects//SoftWare//ccb_source//ccr_apps//src//servicesimpl//resourcereadservices//impl//CCGetSIMProfileStatus2ServiceImplementation.cpp;LineNumber=174;ErrorNo=2;Text=E_ProfServiceFailed: Fehler 2 beim Aufruf von IRIS-Service GetProfileStatus: Die ICC-ID ist im eSIM-System nicht bekannt.\nIRIS Fehlerinfo: Unknown ICCID;TextID=0;Recommendation=0;Level=5;OutputChannel=0;ExceptionClassName=E_TechUnexpectedService;ProcessId=0;ThreadId=0;ChannelId=0; 0  /users/cloud/user1/data/projects/SoftWare/tfc_source/tfc_apps/src/tfcwebserviceprovider/tfcrpc.cpp 398 TfcRpc 0;B2164F67-1BCF-4E57-BC58-6A17B74CA8CD"#.to_string().into_bytes();
        let log_datetime = normalized_datetime(&log_line);
        assert!(log_datetime.is_some());
        assert_eq!(log_datetime.unwrap(), 2224342575025190);
    }

    #[test]
    fn test_normalized_datetime_yoda_carmen_error_14() {
        let log_line = r#"20230729111238;"#.to_string().into_bytes();
        let log_datetime = normalized_datetime(&log_line);
        assert!(log_datetime.is_some());
        assert_eq!(log_datetime.unwrap(), 2224342575025190);
    }

    #[test]
    fn test_normalized_datetime_yoda_19() {
        let log_line = r#"2023-01-26 09:32:28"#.to_string().into_bytes();
        let log_datetime = normalized_datetime(&log_line);
        assert!(log_datetime.is_some());
        assert_eq!(log_datetime.unwrap(), 2224316754763804);
    }

    #[test]
    fn test_parse_date_start_lt() {
        let log_line = r#"30.12.22 02:30:57 M     0 FILE /users/cloud/user1/data/projects/Software/tfc_source/tfc_apps/src/tfcwebserviceprovider/tfcrpc.cpp:615 [CRequestSOAP] PID: 3825 ServiceCall HTMX:\de.de.de/services/ERP/IntangibleAsset/SIMReadServices/getSIMInfo7.getSIMInfo7 CorrelationId: a63b1b3d-59bb-4851-8c98-c655"#.to_string().into_bytes();
        let mut data = Cursor::new(log_line);
        
        let mut out: Vec<u8> = Vec::new();
        let start_end_date: DateTimeHolder = DateTimeHolder::new(Some(&"1.1.20 0:0:0".to_string()), None);
        assert_eq!(start_end_date.end, u64::MAX,  " {} and {}", start_end_date.end, u64::MAX);
        process_file(&start_end_date, None, 0,true, &mut out, &mut data);
        assert_eq!(data.into_inner(), out);
    }

    #[test]
    fn test_parse_date_start_start_and_end() {
        let log_line = r#"30.12.22 02:30:57 M     0 FILE /users/cloud/user1/data/projects/Software/tfc_source/tfc_apps/src/tfcwebserviceprovider/tfcrpc.cpp:615 [CRequestSOAP] PID: 3825 ServiceCall HTMX:\de.de.de/services/ERP/IntangibleAsset/SIMReadServices/getSIMInfo7.getSIMInfo7 CorrelationId: a63b1b3d-59bb-4851-8c98-c655"#.to_string().into_bytes();
        let mut data = Cursor::new(log_line);
        
        let mut out: Vec<u8> = Vec::new();
        let start_end_date: DateTimeHolder = DateTimeHolder::new(Some(&"30.12.22 02:30:57".to_string()), Some(&"31.12.22 0:0:0".to_string()));
        assert_eq!(start_end_date.end, 2223264571064320);
        process_file(&start_end_date, None, 0, false, &mut out, &mut data);
        assert_eq!(data.into_inner(), out);
    }

    #[test]
    fn test_parse_date_start_start_and_end_fast() {
        let log_line = r#"30.12.22 02:30:57 M     0 FILE /users/cloud/user1/data/projects/Software/tfc_source/tfc_apps/src/tfcwebserviceprovider/tfcrpc.cpp:615 [CRequestSOAP] PID: 3825 ServiceCall HTMX:\de.de.de/services/ERP/IntangibleAsset/SIMReadServices/getSIMInfo7.getSIMInfo7 CorrelationId: a63b1b3d-59bb-4851-8c98-c655"#.to_string().into_bytes();
        let mut data = Cursor::new(log_line);
        
        let mut out: Vec<u8> = Vec::new();
        let start_end_date: DateTimeHolder = DateTimeHolder::new(Some(&"30.12.22 02:30:57".to_string()), Some(&"31.12.22 0:0:0".to_string()));
        assert_eq!(start_end_date.end, 2223264571064320);
        process_file(&start_end_date, None, 0, true, &mut out, &mut data);
        assert_eq!(data.into_inner(), out);
    }

    #[test]
    fn test_parse_date_end_date99() {
        let log_line = r#"30.12.99 02:30:57 M     0 FILE /users/cloud/user1/data/projects/Software/tfc_source/tfc_apps/src/tfcwebserviceprovider/tfcrpc.cpp:615 [CRequestSOAP] PID: 3825 ServiceCall HTMX:\de.de.de/services/ERP/IntangibleAsset/SIMReadServices/getSIMInfo7.getSIMInfo7 CorrelationId: a63b1b3d-59bb-4851-8c98-c655"#.to_string().into_bytes();
        let mut data = Cursor::new(log_line);
        let mut out: Vec<u8> = Vec::new();
        let start_end_date: DateTimeHolder = DateTimeHolder::new(None, Some(&"31.12.99 0:0:0".to_string()));
        assert_eq!(start_end_date.start, 0);
        process_file(&start_end_date, None, 0, true, &mut out, &mut data);
        assert_eq!(data.into_inner(), out);
    }

    #[test]
    fn test_parse_date_start_end() {
        let log_line = r#"30.12.22 02:30:57 M     0 FILE /users/cloud/user1/data/projects/Software/tfc_source/tfc_apps/src/tfcwebserviceprovider/tfcrpc.cpp:615 [CRequestSOAP] PID: 3825 ServiceCall HTMX:\de.de.de/services/ERP/IntangibleAsset/SIMReadServices/getSIMInfo7.getSIMInfo7 CorrelationId: a63b1b3d-59bb-4851-8c98-c655"#.to_string().into_bytes();
        let mut data = Cursor::new(log_line);
        
        let mut out: Vec<u8> = Vec::new();
        let start_end_date: DateTimeHolder = DateTimeHolder::new(None, Some(&"31.12.22 0:0:0".to_string()));
        assert_eq!(start_end_date.end, 2223264571064320);
        process_file(&start_end_date, None, 0, true, &mut out, &mut data);
        assert_eq!(data.into_inner(), out);
    }

    #[test]
    fn test_parse_date_start_gt() {
        let log_line = r#"30.12.22 02:30:57 M     0 FILE /users/cloud/user1/data/projects/Software/tfc_source/tfc_apps/src/tfcwebserviceprovider/tfcrpc.cpp:615 [CRequestSOAP] PID: 3825 ServiceCall HTMX:\de.de.de/services/ERP/IntangibleAsset/SIMReadServices/getSIMInfo7.getSIMInfo7 CorrelationId: a63b1b3d-59bb-4851-8c98-c655"#.to_string().into_bytes();
        let mut data = Cursor::new(log_line);
        
        let mut out: Vec<u8> = Vec::new();
        let start_end_date: DateTimeHolder = DateTimeHolder::new(Some(&"1.1.23 0:0:0".to_string()), None);
        assert_eq!(start_end_date.end, u64::MAX,  " {} and {}", start_end_date.end, u64::MAX);
        process_file(&start_end_date, None, 0,true,&mut out, &mut data);
        assert!(out.is_empty());
    }

    #[test]
    fn test_parse_date() {
        let log_line = r#"2023-01-24 13:57:31,828 INFO  [null,d7256a35f724f75f9083233230373335393931] [de.telekom.crm.rest.service.base.impl.ServiceStateContainerFilter] (default task-24) START SERVICE [/EmailVerificationResult/v1/business-partner/email/verification-result] Header: Accept:[application/json, application/*+json] Accept-Encoding:[gzip] Authorization:[Bearer "}"#.to_string().into_bytes();
        let mut data = Cursor::new(log_line);
        
        let mut out: Vec<u8> = Vec::new();
        let start_end_date: DateTimeHolder = DateTimeHolder::new(Some(&"14.01.2023 13:57:30".to_string()), None);
        assert_eq!(start_end_date.end, u64::MAX,  " {} and {}", start_end_date.end, u64::MAX);
        process_file(&start_end_date, None, 0,true,  &mut out, &mut data);
    }

    #[test]
    fn test_parse_date_yoda_comma() {
        let log_line = r#"2023-01-24 13:57:31,}"#.to_string().into_bytes();
        let mut data = Cursor::new(log_line);
        
        let mut out: Vec<u8> = Vec::new();
        let start_end_date: DateTimeHolder = DateTimeHolder::new(Some(&"24.01.2023 13:57:31".to_string()), None);
        assert_eq!(start_end_date.end, u64::MAX,  " {} and {}", start_end_date.end, u64::MAX);
        process_file(&start_end_date, None, 0,true,  &mut out, &mut data);
        assert_eq!(data.into_inner(), out);
    }

    #[test]
    fn test_parse_date_yoda_comma_slow() {
        let log_line = r#"2023-01-24 13:57:31,}"#.to_string().into_bytes();
        let mut data = Cursor::new(log_line);
        
        let mut out: Vec<u8> = Vec::new();
        let start_end_date: DateTimeHolder = DateTimeHolder::new(Some(&"24.01.2023 13:57:31".to_string()), None);
        assert_eq!(start_end_date.end, u64::MAX,  " {} and {}", start_end_date.end, u64::MAX);
        process_file(&start_end_date, None, 0,false,  &mut out, &mut data);
        assert_eq!(data.into_inner(), out);
    }

    #[test]
    fn test_parse_date_yoda_comma_gt() {
        let log_line = r#"2023-01-24 13:57:31,}"#.to_string().into_bytes();
        let mut data = Cursor::new(log_line);
        
        let mut out: Vec<u8> = Vec::new();
        let start_end_date: DateTimeHolder = DateTimeHolder::new(Some(&"24.01.2023 13:57:32".to_string()), None);
        assert_eq!(start_end_date.end, u64::MAX,  " {} and {}", start_end_date.end, u64::MAX);
        process_file(&start_end_date, None, 0,true,  &mut out, &mut data);
        assert!(out.is_empty());
    }

    #[test]
    fn test_normalized_datetime_naive_carmen_line() {
        let log_line = r"30.12.22 00:22:52 H     0 FILE /users/cloud/user1/data/projects/Software/tfc_source/tfc/src/tfctools/TMLogFile.cpp:1595 [TMLogFile] PID: 3825 Der Prozess 3825 auf der Maschine XYZ mit PPID=1 wechselt das Logfile von ../var/TfcWebserviceProvider_prot_4.log zu ../var/TfcWebserviceProvider_prot_5.log.".as_bytes();
        let array = <&[u8; 19]>::try_from(&log_line[..19]);
        let v = match array {
            Ok(v) => v,
            Err(e) => {println!("{e:?}"); &[2u8;19]}
        };
        let log_datetime = normalized_datetime_naive(v);
        assert!(log_datetime.is_some());
        assert_eq!(log_datetime.unwrap(), 2223264554292788);
    }
    
    #[test]
    fn test_normalized_datetime_naive_carmen() {
        let log_line = r"24.12.22 00:10:27,654     carmen  17".as_bytes();
        let array = <&[u8; 19]>::try_from(&log_line[..19]);
        let v = match array {
            Ok(v) => v,
            Err(e) => {println!("{e:?}"); &[2u8;19]}
        };
        let log_datetime = normalized_datetime_naive(v);
        assert!(log_datetime.is_some());
        assert_eq!(log_datetime.unwrap(), 2223264453626395);
    }

    #[test]
    fn test_normalized_datetime_naive_yoda() {
        let log_line = r"2023-01-26 09:32:28     yoda  17".as_bytes();
        let array = <&[u8; 19]>::try_from(&log_line[..19]);
        let v = match array {
            Ok(v) => v,
            Err(e) => {println!("{e:?}"); &[2u8;19]}
        };
        let log_datetime = normalized_datetime_naive(v);
        assert!(log_datetime.is_some());
        assert_eq!(log_datetime.unwrap(), 2224316754763804);
    }

    #[test]
    fn test_normalized_datetime_naive_carmen_err() {
        let log_line = r"20230729111238            carmen, error log".as_bytes();
        let array = <&[u8; 19]>::try_from(&log_line[..19]);
        let v = match array {
            Ok(v) => v,
            Err(e) => {println!("{e:?}"); &[2u8;19]}
        };
        let log_datetime = normalized_datetime_naive(v);
        assert!(log_datetime.is_some());
        assert_eq!(log_datetime.unwrap(), 2224342575025190);
    }
}
