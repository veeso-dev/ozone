use std::io::Cursor;

use ozone_ws::clamav::{ClamAvClient, Scan};

#[actix_web::test]
async fn should_scan_file() {
    let mut client = ClamAvClient::init("127.0.0.1:3310").await.unwrap();

    let stream = vec![
        0xca, 0xfe, 0xba, 0xbe, 0xca, 0xfe, 0xba, 0xbe, 0xca, 0xfe, 0xba, 0xbe,
    ];

    let reader = Cursor::new(stream);

    let res = client.scan(reader, 12).await.expect("failed to send");

    assert_eq!(res, Scan::Safe);
}
