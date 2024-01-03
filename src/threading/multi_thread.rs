use crate::{
    models::{compression_metric::CompressionMetric, part::Part},
    service::{algorithms::Algorithm, io::new_codec},
    utils::utils::split_into_parts,
};
use std::error::Error;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Instant;

/// compute_algorithm benchmarks how long a particular algorithm took to run
pub fn compute_algorithm(text: String, algorithm: Algorithm) -> CompressionMetric {
    // start the timer for benchmarking the time spent
    let start_time = Instant::now();
    let parts = split_into_parts(text);

    // create channels for sending the compressed and decompressed data among threads
    let (compressed_tx, compressed_rx) = mpsc::channel();
    let (decompressed_tx, decompressed_rx) = mpsc::channel();

    // handles to serve as anchors to the threads
    let mut handles = vec![];

    // spawn worker threads
    for part in parts {
        // clone the transmitters and algorithm and move them to each worker thread
        let compressed_tx_clone = compressed_tx.clone();
        let decompressed_tx_clone = decompressed_tx.clone();
        let algo = algorithm.clone();

        let handle = thread::spawn(move || {
            let mut codec = new_codec(part.1.clone(), algo)
                .expect("codec should not be none");

            // encode the text part and send the compressed data to the compressed channel
            codec.encode();
            compressed_tx_clone
                .send(Part(part.0, codec.compressed()))
                .expect("compressed data should be sent to the compressed transmitter");

            // decode the encoded part and send the decompressed data to the decompressed channel
            codec.decode();
            decompressed_tx_clone
                .send(Part(part.0, codec.decompressed()))
                .expect("decompressed data should be sent to the decompressed transmitter");
        });
        handles.push(handle);
    }

    // drop the two sender channels so the aggregator thread does not wait to receive more data
    drop(compressed_tx);
    drop(decompressed_tx);

    // create the encoded and decoded results
    let decoded_result = Arc::new(Mutex::new(Vec::new()));
    let encoded_result = Arc::new(Mutex::new(Vec::new()));

    // clone the results to send across the threads
    let shared_decoded_result = Arc::clone(&decoded_result);
    let shared_encoded_result = Arc::clone(&encoded_result);

    // Spawn the aggregator thread
    let aggregator_handle = thread::spawn(move || {
        let mut encoded_result = shared_encoded_result.lock().unwrap();
        for encoded_message in compressed_rx {
            encoded_result.push(encoded_message);
        }

        let mut decoded_result = shared_decoded_result.lock().unwrap();
        for decoded_message in decompressed_rx {
            decoded_result.push(decoded_message);
        }
    });

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Wait for the aggregator thread to finish
    aggregator_handle.join().unwrap();

    let mut encoded_result = encoded_result.lock().unwrap();
    encoded_result.sort_by(|a, b| a.0.cmp(&b.0));
    let encoded_result = encoded_result
        .iter()
        .map(|p| p.1.clone())
        .collect::<Vec<String>>()
        .join("");

    let mut decoded_result = decoded_result.lock().unwrap();
    decoded_result.sort_by(|a, b| a.0.cmp(&b.0));
    let decoded_result = decoded_result
        .iter()
        .map(|p| p.1.clone())
        .collect::<Vec<String>>()
        .join("");

    let metric = CompressionMetric::new(algorithm, encoded_result, decoded_result, start_time);

    metric
}
