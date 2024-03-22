#usage: install [-u]

if [ "$(whoami)" = "root" ] ; then
  echo "Don't run as root"
  exit 1
fi

if [ "$1" = "-u" ] ; then
  rm -f /usr/local/bin/mccasm
  rm -f /usr/local/bin/mccemu
  exit 0
fi

if ! cargo build --release --bin mccemu --bin mccasm ; then
  echo "Build failed"
  exit 1
fi

if ! sudo cp target/release/mccasm /usr/local/bin/mccasm ; then
  exit 1
fi
if ! sudo cp target/release/mccemu /usr/local/bin/mccemu ; then
  exit 1
fi

if ! sudo chmod +x /usr/local/bin/mccasm ; then
  exit 1
fi
if ! sudo chmod +x /usr/local/bin/mccemu ; then
  exit 1
fi
