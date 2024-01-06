use regex::Regex;
use serde_json::Value;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn extract_receive_data(log_line: &str) -> Option<Value> {
    //     let json_string = String::from("\"{\\\"schema\\\":\\\"iglu:com.snowplowanalytics.snowplow\\\\/payload_data\\\\/jsonschema\\\\/1-0-4\\\",\\\"data\\\":[{\\\"vp\\\":\\\"1179x2556\\\",\\\"eid\\\":\\\"FF9CBD88-425C-4AB5-B48C-6B701F9FCF6F\\\",\\\"p\\\":\\\"mob\\\",\\\"cx\\\":\\\"eyJzY2hlbWEiOiJpZ2x1OmNvbS5zbm93cGxvd2FuYWx5dGljcy5zbm93cGxvd1wvY29udGV4dHNcL2pzb25zY2hlbWFcLzEtMC0xIiwiZGF0YSI6W3sic2NoZW1hIjoiaWdsdTpjb20uc25vd3Bsb3dhbmFseXRpY3Muc25vd3Bsb3dcL21vYmlsZV9jb250ZXh0XC9qc29uc2NoZW1hXC8xLTAtMiIsImRhdGEiOnsicGh5c2ljYWxNZW1vcnkiOjU5MjU0NTM4MjQsImFwcGxlSWRmdiI6IjIwNTJDMDlGLUI2RjktNDg3NC04NjNBLUU2NjQ2ODEyQTY5RSIsImJhdHRlcnlMZXZlbCI6OTAsIm5ldHdvcmtUZWNobm9sb2d5IjoiQ1RSYWRpb0FjY2Vzc1RlY2hub2xvZ3lMVEUiLCJiYXR0ZXJ5U3RhdGUiOiJjaGFyZ2luZyIsIm9zVmVyc2lvbiI6IjE3LjAuMyIsImFwcEF2YWlsYWJsZU1lbW9yeSI6MzE4NDExNDU2MCwibmV0d29ya1R5cGUiOiJ3aWZpIiwib3NUeXBlIjoiaW9zIiwiZGV2aWNlTW9kZWwiOiJpUGhvbmUxNSwyIiwidG90YWxTdG9yYWdlIjoyNTU4NjY3ODU3OTIsImxvd1Bvd2VyTW9kZSI6ZmFsc2UsImRldmljZU1hbnVmYWN0dXJlciI6IkFwcGxlIEluYy4iLCJhdmFpbGFibGVTdG9yYWdlIjoxNjIxNDQwNTEyMCwiY2FycmllciI6Ii0tIn19LHsic2NoZW1hIjoiaWdsdTpjb20uc25vd3Bsb3dhbmFseXRpY3MubW9iaWxlXC9hcHBsaWNhdGlvblwvanNvbnNjaGVtYVwvMS0wLTAiLCJkYXRhIjp7ImJ1aWxkIjoiODgzMjEiLCJ2ZXJzaW9uIjoiNS43LjIifX0seyJzY2hlbWEiOiJpZ2x1OmNvbS5jb252aXZhXC9jdXN0b21fdGFnc1wvanNvbnNjaGVtYVwvMS0wLTAiLCJkYXRhIjp7ImRhdGEiOnsidHZfbW9kZV9zZXR0aW5ncyI6Im9mZiIsInVzZXJfbG9nZ2VkX2luIjoiZmFsc2UiLCJ1c2VyX2lkIjoiPG51bGw-IiwicHJvZHVjdF90eXBlIjoicHJlbWl1bSJ9fX0seyJzY2hlbWEiOiJpZ2x1OmNvbS5jb252aXZhXC9ldmVudF9pbmZvXC9qc29uc2NoZW1hXC8xLTAtMSIsImRhdGEiOnsicHJldmlvdXNFdmVudFRpbWVzdGFtcCI6IjIwMjMtMTEtMjRUMDQ6NTc6NDcuODc5WiIsImV2ZW50SW5kZXgiOjE1fX0seyJzY2hlbWEiOiJpZ2x1OmNvbS5zbm93cGxvd2FuYWx5dGljcy5zbm93cGxvd1wvY2xpZW50X3Nlc3Npb25cL2pzb25zY2hlbWFcLzEtMC0yIiwiZGF0YSI6eyJ1c2VySWQiOiI0OGRmMmM0My00YjUyLTQ5ZjQtODJmMS1iNzNhNDNmMmM2NGEiLCJzdG9yYWdlTWVjaGFuaXNtIjoiTE9DQUxfU1RPUkFHRSIsInNlc3Npb25JbmRleCI6MTAsInByZXZpb3VzU2Vzc2lvbklkIjoiZTJmY2ZkNzUtZmIwNS00OTY3LWIxZTYtODYyYTU5ZWJiYjVjIiwic2Vzc2lvbklkIjoiMjgyNTY0ODMtYTFmZS00YzcxLTkwMjAtYTQzYjU2ZjhmYzFlIiwiZXZlbnRJbmRleCI6MTUsImZpcnN0RXZlbnRJZCI6IjgxRDUxREUxLTZBMjUtNDNFNC04RTlELUEzMkQzM0I3NTU2NiIsImZpcnN0RXZlbnRUaW1lc3RhbXAiOiIyMDIzLTExLTI0VDA0OjUzOjIxLjM2MloifX0seyJzY2hlbWEiOiJpZ2x1OmNvbS5jb252aXZhXC9jbGlkX3NjaGVtYVwvanNvbnNjaGVtYVwvMS0wLTIiLCJkYXRhIjp7ImV2ZW50SW5kZXgiOjE1LCJjbGlkIjoiMjM1ODQyNzE3Ni4zOTkyOTg4MjQ4LjI5NDg0MTM2NzQuMTIzMjU5ODQxOCIsImlpZCI6IjEwMTc3MDg0NzIiLCJjayI6Ijk3ZDdlNmQ0YTU0YTZmMWMwYjdkODUxMjc5Nzk1ZTE0MTc5MTRjNWYifX0seyJzY2hlbWEiOiJpZ2x1OmNvbS5jb252aXZhLm1vYmlsZVwvaW9zX2FwcF9sb2FkdGltZVwvanNvbnNjaGVtYVwvMS0wLTAiLCJkYXRhIjp7ImFwcEFjdGl2ZSI6MTcwMDgwMTYwMTMzMSwidXNlckludGVyYWN0YWJsZSI6MTcwMDgwMTYwMTM1NSwibG9hZGVkVG9NZW1vcnkiOjE3MDA4MDE2MDA5MzEsImZpbmlzaGVkTGF1bmNoIjoxNzAwODAxNjAxMzI0fX0seyJzY2hlbWEiOiJpZ2x1OmNvbS5jb252aXZhXC9kZXZpY2VfcGVyZm9ybWFuY2VfY29udGV4dFwvanNvbnNjaGVtYVwvMS0wLTAiLCJkYXRhIjp7ImNwdVVzYWdlIjo1fX0seyJzY2hlbWEiOiJpZ2x1OmNvbS5zbm93cGxvd2FuYWx5dGljcy5tb2JpbGVcL3NjcmVlblwvanNvbnNjaGVtYVwvMS0wLTAiLCJkYXRhIjp7InRvcFZpZXdDb250cm9sbGVyIjoiVEZQcmVzZW50YXRpb24uQXBwQ29vcmRpbmF0b3IiLCJpZCI6Ijc3Qzg2MkZFLUNFOTUtNDQwQi1BM0E0LTg5QTM4RUM4OTkwNyIsIm5hbWUiOiJURlByZXNlbnRhdGlvbi5BcHBDb29yZGluYXRvciIsInR5cGUiOiJEZWZhdWx0Iiwidmlld0NvbnRyb2xsZXIiOiJURlByZXNlbnRhdGlvbi5BcHBDb29yZGluYXRvciJ9fSx7InNjaGVtYSI6ImlnbHU6Y29tLmNvbnZpdmEubW9iaWxlXC9pb3Nfc2NyZWVuX2xvYWR0aW1lXC9qc29uc2NoZW1hXC8xLTAtMCIsImRhdGEiOnsic2NyZWVuTG9hZGVkVG9NZW1vcnkiOjE3MDA4MDE2MDExNjEsInNjcmVlbkludGVyYWN0YWJsZSI6MTcwMDgwMTYwMTM2Mn19LHsic2NoZW1hIjoiaWdsdTpjb20uc25vd3Bsb3dhbmFseXRpY3MubW9iaWxlXC9hcHBsaWNhdGlvbl9saWZlY3ljbGVcL2pzb25zY2hlbWFcLzEtMC0wIiwiZGF0YSI6eyJpbmRleCI6MywiaXNWaXNpYmxlIjpmYWxzZX19XX0\\\",\\\"tz\\\":\\\"Asia\\\\/Bangkok\\\",\\\"stm\\\":\\\"1700801868882\\\",\\\"dtm\\\":\\\"1700801867899\\\",\\\"tv\\\":\\\"ios-0.2.16\\\",\\\"tna\\\":\\\"CAT\\\",\\\"ue_px\\\":\\\"eyJzY2hlbWEiOiJpZ2x1OmNvbS5zbm93cGxvd2FuYWx5dGljcy5zbm93cGxvd1wvdW5zdHJ1Y3RfZXZlbnRcL2pzb25zY2hlbWFcLzEtMC0wIiwiZGF0YSI6eyJzY2hlbWEiOiJpZ2x1OmNvbS5zbm93cGxvd2FuYWx5dGljcy5zbm93cGxvd1wvYXBwbGljYXRpb25fYmFja2dyb3VuZFwvanNvbnNjaGVtYVwvMS0wLTAiLCJkYXRhIjp7ImJhY2tncm91bmRJbmRleCI6M319fQ\\\",\\\"e\\\":\\\"ue\\\",\\\"lang\\\":\\\"nl-NL\\\",\\\"aid\\\":\\\"IOS\\\",\\\"res\\\":\\\"1179x2556\\\"}]}\"");
    let re = Regex::new(r#"Receive data: b"(.+?)", Ip address:"#).unwrap();

    if let Some(captures) = re.captures(log_line) {
        println!("match found");
        if let Some(data_str) = captures.get(1) {
            println!("data: {:?}", data_str.as_str());
            // Assuming that the data is a valid JSON string
            let value = serde_json::from_str(data_str.as_str()).ok();
            println!("value: {:?}", value);
            return value;
        }
    }

    None
}


fn process_log_file(file_path: &str) -> io::Result<()> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        if let Ok(log_line) = line {
            println!("log_line: {:?}", log_line);
            // if let Some(receive_data) = extract_receive_data(&log_line) {
            //     println!("Received data: {:?}", receive_data);
            // }
        }
    }

    Ok(())
}

fn main() {
    // Replace "your_log_file.txt" with the actual file path
    let file_path = "/Users/mkumar/Downloads/search-results-2023-12-18T00_58_58.309-0800.csv";

    if let Err(err) = process_log_file(file_path) {
        eprintln!("Error reading file: {:?}", err);
    }

    // let log_line = "";
    // let res = extract_receive_data(log_line);
    // println!("")
}
