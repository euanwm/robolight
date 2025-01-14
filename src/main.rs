use std::process::Command;
use std::{thread, time::Duration};
use tokio;

use tapo::{requests::Color, ApiClient};

// yeah this should definitely be in a config file
const TAPO_IP: &str = ""; // local IP of the light
const TAPO_USERNAME: &str = ""; // username is your email the account is attached to
const TAPO_PASSWORD: &str = ""; // duh
const BRIGHTNESS: i8 = 10; // omg such dim (0-100)

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(running_loop());
}

fn camera_on(interval_rate_s: i8) -> Result<bool, Box<dyn std::error::Error>> {
    let output = Command::new("ps").arg("-aux").output()?;
    let output_str = String::from_utf8_lossy(&output.stdout); // seriously, what sort of type is a fucking Cow?

    let mut count = 0;
    for _ in 0..interval_rate_s {
        for line in output_str.lines() {
            if line.contains("uvcvideo") && line.contains("root") {
                count += 1;
            }
        }
        thread::sleep(Duration::from_secs(1));
    }
    // there's 1 background process running on the port so any additional processes
    // are from the camera being turned on
    if count / interval_rate_s >= 1 {
        return Ok(true);
    }
    Ok(false)
}

async fn running_loop() {
    println!("init light");
    let robolight = ApiClient::new(TAPO_USERNAME, TAPO_PASSWORD)
        .l530(TAPO_IP)
        .await;

    match robolight {
        Ok(robolight) => {
            println!("starting process");
            // cope harder soy boy
            while true {
                if camera_on(3).unwrap() {
                    println!("camera on");
                    robolight.set_brightness(BRIGHTNESS as u8).await.unwrap();
                    robolight.set_color(Color::DeepSkyBlue).await.unwrap();
                }
                if !camera_on(3).unwrap() {
                    println!("camera off");
                    robolight.set_brightness(BRIGHTNESS as u8).await.unwrap();
                    robolight.set_color(Color::WarmWhite).await.unwrap();
                }
            }
            println!("dying kthnxbai"); // unreachable because i'm a lazy prick
        }
        Err(e) => {
            println!("error: {}", e);
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn set_light_settings() {
        let brightness = 10;
        let light = ApiClient::new(TAPO_USERNAME, TAPO_PASSWORD)
            .l530(TAPO_IP)
            .await
            .unwrap();
        light.set_brightness(brightness).await.unwrap();
    }
}
