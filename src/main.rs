use std::alloc::System;
use std::fmt::Debug;
use std::process::exit;
use console::Key::PageDown;
use dialoguer::{Confirm, Input, Select};
use dialoguer::theme::ColorfulTheme;
use machine_info::GraphicCard;
use serde::Deserialize;

struct SystemInfo {
    os: String,
    cpu_vendor: String,
    cpu_generation: String,
    cpu_model: String,
    gpus: Vec<String>,
    ram: u64,
    storage_devices: Vec<String>,
    oem_model: String,
    ethernet_chipset: String,
    wlan_chipset: String,
    bluetooth_chipset: String,
}

struct CpuInfo {vendor: String, model: String}
impl CpuInfo {
    fn new() -> CpuInfo {
        let cpu_string = machine_info::Machine::new().system_info().processor.brand;
        let parts: Vec<&str> = cpu_string.split_whitespace().collect();
        let vendor = parts[0].to_string().replace("(R)", "");
        let model = parts[3].to_string();

        CpuInfo {
            vendor,
            model,
        }
    }

    fn get_generation(&self) -> String {
        if let Some(pos) = self.model.find('-') {
            if self.model.len() > pos + 1 {
                return self.model[pos + 1..pos + 2].to_string();
            }
        }
        "Unknown".to_string()
    }
}

fn main() {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose what you would like to do: ")
        .items(&["Create new EFI", "Exit"])
        .interact()
        .unwrap();

    match selection {
        0 => {},
        1 => exit(0),
        _ => println!("Invalid selection")
    }

    SystemInfo::new();

}

impl Host for SystemInfo {
    fn new() -> Self {
        let mut computer = machine_info::Machine::new();
        let sys_info = computer.system_info();

        let os_name = sys_info.distribution;
        
        machine_info::Machine::new().system_info().processor

        let cpu_info = CpuInfo::split_create_struct(&sys_info.processor.brand);
        let cpu_name = cpu_info.vendor.clone();
        let cpu_generation = cpu_info.get_generation();
        let cpu_model = cpu_info.model.clone();

        //let graphics_card = sys_info.graphics;
        let graphics_card = vec!["".to_string()];

        fn bytes_to_gb(bytes: u64) -> u64 {
            let gb = bytes as f64 / (1024.0 * 1024.0 * 1024.0);
            let rounded_gb = 2u64.pow((gb.log2().round() as u32).max(1));
            rounded_gb
        }

        let ram = bytes_to_gb(sys_info.memory);

        let storage_devices = vec![String::from("To be implemented")];

        let oem_model = String::from("Unable to do, ask the user for additional information");

        let ethernet_chipset = String::from("Unable to do, ask the user for additional information");

        let wlan_chipset = String::from("Unable to do, ask the user for additional information");
        let bluetooth_chipset = String::from("Unable to do, ask the user for additional information");

        // SystemInfo {
        //      os_name: "Windows",
        //      kernel_version: "22631",
        //      os_version: "11 (22631)",
        //      hostname: "home-pc",
        //      distribution: "windows",
        //      memory: 34308382720,
        //      processor: Processor {
        //           frequency: 0,
        //           vendor: "GenuineIntel",
        //           brand: "Intel(R) Core(TM) i7-8700 CPU @ 3.20GHz"
        //      },
        //      total_processors: 12,
        //      graphics: [
        //           GraphicCard {
        //               id: "GPU-c71a23ac-6bb0-2c7b-0836-4c50f65f9d4e",
        //               name: "NVIDIA GeForce GTX 1050 Ti",
        //               brand: "GeForce", memory: 4294967296, temperature: 33
        //      }],
        //      disks: [Disk {
        //          name: "",
        //          fs: "NTFS",
        //          storage_type: "SSD",
        //          mount_point: "C:\\",
        //          available: 354250145792,
        //          size: 999275098112
        //      }],
        //      cameras: [],
        //      nvidia: Some(NvidiaInfo {
        //          driver_version: "565.90",
        //          nvml_version: "12.565.90",
        //          cuda_version: 12070
        //      }),
        //      vaapi: false,
        //      model: None
        // }

        SystemInfo {
            os: os_name,
            cpu_vendor: cpu_name,
            cpu_generation,
            cpu_model,
            gpus: graphics_card,
            ram,
            storage_devices,
            oem_model,
            ethernet_chipset,
            wlan_chipset,
            bluetooth_chipset,
        }
    }
}

impl SystemInfo {
    fn new() -> Self {
        let selection = |prompt: &str, items: &Vec<&str>| -> usize {
            Select::with_theme(&ColorfulTheme::default())
                .with_prompt(prompt)
                .items(items)
                .interact()
                .unwrap()
        };

        let input = |prompt: &str, default: String| -> String {
            Input::with_theme(&ColorfulTheme::default())
                .with_prompt(prompt)
                .default(default)
                .show_default(true)
                .interact_text()
                .unwrap()
        };

        let os: String = match selection(
            "What OS do you have?", 
            &vec!["Windows", "Linux", "macOS"],
        ) {
            0 => {"windows".to_string()}
            1 => {"linux".to_string()}
            2 => {"macos".to_string()}
            _ => {"".to_string()}
        };

        let cpu_vendor = input(
            "What is your CPU Brand/Vendor",
            CpuInfo::new().vendor
        );

        let cpu_generation = input("What is your CPU generation? (8, 9, 10)"c ,);
        
        let cpu_model = input("What model is your CPU (i7-8700)");

        let mut hasAllGPU = false;
        let mut gpus = Vec::new();
        while !hasAllGPU {
            let gpu = input("What gpu do you have? (NVIDIA GTX 1050 Ti)");
            gpus.push(gpu);
            let confirmation = Confirm::new()
                .with_prompt("Do you want to add more?")
                .interact()
                .unwrap();
            
            if confirmation {
                hasAllGPU = true;
            }
            println!("Adding more GPUs")
        }

        let ram = input("How much memory does your computer have? [write as int, and in gigabytes]").parse::<u64>().unwrap();

        let mut hasAllStorage = false;
        let mut storageDevices = Vec::new();
        while !hasAllStorage {
            let storage = input("What storage device do you have?");
            storageDevices.push(storage);
            let confirmation = Confirm::new()
                .with_prompt("Do you want to add more?")
                .interact()
                .unwrap();

            if confirmation {
                hasAllStorage = true;
            }
            println!("Adding more GPUs")
        }


        Self {
            os,
            cpu_vendor,
            cpu_generation,
            cpu_model,
            gpus,
            ram,
            storage_devices: vec![],
            oem_model: "".to_string(),
            ethernet_chipset: "".to_string(),
            wlan_chipset: "".to_string(),
            bluetooth_chipset: "".to_string(),
        }
    }
}