use ieee80211::*;

const BEACON_PACKET: [u8; 110] = [
    0x80, 0x00, 0x00, 0x70, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x01, 0xe3, 0x41, 0xbd, 0x6e,
    0x00, 0x01, 0xe3, 0x41, 0xbd, 0x6e, 0x10, 0xf0, 0x84, 0x21, 0x1a, 0x69, 0x02, 0x00, 0x00, 0x00,
    0x64, 0x00, 0x11, 0x04, 0x00, 0x09, 0x6d, 0x61, 0x72, 0x74, 0x69, 0x6e, 0x65, 0x74, 0x33, 0x01,
    0x08, 0x82, 0x84, 0x8b, 0x96, 0x24, 0x30, 0x48, 0x6c, 0x03, 0x01, 0x0b, 0x05, 0x04, 0x00, 0x01,
    0x00, 0x00, 0x2a, 0x01, 0x04, 0x2f, 0x01, 0x04, 0x32, 0x04, 0x0c, 0x12, 0x18, 0x60, 0xdd, 0x06,
    0x00, 0x10, 0x18, 0x01, 0x01, 0x00, 0xdd, 0x16, 0x00, 0x50, 0xf2, 0x01, 0x01, 0x00, 0x00, 0x50,
    0xf2, 0x02, 0x01, 0x00, 0x00, 0x50, 0xf2, 0x02, 0x01, 0x00, 0x00, 0x50, 0xf2, 0x02,
];

#[test]
fn test_example() {
    let frame = Frame::new(&BEACON_PACKET[..]);

    let receiver_address = frame.receiver_address();
    println!("receiver_address: {}", receiver_address);
    // receiver_address: ff-ff-ff-ff-ff-ff

    let layer = frame.next_layer().unwrap();
    let transmitter_address = match layer {
        FrameLayer::Management(ref management_frame) => management_frame.transmitter_address(),
        FrameLayer::Control(ref control_frame) => control_frame.transmitter_address(),
        FrameLayer::Data(ref data_frame) => data_frame.transmitter_address(),
    };

    if let Some(transmitter_address) = transmitter_address {
        println!("transmitter_address: {}", transmitter_address);
        // transmitter_address: 00-01-e3-41-bd-6e
    }

    if let FrameLayer::Management(ref management_frame) = layer {
        let management_frame_layer = management_frame.next_layer().unwrap();
        if let ManagementFrameLayer::Beacon(ref beacon_frame) = management_frame_layer {
            let ssid = String::from_utf8(beacon_frame.ssid().unwrap()).unwrap();
            println!("ssid: {}", ssid);
            // ssid: martinet3

            println!(
                "channel: {}",
                beacon_frame.tagged_parameters().unwrap().channel().unwrap()
            );
            // channel: 11

            println!("privacy: {}", beacon_frame.capabilities_info().privacy);
            // privacy: true
        }
    }
}
