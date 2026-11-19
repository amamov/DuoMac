//! duomac-pair: the USB half of DuoMac's installer.
//!
//!   duomac-pair devices                 UDIDs of iPhones usbmuxd sees over USB
//!   duomac-pair devmode UDID            "on" / "off"; when off, reveals the Developer Mode switch in Settings
//!   duomac-pair find-app UDID           bundle id of the installed DuoMac (sideloaders append a team id to it)
//!   duomac-pair pair UDID [BUNDLE_ID]   iOS 27 remote pairing over USB, written into DuoMac's
//!                                       Documents/StikJIT/pairingFile.plist (what its built-in JIT authenticates with)
//!   duomac-pair pair-file UDID OUT      the same pairing, written to a local file instead
use idevice::remote_pairing::{RemotePairingLockdownService, RpPairingFile};
use idevice::services::afc::opcode::AfcFopenMode;
use idevice::services::amfi::AmfiClient;
use idevice::services::house_arrest::HouseArrestClient;
use idevice::services::installation_proxy::InstallationProxyClient;
use idevice::usbmuxd::{UsbmuxdAddr, UsbmuxdConnection};
use idevice::IdeviceService;

type Res<T> = Result<T, Box<dyn std::error::Error>>;

const PAIRING_DIR: &str = "/Documents/StikJIT";
const PAIRING_PATH: &str = "/Documents/StikJIT/pairingFile.plist";

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let a: Vec<&str> = args.iter().map(String::as_str).collect();
    let result = match a.as_slice() {
        ["devices"] => devices().await,
        ["devmode", udid] => devmode(udid).await,
        ["find-app", udid] => find_app(udid).await.map(|id| println!("{id}")),
        ["pair", udid] => pair_into_app(udid, None).await,
        ["pair", udid, bundle] => pair_into_app(udid, Some(bundle)).await,
        ["pair-file", udid, out] => pairing(udid).await.and_then(|f| Ok(std::fs::write(out, f.to_bytes())?)),
        _ => {
            eprintln!("usage: duomac-pair devices | devmode UDID | find-app UDID | pair UDID [BUNDLE_ID] | pair-file UDID OUT");
            std::process::exit(2);
        }
    };
    if let Err(e) = result {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

fn provider(device: &idevice::usbmuxd::UsbmuxdDevice) -> idevice::provider::UsbmuxdProvider {
    device.to_provider(UsbmuxdAddr::default(), "duomac-pair")
}

async fn device(udid: &str) -> Res<idevice::usbmuxd::UsbmuxdDevice> {
    let mut mux = UsbmuxdConnection::default().await?;
    Ok(mux.get_device(udid).await?)
}

async fn devices() -> Res<()> {
    let mut mux = UsbmuxdConnection::default().await?;
    for d in mux.get_devices().await? {
        if matches!(d.connection_type, idevice::usbmuxd::Connection::Usb) {
            println!("{}", d.udid);
        }
    }
    Ok(())
}

async fn devmode(udid: &str) -> Res<()> {
    let p = provider(&device(udid).await?);
    let mut amfi = AmfiClient::connect(&p).await?;
    if amfi.get_developer_mode_status().await? {
        println!("on");
    } else {
        // Shows Settings > Privacy & Security > Developer Mode, which iOS hides until something asks for it.
        let _ = amfi.reveal_developer_mode_option_in_ui().await;
        println!("off");
    }
    Ok(())
}

async fn find_app(udid: &str) -> Res<String> {
    let p = provider(&device(udid).await?);
    let mut proxy = InstallationProxyClient::connect(&p).await?;
    let apps = proxy.get_apps(Some("User"), None).await?;
    // ponytail: matched by name and by our bundle id prefix; a sideloader may rename either one, not both
    let mut ids: Vec<&String> = apps
        .iter()
        .filter(|(id, info)| {
            let name = info.as_dictionary()
                .and_then(|d| d.get("CFBundleDisplayName").or_else(|| d.get("CFBundleName")))
                .and_then(|v| v.as_string())
                .unwrap_or("");
            name == "DuoMac" || id.starts_with("io.duomac.app")
        })
        .map(|(id, _)| id)
        .collect();
    ids.sort();
    match ids.as_slice() {
        [one] => Ok(one.to_string()),
        [] => Err("DuoMac is not installed on this iPhone".into()),
        many => Err(format!("more than one DuoMac is installed: {many:?}; pass the bundle id").into()),
    }
}

async fn pairing(udid: &str) -> Res<RpPairingFile> {
    let p = provider(&device(udid).await?);
    let service = RemotePairingLockdownService::connect(&p).await?;
    let host = "DuoMac";
    let mut client = service.into_client(host)?;
    let mut file = RpPairingFile::generate(host);
    eprintln!("Pairing with the iPhone (unlock it, and allow the pairing if it asks)...");
    client.connect(&mut file, || async { "000000".to_string() }).await?;
    Ok(file)
}

async fn pair_into_app(udid: &str, bundle: Option<&str>) -> Res<()> {
    let bundle = match bundle {
        Some(b) => b.to_string(),
        None => find_app(udid).await?,
    };
    let file = pairing(udid).await?;
    let p = provider(&device(udid).await?);
    let mut afc = HouseArrestClient::connect(&p).await?.vend_container(&bundle).await?;
    let _ = afc.mk_dir(PAIRING_DIR).await; // already there after an earlier pairing
    let mut fd = afc.open(PAIRING_PATH, AfcFopenMode::WrOnly).await?;
    fd.write_entire(&file.to_bytes()).await?;
    fd.close().await?;
    println!("paired; the pairing file is in {bundle}");
    Ok(())
}
