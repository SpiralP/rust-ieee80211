#![allow(dead_code)]

use criterion::*;

include!("../tests/main_test.rs");

fn criterion_benchmark(c: &mut Criterion) {
  c.bench_function("new beacon", |b| b.iter(|| Frame::new(&BEACON_PACKET[..])));

  c.bench_function("beacon ssid", |b| {
    b.iter(|| {
      let frame = Frame::new(&BEACON_PACKET[..]);

      match frame.next_layer().unwrap() {
        FrameLayer::Management(management_frame) => match management_frame.next_layer().unwrap() {
          ManagementFrameLayer::Beacon(beacon_frame) => beacon_frame.ssid(),
          _ => unreachable!(),
        },
        _ => unreachable!(),
      }
    })
  });

  c.bench_function("beacon test", |b| {
    b.iter(|| {
      let frame = Frame::new(&BEACON_PACKET[..]);

      match frame.next_layer().unwrap() {
        FrameLayer::Management(management_frame) => match management_frame.next_layer().unwrap() {
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
      }
    })
  });

  c.bench_function("byteorder LittleEndian::read_u16", |b| {
    use byteorder::{ByteOrder, LittleEndian};

    const BYTES: [u8; 2] = [0x12, 0x34];

    assert_eq!(LittleEndian::read_u16(&BYTES), 0x3412u16);

    b.iter(|| {
      //
      LittleEndian::read_u16(&BYTES)
    })
  });

  c.bench_function("manual LittleEndian read_u16", |b| {
    const BYTES: [u8; 2] = [0x12, 0x34];

    {
      let left = u16::from(BYTES[1]);
      let right = u16::from(BYTES[0]);

      assert_eq!((right | (left << 8)), 0x3412u16);
    }

    b.iter(|| {
      let left = u16::from(BYTES[1]);
      let right = u16::from(BYTES[0]);
      right | (left << 8)
    })
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
