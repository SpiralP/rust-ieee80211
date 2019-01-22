#![allow(dead_code)]

use criterion::*;

include!("../tests/test_ieee802_11.rs");

fn criterion_benchmark(c: &mut Criterion) {
  c.bench_function("new beacon", |b| {
    b.iter(|| IEEE802_11Frame::new(&IEEE802_11_BEACON_PACKET))
  });

  c.bench_function("beacon ssid", |b| {
    b.iter(|| {
      let frame = IEEE802_11Frame::new(&IEEE802_11_BEACON_PACKET);

      match frame.next_layer() {
        IEEE802_11FrameLayer::Management(management_frame) => match management_frame.next_layer() {
          Some(management_frame_layer) => match management_frame_layer {
            ManagementFrameLayer::Beacon(beacon_frame) => beacon_frame.ssid(),
            _ => unreachable!(),
          },
          _ => unreachable!(),
        },
        _ => unreachable!(),
      }
    })
  });

  c.bench_function("beacon test", |b| {
    b.iter(|| {
      let frame = IEEE802_11Frame::new(&IEEE802_11_BEACON_PACKET);

      match frame.next_layer() {
        IEEE802_11FrameLayer::Management(management_frame) => match management_frame.next_layer() {
          Some(management_frame_layer) => match management_frame_layer {
            ManagementFrameLayer::Beacon(beacon_frame) => {
              black_box(beacon_frame.transmitter_address());
              black_box(beacon_frame.receiver_address());
              black_box(beacon_frame.destination_address());
              black_box(beacon_frame.source_address());
              black_box(beacon_frame.bssid_address());
              // black_box(beacon_frame.station_address());
              black_box(beacon_frame.ssid());
            }
            _ => unreachable!(),
          },
          _ => unreachable!(),
        },
        _ => unreachable!(),
      }
    })
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
