use std::env::var;
use std::path::Path;

#[cfg(feature = "metamod")]
mod metamod {
    use std::collections::HashMap;
    use std::env::var;
    use std::path::Path;

    use lazy_static::lazy_static;

    struct SDK {
        pub path: &'static str,
        pub code: &'static str,
        pub define: &'static str,
    }

    impl SDK {
        fn new(path: &'static str, code: &'static str, define: &'static str) -> SDK {
            SDK { path, code, define }
        }
    }

    lazy_static! {
        static ref POSSIBLE_SDKS: HashMap<&'static str, SDK> = {
            let mut map = HashMap::new();
            map.insert("episode1", SDK::new("HL2SDK", "1", "EPISODEONE"));
            map.insert("ep2", SDK::new("HL2SDKOB", "3", "ORANGEBOX"));
            map.insert("css", SDK::new("HL2SDKCSS", "6", "CSS"));
            map.insert("hl2dm", SDK::new("HL2SDKHL2DM", "7", "HL2DM"));
            map.insert("dods", SDK::new("HL2SDKDODS", "8", "DODS"));
            map.insert("sdk2013", SDK::new("HL2SDK2013", "9", "SDK2013"));
            map.insert("tf2", SDK::new("HL2SDKTF2", "11", "TF2"));
            map.insert("l4d", SDK::new("HL2SDKL4D", "12", "LEFT4DEAD"));
            map.insert("nucleardawn", SDK::new("HL2SDKND", "13", "NUCLEARDAWN"));
            map.insert("l4d2", SDK::new("HL2SDKL4D2", "15", "LEFT4DEAD2"));
            map.insert("darkm", SDK::new("HL2SDK-DARKM", "2", "DARKMESSIAH"));
            map.insert("swarm", SDK::new("HL2SDK-SWARM", "16", "ALIENSWARM"));
            map.insert("bgt", SDK::new("HL2SDK-BGT", "4", "BLOODYGOODTIME"));
            map.insert("eye", SDK::new("HL2SDK-EYE", "5", "EYE"));
            map.insert("csgo", SDK::new("HL2SDKCSGO", "21", "CSGO"));
            map.insert("portal2", SDK::new("HL2SDKPORTAL2", "17", "PORTAL2"));
            map.insert("blade", SDK::new("HL2SDKBLADE", "18", "BLADE"));
            map.insert(
                "insurgency",
                SDK::new("HL2SDKINSURGENCY", "19", "INSURGENCY"),
            );
            map.insert("contagion", SDK::new("HL2SDKCONTAGION", "14", "CONTAGION"));
            map.insert("bms", SDK::new("HL2SDKBMS", "10", "BMS"));
            map.insert("doi", SDK::new("HL2SDKDOI", "20", "DOI"));
            map
        };
    }

    pub fn configure_for_hl2<P: AsRef<Path>>(mm_root: P, config: &mut cpp_build::Config) {
        let mms_path;
        if cfg!(feature = "episode1") {
            mms_path = mm_root.as_ref().join("core-legacy");
        } else {
            mms_path = mm_root.as_ref().join("core");
        }

        config.include(&mms_path);
        config.include(mms_path.join("sourcehook"));

        let mut sdk = None;
        if cfg!(feature = "csgo") {
            sdk = Some(POSSIBLE_SDKS.get("csgo").unwrap())
        }

        let sdk = sdk.unwrap();
        let sdk_path = var(sdk.path).unwrap();
        let sdk_path = Path::new(&sdk_path);
        config.define(format!("SE_{}", sdk.define).as_str(), Some(sdk.code));

        config.include(sdk_path.join("public"));
        config.include(sdk_path.join("public/engine"));
        config.include(sdk_path.join("public/mathlib"));
        config.include(sdk_path.join("public/vstdlib"));
        config.include(sdk_path.join("public/tier0"));
        config.include(sdk_path.join("public/tier1"));

        config.include(sdk_path.join("public/game/server"));
        config.include(sdk_path.join("public/toolframework"));
        config.include(sdk_path.join("game/shared"));
        config.include(sdk_path.join("common"));

        #[cfg(target_env = "msvc")]
        {
            #[cfg(debug_assertions)]
            {
                config.object(sdk_path.join("lib/win32/release/vs2017/libprotobuf.lib"));
            }
            #[cfg(not(debug_assertions))]
            {
                config.object(sdk_path.join("lib/win32/release/vs2017/libprotobuf.lib"));
            }
        }
        #[cfg(target_env = "gnu")]
        {
            /*
            println!(
                "cargo:rustc-link-search=native={}",
                sdk_path.join("lib/linux32/release").to_str().unwrap()
            );
            println!("cargo:rustc-link-lib=static=protobuf");
            // config.object(sdk_path.join("lib/linux32/release/libprotobuf.a"));
            */
            let protobuf_path = sdk_path.join("lib/linux32/release/libprotobuf.a");
            if !protobuf_path.exists() {
                panic!("Protocol Buffers library not found at: {}", protobuf_path.display());
            }
            
            // Add the search path
            println!(
                "cargo:rustc-link-search=native={}",
                sdk_path.join("lib/linux32/release").to_str().unwrap()
            );
            
            // Link statically with whole-archive to ensure all symbols are included
            println!("cargo:rustc-link-arg=-Wl,--whole-archive");
            println!("cargo:rustc-link-lib=static=protobuf");
            println!("cargo:rustc-link-arg=-Wl,--no-whole-archive");
            
            // Add additional dependencies that protobuf might need
            println!("cargo:rustc-link-lib=stdc++");
            println!("cargo:rustc-link-lib=m");
            println!("cargo:rustc-link-lib=pthread");
        }

        config.include(sdk_path.join("common/protobuf-2.5.0/src"));

        #[cfg(target_env = "msvc")]
        {
            config.define("COMPILER_MSVC", None);
            config.define("COMPILER_MSVC32", None);

            println!("cargo:rustc-link-search=native={}", sdk_path.join("lib/public").to_str().unwrap());

            println!("cargo:rustc-link-lib=tier0");
            println!("cargo:rustc-link-lib=tier1");
            println!("cargo:rustc-link-lib=vstdlib");
            println!("cargo:rustc-link-lib=mathlib");
            if cfg!(feature = "csgo") {
                println!("cargo:rustc-link-lib=interfaces");
            }
        }

        #[cfg(target_env = "gnu")]
        {
            config.define("COMPILER_GCC", None);

            config.object(sdk_path.join("lib/linux/libtier0.so"));
            config.object(sdk_path.join("lib/linux/libvstdlib.so"));
            config.object(sdk_path.join("lib/linux32/release/libprotobuf.a"));
        }
        /*
        config.file(sdk_path.join("common/protobuf-2.5.0/src/google/protobuf/message.cc"));
        config.file(sdk_path.join("common/protobuf-2.5.0/src/google/protobuf/descriptor.cc"));
        config.file(sdk_path.join("common/protobuf-2.5.0/src/google/protobuf/descriptor.pb.cc"));
        config.file(sdk_path.join("common/protobuf-2.5.0/src/google/protobuf/io/coded_stream.cc"));
        config.file(sdk_path.join("common/protobuf-2.5.0/src/google/protobuf/extension_set.cc"));
        config.file(sdk_path.join("common/protobuf-2.5.0/src/google/protobuf/generated_message_util.cc"));
        config.file(sdk_path.join("common/protobuf-2.5.0/src/google/protobuf/io/zero_copy_stream.cc"));
        config.file(sdk_path.join("common/protobuf-2.5.0/src/google/protobuf/io/zero_copy_stream_impl_lite.cc"));
        config.file(sdk_path.join("common/protobuf-2.5.0/src/google/protobuf/unknown_field_set.cc"));
        config.file(sdk_path.join("common/protobuf-2.5.0/src/google/protobuf/wire_format_lite.cc"));
        */
        config.file(sdk_path.join("public/engine/protobuf/netmessages.pb.cc"));
    }
}

