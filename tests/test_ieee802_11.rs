use packet::ieee802_11::*;
use packet::MacAddress;

struct TestItem<'a> {
  bytes: &'a [u8],
  subtype: FrameSubtype,
  ds_status: DSStatus,

  more_fragments: bool,
  retry: bool,
  pwr_mgt: bool,
  more_data: bool,
  protected: bool,
  order: bool,

  duration_id: DurationID,

  receiver_address: MacAddress,
  transmitter_address: Option<MacAddress>,

  destination_address: Option<MacAddress>,
  source_address: Option<MacAddress>,

  bssid_address: Option<MacAddress>,
  station_address: Option<MacAddress>,

  fragment_number: Option<u8>,
  sequence_number: Option<u16>,

  ssid: Option<Vec<u8>>,
}

fn check<T: std::fmt::Debug + std::cmp::PartialEq>(original: Option<T>, test: Option<T>, s: &str) {
  assert_eq!(original.is_some(), test.is_some(), "{}.is_some()", s);
  if test.is_some() {
    assert_eq!(original.unwrap(), test.unwrap(), "{}", s);
  }
}

#[allow(clippy::cyclomatic_complexity)]
fn test_test_item(test_item: TestItem) {
  let frame = IEEE802_11Frame::new(&test_item.bytes);

  assert_eq!(frame.version(), FrameVersion::Standard);

  assert_eq!(frame.subtype(), test_item.subtype, "subtype");

  assert_eq!(frame.ds_status(), test_item.ds_status, "ds_status");

  assert_eq!(
    frame.more_fragments(),
    test_item.more_fragments,
    "more_fragments"
  );
  assert_eq!(frame.retry(), test_item.retry, "retry");
  assert_eq!(frame.pwr_mgt(), test_item.pwr_mgt, "pwr_mgt");
  assert_eq!(frame.more_data(), test_item.more_data, "more_data");
  assert_eq!(frame.protected(), test_item.protected, "protected");
  assert_eq!(frame.order(), test_item.order, "order");

  assert_eq!(frame.duration_or_id(), test_item.duration_id, "duration_id");

  assert_eq!(
    frame.receiver_address(),
    test_item.receiver_address,
    "receiver_address"
  );

  let transmitter_address;
  let destination_address;
  let source_address;
  let bssid_address;
  let station_address;
  let fragment_number;
  let sequence_number;

  match &frame.next_layer() {
    IEEE802_11FrameLayer::Management(layer) => {
      transmitter_address = layer.transmitter_address();
      destination_address = layer.destination_address();
      source_address = layer.source_address();
      bssid_address = layer.bssid_address();
      station_address = layer.station_address();

      fragment_number = Some(layer.fragment_number());
      sequence_number = Some(layer.sequence_number());

      let ssid = {
        if let Some(layer) = layer.next_layer() {
          match layer {
            ManagementFrameLayer::Beacon(beacon_frame) => beacon_frame.ssid(),
            ManagementFrameLayer::ProbeRequest(probe_request_frame) => probe_request_frame.ssid(),
            ManagementFrameLayer::ProbeResponse(probe_response_frame) => {
              probe_response_frame.ssid()
            }
          }
        } else {
          None
        }
      };

      check(ssid, test_item.ssid, "ssid");
    }
    IEEE802_11FrameLayer::Control(layer) => {
      transmitter_address = layer.transmitter_address();
      destination_address = layer.destination_address();
      source_address = layer.source_address();
      bssid_address = layer.bssid_address();
      station_address = layer.station_address();
      fragment_number = None;
      sequence_number = None;
    }
    IEEE802_11FrameLayer::Data(layer) => {
      transmitter_address = layer.transmitter_address();
      destination_address = layer.destination_address();
      source_address = layer.source_address();
      bssid_address = layer.bssid_address();
      station_address = layer.station_address();
      fragment_number = Some(layer.fragment_number());
      sequence_number = Some(layer.sequence_number());
    }
  }

  check(
    transmitter_address,
    test_item.transmitter_address,
    "transmitter_address",
  );

  check(
    destination_address,
    test_item.destination_address,
    "destination_address",
  );

  check(source_address, test_item.source_address, "source_address");

  check(bssid_address, test_item.bssid_address, "bssid_address");

  check(
    station_address,
    test_item.station_address,
    "station_address",
  );

  check(
    fragment_number,
    test_item.fragment_number,
    "fragment_number",
  );

  check(
    sequence_number,
    test_item.sequence_number,
    "sequence_number",
  );
}

// Management
include!("./ieee802_11_packets/beacon.rs");
include!("./ieee802_11_packets/authentication.rs");
include!("./ieee802_11_packets/probe_request.rs");
include!("./ieee802_11_packets/probe_response.rs");

// Control
include!("./ieee802_11_packets/power_save_poll.rs");
include!("./ieee802_11_packets/ack.rs");
include!("./ieee802_11_packets/cts.rs");
include!("./ieee802_11_packets/rts.rs");
include!("./ieee802_11_packets/block_ack.rs");
include!("./ieee802_11_packets/cf_end.rs");
include!("./ieee802_11_packets/block_ack_request.rs");

// Data
include!("./ieee802_11_packets/null_data.rs");
