use core::time::Duration;

fn main() {
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        if p.port_name.contains("USB"){
            println!("{}", p.port_name);
        }
    }
    let mut port = serialport::new("/dev/ttyUSB0", 460800)
    .timeout(Duration::from_millis(10))
    .open().expect("Failed to open port");
    let output = "This is a test. This is only a test.".as_bytes();
    port.write(output).expect("Write failed!");
    let mut serial_buf: Vec<u8> = vec![0; 32];
    port.read(serial_buf.as_mut_slice()).expect("Found no data!");

}