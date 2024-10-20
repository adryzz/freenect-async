use freenect_async::{
    context::{FreenectContext, FreenectLogLevel}, formats::{FreenectDepthFormat, FreenectResolution, FreenectVideoFormat}, motors_led::FreenectLedState, FreenectError
};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    run().await.unwrap();
}

async fn run() -> Result<(), FreenectError> {
    let ctx = FreenectContext::new()?;

    let mut ctx = ctx.setup_all();
    ctx.set_log_level(FreenectLogLevel::Spew);
    let num = ctx.list_devices()?;
    println!("Devices connected: {}", num);
    if num == 0 {
        return Ok(());
    }
    println!("Opening first device.");

    let mut dev = ctx.open_device(0)?;

    dev.set_led(FreenectLedState::Yellow)?;

    println!("Selected modes:");
    let vmodes = dev.get_supported_video_modes();
    let vmode = vmodes
        .iter()
        .filter(|a| a.resolution == FreenectResolution::Medium)
        .filter(|a| a.format == FreenectVideoFormat::Rgb.into())
        .next()
        .unwrap();
    let dmodes = dev.get_supported_depth_modes();
    let dmode = dmodes
        .iter()
        .filter(|a| a.format == FreenectDepthFormat::DepthMillimeters.into())
        .next()
        .unwrap();

    println!("{}", vmode);
    println!("{}", dmode);

    dev.set_ir_brightness(50)?;
    dev.set_tilt_degree(-30.0)?;
    std::thread::sleep(std::time::Duration::from_millis(1000));
    dev.set_tilt_degree(30.0)?;
    std::thread::sleep(std::time::Duration::from_millis(1000));
    dev.set_tilt_degree(0.0)?;

    let mut stream = dev.start_video_stream(&vmode)?;

    stream.dev_ref().set_tilt_degree(0.0)?;
    loop {
        use lending_stream::LendingStream;
        let res = stream.next().await.unwrap().unwrap();
        dbg!(res.timestamp);
        stream.dev_ref().set_led(FreenectLedState::Green)?;
        //stream.try_read_camera_frame()?;
        //stream.try_read_depth_frame()?;
    }
    Ok(())
}
