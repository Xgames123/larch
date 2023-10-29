#usage: install [-u]

if [ "$1" = "-u" ] ; then
  rm -f /usr/local/bin/mccasm
  exit 0
fi

if ! cargo build --release ; then
  echo "Build failed"
  echo "NOTE: Not don't run this as root"
  exit -1
fi

if ! sudo cp target/release/mccasm /usr/local/bin/mccasm ; then
  exit -1
fi

if ! sudo chmod +x /usr/local/bin/mccasm ; then
  exit -1
fi