fn main() {
    let mut config = cpp_build::Config::new();

    let sm_root_string = var("SOURCEMOD18")
        .or_else(|_| var("SOURCEMOD"))
        .or_else(|_| var("SOURCEMOD_DEV"))
        .unwrap();

    let sm_root = Path::new(&sm_root_string);

    #[cfg(debug_assertions)]
    {
        //config.define("DEBUG", None);
        //config.define("_DEBUG", None);
    }
    #[cfg(not(debug_assertions))]
    {
        config.define("NDEBUG", None);
    }

    #[cfg(target_env = "gnu")]
    {
        config.define("stricmp", Some("strcasecmp"));
        config.define("_stricmp", Some("strcasecmp"));
        config.define("_snprintf", Some("snprintf"));
        config.define("_vsnprintf", Some("vsnprintf"));
        config.define("HAVE_STDINT_H", None);
        config.define("GNUC", None);

        config.flag("-pipe");
        config.flag("-fno-strict-aliasing");
        config.flag("-Wall");
        config.flag("-Werror");
        config.flag("-Wno-unused");
        config.flag("-Wno-switch");
        config.flag("-Wno-array-bounds");
        config.flag("-msse");
        config.flag("-m32");
        config.flag("-fvisibility=hidden");

        config.flag("-std=c++14");
        config.flag("-fno-exceptions");
        config.flag("-fno-threadsafe-statics");
        config.flag("-Wno-non-virtual-dtor");
        config.flag("-Wno-overloaded-virtual");
        config.flag("-Wno-ignored-qualifiers");
        config.flag("-Wno-extra");
        config.flag("-fvisibility-inlines-hidden");

        //config.flag_if_supported("-Wno-inconsistent-missing-override");
        config.flag_if_supported("-Wno-narrowing");
        config.flag_if_supported("-Wno-delete-non-virtual-dtor");
        config.flag_if_supported("-Wno-unused-result");
        config.flag_if_supported("-Wno-sized-deallocation");

        //config.flag_if_supported("-Wno-implicit-exception-spec-mismatch");
        //config.flag_if_supported("-Wno-deprecated-register");
        config.flag_if_supported("-Wno-deprecated");
        //config.flag_if_supported("-Wno-sometimes-uninitialized");

        config.flag_if_supported("-mfpmath=sse");
    }
    #[cfg(target_env = "msvc")]
    {
        config.define("_CRT_SECURE_NO_DEPRECATE", None);
        config.define("_CRT_SECURE_NO_WARNINGS", None);
        config.define("_CRT_NONSTDC_NO_DEPRECATE", None);
        config.define("_ITERATOR_DEBUG_LEVEL", Some("0"));

        config.flag("/EHsc");
        config.flag("/GR-");

        config.flag("/Oy-");

        config.object("legacy_stdio_definitions.lib");
    }

    #[cfg(target_os = "windows")]
    {
        config.define("WIN32", None);
        config.define("_WINDOWS", None);
    }
    #[cfg(target_os = "linux")]
    {
        config.define("_LINUX", None);
        config.define("POSIX", None);
        config.flag("-Wl,--exclude-libs,ALL");
        config.flag("-lm");
        config.flag_if_supported("-static-libgcc");
        config.flag_if_supported("-lgcc_eh");
    }
    #[cfg(target_os = "mac")]
    {
        config.define("OSX", None);
        config.define("_OSX", None);
        config.define("POSIX", None);
        config.flag("-mmacosx-version-min=10.5");
        config.flag("-arch=i386");
        config.flag("-lstdc++");
        config.flag("-stdlib=libstdc++");
    }

    config
        .include("sm")
        .include(sm_root.join("public"))
        .include(sm_root.join("public/extensions"))
        .include(sm_root.join("sourcepawn/include"))
        .include(sm_root.join("public/amtl/amtl"))
        .include(sm_root.join("public/amtl"));

    // Check if the file exists in the sm_root path first
    let smsdk_path = sm_root.join("public/smsdk_ext.cpp");
    if !smsdk_path.exists() {
        // Try the deps path as a fallback
        let deps_path = Path::new("deps/sourcemod/public/smsdk_ext.cpp");
        if deps_path.exists() {
            config.file(deps_path);
        } else {
            panic!("Could not find smsdk_ext.cpp in either {} or {}", 
                smsdk_path.display(), deps_path.display());
        }
    } else {
        config.file(smsdk_path);
    }
    config.file("sm/CDetour/detours.cpp");

    let mut c = cc::Build::new();
    #[cfg(target_os = "windows")]
    {
        c.define("WIN32", None);
        c.define("_WINDOWS", None);
    }
    #[cfg(target_os = "linux")]
    {
        c.define("_LINUX", None);
        c.define("POSIX", None);
        c.flag("-Wno-sign-compare");
    }

    c.include("sm");
    c.file("sm/asm/asm.c");
    c.file("sm/libudis86/decode.c");
    c.file("sm/libudis86/itab.c");
    c.file("sm/libudis86/syn.c");
    c.file("sm/libudis86/syn-att.c");
    c.file("sm/libudis86/syn-intel.c");
    c.file("sm/libudis86/udis86.c");

    c.compile("asm");

    println!("cargo:rustc-link-lib=static=asm");

    #[cfg(feature = "metamod")]
    {
        let mm_root_string = var("MMSOURCE110")
            .or_else(|_| var("MMSOURCE"))
            .or_else(|_| var("MMSOURCE_DEV"))
            .unwrap();
        let mm_root = Path::new(&mm_root_string);
        metamod::configure_for_hl2(mm_root, &mut config);
    }

    config.build("src/lib.rs");
}
