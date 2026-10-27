# GUIDE FOR ADMINISTRATORS

This implementation of the Valkyrie API server is meant to be used via
Docker Compose. The Docker Compose file only has two containers running:
A PostgreSQL server and the Valkyrie API server itself. A minimal `docker-compose.yml`
file could look something as is illustrated in the code snippet below.
The section below this code snippet explains what each of the
configuration variables present in the section for the Valkyrie API
server container mean and what they each control.

```YAML
services:
  database:
    image: postgres:18-alpine
    container_name: database
    environment:
      POSTGRES_USER: valkyrie
      POSTGRES_PASSWORD: PV18123456
      POSTGRES_DB: valkyrie
    volumes:
      - postgres_data:/var/lib/postgresql/
    ports:
      - "4000:5432"
    restart: unless-stopped
  app:
    build: .
    container_name: valkyrie
    environment:
      - VALKYRIE_USE_SSL=NO
      - VALKYRIE_INSTANCE_IP=0.0.0.0
      - VALKYRIE_INSTANCE_PORT=3000
      - VALYKRIE_INSTANCE_DOMAIN=localhost:3000
      - VALKYRIE_INSTANCE_NAME=Valkyrie Local
      - VALKYRIE_INSTANCE_DESCRIPTION=Your instance description
      - VALKYRIE_INSTANCE_POSTGRES_URL=postgres://valkyrie:PV18123456@database:5432/valkyrie
      - VALYKRIE_INSTANCE_ADMIN_USERNAME=your_admin_handle
      - VALKYRIE_INSTANCE_ADMIN_PASSWORD=your_admin_password
      - VALKYRIE_INSTANCE_ADMIN_DISPLAY_NAME=Administrator
      - VALKYRIE_INSTANCE_ADMIN_DESCRIPTION=Your Instance Admin description
    ports:
      - "3000:3000"
    depends_on:
      - database
    restart: unless-stopped

volumes:
  postgres_data:
```

- `VALKYRIE_USE_SSL`: This environment variable controls whether the current instance of the Valkyrie server should make its own data accessible over HTTPS. This variable can be set to either `YES` or `NO`.
- `VALKYRIE_INSTANCE_IP`: The environment variable representing the *internal* IPv4 address the Valkyrie server is served on.
- `VALKYRIE_INSTANCE_PORT`: The environment variable representing the *internal* port the Valkyrie server is served on.
- `VALYKRIE_INSTANCE_DOMAIN`: The environment variable containing the outward-facing domain that the Valkyrie server is accessible on.
- `VALKYRIE_INSTANCE_NAME`: The environment variable representing the current instance's name. This can be changed later via the dedicated API service for carrying out this task.
- `VALKYRIE_INSTANCE_DESCRIPTION`: The environment variable representing the current instance's description. This can be changed later via the dedicated API service for carrying out this task.
- `VALKYRIE_INSTANCE_POSTGRES_URL`: The environment variable containing the URL to the PostgreSQL database for saving data.
- `VALYKRIE_INSTANCE_ADMIN_USERNAME`: The environment variable representing the username of the instance's administrator. This cannot be changed.
- `VALKYRIE_INSTANCE_ADMIN_PASSWORD`: The environment variable representing the admininstrator user's password.  This can be changed later via the dedicated API service for carrying out this task.
- `VALKYRIE_INSTANCE_ADMIN_DISPLAY_NAME`: The environment variable representing the admininstrator user's display name.  This can be changed later via the dedicated API service for carrying out this task.
- `VALKYRIE_INSTANCE_ADMIN_DESCRIPTION`: The environment variable representing the admininstrator user's description.  This can be changed later via the dedicated API service for carrying out this task.


