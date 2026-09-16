{
  description = "Real-time Network Security Analytics & Alerting System";

  inputs = {
    nixpkgs.url = "nixpkgs";
  };

  outputs = { self, nixpkgs }:
    let
      supportedSystems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
      forEachSupportedSystem = f: nixpkgs.lib.genAttrs supportedSystems (system: f {
        pkgs = import nixpkgs { inherit system; };
      });
    in
    {
      devShells = forEachSupportedSystem ({ pkgs }: {
        default = pkgs.mkShell {
          packages = with pkgs; [
            rustc
            cargo
            rustfmt
            clippy
            rust-analyzer
            pkg-config
            openssl
            libpcap
            sqlx-cli
            trunk
            wasm-bindgen-cli
            lld
            postgresql
            git
          ];

          shellHook = ''
            export LIBPCAP_LIBDIR="${pkgs.libpcap}/lib"
            export PKG_CONFIG_PATH="${pkgs.openssl.dev}/lib/pkgconfig:${pkgs.libpcap}/lib/pkgconfig:$PKG_CONFIG_PATH"
            echo "🛡️ Real-time Network Security Analytics dev environment loaded!"
          '';
        };
      });
    };
}
