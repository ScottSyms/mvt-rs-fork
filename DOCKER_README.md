# MVT-Server Docker Setup

Este proyecto incluye configuración de Docker para ejecutar MVT-RS junto a Redis.

## Requisitos

- Docker
- Docker Compose

## Estructura de archivos

```
mvt-rs/
├── Dockerfile                # Imagen multi-stage para MVT-RS
├── docker-compose.yml       # Orquestación de servicios
├── .dockerignore           # Archivos excluidos del build
├── config/                 # Configuraciones de la aplicación
├── cache/                  # Directorio de caché
├── map_assets/            # Recursos de mapas
└── DOCKER_README.md       # Este archivo
```

## Servicios incluidos

### 1. mvt-server
- **Puerto**: 5880
- **Descripción**: Aplicación principal MVT-RS
- **Dependencias**: Redis

### 2. redis
- **Puerto**: 6379
- **Imagen**: redis:7-alpine
- **Funcionalidad**: Caché para tiles

### 4. adminer (opcional)
- **Puerto**: 8080
- **Funcionalidad**: Administrador web de base de datos

## Comandos básicos

### Iniciar todos los servicios
```bash
docker-compose up -d
```

### Ver logs
```bash
# Todos los servicios
docker-compose logs -f

# Solo MVT-RS
docker-compose logs -f mvt-server

```

### Detener servicios
```bash
docker-compose down
```

### Reconstruir la aplicación
```bash
docker-compose build --no-cache mvt-server
docker-compose up -d
```

### Acceder al contenedor
```bash
# MVT-RS
docker-compose exec mvt-server bash

```

## Configuración

### Variables de entorno importantes

Las siguientes variables están configuradas en `docker-compose.yml`:

```yaml
environment:
  # Redis (opcional)
  REDISCONN: "redis://redis:6379"
  
  # Seguridad (¡CAMBIAR EN PRODUCCIÓN!)
  JWTSECRET: "supersecretjwt-changeme-in-production"
  SESSIONSECRET: "supersecretsession-changeme-in-production"
```

### Personalización

Para personalizar la configuración:

1. **Crear archivo `.env`** (opcional):
```bash
# Copiar el ejemplo
cp .env.example .env
# Editar con tus valores
```

2. **Modificar docker-compose.yml** para usar el archivo `.env`:
```yaml
env_file:
  - .env
```

## Acceso a los servicios

Una vez iniciados los servicios:

- **MVT-RS**: http://localhost:5880
- **Adminer**: http://localhost:8080
- **Redis**: localhost:6379

## Configuración de datos geoespaciales


- `redis_data`: Datos de Redis
- `./config`: Configuraciones de MVT-RS
- `./cache`: Caché de tiles
- `./map_assets`: Recursos de mapas

## Problemas comunes

### 1. Puerto ocupado
```bash
# Verificar puertos en uso
netstat -tlnp | grep :5880

# Cambiar puerto en docker-compose.yml
ports:
  - "5881:5880"  # Cambiar 5880 por otro puerto
```

### 2. Problemas de permisos
```bash
# Dar permisos a directorios
chmod -R 755 config cache map_assets
```

### 3. Reiniciar servicios
```bash
# Reiniciar solo MVT-RS
docker-compose restart mvt-server

# Reiniciar todo
docker-compose restart
```

## Producción

Para producción, asegúrate de:

1. **Cambiar secretos** en `docker-compose.yml`
2. **Usar variables de entorno** en lugar de valores hardcodeados
3. **Configurar proxy reverso** (nginx, traefik)
4. **Habilitar SSL/TLS**
5. **Configurar backups** de Redis
6. **Monitorear recursos** y logs

### Ejemplo de producción con nginx

```yaml
# docker-compose.prod.yml
version: '3.8'

services:
  mvt-server:
    build: .
    environment:
      JWTSECRET: "${JWT_SECRET}"
      SESSIONSECRET: "${SESSION_SECRET}"
    networks:
      - internal
    # No exponer puertos directamente
    
  nginx:
    image: nginx:alpine
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf:ro
      - ./certs:/etc/nginx/certs:ro
    depends_on:
      - mvt-server
    networks:
      - internal

networks:
  internal:
    driver: bridge
```

## Soporte

Si tienes problemas:

1. Revisa los logs: `docker-compose logs -f`
2. Verifica el estado: `docker-compose ps`
3. Consulta la documentación original del proyecto
4. Abre un issue en el repositorio del proyecto
