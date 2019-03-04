{ stdenv, fetchFromGitHub, openssl, pkgconfig }:

let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  pkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
  rustChannel = pkgs.rustChannelOf {
     date = "2019-02-27";
     channel = "nightly";
  };
  rustPlatform = pkgs.recurseIntoAttrs (pkgs.makeRustPlatform {
    rustc = rustChannel.rustc;
    cargo = rustChannel.cargo;
  });

in rustPlatform.buildRustPackage rec {
  version = "0.5.0";
  name = "openfairdb-${version}";
  src = fetchFromGitHub {
    owner = "slowtec";
    repo = "openfairdb";
    rev = "v${version}";
    sha256 = "1h1s0ymnqqm3s8dww6bj7ar60nk5q3bh94vdwslq7h3z5sm0ahg8";
  };

  buildInputs = with pkgs; [ openssl pkgconfig ];

  cargoSha256 = "1yans6j134z1mzgjz9sz3ajqsbx775a9jzscvq39sbysmgrzzijh";

  meta = with stdenv.lib; {
    description = "Mapping for Good";
    homepage = http://www.openfairdb.org;
    license = with licenses; [ agpl3 ];
    maintainers = [ maintainers.flosse ];
  };
}
