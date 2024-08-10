use cudarc::driver::{self, LaunchAsync, LaunchConfig};
use std::error::Error;
fn main() {
    match cuda_test(100) {
        Ok(_) => println!("Success"),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn cuda_test(length: usize) -> Result<usize, Box<dyn Error>> {
    let kernel_src: String = std::fs::read_to_string("sin.cu").unwrap();
    let dev = driver::CudaDevice::new(0)?;
    let input_vector: Vec<f32> = (1..length).map(|i| i as f32).collect();
    let inp = dev.htod_copy(input_vector)?;
    let mut out = dev.alloc_zeros::<f32>(length)?;

    let ptx = cudarc::nvrtc::compile_ptx(kernel_src)?;

    dev.load_ptx(ptx, "sin", &["sin_kernel"])?;
    let sin_kernel = dev.get_func("sin", "sin_kernel").unwrap();
    let cfg = LaunchConfig::for_num_elems(length as u32);
    unsafe { sin_kernel.launch(cfg, (&mut out, &inp, length)) }?;

    let out_host: Vec<f32> = dev.dtoh_sync_copy(&out)?;
    println!("{:?}", out_host);
    Ok(out_host.len())
}
