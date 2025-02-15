{ cargo
, makeWrapper
, openssl
, pkg-config
, rustc
, rustPlatform
, bash
, desktop-file-utils
, hicolor-icon-theme
, shared-mime-info
, gtk3
, substitute
}:

rustPlatform.buildRustPackage rec {
  pname = "libxinux-helper";
  version = "0.0.1";

  src = [ ../../libxinux-helper ];

  cargoLock = {
    lockFile = ../../libxinux-helper/Cargo.lock;
  };

  nativeBuildInputs = [
    cargo
    makeWrapper
    pkg-config
    rustc
    rustPlatform.cargoSetupHook
  ];

  buildInputs = [
    desktop-file-utils
    hicolor-icon-theme
    shared-mime-info
    gtk3
  ];

  postInstall = ''
    mv $out/bin $out/libexec
    # add update-icons.trigger
    install -Dm755 ${./update-icons.trigger} $out/share/libxinux/triggers/update-icons.trigger
    substitute ${./update-icons.trigger} $out/share/libxinux/triggers/update-icons.trigger \
      --subst-var-by bash ${bash} \
      --subst-var-by desktop-file-utils ${desktop-file-utils} \
      --subst-var-by hicolor-icon-theme ${hicolor-icon-theme} \
      --subst-var-by shared-mime-info ${shared-mime-info} \
      --subst-var-by gtk3 ${gtk3}
  '';
}
