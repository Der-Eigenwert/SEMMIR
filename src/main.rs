use std::io::Write;
use std::process::Command;

const SAMPLE_RATE: f32 = 48_000_f32;
const PITCH_STANDARD: f32 = 440_f32;
const VOLUME: f32 = 0.5_f32;
const BPM: f32 = 100_f32;

fn note(n: i32) -> f32 {
    let a = 2_f32.powf(1.0 / 12.0);

    PITCH_STANDARD * a.powi(n)
}

fn to_hz(i: u32) -> f32 {
    2_f32 * std::f32::consts::PI * (i as f32) / SAMPLE_RATE
}

fn pulse(n: i32, beats: f32) -> Vec<f32> {
    let mut v = vec![];

    let duration = 60_f32 * beats / BPM;
    for i in 0..((duration * SAMPLE_RATE) as u32) {
        let f = note(n) * to_hz(i);

        v.push(f);
    }

    for x in v.iter_mut() {
        *x = VOLUME * x.sin();
    }

    let attack_time = (0.05 * (duration * SAMPLE_RATE)) as usize;
    for i in 0..attack_time {
        v[i] *= i as f32 / attack_time as f32;
    }

    let release_time = (0.05 * (duration * SAMPLE_RATE)) as usize;
    let len = v.len();
    for i in 0..release_time {
        v[len - i - 1] *= i as f32 / release_time as f32;
    }

    v
}

fn main() -> std::io::Result<()> {
    let mut file = ::std::fs::File::create("output.bin")?;

    let mut v = vec![];

    // v.append(&mut pulse(0,  2.0));
    // v.append(&mut pulse(2,  2.0));
    // v.append(&mut pulse(4,  2.0));
    // v.append(&mut pulse(5,  2.0));
    // v.append(&mut pulse(7,  2.0));
    // v.append(&mut pulse(9,  2.0));
    // v.append(&mut pulse(11, 2.0));
    // v.append(&mut pulse(12, 2.0));

    v.append(&mut pulse(0, 0.5));
    v.append(&mut pulse(0, 0.5));
    v.append(&mut pulse(4, 0.5));
    v.append(&mut pulse(4, 0.5));
    v.append(&mut pulse(5, 0.5));
    v.append(&mut pulse(5, 0.5));
    v.append(&mut pulse(4, 1.0));

    v.append(&mut pulse(3, 0.5));
    v.append(&mut pulse(3, 0.5));
    v.append(&mut pulse(2, 0.5));
    v.append(&mut pulse(2, 0.5));
    v.append(&mut pulse(1, 0.5));
    v.append(&mut pulse(1, 0.5));
    v.append(&mut pulse(0, 1.0));

    for x in &v {
        file.write_all(&x.to_le_bytes())?;
    }

    let _ = Command::new("aplay")
        .arg("-f").arg("float_le")
        .arg("-r").arg(format!("{}", SAMPLE_RATE))
        .arg("output.bin")
        .output()?;

    Ok(())
}
