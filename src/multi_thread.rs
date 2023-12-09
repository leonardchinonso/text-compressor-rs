use std::error::Error;
use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use crate::algorithms::Algorithm;
use crate::io::{
    self,
    args::Argument,
    file::File
};
use crate::models::part::Part;
use crate::models::response::Response;
use crate::pkg::traits::Reader;
use crate::utils::utils::split_into_parts;


pub fn benchmark_algorithms(args: Argument) -> Result<Vec<Response>, Box<dyn Error>> {
    let mut file = File::new(&args.file_name(), "test_data/out_data.txt");
    let text = file.read().expect("cannot read file!");

    let algorithms = [Algorithm::Huffman, Algorithm::Lzw, Algorithm::Bwt, Algorithm::Rle];
    let mut responses: Vec<Response> = vec![Response::build(); algorithms.len()];

    algorithms.into_iter().enumerate().for_each(|(i, a)| {
        responses[i] = compute_algorithm(text.clone(), a).unwrap();
    });

    Ok(responses)
}

// TODO: Add benchmarking
/// compute_algorithm benchmarks how long a particular algorithm took to run
fn compute_algorithm(text: String, algorithm: Algorithm) -> Result<Response, Box<dyn Error>> {
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
            let mut codec = io::new_codec(part.1.clone(), algo).unwrap();

            // encode the text part and send the compressed data to the compressed channel
            codec.encode();
            compressed_tx_clone.send(Part(part.0, codec.compressed())).unwrap();

            // decode the encoded part and send the decompressed data to the decompressed channel
            codec.decode();
            decompressed_tx_clone.send(Part(part.0, codec.decompressed())).unwrap();
        });
        handles.push(handle);
    }

    // drop the two sender channels so the aggregator thread does not wait to receive more data
    drop(compressed_tx);
    drop(decompressed_tx);

    // create the encoded and decoded results
    let mut decoded_result = Arc::new(Mutex::new(Vec::new()));
    let mut encoded_result = Arc::new(Mutex::new(Vec::new()));

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

    let mut response = Response::build();

    let mut encoded_result = encoded_result.lock().unwrap();
    encoded_result.sort_by(|a, b| a.0.cmp(&b.0));
    let encoded_result = encoded_result.iter().map(|p| p.1.clone()).collect::<Vec<String>>().join("");

    let mut decoded_result = decoded_result.lock().unwrap();
    decoded_result.sort_by(|a, b| a.0.cmp(&b.0));
    let decoded_result = decoded_result.iter().map(|p| p.1.clone()).collect::<Vec<String>>().join("");

    response.set_encoded(encoded_result);
    response.set_decoded(decoded_result);

    Ok(response)
}
