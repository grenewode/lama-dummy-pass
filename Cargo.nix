# This file was @generated by cargo2nix 0.8.2.
# It is not intended to be manually edited.

{
  release ? true,
  rootFeatures ? [
    "imposter-pass/default"
  ],
  rustPackages,
  buildRustPackages,
  hostPlatform,
  mkRustCrate,
  rustLib,
  lib,
}:
let
  inherit (rustLib) fetchCratesIo fetchCrateLocal fetchCrateGit fetchCrateAlternativeRegistry expandFeatures decideProfile genDrvsByProfile;
  profilesByName = {
  };
  rootFeatures' = expandFeatures rootFeatures;
  overridableMkRustCrate = f:
    let
      drvs = genDrvsByProfile profilesByName ({ profile, profileName }: mkRustCrate ({ inherit release profile; } // (f profileName)));
    in { compileMode ? null, profileName ? decideProfile compileMode release }:
      let drv = drvs.${profileName}; in if compileMode == null then drv else drv.override { inherit compileMode; };
in
{
  cargo2nixVersion = "0.8.2";
  workspace = {
    imposter-pass = rustPackages.unknown.imposter-pass."0.4.0";
  };
  "registry+https://github.com/rust-lang/crates.io-index".ansi_term."0.11.0" = overridableMkRustCrate (profileName: rec {
    name = "ansi_term";
    version = "0.11.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "ee49baf6cb617b853aa8d93bf420db2383fab46d314482ca2803b40d5fde979b"; };
    dependencies = {
      ${ if hostPlatform.parsed.kernel.name == "windows" then "winapi" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".winapi."0.3.9" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".atty."0.2.14" = overridableMkRustCrate (profileName: rec {
    name = "atty";
    version = "0.2.14";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "d9b39be18770d11421cdb1b9947a45dd3f37e93092cbf377614828a319d5fee8"; };
    dependencies = {
      ${ if hostPlatform.parsed.kernel.name == "hermit" then "hermit_abi" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".hermit-abi."0.1.15" { inherit profileName; };
      ${ if hostPlatform.isUnix then "libc" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".libc."0.2.74" { inherit profileName; };
      ${ if hostPlatform.isWindows then "winapi" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".winapi."0.3.9" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".bitflags."1.2.1" = overridableMkRustCrate (profileName: rec {
    name = "bitflags";
    version = "1.2.1";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "cf1de2fe8c75bc145a2f577add951f8134889b4795d47466a54a5c846d691693"; };
    features = builtins.concatLists [
      [ "default" ]
    ];
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".clap."2.33.1" = overridableMkRustCrate (profileName: rec {
    name = "clap";
    version = "2.33.1";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "bdfa80d47f954d53a35a64987ca1422f495b8d6483c0fe9f7117b36c2a792129"; };
    features = builtins.concatLists [
      [ "ansi_term" ]
      [ "atty" ]
      [ "color" ]
      [ "default" ]
      [ "strsim" ]
      [ "suggestions" ]
      [ "vec_map" ]
    ];
    dependencies = {
      ${ if !hostPlatform.isWindows then "ansi_term" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".ansi_term."0.11.0" { inherit profileName; };
      atty = rustPackages."registry+https://github.com/rust-lang/crates.io-index".atty."0.2.14" { inherit profileName; };
      bitflags = rustPackages."registry+https://github.com/rust-lang/crates.io-index".bitflags."1.2.1" { inherit profileName; };
      strsim = rustPackages."registry+https://github.com/rust-lang/crates.io-index".strsim."0.8.0" { inherit profileName; };
      textwrap = rustPackages."registry+https://github.com/rust-lang/crates.io-index".textwrap."0.11.0" { inherit profileName; };
      unicode_width = rustPackages."registry+https://github.com/rust-lang/crates.io-index".unicode-width."0.1.8" { inherit profileName; };
      vec_map = rustPackages."registry+https://github.com/rust-lang/crates.io-index".vec_map."0.8.2" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".heck."0.3.1" = overridableMkRustCrate (profileName: rec {
    name = "heck";
    version = "0.3.1";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "20564e78d53d2bb135c343b3f47714a56af2061f1c928fdb541dc7b9fdd94205"; };
    dependencies = {
      unicode_segmentation = rustPackages."registry+https://github.com/rust-lang/crates.io-index".unicode-segmentation."1.6.0" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".hermit-abi."0.1.15" = overridableMkRustCrate (profileName: rec {
    name = "hermit-abi";
    version = "0.1.15";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "3deed196b6e7f9e44a2ae8d94225d80302d81208b1bb673fd21fe634645c85a9"; };
    features = builtins.concatLists [
      [ "default" ]
    ];
    dependencies = {
      libc = rustPackages."registry+https://github.com/rust-lang/crates.io-index".libc."0.2.74" { inherit profileName; };
    };
  });
  
  "unknown".imposter-pass."0.4.0" = overridableMkRustCrate (profileName: rec {
    name = "imposter-pass";
    version = "0.4.0";
    registry = "unknown";
    src = fetchCrateLocal ./.;
    dependencies = {
      clap = rustPackages."registry+https://github.com/rust-lang/crates.io-index".clap."2.33.1" { inherit profileName; };
      rpassword = rustPackages."registry+https://github.com/rust-lang/crates.io-index".rpassword."4.0.5" { inherit profileName; };
      serde = rustPackages."registry+https://github.com/rust-lang/crates.io-index".serde."1.0.114" { inherit profileName; };
      serde_json = rustPackages."registry+https://github.com/rust-lang/crates.io-index".serde_json."1.0.57" { inherit profileName; };
      structopt = rustPackages."registry+https://github.com/rust-lang/crates.io-index".structopt."0.3.15" { inherit profileName; };
      thiserror = rustPackages."registry+https://github.com/rust-lang/crates.io-index".thiserror."1.0.20" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".itoa."0.4.6" = overridableMkRustCrate (profileName: rec {
    name = "itoa";
    version = "0.4.6";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "dc6f3ad7b9d11a0c00842ff8de1b60ee58661048eb8049ed33c73594f359d7e6"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".lazy_static."1.4.0" = overridableMkRustCrate (profileName: rec {
    name = "lazy_static";
    version = "1.4.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "e2abad23fbc42b3700f2f279844dc832adb2b2eb069b2df918f455c4e18cc646"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".libc."0.2.74" = overridableMkRustCrate (profileName: rec {
    name = "libc";
    version = "0.2.74";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "a2f02823cf78b754822df5f7f268fb59822e7296276d3e069d8e8cb26a14bd10"; };
    features = builtins.concatLists [
      [ "default" ]
      [ "std" ]
    ];
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".proc-macro-error."1.0.4" = overridableMkRustCrate (profileName: rec {
    name = "proc-macro-error";
    version = "1.0.4";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "da25490ff9892aab3fcf7c36f08cfb902dd3e71ca0f9f9517bea02a73a5ce38c"; };
    features = builtins.concatLists [
      [ "default" ]
      [ "syn" ]
      [ "syn-error" ]
    ];
    dependencies = {
      proc_macro_error_attr = buildRustPackages."registry+https://github.com/rust-lang/crates.io-index".proc-macro-error-attr."1.0.4" { profileName = "__noProfile"; };
      proc_macro2 = rustPackages."registry+https://github.com/rust-lang/crates.io-index".proc-macro2."1.0.19" { inherit profileName; };
      quote = rustPackages."registry+https://github.com/rust-lang/crates.io-index".quote."1.0.7" { inherit profileName; };
      syn = rustPackages."registry+https://github.com/rust-lang/crates.io-index".syn."1.0.37" { inherit profileName; };
    };
    buildDependencies = {
      version_check = buildRustPackages."registry+https://github.com/rust-lang/crates.io-index".version_check."0.9.2" { profileName = "__noProfile"; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".proc-macro-error-attr."1.0.4" = overridableMkRustCrate (profileName: rec {
    name = "proc-macro-error-attr";
    version = "1.0.4";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "a1be40180e52ecc98ad80b184934baf3d0d29f979574e439af5a55274b35f869"; };
    dependencies = {
      proc_macro2 = rustPackages."registry+https://github.com/rust-lang/crates.io-index".proc-macro2."1.0.19" { inherit profileName; };
      quote = rustPackages."registry+https://github.com/rust-lang/crates.io-index".quote."1.0.7" { inherit profileName; };
    };
    buildDependencies = {
      version_check = buildRustPackages."registry+https://github.com/rust-lang/crates.io-index".version_check."0.9.2" { profileName = "__noProfile"; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".proc-macro2."1.0.19" = overridableMkRustCrate (profileName: rec {
    name = "proc-macro2";
    version = "1.0.19";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "04f5f085b5d71e2188cb8271e5da0161ad52c3f227a661a3c135fdf28e258b12"; };
    features = builtins.concatLists [
      [ "default" ]
      [ "proc-macro" ]
    ];
    dependencies = {
      unicode_xid = rustPackages."registry+https://github.com/rust-lang/crates.io-index".unicode-xid."0.2.1" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".quote."1.0.7" = overridableMkRustCrate (profileName: rec {
    name = "quote";
    version = "1.0.7";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "aa563d17ecb180e500da1cfd2b028310ac758de548efdd203e18f283af693f37"; };
    features = builtins.concatLists [
      [ "default" ]
      [ "proc-macro" ]
    ];
    dependencies = {
      proc_macro2 = rustPackages."registry+https://github.com/rust-lang/crates.io-index".proc-macro2."1.0.19" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".rpassword."4.0.5" = overridableMkRustCrate (profileName: rec {
    name = "rpassword";
    version = "4.0.5";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "99371657d3c8e4d816fb6221db98fa408242b0b53bac08f8676a41f8554fe99f"; };
    dependencies = {
      ${ if hostPlatform.isUnix then "libc" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".libc."0.2.74" { inherit profileName; };
      ${ if hostPlatform.isWindows then "winapi" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".winapi."0.3.9" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".ryu."1.0.5" = overridableMkRustCrate (profileName: rec {
    name = "ryu";
    version = "1.0.5";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "71d301d4193d031abdd79ff7e3dd721168a9572ef3fe51a1517aba235bd8f86e"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".serde."1.0.114" = overridableMkRustCrate (profileName: rec {
    name = "serde";
    version = "1.0.114";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "5317f7588f0a5078ee60ef675ef96735a1442132dc645eb1d12c018620ed8cd3"; };
    features = builtins.concatLists [
      [ "default" ]
      [ "derive" ]
      [ "serde_derive" ]
      [ "std" ]
    ];
    dependencies = {
      serde_derive = buildRustPackages."registry+https://github.com/rust-lang/crates.io-index".serde_derive."1.0.114" { profileName = "__noProfile"; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".serde_derive."1.0.114" = overridableMkRustCrate (profileName: rec {
    name = "serde_derive";
    version = "1.0.114";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "2a0be94b04690fbaed37cddffc5c134bf537c8e3329d53e982fe04c374978f8e"; };
    features = builtins.concatLists [
      [ "default" ]
    ];
    dependencies = {
      proc_macro2 = rustPackages."registry+https://github.com/rust-lang/crates.io-index".proc-macro2."1.0.19" { inherit profileName; };
      quote = rustPackages."registry+https://github.com/rust-lang/crates.io-index".quote."1.0.7" { inherit profileName; };
      syn = rustPackages."registry+https://github.com/rust-lang/crates.io-index".syn."1.0.37" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".serde_json."1.0.57" = overridableMkRustCrate (profileName: rec {
    name = "serde_json";
    version = "1.0.57";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "164eacbdb13512ec2745fb09d51fd5b22b0d65ed294a1dcf7285a360c80a675c"; };
    features = builtins.concatLists [
      [ "default" ]
      [ "std" ]
    ];
    dependencies = {
      itoa = rustPackages."registry+https://github.com/rust-lang/crates.io-index".itoa."0.4.6" { inherit profileName; };
      ryu = rustPackages."registry+https://github.com/rust-lang/crates.io-index".ryu."1.0.5" { inherit profileName; };
      serde = rustPackages."registry+https://github.com/rust-lang/crates.io-index".serde."1.0.114" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".strsim."0.8.0" = overridableMkRustCrate (profileName: rec {
    name = "strsim";
    version = "0.8.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "8ea5119cdb4c55b55d432abb513a0429384878c15dde60cc77b1c99de1a95a6a"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".structopt."0.3.15" = overridableMkRustCrate (profileName: rec {
    name = "structopt";
    version = "0.3.15";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "de2f5e239ee807089b62adce73e48c625e0ed80df02c7ab3f068f5db5281065c"; };
    features = builtins.concatLists [
      [ "default" ]
    ];
    dependencies = {
      clap = rustPackages."registry+https://github.com/rust-lang/crates.io-index".clap."2.33.1" { inherit profileName; };
      lazy_static = rustPackages."registry+https://github.com/rust-lang/crates.io-index".lazy_static."1.4.0" { inherit profileName; };
      structopt_derive = buildRustPackages."registry+https://github.com/rust-lang/crates.io-index".structopt-derive."0.4.8" { profileName = "__noProfile"; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".structopt-derive."0.4.8" = overridableMkRustCrate (profileName: rec {
    name = "structopt-derive";
    version = "0.4.8";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "510413f9de616762a4fbeab62509bf15c729603b72d7cd71280fbca431b1c118"; };
    dependencies = {
      heck = rustPackages."registry+https://github.com/rust-lang/crates.io-index".heck."0.3.1" { inherit profileName; };
      proc_macro_error = rustPackages."registry+https://github.com/rust-lang/crates.io-index".proc-macro-error."1.0.4" { inherit profileName; };
      proc_macro2 = rustPackages."registry+https://github.com/rust-lang/crates.io-index".proc-macro2."1.0.19" { inherit profileName; };
      quote = rustPackages."registry+https://github.com/rust-lang/crates.io-index".quote."1.0.7" { inherit profileName; };
      syn = rustPackages."registry+https://github.com/rust-lang/crates.io-index".syn."1.0.37" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".syn."1.0.37" = overridableMkRustCrate (profileName: rec {
    name = "syn";
    version = "1.0.37";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "239f255b9e3429350f188c27b807fc9920a15eb9145230ff1a7d054c08fec319"; };
    features = builtins.concatLists [
      [ "clone-impls" ]
      [ "default" ]
      [ "derive" ]
      [ "full" ]
      [ "parsing" ]
      [ "printing" ]
      [ "proc-macro" ]
      [ "quote" ]
      [ "visit" ]
    ];
    dependencies = {
      proc_macro2 = rustPackages."registry+https://github.com/rust-lang/crates.io-index".proc-macro2."1.0.19" { inherit profileName; };
      quote = rustPackages."registry+https://github.com/rust-lang/crates.io-index".quote."1.0.7" { inherit profileName; };
      unicode_xid = rustPackages."registry+https://github.com/rust-lang/crates.io-index".unicode-xid."0.2.1" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".textwrap."0.11.0" = overridableMkRustCrate (profileName: rec {
    name = "textwrap";
    version = "0.11.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "d326610f408c7a4eb6f51c37c330e496b08506c9457c9d34287ecc38809fb060"; };
    dependencies = {
      unicode_width = rustPackages."registry+https://github.com/rust-lang/crates.io-index".unicode-width."0.1.8" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".thiserror."1.0.20" = overridableMkRustCrate (profileName: rec {
    name = "thiserror";
    version = "1.0.20";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "7dfdd070ccd8ccb78f4ad66bf1982dc37f620ef696c6b5028fe2ed83dd3d0d08"; };
    dependencies = {
      thiserror_impl = buildRustPackages."registry+https://github.com/rust-lang/crates.io-index".thiserror-impl."1.0.20" { profileName = "__noProfile"; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".thiserror-impl."1.0.20" = overridableMkRustCrate (profileName: rec {
    name = "thiserror-impl";
    version = "1.0.20";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "bd80fc12f73063ac132ac92aceea36734f04a1d93c1240c6944e23a3b8841793"; };
    dependencies = {
      proc_macro2 = rustPackages."registry+https://github.com/rust-lang/crates.io-index".proc-macro2."1.0.19" { inherit profileName; };
      quote = rustPackages."registry+https://github.com/rust-lang/crates.io-index".quote."1.0.7" { inherit profileName; };
      syn = rustPackages."registry+https://github.com/rust-lang/crates.io-index".syn."1.0.37" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".unicode-segmentation."1.6.0" = overridableMkRustCrate (profileName: rec {
    name = "unicode-segmentation";
    version = "1.6.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "e83e153d1053cbb5a118eeff7fd5be06ed99153f00dbcd8ae310c5fb2b22edc0"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".unicode-width."0.1.8" = overridableMkRustCrate (profileName: rec {
    name = "unicode-width";
    version = "0.1.8";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "9337591893a19b88d8d87f2cec1e73fad5cdfd10e5a6f349f498ad6ea2ffb1e3"; };
    features = builtins.concatLists [
      [ "default" ]
    ];
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".unicode-xid."0.2.1" = overridableMkRustCrate (profileName: rec {
    name = "unicode-xid";
    version = "0.2.1";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "f7fe0bb3479651439c9112f72b6c505038574c9fbb575ed1bf3b797fa39dd564"; };
    features = builtins.concatLists [
      [ "default" ]
    ];
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".vec_map."0.8.2" = overridableMkRustCrate (profileName: rec {
    name = "vec_map";
    version = "0.8.2";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "f1bddf1187be692e79c5ffeab891132dfb0f236ed36a43c7ed39f1165ee20191"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".version_check."0.9.2" = overridableMkRustCrate (profileName: rec {
    name = "version_check";
    version = "0.9.2";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "b5a972e5669d67ba988ce3dc826706fb0a8b01471c088cb0b6110b805cc36aed"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".winapi."0.3.9" = overridableMkRustCrate (profileName: rec {
    name = "winapi";
    version = "0.3.9";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "5c839a674fcd7a98952e593242ea400abe93992746761e38641405d28b00f419"; };
    features = builtins.concatLists [
      [ "consoleapi" ]
      [ "errhandlingapi" ]
      [ "fileapi" ]
      [ "handleapi" ]
      [ "minwinbase" ]
      [ "minwindef" ]
      [ "processenv" ]
      [ "std" ]
      [ "winbase" ]
      [ "wincon" ]
      [ "winnt" ]
    ];
    dependencies = {
      ${ if hostPlatform.config == "i686-pc-windows-gnu" || hostPlatform.config == "i686-pc-windows-gnu" || hostPlatform.config == "i686-pc-windows-gnu" then "winapi_i686_pc_windows_gnu" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".winapi-i686-pc-windows-gnu."0.4.0" { inherit profileName; };
      ${ if hostPlatform.config == "x86_64-pc-windows-gnu" || hostPlatform.config == "x86_64-pc-windows-gnu" || hostPlatform.config == "x86_64-pc-windows-gnu" then "winapi_x86_64_pc_windows_gnu" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".winapi-x86_64-pc-windows-gnu."0.4.0" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".winapi-i686-pc-windows-gnu."0.4.0" = overridableMkRustCrate (profileName: rec {
    name = "winapi-i686-pc-windows-gnu";
    version = "0.4.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "ac3b87c63620426dd9b991e5ce0329eff545bccbbb34f3be09ff6fb6ab51b7b6"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".winapi-x86_64-pc-windows-gnu."0.4.0" = overridableMkRustCrate (profileName: rec {
    name = "winapi-x86_64-pc-windows-gnu";
    version = "0.4.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "712e227841d057c1ee1cd2fb22fa7e5a5461ae8e48fa2ca79ec42cfc1931183f"; };
  });
  
}
