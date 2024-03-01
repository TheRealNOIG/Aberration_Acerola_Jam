pub fn scale_buffer(buffer: &[u32], src_width: usize, src_height: usize, dest_width: usize, dest_height: usize) -> Vec<u32> {
    let mut scaled_buffer = vec![0; dest_width * dest_height];

    let x_ratio = (src_width << 16) / dest_width;
    let y_ratio = (src_height << 16) / dest_height;

    for y in 0..dest_height {
        for x in 0..dest_width {
            let src_x = (x * x_ratio) >> 16;
            let src_y = (y * y_ratio) >> 16;

            let src_index = src_y * src_width + src_x;

            scaled_buffer[y * dest_width + x] = buffer[src_index];
        }
    }

    scaled_buffer
}
