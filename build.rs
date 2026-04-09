use embed_manifest::{embed_manifest, new_manifest};
use embed_manifest::manifest::ExecutionLevel;

fn main() {
    #[cfg(target_os = "windows")]
    {
        let manifest = new_manifest("CPU.Temp.Reader")
            .requested_execution_level(ExecutionLevel::RequireAdministrator);

        embed_manifest(manifest).expect("Не вдалося вбудувати Windows manifest");
    }

    println!("cargo:rerun-if-changed=build.rs");
}