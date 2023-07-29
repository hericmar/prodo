#!/bin/sh

set -e

PRODO_USER=prodo
PRODO_GROUP=prodo
PRODO_DIR=/usr/share/webapps/prodo
PRODO_CONFIG_DIR=/etc/prodo

# Check if website is built
if [ ! -d "static/assets" ]; then
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
cp bin/prodo.sh /usr/bin/prodo
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
  chown prodo:prodo /var/log/prodo.log
  chmod 700 /var/log/prodo.log
fi

# Create python virtual environment if it doesn't exist
if [ ! -d "$PRODO_DIR/venv" ]; then
  echo "Prodo virtual environment not found. Creating..."
  python -m venv $PRODO_DIR/venv
  chown -R $PRODO_USER:$PRODO_GROUP $PRODO_DIR/venv
fi

# Copy files to prodo directory
echo "Copying files to Prodo directory..."
cp -r base $PRODO_DIR
cp -r ical $PRODO_DIR
cp -r prodo $PRODO_DIR
cp -r tasks $PRODO_DIR
cp -r users $PRODO_DIR
cp -r static $PRODO_DIR

cp manage.py $PRODO_DIR
cp requirements.txt $PRODO_DIR

# Activate virtual environment
source $PRODO_DIR/venv/bin/activate

# Install dependencies
echo "Installing dependencies..."
pip install -r $PRODO_DIR/requirements.txt

# Create hourly cron job 'prodo runcrons' for prodo user
echo "Creating cron job..."
echo "0 * * * * prodo runcrons >> /var/log/prodo.log 2>&1" | crontab -u $PRODO_USER -
