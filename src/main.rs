use std::{fs::File, io::Write};

mod ring_buffer;
mod vibrato;
mod lfo;

fn show_info() {
    eprintln!("MUSI-6106 Assignment Executable");
    eprintln!("(c) 2024 Stephen Garrett & Ian Clester");
}

fn main() {
   show_info();

    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <input wave filename> <output text filename>", args[0]);
        return
    }

    // Open the input wave file
    let mut reader = hound::WavReader::open(&args[1]).unwrap();
    let spec = reader.spec();
    let channels = spec.channels;

    // Read audio data and write it to the output text file (one column per channel)
    let mut out = File::create(&args[2]).expect("Unable to create file");
    for (i, sample) in reader.samples::<i16>().enumerate() {
        let sample = sample.unwrap() as f32 / (1 << 15) as f32;
        write!(out, "{}{}", sample, if i % channels as usize == (channels - 1).into() { "\n" } else { " " }).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::vibrato::Vibrato;

    #[test]
    fn test_vibrato_delayed_input_when_mod_amplitude_is_zero() {
        let samplerate = 44100.0;
        let mod_freq = 5.0;
        let mod_depth = 0.0; // Modulation amplitude is zero
        let delay_time_sec = 0.1;
        let mut vibrato = Vibrato::new(samplerate, mod_freq, mod_depth, delay_time_sec);

        // Prepare input signal (e.g., delayed input)
        let input_signal = vec![0.0, 0.1, 0.2, 0.3, 0.4];
        let expected_output = input_signal.clone(); // Expected output is the same as input

        // Process input signal
        let output_signal = vibrato.process(&input_signal);

        // Compare output with expected output
        assert_eq!(output_signal, expected_output);
    }

    #[test]
    fn test_vibrato_dc_input_results_in_dc_output() {
        let samplerate = 44100.0;
        let mod_freq = 5.0;
        let mod_depth = 0.5; // Non-zero modulation amplitude
        let delay_time_sec = 0.1;
        let mut vibrato = Vibrato::new(samplerate, mod_freq, mod_depth, delay_time_sec);

        // Prepare input signal (DC input)
        let input_signal = vec![0.5; 5]; // All samples are the same DC value
        let expected_output = vec![0.5; 5]; // Expected output should be the same DC value

        // Process input signal
        let output_signal = vibrato.process(&input_signal);

        // Compare output with expected output
        assert_eq!(output_signal, expected_output);
    }

    #[test]
    fn test_vibrato_varying_input_block_size() {
        let samplerate = 44100.0;
        let mod_freq = 5.0;
        let mod_depth = 0.5;
        let delay_time_sec = 0.1;
        let mut vibrato = Vibrato::new(samplerate, mod_freq, mod_depth, delay_time_sec);

        // Prepare input signal with different block sizes
        let input_signals = vec![
            vec![0.1, 0.2, 0.3],
            vec![0.4, 0.5, 0.6, 0.7],
            vec![0.8, 0.9],
        ];
        let expected_outputs = vec![
            vec![0.1, 0.2, 0.3],
            vec![0.4, 0.5, 0.6, 0.7],
            vec![0.8, 0.9],
        ];

        // Process each input signal
        for (input_signal, expected_output) in input_signals.iter().zip(expected_outputs.iter()) {
            let output_signal = vibrato.process(&input_signal);
            assert_eq!(output_signal, *expected_output);
        }
    }

    #[test]
    fn test_vibrato_zero_input_signal() {
        let samplerate = 44100.0;
        let mod_freq = 5.0;
        let mod_depth = 0.5;
        let delay_time_sec = 0.1;
        let mut vibrato = Vibrato::new(samplerate, mod_freq, mod_depth, delay_time_sec);

        // Prepare zero input signal
        let input_signal = vec![0.0; 5];
        let expected_output = vec![0.0; 5]; // Expected output should also be zero

        // Process input signal
        let output_signal = vibrato.process(&input_signal);

        // Compare output with expected output
        assert_eq!(output_signal, expected_output);
    }
}
