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
}

#[allow(clippy::cyclomatic_complexity)]
fn test_test_item(test_item: TestItem) {
  let frame = IEEE802_11Frame::new(&test_item.bytes);

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

  assert_eq!(
    frame.transmitter_address().is_some(),
    test_item.transmitter_address.is_some(),
    "transmitter_address.is_some()"
  );
  if test_item.transmitter_address.is_some() {
    assert_eq!(
      frame.transmitter_address().unwrap(),
      test_item.transmitter_address.unwrap(),
      "transmitter_address"
    );
  }

  assert_eq!(
    frame.bssid_address().is_some(),
    test_item.bssid_address.is_some(),
    "bssid_address.is_some()"
  );
  if test_item.bssid_address.is_some() {
    assert_eq!(
      frame.bssid_address().unwrap(),
      test_item.bssid_address.unwrap(),
      "bssid_address"
    );
  }

  assert_eq!(
    frame.station_address().is_some(),
    test_item.station_address.is_some(),
    "station_address.is_some()"
  );
  if test_item.station_address.is_some() {
    assert_eq!(
      frame.station_address().unwrap(),
      test_item.station_address.unwrap(),
      "station_address"
    );
  }

  assert_eq!(
    frame.source_address().is_some(),
    test_item.source_address.is_some(),
    "source_address.is_some()"
  );
  if test_item.source_address.is_some() {
    assert_eq!(
      frame.source_address().unwrap(),
      test_item.source_address.unwrap(),
      "source_address"
    );
  }

  assert_eq!(
    frame.destination_address().is_some(),
    test_item.destination_address.is_some(),
    "destination_address.is_some()"
  );
  if test_item.destination_address.is_some() {
    assert_eq!(
      frame.destination_address().unwrap(),
      test_item.destination_address.unwrap(),
      "destination_address"
    );
  }

  // TODO
  assert_eq!(
    frame.fragment_number().is_some(),
    test_item.fragment_number.is_some(),
    "fragment_number.is_some()"
  );
  if test_item.fragment_number.is_some() {
    assert_eq!(
      frame.fragment_number().unwrap(),
      test_item.fragment_number.unwrap(),
      "fragment_number"
    );
  }

  assert_eq!(
    frame.sequence_number().is_some(),
    test_item.sequence_number.is_some(),
    "sequence_number.is_some()"
  );
  if test_item.sequence_number.is_some() {
    assert_eq!(
      frame.sequence_number().unwrap(),
      test_item.sequence_number.unwrap(),
      "sequence_number"
    );
  }
}

include!("./ieee802_11_packets/beacon.rs");
include!("./ieee802_11_packets/authentication.rs");
include!("./ieee802_11_packets/power_save_poll.rs");
include!("./ieee802_11_packets/null_data.rs");
include!("./ieee802_11_packets/probe_request.rs");
include!("./ieee802_11_packets/probe_response.rs");
