#!/bin/sh

set -e

PRODO_USER=prodo
PRODO_GROUP=prodo
PRODO_DIR=/usr/share/webapps/prodo
PRODO_CONFIG_DIR=/etc/prodo

echo "Installing Prodo..."

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
  chown -R root:$PRODO_GROUP $PRODO_CONFIG_DIR
  chmod -R 750 $PRODO_CONFIG_DIR
fi

# Copy prodo.sh to /usr/bin if it doesn't exist
if [ ! -f "/usr/bin/prodo" ]; then
  echo "Prodo executable not found. Creating..."
  cp bin/prodo.sh /usr/bin/prodo
  chown root:$PRODO_GROUP /usr/bin/prodo
  chmod 750 /usr/bin/prodo
fi

# Create python virtual environment if it doesn't exist
if [ ! -d "$PRODO_DIR/venv" ]; then
  echo "Prodo virtual environment not found. Creating..."
  python -m venv $PRODO_DIR/venv
  chown -R $PRODO_USER:$PRODO_GROUP $PRODO_DIR/venv
fi

# Copy files to prodo directory
echo "Copying files to Prodo directory..."
cp -r . $PRODO_DIR

# Activate virtual environment
source $PRODO_DIR/venv/bin/activate

# Install dependencies
pip install -r $PRODO_DIR/requirements.txt

# Build frontend
cd $PRODO_DIR/webapp
pnpm install
pnpm build
