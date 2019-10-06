#!/usr/bin/env nix-shell
with import <nixpkgs> {};
stdenv.mkDerivation {
    name = "SpamWatchAPI";
    buildInputs = [ postgresql ];
    shellHook =
  ''
    export PGDATA=~/main/db/postgres
    mkdir -p $PGDATA
    initdb
    pg_ctl -l /tmp/postgres.log start
    tail -f /tmp/postgres.log
  '';
}

