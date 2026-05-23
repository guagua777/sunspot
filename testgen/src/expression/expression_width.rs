use std::io::Write;

use acir::circuit::ExpressionWidth;
use tracing::trace;

// 是否指定表达式的宽度
// 创建一个 ExpressionWidth::Unbounded 值
// 用 bincode 将它序列化成二进制字节
fn generate_expression_width_test_unbounded(path: &str) {
    let file_name = format!("{path}/expression_width_unbounded.bin");

    // Check if the file already exists
    if std::path::Path::new(&file_name).exists() {
        trace!("File {} already exists, skipping generation.", file_name);
        return;
    }

    let unbounded_expression_width = ExpressionWidth::Unbounded;
    // Create the file and write the test code
    let mut file = std::fs::File::create(&file_name).expect("Failed to create file");

    // Fixed-width little-endian encoding ensures deterministic byte layout across platforms.
    let config = bincode::config::standard()
        .with_fixed_int_encoding()
        .with_little_endian();
    let data = bincode::serde::encode_to_vec(unbounded_expression_width, config)
        .expect("Failed to encode data");
    file.write_all(data.as_slice())
        .expect("Failed to write to file");

    trace!("Generated unbounded expression width test at {}", file_name);
}

pub fn generate_expression_width_test_bounded(path: &str) {
    let file_name = format!("{path}/expression_width_bounded.bin");

    // Check if the file already exists
    if std::path::Path::new(&file_name).exists() {
        trace!("File {} already exists, skipping generation.", file_name);
        return;
    }

    let bounded_expression_width = ExpressionWidth::Bounded { width: 10 };

    // Create the file and write the test code
    let mut file = std::fs::File::create(&file_name).expect("Failed to create file");
    let config = bincode::config::standard()
        .with_fixed_int_encoding()
        .with_little_endian();
    let data = bincode::serde::encode_to_vec(bounded_expression_width, config)
        .expect("Failed to encode data");
    file.write_all(data.as_slice())
        .expect("Failed to write to file");

    trace!("Generated bounded expression width test at {}", file_name);
}

pub fn generate_tests(directory: &str) {
    let directory = format!("{directory}/expression_width/");
    // Create the directory if it doesn't exist
    // 创建路径
    std::fs::create_dir_all(&directory).expect("Failed to create directory");

    generate_expression_width_test_unbounded(&directory);
    generate_expression_width_test_bounded(&directory);

    trace!("Expression tests generated in {}", directory);
}
