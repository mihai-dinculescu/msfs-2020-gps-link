use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=SimConnect.dll");
    println!("cargo:rerun-if-changed=SimConnect.lib");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let binary_name = "SimConnect";
    let binary_name_dll = &format!("{binary_name}.dll");

    let simconnect_path = PathBuf::from(env::current_dir().unwrap().to_str().unwrap());
    let simconnect_dir = simconnect_path.as_path().to_str().unwrap();
    println!("cargo:rustc-link-search={}", simconnect_dir);
    println!("cargo:rustc-link-lib={}", binary_name);

    // copy the dll
    let mut source_path = PathBuf::from(simconnect_dir);
    source_path.push(binary_name_dll);

    let mut target_path = out_path.clone();
    target_path.pop();
    target_path.pop();
    target_path.pop();
    target_path.push(binary_name_dll);

    std::fs::copy(source_path.as_path(), target_path.as_path()).unwrap();

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .clang_args(&["-x", "c++"])
        .allowlist_function("SimConnect_Open")
        .allowlist_function("SimConnect_Close")
        .allowlist_function("SimConnect_MapClientEventToSimEvent")
        .allowlist_function("SimConnect_AddClientEventToNotificationGroup")
        .allowlist_function("SimConnect_SetNotificationGroupPriority")
        .allowlist_function("SimConnect_CallDispatch")
        .allowlist_function("SimConnect_GetNextDispatch")
        .allowlist_function("SimConnect_AddToDataDefinition")
        .allowlist_function("SimConnect_RequestDataOnSimObject")
        .allowlist_function("SimConnect_SubscribeToFacilities")
        .allowlist_function("SimConnect_RequestFacilitiesList")
        .allowlist_type("SIMCONNECT_RECV")
        .allowlist_type("SIMCONNECT_RECV_ID")
        .allowlist_type("SIMCONNECT_RECV_EVENT")
        .allowlist_type("SIMCONNECT_RECV_SIMOBJECT_DATA")
        .allowlist_type("SIMCONNECT_RECV_AIRPORT_LIST")
        .allowlist_type("SIMCONNECT_CLIENT_DATA_PERIOD")
        .allowlist_type("SIMCONNECT_RECV_OPEN")
        .allowlist_type("SIMCONNECT_RECV_EXCEPTION")
        .allowlist_var("SIMCONNECT_DATA_REQUEST_FLAG_CHANGED")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
