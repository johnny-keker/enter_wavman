extern crate byteorder;
extern crate rand;

#[allow(dead_code)]
use notes_data;
use notes_tools;
use wav_tools;

#[allow(unused_imports)] // seems like its rust bug
use rand::prelude::*;
use std::io::prelude::*;

pub struct SoundProperties {
  sample_rate: u32,
  num_samples: u32,
  bpm: u32,
  iterator: u32,
}

pub struct PartProperties {
  note: f32,
  amplitude: f32,
  chord: notes_data::Chord,
}

impl SoundProperties {
  pub fn seconds_per_beat(&self) -> u32 {
    (4 * 60) / self.bpm
  }
  pub fn thirty_second_note_duration(&self) -> u32 {
    (0.0625 * (self.seconds_per_beat() * self.sample_rate) as f32) as u32
  }
  pub fn num_beats(&self) -> u32 {
    (self.bpm * self.num_samples) / (4 * 60)
  }
  pub fn get_duration_in_parts(&self, duration: f32) -> u32 {
    (duration * (self.seconds_per_beat() * self.sample_rate) as f32) as u32
      / self.thirty_second_note_duration()
  }
}

pub fn generate_ambient(sample_rate: u32, num_samples: u32, bpm: u32) -> std::io::Result<Vec<u8>> {
  use byteorder::{LittleEndian, WriteBytesExt}; // seems like its rust bug

  /* header */

  let mut buf: Vec<u8> = wav_tools::get_wav_header(sample_rate, num_samples);
  /* data */
  let mut props = SoundProperties {
    sample_rate,
    num_samples,
    bpm,
    iterator: 0,
  };
  let durations: [f32; 4] = [0.5, 0.25, 0.125, 0.0625]; // notes durations TODO: transfer it to notes_data
  let mut key: notes_tools::UserKey = notes_tools::init_notes(); // our random key for the song
  for _beat in 0..(props.num_beats()) {
    generate_beat(&mut key, durations, &mut buf, &mut props);
  }

  /* data size */

  let mut size_buf: Vec<u8> = Vec::new();
  size_buf
    .write_u32::<LittleEndian>(buf.len() as u32 - 8)
    .unwrap();
  buf.splice(4..8, size_buf);
  Ok(buf)
}
/*
    (\_/)
    (*-*)<Nya! Im sorry for bad architecture!
    ("Y")   usagi by @KaluginaMarina
*/
fn write_part(bufer: &mut Vec<u8>, sound: &mut PartProperties, props: &mut SoundProperties) -> u32 {
  use byteorder::{LittleEndian, WriteBytesExt};
  use std::f32::consts::PI;
  let j = props.iterator + props.thirty_second_note_duration();
  for i in props.iterator..j {
    let bass =
      16.0 * (2.0 * PI * sound.chord[0] * (i as f32) / (props.sample_rate as f32) as f32).sin();
    let bass_third =
      16.0 * (2.0 * PI * sound.chord[1] * (i as f32) / (props.sample_rate as f32) as f32).sin();
    let bass_quint =
      16.0 * (2.0 * PI * sound.chord[2] * (i as f32) / (props.sample_rate as f32) as f32).sin();
    let lead = ((sound.amplitude * 16.0) + 16.0)
      * (2.0 * PI * sound.note * (i as f32) / (props.sample_rate as f32) as f32).sin()
      - 128.0;
    bufer
      .write_u8((lead + bass + bass_third + bass_quint) as u8)
      .unwrap();
  }
  return j;
}

fn generate_beat(
  key: &mut notes_tools::UserKey,
  durations: [f32; 4],
  buf: &mut Vec<u8>,
  props: &mut SoundProperties,
) {
  let mut rng = thread_rng(); // rand init
  let note_shift_range = rng.gen_range(2, 4); // so we wouldnt have huge jumps
  let mut beat_remain = 1.0; // so we`ll know when beat is done
  let mut curr_index = rng.gen_range(0, key.lead.len()) as u32; // so we wouldnt have huge jumps
  while beat_remain != 0.0 {
    let avail_durations: Vec<f32> = durations
      .iter()
      .filter(|&&n| n <= beat_remain)
      .cloned()
      .collect(); // calculate available durations
    let curr_duration = rng.choose(&avail_durations).unwrap(); // choose random duration
    curr_index = rng.gen_range(
      curr_index.checked_sub(note_shift_range).unwrap_or(0),
      std::cmp::min(curr_index + note_shift_range, key.lead.len() as u32),
    ) as u32; // so we woudnt have huge jumps
    let mut sound = PartProperties {
      note: key.lead[curr_index as usize],
      amplitude: 1.5,
      chord: **rng.choose(&key.chords).unwrap(),
    };
    for _step in 0..(props.get_duration_in_parts(*curr_duration)) {
      props.iterator = write_part(buf, &mut sound, props);
    }
    beat_remain -= curr_duration;
  }
}
