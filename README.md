# Prodo
Simple web application for managing todo lists with export to calendar using
iCalendar format.

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
cp .env.example .env
```

```bash
# Run development database
docker-compose up -d database
```

## References
- RFC 5545 - Internet Calendaring and Scheduling Core Object Specification (iCalendar) - https://tools.ietf.org/html/rfc5545
