use ocl::{Platform, Device, ProQue, Buffer};
use ocl::core;
use ocl::core::{DeviceInfo};

#[derive(Clone)]
pub struct Matrix {
    n_rows: usize,
    n_cols: usize,
    rs: usize,
    cs: usize,
    matrix: Vec<f64>
}

pub struct Context {
    pub compute_units: u32,
    pro_que: ocl::ProQue
}

impl Context {
    pub fn new(device: Option<Device>, ocl_kernel: &str) -> Option<Context> {
        // pretty much from https://docs.rs/crate/ocl-algebra/0.1.0/source/src/lib.rs
        let mut ocl_device = device;

        if None == ocl_device {
            let platforms = Platform::list();
            let mut compute_units = 0;

            for p_idx in 0..platforms.len() {
                let platform = &platforms[p_idx];
                let devices = Device::list_all(platform);
                for d_idx in 0..devices.len() {
                    let device = devices[d_idx];
                    let device_info_result = core::get_device_info(&device, DeviceInfo::MaxComputeUnits);
                    let units = device_info_result.to_string().parse().unwrap();
                    if units > compute_units {
                        ocl_device = Some(device);
                        compute_units = units;
                    }
                }
            }

            // something went wrong if there are no compute units!
            if compute_units == 0 {
                return None
            }
        }
        
        let que = ProQue::builder()
                    .device(ocl_device.unwrap())
                    .src(ocl_kernel)
                    .build().expect("Build ProQue");
        Some(Context{
            compute_units: compute_units,
            pro_que: que
        })
    }
}