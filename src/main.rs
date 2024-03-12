use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter, WriteType};
use btleplug::platform::{Adapter, Manager, Peripheral};

use std::error::Error;

use std::time::Duration;
use tokio::time;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let manager = Manager::new().await.unwrap();

    // get the first bluetooth adapter
    let adapters = manager.adapters().await?;
    let central = adapters.into_iter().nth(0).unwrap();

    // start scanning for devices
    central.start_scan(ScanFilter::default()).await?;
    // instead of waiting, you can use central.events() to get a stream which will
    // notify you of new devices, for an example of that see examples/event_driven_discovery.rs
    time::sleep(Duration::from_secs(2)).await;

    for peripheral in central.peripherals().await? {
        let properties = peripheral.properties().await?.unwrap();
        println!("Device Name: {:?}", properties.local_name);
        println!("Address: {:?}", properties.address);
        println!("Signal Strength: {:?}", properties.tx_power_level);
        println!("--------------------------------------------------");
    }
    Ok(())
}
