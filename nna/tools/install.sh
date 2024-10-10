#usage: install [-u]

if [ "$(whoami)" = "root" ] ; then
  echo "Don't run as root"
  exit 1
fi

tools=(nnaasm)

if [ "$1" = "-u" ] ; then
  for tool in tools ; do
    sudo rm -f /usr/local/bin/$tool
  done
  exit 0
fi


for tool in tools ; do
  if ! cargo build --release --bin $tool ; then
    echo "Build failed"
    exit 1
  fi
  if ! sudo cp target/release/$tool /usr/local/bin/$tool ; then
    exit 1
  fi
  if ! sudo chmod +x /usr/local/bin/$tool ; then
    exit 1
  fi
done
