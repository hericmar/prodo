# Prodo
Simple web application for managing todo lists with export to calendar using iCalendar format.

Application is available at `prodo.phire.cz`.

## Installation
Build and install application using `make`:
```bash
make && make install
```

Create database and collect static files into `static` directory:
```bash
prodo migrate
prodo collectstatic
```

## Development
```bash
# Create virtual environment
python -m venv venv
# Activate virtual environment
source venv/bin/activate
# Install dependencies
pip install -r requirements.txt

# Run development server
DEBUG=True python manage.py runserver
```

## References
- RFC 5545 - Internet Calendaring and Scheduling Core Object Specification (iCalendar) - https://tools.ietf.org/html/rfc5545
