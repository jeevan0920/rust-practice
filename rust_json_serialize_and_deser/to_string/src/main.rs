use serde_json::{json, Value};
use serde_json::from_str;


#[allow(overflowing_literals)]
fn main() {
    // let json_object = json!(
    //     {
    //         "schema": "iglu:com.snowplowanalytics.snowplow/contexts/jsonschema/1-0-1",
    //         "data": [
    //           {
    //             "schema": "iglu:com.snowplowanalytics.snowplow/client_session/jsonschema/1-0-2",
    //             "data": {
    //               "sessionIndex": 1,
    //               "storageMechanism": "LOCAL_STORAGE",
    //               "firstEventTimestamp": "2023-03-10T15:06:18.428Z",
    //               "firstEventId": "52b3a792-50a9-49cb-a58e-7f4607cfc19d",
    //               "sessionId": "588410f6-c56b-4172-90e1-15c900e249ef",
    //               "eventIndex": 13,
    //               "previousSessionId": null,
    //               "userId": "28e5a620-c844-4270-a076-2ea3c594240b"
    //             }
    //           },
    //           {
    //             "schema": "iglu:com.snowplowanalytics.mobile/application/jsonschema/1-0-0",
    //             "data": {
    //               "build": "1",
    //               "version": "1.0"
    //             }
    //           },
    //           {
    //             "schema": "iglu:com.snowplowanalytics.snowplow/mobile_context/jsonschema/1-0-2",
    //             "data": {
    //               "carrier": "T-Mobile",
    //               "totalStorage": 6240665600,
    //               "systemAvailableMemory": 740384768,
    //               "osVersion": "12",
    //               "batteryState": "unplugged",
    //               "availableStorage": 4881596416,
    //               "osType": "android",
    //               "androidIdfa": "e905f781-8d2f-4ed1-aa94-4aac51940897",
    //               "deviceModel": "sdk_gphone64_arm64",
    //               "lowPowerMode": false,
    //               "deviceManufacturer": "Google",
    //               "networkType": "wifi",
    //               "physicalMemory": 2061529088,
    //               "batteryLevel": 100
    //             }
    //           },
    //           {
    //             "schema": "iglu:com.conviva/clid_schema/jsonschema/1-0-1",
    //             "data": {
    //               "iid": "1643305591",
    //               "ck": "4d2f03dddf417990f520f09d79b11ab014c39dab",
    //               "clid": "2060753275.956347642.1551155978.949878434"
    //             }
    //           },
    //           {
    //             "schema": "iglu:com.conviva.mobile/android_app_loadtime/jsonschema/1-0-1",
    //             "data": {
    //               "onCreate": 1678460733036,
    //               "appLoad": 1678460732934,
    //               "onResume": 1678460733143
    //             }
    //           },
    //           {
    //             "schema": "iglu:com.conviva/event_info/jsonschema/1-0-1",
    //             "data": {
    //               "previousEventTimestamp": "2023-03-10T15:06:55.097Z",
    //               "eventIndex": 13
    //             }
    //           },
    //           {
    //             "schema": "iglu:com.conviva/custom_tags/jsonschema/1-0-0",
    //             "data": {
    //               "data": {
    //                 "int1": "0",
    //                 "test": "pass",
    //                 "double": "1234.1",
    //                 "double1": "0.0",
    //                 "boolean2": "false",
    //                 "boolean1": "true",
    //                 "int": "1234"
    //               }
    //             }
    //           },
    //           {
    //             "schema": "iglu:com.snowplowanalytics.mobile/screen/jsonschema/1-0-0",
    //             "data": {
    //               "activity": "selffinishingactivity.SelfFinishingActivity",
    //               "name": "selffinishingactivity.SelfFinishingActivity",
    //               "id": "046fe892-5463-434e-8c72-e7305869b8f4",
    //               "type": "selffinishingactivity.SelfFinishingActivity"
    //             }
    //           },
    //           {
    //             "schema": "iglu:com.conviva.mobile/android_screen_loadtime/jsonschema/1-0-0",
    //             "data": {
    //               "onCreate": 1678460771665,
    //               "onResume": 1678460771684
    //             }
    //           },
    //           {
    //             "schema": "iglu:com.conviva.mobile/bundle_info/jsonschema/1-0-0",
    //             "data": {
    //               "info": "{\"action\":\"android.intent.action.VIEW\"}"
    //             }
    //           },
    //           {
    //             "schema": "iglu:com.snowplowanalytics.mobile/application_lifecycle/jsonschema/1-0-0",
    //             "data": {
    //               "isVisible": true
    //             }
    //           }
    //         ]
    //       }          
    // );

    // let json_string = json_object.to_string();

    // println!("{:#?}", json_string);

    // let json_object: Value = from_str("{\"data\":[{\"data\":{\"eventIndex\":13,\"firstEventId\":\"52b3a792-50a9-49cb-a58e-7f4607cfc19d\",\"firstEventTimestamp\":\"2023-03-10T15:06:18.428Z\",\"previousSessionId\":null,\"sessionId\":\"588410f6-c56b-4172-90e1-15c900e249ef\",\"sessionIndex\":1,\"storageMechanism\":\"LOCAL_STORAGE\",\"userId\":\"28e5a620-c844-4270-a076-2ea3c594240b\"},\"schema\":\"iglu:com.snowplowanalytics.snowplow/client_session/jsonschema/1-0-2\"},{\"data\":{\"build\":\"1\",\"version\":\"1.0\"},\"schema\":\"iglu:com.snowplowanalytics.mobile/application/jsonschema/1-0-0\"},{\"data\":{\"androidIdfa\":\"e905f781-8d2f-4ed1-aa94-4aac51940897\",\"availableStorage\":586629120,\"batteryLevel\":100,\"batteryState\":\"unplugged\",\"carrier\":\"T-Mobile\",\"deviceManufacturer\":\"Google\",\"deviceModel\":\"sdk_gphone64_arm64\",\"lowPowerMode\":false,\"networkType\":\"wifi\",\"osType\":\"android\",\"osVersion\":\"12\",\"physicalMemory\":2061529088,\"systemAvailableMemory\":740384768,\"totalStorage\":1945698304},\"schema\":\"iglu:com.snowplowanalytics.snowplow/mobile_context/jsonschema/1-0-2\"},{\"data\":{\"ck\":\"4d2f03dddf417990f520f09d79b11ab014c39dab\",\"clid\":\"2060753275.956347642.1551155978.949878434\",\"iid\":\"1643305591\"},\"schema\":\"iglu:com.conviva/clid_schema/jsonschema/1-0-1\"},{\"data\":{\"appLoad\":1678460733036,\"onCreate\":1678460732934,\"onResume\":1678460733143},\"schema\":\"iglu:com.conviva.mobile/android_app_loadtime/jsonschema/1-0-1\"},{\"data\":{\"eventIndex\":13,\"previousEventTimestamp\":\"2023-03-10T15:06:55.097Z\"},\"schema\":\"iglu:com.conviva/event_info/jsonschema/1-0-1\"},{\"data\":{\"data\":{\"boolean1\":\"true\",\"boolean2\":\"false\",\"double\":\"1234.1\",\"double1\":\"0.0\",\"int\":\"1234\",\"int1\":\"0\",\"test\":\"pass\"}},\"schema\":\"iglu:com.conviva/custom_tags/jsonschema/1-0-0\"},{\"data\":{\"activity\":\"selffinishingactivity.SelfFinishingActivity\",\"id\":\"046fe892-5463-434e-8c72-e7305869b8f4\",\"name\":\"selffinishingactivity.SelfFinishingActivity\",\"type\":\"selffinishingactivity.SelfFinishingActivity\"},\"schema\":\"iglu:com.snowplowanalytics.mobile/screen/jsonschema/1-0-0\"},{\"data\":{\"onCreate\":-871441071,\"onResume\":-871441052},\"schema\":\"iglu:com.conviva.mobile/android_screen_loadtime/jsonschema/1-0-0\"},{\"data\":{\"info\":\"{\\\"action\\\":\\\"android.intent.action.VIEW\\\"}\"},\"schema\":\"iglu:com.conviva.mobile/bundle_info/jsonschema/1-0-0\"},{\"data\":{\"isVisible\":true},\"schema\":\"iglu:com.snowplowanalytics.mobile/application_lifecycle/jsonschema/1-0-0\"}],\"schema\":\"iglu:com.snowplowanalytics.snowplow/contexts/jsonschema/1-0-1\"}").unwrap();
    // let json_object: Value = from_str("{\"data\":{\"click_count\":0},\"schema\":\"iglu:com.conviva/periodic_heartbeat/jsonschema/1-0-0\"}").unwrap();
    // println!("{:#?}", json_object);

    // let json_object = json!(
    //     {
    //         "schema":"iglu:com.conviva/periodic_heartbeat/jsonschema/1-0-0",
    //         "data":{
    //             "click_count":0
    //         }
    //     }
    // );

    // let json_string = json_object.to_string();
    // println!("{:#?}", json_string);

    // let json_object: Value = from_str("{\"data\":[{\"data\":{\"eventIndex\":13,\"firstEventId\":\"52b3a792-50a9-49cb-a58e-7f4607cfc19d\",\"firstEventTimestamp\":\"2023-03-10T15:06:18.428Z\",\"previousSessionId\":null,\"sessionId\":\"588410f6-c56b-4172-90e1-15c900e249ef\",\"sessionIndex\":1,\"storageMechanism\":\"LOCAL_STORAGE\",\"userId\":\"28e5a620-c844-4270-a076-2ea3c594240b\"},\"schema\":\"iglu:com.snowplowanalytics.snowplow/client_session/jsonschema/1-0-2\"},{\"data\":{\"build\":\"1\",\"version\":\"1.0\"},\"schema\":\"iglu:com.snowplowanalytics.mobile/application/jsonschema/1-0-0\"},{\"data\":{\"androidIdfa\":\"e905f781-8d2f-4ed1-aa94-4aac51940897\",\"availableStorage\":586629120,\"batteryLevel\":100,\"batteryState\":\"unplugged\",\"carrier\":\"T-Mobile\",\"deviceManufacturer\":\"Google\",\"deviceModel\":\"sdk_gphone64_arm64\",\"lowPowerMode\":false,\"networkType\":\"wifi\",\"osType\":\"android\",\"osVersion\":\"12\",\"physicalMemory\":2061529088,\"systemAvailableMemory\":740384768,\"totalStorage\":1945698304},\"schema\":\"iglu:com.snowplowanalytics.snowplow/mobile_context/jsonschema/1-0-2\"},{\"data\":{\"ck\":\"4d2f03dddf417990f520f09d79b11ab014c39dab\",\"clid\":\"2060753275.956347642.1551155978.949878434\",\"iid\":\"1643305591\"},\"schema\":\"iglu:com.conviva/clid_schema/jsonschema/1-0-1\"},{\"data\":{\"appLoad\":1678460733036,\"onCreate\":1678460732934,\"onResume\":1678460733143},\"schema\":\"iglu:com.conviva.mobile/android_app_loadtime/jsonschema/1-0-1\"},{\"data\":{\"eventIndex\":13,\"previousEventTimestamp\":\"2023-03-10T15:06:55.097Z\"},\"schema\":\"iglu:com.conviva/event_info/jsonschema/1-0-1\"},{\"data\":{\"data\":{\"boolean1\":\"true\",\"boolean2\":\"false\",\"double\":\"1234.1\",\"double1\":\"0.0\",\"int\":\"1234\",\"int1\":\"0\",\"test\":\"pass\"}},\"schema\":\"iglu:com.conviva/custom_tags/jsonschema/1-0-0\"},{\"data\":{\"activity\":\"selffinishingactivity.SelfFinishingActivity\",\"id\":\"046fe892-5463-434e-8c72-e7305869b8f4\",\"name\":\"selffinishingactivity.SelfFinishingActivity\",\"type\":\"selffinishingactivity.SelfFinishingActivity\"},\"schema\":\"iglu:com.snowplowanalytics.mobile/screen/jsonschema/1-0-0\"},{\"data\":{\"onCreate\":1678460771665,\"onResume\":1678460771665},\"schema\":\"iglu:com.conviva.mobile/android_screen_loadtime/jsonschema/1-0-0\"},{\"data\":{\"info\":\"{\\\"action\\\":\\\"android.intent.action.VIEW\\\"}\"},\"schema\":\"iglu:com.conviva.mobile/bundle_info/jsonschema/1-0-0\"},{\"data\":{\"isVisible\":true},\"schema\":\"iglu:com.snowplowanalytics.mobile/application_lifecycle/jsonschema/1-0-0\"}],\"schema\":\"iglu:com.snowplowanalytics.snowplow/contexts/jsonschema/1-0-1\"}").unwrap();
    // println!("{:#?}", serde_json::to_string_pretty(&json_object.to_string()));

    // let json_object = json!(
    //     {
    //         "data": [
    //           {
    //             "data": {
    //               "eventIndex": null,
    //               "firstEventId": "52b3a792-50a9-49cb-a58e-7f4607cfc19d",
    //               "firstEventTimestamp": "2023-03-10T15:06:18.428Z",
    //               "previousSessionId": null,
    //               "sessionId": "588410f6-c56b-4172-90e1-15c900e249ef",
    //               "sessionIndex": 1,
    //               "storageMechanism": "LOCAL_STORAGE",
    //               "userId": "28e5a620-c844-4270-a076-2ea3c594240b"
    //             },
    //             "schema": "iglu:com.snowplowanalytics.snowplow/client_session/jsonschema/1-0-2"
    //           }
    //         ],
    //         "schema": "iglu:com.snowplowanalytics.snowplow/contexts/jsonschema/1-0-1"
    //     }
    // );
    // println!("{:?}", json_object.to_string());

    // let val: Value = from_str("{\"clid\":\"1624290842.1731473025.1456792225.994695956\",\"iid\":1135079927,\"name\":\"c3.video.attempt\",\"sid\":-779950873,\"st\":22}").unwrap();
    // println!("{}", val);

    // from json to string
    // let json_object = json!(
    //     {
    //         "schema": "iglu:com.snowplowanalytics.snowplow/unstruct_event/jsonschema/1-0-0",
    //         "data": {
    //             "schema": "iglu:com.conviva/periodic_heartbeat/jsonschema/1-0-0",
    //             "data": {
    //                 "click_count": 0
    //             }
    //         }
    //     }
    // );
    // println!("{:?}", json_object.to_string());

    // For parsing the collector payload event from sumologic
    let val: Value = from_str("\"{\\\"schema\\\":\\\"iglu:com.snowplowanalytics.snowplow\\\\/payload_data\\\\/jsonschema\\\\/1-0-4\\\",\\\"data\\\":[{\\\"vp\\\":\\\"1179x2556\\\",\\\"eid\\\":\\\"FF9CBD88-425C-4AB5-B48C-6B701F9FCF6F\\\",\\\"p\\\":\\\"mob\\\",\\\"cx\\\":\\\"eyJzY2hlbWEiOiJpZ2x1OmNvbS5zbm93cGxvd2FuYWx5dGljcy5zbm93cGxvd1wvY29udGV4dHNcL2pzb25zY2hlbWFcLzEtMC0xIiwiZGF0YSI6W3sic2NoZW1hIjoiaWdsdTpjb20uc25vd3Bsb3dhbmFseXRpY3Muc25vd3Bsb3dcL21vYmlsZV9jb250ZXh0XC9qc29uc2NoZW1hXC8xLTAtMiIsImRhdGEiOnsicGh5c2ljYWxNZW1vcnkiOjU5MjU0NTM4MjQsImFwcGxlSWRmdiI6IjIwNTJDMDlGLUI2RjktNDg3NC04NjNBLUU2NjQ2ODEyQTY5RSIsImJhdHRlcnlMZXZlbCI6OTAsIm5ldHdvcmtUZWNobm9sb2d5IjoiQ1RSYWRpb0FjY2Vzc1RlY2hub2xvZ3lMVEUiLCJiYXR0ZXJ5U3RhdGUiOiJjaGFyZ2luZyIsIm9zVmVyc2lvbiI6IjE3LjAuMyIsImFwcEF2YWlsYWJsZU1lbW9yeSI6MzE4NDExNDU2MCwibmV0d29ya1R5cGUiOiJ3aWZpIiwib3NUeXBlIjoiaW9zIiwiZGV2aWNlTW9kZWwiOiJpUGhvbmUxNSwyIiwidG90YWxTdG9yYWdlIjoyNTU4NjY3ODU3OTIsImxvd1Bvd2VyTW9kZSI6ZmFsc2UsImRldmljZU1hbnVmYWN0dXJlciI6IkFwcGxlIEluYy4iLCJhdmFpbGFibGVTdG9yYWdlIjoxNjIxNDQwNTEyMCwiY2FycmllciI6Ii0tIn19LHsic2NoZW1hIjoiaWdsdTpjb20uc25vd3Bsb3dhbmFseXRpY3MubW9iaWxlXC9hcHBsaWNhdGlvblwvanNvbnNjaGVtYVwvMS0wLTAiLCJkYXRhIjp7ImJ1aWxkIjoiODgzMjEiLCJ2ZXJzaW9uIjoiNS43LjIifX0seyJzY2hlbWEiOiJpZ2x1OmNvbS5jb252aXZhXC9jdXN0b21fdGFnc1wvanNvbnNjaGVtYVwvMS0wLTAiLCJkYXRhIjp7ImRhdGEiOnsidHZfbW9kZV9zZXR0aW5ncyI6Im9mZiIsInVzZXJfbG9nZ2VkX2luIjoiZmFsc2UiLCJ1c2VyX2lkIjoiPG51bGw-IiwicHJvZHVjdF90eXBlIjoicHJlbWl1bSJ9fX0seyJzY2hlbWEiOiJpZ2x1OmNvbS5jb252aXZhXC9ldmVudF9pbmZvXC9qc29uc2NoZW1hXC8xLTAtMSIsImRhdGEiOnsicHJldmlvdXNFdmVudFRpbWVzdGFtcCI6IjIwMjMtMTEtMjRUMDQ6NTc6NDcuODc5WiIsImV2ZW50SW5kZXgiOjE1fX0seyJzY2hlbWEiOiJpZ2x1OmNvbS5zbm93cGxvd2FuYWx5dGljcy5zbm93cGxvd1wvY2xpZW50X3Nlc3Npb25cL2pzb25zY2hlbWFcLzEtMC0yIiwiZGF0YSI6eyJ1c2VySWQiOiI0OGRmMmM0My00YjUyLTQ5ZjQtODJmMS1iNzNhNDNmMmM2NGEiLCJzdG9yYWdlTWVjaGFuaXNtIjoiTE9DQUxfU1RPUkFHRSIsInNlc3Npb25JbmRleCI6MTAsInByZXZpb3VzU2Vzc2lvbklkIjoiZTJmY2ZkNzUtZmIwNS00OTY3LWIxZTYtODYyYTU5ZWJiYjVjIiwic2Vzc2lvbklkIjoiMjgyNTY0ODMtYTFmZS00YzcxLTkwMjAtYTQzYjU2ZjhmYzFlIiwiZXZlbnRJbmRleCI6MTUsImZpcnN0RXZlbnRJZCI6IjgxRDUxREUxLTZBMjUtNDNFNC04RTlELUEzMkQzM0I3NTU2NiIsImZpcnN0RXZlbnRUaW1lc3RhbXAiOiIyMDIzLTExLTI0VDA0OjUzOjIxLjM2MloifX0seyJzY2hlbWEiOiJpZ2x1OmNvbS5jb252aXZhXC9jbGlkX3NjaGVtYVwvanNvbnNjaGVtYVwvMS0wLTIiLCJkYXRhIjp7ImV2ZW50SW5kZXgiOjE1LCJjbGlkIjoiMjM1ODQyNzE3Ni4zOTkyOTg4MjQ4LjI5NDg0MTM2NzQuMTIzMjU5ODQxOCIsImlpZCI6IjEwMTc3MDg0NzIiLCJjayI6Ijk3ZDdlNmQ0YTU0YTZmMWMwYjdkODUxMjc5Nzk1ZTE0MTc5MTRjNWYifX0seyJzY2hlbWEiOiJpZ2x1OmNvbS5jb252aXZhLm1vYmlsZVwvaW9zX2FwcF9sb2FkdGltZVwvanNvbnNjaGVtYVwvMS0wLTAiLCJkYXRhIjp7ImFwcEFjdGl2ZSI6MTcwMDgwMTYwMTMzMSwidXNlckludGVyYWN0YWJsZSI6MTcwMDgwMTYwMTM1NSwibG9hZGVkVG9NZW1vcnkiOjE3MDA4MDE2MDA5MzEsImZpbmlzaGVkTGF1bmNoIjoxNzAwODAxNjAxMzI0fX0seyJzY2hlbWEiOiJpZ2x1OmNvbS5jb252aXZhXC9kZXZpY2VfcGVyZm9ybWFuY2VfY29udGV4dFwvanNvbnNjaGVtYVwvMS0wLTAiLCJkYXRhIjp7ImNwdVVzYWdlIjo1fX0seyJzY2hlbWEiOiJpZ2x1OmNvbS5zbm93cGxvd2FuYWx5dGljcy5tb2JpbGVcL3NjcmVlblwvanNvbnNjaGVtYVwvMS0wLTAiLCJkYXRhIjp7InRvcFZpZXdDb250cm9sbGVyIjoiVEZQcmVzZW50YXRpb24uQXBwQ29vcmRpbmF0b3IiLCJpZCI6Ijc3Qzg2MkZFLUNFOTUtNDQwQi1BM0E0LTg5QTM4RUM4OTkwNyIsIm5hbWUiOiJURlByZXNlbnRhdGlvbi5BcHBDb29yZGluYXRvciIsInR5cGUiOiJEZWZhdWx0Iiwidmlld0NvbnRyb2xsZXIiOiJURlByZXNlbnRhdGlvbi5BcHBDb29yZGluYXRvciJ9fSx7InNjaGVtYSI6ImlnbHU6Y29tLmNvbnZpdmEubW9iaWxlXC9pb3Nfc2NyZWVuX2xvYWR0aW1lXC9qc29uc2NoZW1hXC8xLTAtMCIsImRhdGEiOnsic2NyZWVuTG9hZGVkVG9NZW1vcnkiOjE3MDA4MDE2MDExNjEsInNjcmVlbkludGVyYWN0YWJsZSI6MTcwMDgwMTYwMTM2Mn19LHsic2NoZW1hIjoiaWdsdTpjb20uc25vd3Bsb3dhbmFseXRpY3MubW9iaWxlXC9hcHBsaWNhdGlvbl9saWZlY3ljbGVcL2pzb25zY2hlbWFcLzEtMC0wIiwiZGF0YSI6eyJpbmRleCI6MywiaXNWaXNpYmxlIjpmYWxzZX19XX0\\\",\\\"tz\\\":\\\"Asia\\\\/Bangkok\\\",\\\"stm\\\":\\\"1700801868882\\\",\\\"dtm\\\":\\\"1700801867899\\\",\\\"tv\\\":\\\"ios-0.2.16\\\",\\\"tna\\\":\\\"CAT\\\",\\\"ue_px\\\":\\\"eyJzY2hlbWEiOiJpZ2x1OmNvbS5zbm93cGxvd2FuYWx5dGljcy5zbm93cGxvd1wvdW5zdHJ1Y3RfZXZlbnRcL2pzb25zY2hlbWFcLzEtMC0wIiwiZGF0YSI6eyJzY2hlbWEiOiJpZ2x1OmNvbS5zbm93cGxvd2FuYWx5dGljcy5zbm93cGxvd1wvYXBwbGljYXRpb25fYmFja2dyb3VuZFwvanNvbnNjaGVtYVwvMS0wLTAiLCJkYXRhIjp7ImJhY2tncm91bmRJbmRleCI6M319fQ\\\",\\\"e\\\":\\\"ue\\\",\\\"lang\\\":\\\"nl-NL\\\",\\\"aid\\\":\\\"IOS\\\",\\\"res\\\":\\\"1179x2556\\\"}]}\"").unwrap();
    println!("first json {:#?}", val);

    let second: Value = from_str(&val.to_string()).unwrap();
    println!("second json {:#?}", second);
    // let second = format!("{:?}", second);

    println!("===============================================================");

    // let third: Value = from_str("{\"schema\":\"iglu:com.snowplowanalytics.snowplow\\/payload_data\\/jsonschema\\/1-0-4\",\"data\":[{\"vp\":\"1179x2556\",\"eid\":\"FF9CBD88-425C-4AB5-B48C-6B701F9FCF6F\",\"p\":\"mob\",\"cx\":\"eyJzY2hlbWEiOiJpZ2x1OmNvbS5zbm93cGxvd2FuYWx5dGljcy5zbm93cGxvd1wvY29udGV4dHNcL2pzb25zY2hlbWFcLzEtMC0xIiwiZGF0YSI6W3sic2NoZW1hIjoiaWdsdTpjb20uc25vd3Bsb3dhbmFseXRpY3Muc25vd3Bsb3dcL21vYmlsZV9jb250ZXh0XC9qc29uc2NoZW1hXC8xLTAtMiIsImRhdGEiOnsicGh5c2ljYWxNZW1vcnkiOjU5MjU0NTM4MjQsImFwcGxlSWRmdiI6IjIwNTJDMDlGLUI2RjktNDg3NC04NjNBLUU2NjQ2ODEyQTY5RSIsImJhdHRlcnlMZXZlbCI6OTAsIm5ldHdvcmtUZWNobm9sb2d5IjoiQ1RSYWRpb0FjY2Vzc1RlY2hub2xvZ3lMVEUiLCJiYXR0ZXJ5U3RhdGUiOiJjaGFyZ2luZyIsIm9zVmVyc2lvbiI6IjE3LjAuMyIsImFwcEF2YWlsYWJsZU1lbW9yeSI6MzE4NDExNDU2MCwibmV0d29ya1R5cGUiOiJ3aWZpIiwib3NUeXBlIjoiaW9zIiwiZGV2aWNlTW9kZWwiOiJpUGhvbmUxNSwyIiwidG90YWxTdG9yYWdlIjoyNTU4NjY3ODU3OTIsImxvd1Bvd2VyTW9kZSI6ZmFsc2UsImRldmljZU1hbnVmYWN0dXJlciI6IkFwcGxlIEluYy4iLCJhdmFpbGFibGVTdG9yYWdlIjoxNjIxNDQwNTEyMCwiY2FycmllciI6Ii0tIn19LHsic2NoZW1hIjoiaWdsdTpjb20uc25vd3Bsb3dhbmFseXRpY3MubW9iaWxlXC9hcHBsaWNhdGlvblwvanNvbnNjaGVtYVwvMS0wLTAiLCJkYXRhIjp7ImJ1aWxkIjoiODgzMjEiLCJ2ZXJzaW9uIjoiNS43LjIifX0seyJzY2hlbWEiOiJpZ2x1OmNvbS5jb252aXZhXC9jdXN0b21fdGFnc1wvanNvbnNjaGVtYVwvMS0wLTAiLCJkYXRhIjp7ImRhdGEiOnsidHZfbW9kZV9zZXR0aW5ncyI6Im9mZiIsInVzZXJfbG9nZ2VkX2luIjoiZmFsc2UiLCJ1c2VyX2lkIjoiPG51bGw-IiwicHJvZHVjdF90eXBlIjoicHJlbWl1bSJ9fX0seyJzY2hlbWEiOiJpZ2x1OmNvbS5jb252aXZhXC9ldmVudF9pbmZvXC9qc29uc2NoZW1hXC8xLTAtMSIsImRhdGEiOnsicHJldmlvdXNFdmVudFRpbWVzdGFtcCI6IjIwMjMtMTEtMjRUMDQ6NTc6NDcuODc5WiIsImV2ZW50SW5kZXgiOjE1fX0seyJzY2hlbWEiOiJpZ2x1OmNvbS5zbm93cGxvd2FuYWx5dGljcy5zbm93cGxvd1wvY2xpZW50X3Nlc3Npb25cL2pzb25zY2hlbWFcLzEtMC0yIiwiZGF0YSI6eyJ1c2VySWQiOiI0OGRmMmM0My00YjUyLTQ5ZjQtODJmMS1iNzNhNDNmMmM2NGEiLCJzdG9yYWdlTWVjaGFuaXNtIjoiTE9DQUxfU1RPUkFHRSIsInNlc3Npb25JbmRleCI6MTAsInByZXZpb3VzU2Vzc2lvbklkIjoiZTJmY2ZkNzUtZmIwNS00OTY3LWIxZTYtODYyYTU5ZWJiYjVjIiwic2Vzc2lvbklkIjoiMjgyNTY0ODMtYTFmZS00YzcxLTkwMjAtYTQzYjU2ZjhmYzFlIiwiZXZlbnRJbmRleCI6MTUsImZpcnN0RXZlbnRJZCI6IjgxRDUxREUxLTZBMjUtNDNFNC04RTlELUEzMkQzM0I3NTU2NiIsImZpcnN0RXZlbnRUaW1lc3RhbXAiOiIyMDIzLTExLTI0VDA0OjUzOjIxLjM2MloifX0seyJzY2hlbWEiOiJpZ2x1OmNvbS5jb252aXZhXC9jbGlkX3NjaGVtYVwvanNvbnNjaGVtYVwvMS0wLTIiLCJkYXRhIjp7ImV2ZW50SW5kZXgiOjE1LCJjbGlkIjoiMjM1ODQyNzE3Ni4zOTkyOTg4MjQ4LjI5NDg0MTM2NzQuMTIzMjU5ODQxOCIsImlpZCI6IjEwMTc3MDg0NzIiLCJjayI6Ijk3ZDdlNmQ0YTU0YTZmMWMwYjdkODUxMjc5Nzk1ZTE0MTc5MTRjNWYifX0seyJzY2hlbWEiOiJpZ2x1OmNvbS5jb252aXZhLm1vYmlsZVwvaW9zX2FwcF9sb2FkdGltZVwvanNvbnNjaGVtYVwvMS0wLTAiLCJkYXRhIjp7ImFwcEFjdGl2ZSI6MTcwMDgwMTYwMTMzMSwidXNlckludGVyYWN0YWJsZSI6MTcwMDgwMTYwMTM1NSwibG9hZGVkVG9NZW1vcnkiOjE3MDA4MDE2MDA5MzEsImZpbmlzaGVkTGF1bmNoIjoxNzAwODAxNjAxMzI0fX0seyJzY2hlbWEiOiJpZ2x1OmNvbS5jb252aXZhXC9kZXZpY2VfcGVyZm9ybWFuY2VfY29udGV4dFwvanNvbnNjaGVtYVwvMS0wLTAiLCJkYXRhIjp7ImNwdVVzYWdlIjo1fX0seyJzY2hlbWEiOiJpZ2x1OmNvbS5zbm93cGxvd2FuYWx5dGljcy5tb2JpbGVcL3NjcmVlblwvanNvbnNjaGVtYVwvMS0wLTAiLCJkYXRhIjp7InRvcFZpZXdDb250cm9sbGVyIjoiVEZQcmVzZW50YXRpb24uQXBwQ29vcmRpbmF0b3IiLCJpZCI6Ijc3Qzg2MkZFLUNFOTUtNDQwQi1BM0E0LTg5QTM4RUM4OTkwNyIsIm5hbWUiOiJURlByZXNlbnRhdGlvbi5BcHBDb29yZGluYXRvciIsInR5cGUiOiJEZWZhdWx0Iiwidmlld0NvbnRyb2xsZXIiOiJURlByZXNlbnRhdGlvbi5BcHBDb29yZGluYXRvciJ9fSx7InNjaGVtYSI6ImlnbHU6Y29tLmNvbnZpdmEubW9iaWxlXC9pb3Nfc2NyZWVuX2xvYWR0aW1lXC9qc29uc2NoZW1hXC8xLTAtMCIsImRhdGEiOnsic2NyZWVuTG9hZGVkVG9NZW1vcnkiOjE3MDA4MDE2MDExNjEsInNjcmVlbkludGVyYWN0YWJsZSI6MTcwMDgwMTYwMTM2Mn19LHsic2NoZW1hIjoiaWdsdTpjb20uc25vd3Bsb3dhbmFseXRpY3MubW9iaWxlXC9hcHBsaWNhdGlvbl9saWZlY3ljbGVcL2pzb25zY2hlbWFcLzEtMC0wIiwiZGF0YSI6eyJpbmRleCI6MywiaXNWaXNpYmxlIjpmYWxzZX19XX0\",\"tz\":\"Asia\\/Bangkok\",\"stm\":\"1700801868882\",\"dtm\":\"1700801867899\",\"tv\":\"ios-0.2.16\",\"tna\":\"CAT\",\"ue_px\":\"eyJzY2hlbWEiOiJpZ2x1OmNvbS5zbm93cGxvd2FuYWx5dGljcy5zbm93cGxvd1wvdW5zdHJ1Y3RfZXZlbnRcL2pzb25zY2hlbWFcLzEtMC0wIiwiZGF0YSI6eyJzY2hlbWEiOiJpZ2x1OmNvbS5zbm93cGxvd2FuYWx5dGljcy5zbm93cGxvd1wvYXBwbGljYXRpb25fYmFja2dyb3VuZFwvanNvbnNjaGVtYVwvMS0wLTAiLCJkYXRhIjp7ImJhY2tncm91bmRJbmRleCI6M319fQ\",\"e\":\"ue\",\"lang\":\"nl-NL\",\"aid\":\"IOS\",\"res\":\"1179x2556\"}]}").unwrap();
    // let third:Value = from_str("{\"schema\":\"iglu:com.snowplowanalytics.snowplow\\/payload_data\\/jsonschema\\/1-0-4\",\"data\":[{\"vp\":\"1179x2556\",\"eid\":\"FF9CBD88-425C-4AB5-B48C-6B701F9FCF6F\",\"p\":\"mob\",\"cx\":\"eyJzY2hlbWEiOiJpZ2x1OmNvbS5zbm93cGxvd2FuYWx5dGljcy5zbm93cGxvd1wvY29udGV4dHNcL2pzb25zY2hlbWFcLzEtMC0xIiwiZGF0YSI6W3sic2NoZW1hIjoiaWdsdTpjb20uc25vd3Bsb3dhbmFseXRpY3Muc25vd3Bsb3dcL21vYmlsZV9jb250ZXh0XC9qc29uc2NoZW1hXC8xLTAtMiIsImRhdGEiOnsicGh5c2ljYWxNZW1vcnkiOjU5MjU0NTM4MjQsImFwcGxlSWRmdiI6IjIwNTJDMDlGLUI2RjktNDg3NC04NjNBLUU2NjQ2ODEyQTY5RSIsImJhdHRlcnlMZXZlbCI6OTAsIm5ldHdvcmtUZWNobm9sb2d5IjoiQ1RSYWRpb0FjY2Vzc1RlY2hub2xvZ3lMVEUiLCJiYXR0ZXJ5U3RhdGUiOiJjaGFyZ2luZyIsIm9zVmVyc2lvbiI6IjE3LjAuMyIsImFwcEF2YWlsYWJsZU1lbW9yeSI6MzE4NDExNDU2MCwibmV0d29ya1R5cGUiOiJ3aWZpIiwib3NUeXBlIjoiaW9zIiwiZGV2aWNlTW9kZWwiOiJpUGhvbmUxNSwyIiwidG90YWxTdG9yYWdlIjoyNTU4NjY3ODU3OTIsImxvd1Bvd2VyTW9kZSI6ZmFsc2UsImRldmljZU1hbnVmYWN0dXJlciI6IkFwcGxlIEluYy4iLCJhdmFpbGFibGVTdG9yYWdlIjoxNjIxNDQwNTEyMCwiY2FycmllciI6Ii0tIn19LHsic2NoZW1hIjoiaWdsdTpjb20uc25vd3Bsb3dhbmFseXRpY3MubW9iaWxlXC9hcHBsaWNhdGlvblwvanNvbnNjaGVtYVwvMS0wLTAiLCJkYXRhIjp7ImJ1aWxkIjoiODgzMjEiLCJ2ZXJzaW9uIjoiNS43LjIifX0seyJzY2hlbWEiOiJpZ2x1OmNvbS5jb252aXZhXC9jdXN0b21fdGFnc1wvanNvbnNjaGVtYVwvMS0wLTAiLCJkYXRhIjp7ImRhdGEiOnsidHZfbW9kZV9zZXR0aW5ncyI6Im9mZiIsInVzZXJfbG9nZ2VkX2luIjoiZmFsc2UiLCJ1c2VyX2lkIjoiPG51bGw-IiwicHJvZHVjdF90eXBlIjoicHJlbWl1bSJ9fX0seyJzY2hlbWEiOiJpZ2x1OmNvbS5jb252aXZhXC9ldmVudF9pbmZvXC9qc29uc2NoZW1hXC8xLTAtMSIsImRhdGEiOnsicHJldmlvdXNFdmVudFRpbWVzdGFtcCI6IjIwMjMtMTEtMjRUMDQ6NTc6NDcuODc5WiIsImV2ZW50SW5kZXgiOjE1fX0seyJzY2hlbWEiOiJpZ2x1OmNvbS5zbm93cGxvd2FuYWx5dGljcy5zbm93cGxvd1wvY2xpZW50X3Nlc3Npb25cL2pzb25zY2hlbWFcLzEtMC0yIiwiZGF0YSI6eyJ1c2VySWQiOiI0OGRmMmM0My00YjUyLTQ5ZjQtODJmMS1iNzNhNDNmMmM2NGEiLCJzdG9yYWdlTWVjaGFuaXNtIjoiTE9DQUxfU1RPUkFHRSIsInNlc3Npb25JbmRleCI6MTAsInByZXZpb3VzU2Vzc2lvbklkIjoiZTJmY2ZkNzUtZmIwNS00OTY3LWIxZTYtODYyYTU5ZWJiYjVjIiwic2Vzc2lvbklkIjoiMjgyNTY0ODMtYTFmZS00YzcxLTkwMjAtYTQzYjU2ZjhmYzFlIiwiZXZlbnRJbmRleCI6MTUsImZpcnN0RXZlbnRJZCI6IjgxRDUxREUxLTZBMjUtNDNFNC04RTlELUEzMkQzM0I3NTU2NiIsImZpcnN0RXZlbnRUaW1lc3RhbXAiOiIyMDIzLTExLTI0VDA0OjUzOjIxLjM2MloifX0seyJzY2hlbWEiOiJpZ2x1OmNvbS5jb252aXZhXC9jbGlkX3NjaGVtYVwvanNvbnNjaGVtYVwvMS0wLTIiLCJkYXRhIjp7ImV2ZW50SW5kZXgiOjE1LCJjbGlkIjoiMjM1ODQyNzE3Ni4zOTkyOTg4MjQ4LjI5NDg0MTM2NzQuMTIzMjU5ODQxOCIsImlpZCI6IjEwMTc3MDg0NzIiLCJjayI6Ijk3ZDdlNmQ0YTU0YTZmMWMwYjdkODUxMjc5Nzk1ZTE0MTc5MTRjNWYifX0seyJzY2hlbWEiOiJpZ2x1OmNvbS5jb252aXZhLm1vYmlsZVwvaW9zX2FwcF9sb2FkdGltZVwvanNvbnNjaGVtYVwvMS0wLTAiLCJkYXRhIjp7ImFwcEFjdGl2ZSI6MTcwMDgwMTYwMTMzMSwidXNlckludGVyYWN0YWJsZSI6MTcwMDgwMTYwMTM1NSwibG9hZGVkVG9NZW1vcnkiOjE3MDA4MDE2MDA5MzEsImZpbmlzaGVkTGF1bmNoIjoxNzAwODAxNjAxMzI0fX0seyJzY2hlbWEiOiJpZ2x1OmNvbS5jb252aXZhXC9kZXZpY2VfcGVyZm9ybWFuY2VfY29udGV4dFwvanNvbnNjaGVtYVwvMS0wLTAiLCJkYXRhIjp7ImNwdVVzYWdlIjo1fX0seyJzY2hlbWEiOiJpZ2x1OmNvbS5zbm93cGxvd2FuYWx5dGljcy5tb2JpbGVcL3NjcmVlblwvanNvbnNjaGVtYVwvMS0wLTAiLCJkYXRhIjp7InRvcFZpZXdDb250cm9sbGVyIjoiVEZQcmVzZW50YXRpb24uQXBwQ29vcmRpbmF0b3IiLCJpZCI6Ijc3Qzg2MkZFLUNFOTUtNDQwQi1BM0E0LTg5QTM4RUM4OTkwNyIsIm5hbWUiOiJURlByZXNlbnRhdGlvbi5BcHBDb29yZGluYXRvciIsInR5cGUiOiJEZWZhdWx0Iiwidmlld0NvbnRyb2xsZXIiOiJURlByZXNlbnRhdGlvbi5BcHBDb29yZGluYXRvciJ9fSx7InNjaGVtYSI6ImlnbHU6Y29tLmNvbnZpdmEubW9iaWxlXC9pb3Nfc2NyZWVuX2xvYWR0aW1lXC9qc29uc2NoZW1hXC8xLTAtMCIsImRhdGEiOnsic2NyZWVuTG9hZGVkVG9NZW1vcnkiOjE3MDA4MDE2MDExNjEsInNjcmVlbkludGVyYWN0YWJsZSI6MTcwMDgwMTYwMTM2Mn19LHsic2NoZW1hIjoiaWdsdTpjb20uc25vd3Bsb3dhbmFseXRpY3MubW9iaWxlXC9hcHBsaWNhdGlvbl9saWZlY3ljbGVcL2pzb25zY2hlbWFcLzEtMC0wIiwiZGF0YSI6eyJpbmRleCI6MywiaXNWaXNpYmxlIjpmYWxzZX19XX0\",\"tz\":\"Asia\\/Bangkok\",\"stm\":\"1700801868882\",\"dtm\":\"1700801867899\",\"tv\":\"ios-0.2.16\",\"tna\":\"CAT\",\"ue_px\":\"eyJzY2hlbWEiOiJpZ2x1OmNvbS5zbm93cGxvd2FuYWx5dGljcy5zbm93cGxvd1wvdW5zdHJ1Y3RfZXZlbnRcL2pzb25zY2hlbWFcLzEtMC0wIiwiZGF0YSI6eyJzY2hlbWEiOiJpZ2x1OmNvbS5zbm93cGxvd2FuYWx5dGljcy5zbm93cGxvd1wvYXBwbGljYXRpb25fYmFja2dyb3VuZFwvanNvbnNjaGVtYVwvMS0wLTAiLCJkYXRhIjp7ImJhY2tncm91bmRJbmRleCI6M319fQ\",\"e\":\"ue\",\"lang\":\"nl-NL\",\"aid\":\"IOS\",\"res\":\"1179x2556\"}]}").unwrap();
    let third: Value = from_str(&serde_json::to_string_pretty(&second.to_string()).unwrap()).unwrap();
    println!("third json {:#?}", third);
}