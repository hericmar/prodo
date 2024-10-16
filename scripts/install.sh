#!/bin/sh

set -e

PRODO_USER=prodo
PRODO_GROUP=prodo
PRODO_DIR=/usr/share/webapps/prodo
PRODO_CONFIG_DIR=/etc/prodo

# Check if website is built
if [ ! -d "webapp/dist/pwa" ]; then
  echo "Website not built. Please run 'make' first."
  exit 1
fi

# Create prodo user if it doesn't exist
if ! id -u $PRODO_USER > /dev/null 2>&1; then
  echo "Prodo user not found. Creating..."
  adduser -D -H -s /bin/false $PRODO_USER
fi

# Create prodo directory if it doesn't exist
if [ ! -d "$PRODO_DIR" ]; then
  echo "Prodo directory not found. Creating..."
  mkdir -p $PRODO_DIR
  mkdir -p $PRODO_DIR/static
  chown -R $PRODO_USER:$PRODO_GROUP $PRODO_DIR
fi

# Create prodo config directory if it doesn't exist
if [ ! -d "$PRODO_CONFIG_DIR" ]; then
  echo "Prodo config directory not found. Creating..."
  mkdir -p $PRODO_CONFIG_DIR
  cp etc/prodo/* $PRODO_CONFIG_DIR
  chown -R root:$PRODO_GROUP $PRODO_CONFIG_DIR
  chmod -R 750 $PRODO_CONFIG_DIR
fi

# Copy prodo.sh to /usr/bin
echo "Prodo executable not found. Creating..."
cp target/release/prodo /usr/bin/prodo
chown root:$PRODO_GROUP /usr/bin/prodo
chmod 750 /usr/bin/prodo

# If open-rc is installed, copy etc/init.d/prodo to /etc/init.d
if [ -d "/etc/init.d" ]; then
  echo "OpenRC found. Creating init script..."
  cp etc/init.d/prodo /etc/init.d/prodo
  chown root:root /etc/init.d/prodo
  chmod 700 /etc/init.d/prodo
fi

# Create log file if does not exists
if [ ! -f "/var/log/prodo.log" ]; then
  echo "Creating /var/log/prodo.log file..."
  touch /var/log/prodo.log
  chown ${PRODO_GROUP}:${PRODO_USER} /var/log/prodo.log
  chmod 700 /var/log/prodo.log
fi

# Create hourly cron job 'prodo runcrons' for prodo user
echo "Creating cron job..."
echo "0 * * * * prodo cron run" | crontab -u $PRODO_USER -

cp -r webapp/dist/pwa/* $PRODO_DIR/static
