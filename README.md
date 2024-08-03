# Prodo
Simple web application for managing todo lists with export to calendar using
iCalendar format.

## Installation
Build and install application using `make`:
```bash
make && make install
```

## Development
```bash
cp .env.example .env
```

```bash
# Run development database
docker-compose up -d database
```

### See also
- [Material Design Icons](https://fonts.google.com/icons/)

## References
- RFC 5545 - Internet Calendaring and Scheduling Core Object Specification (iCalendar) - https://tools.ietf.org/html/rfc5545
