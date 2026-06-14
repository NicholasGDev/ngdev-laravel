// ─── Docker Templates — Laravel PHP 8.3 Multi-DB ─────────────────────────────

// ─────────────────────────────────────────────────────────────────────────────
// Dockerfile DEV
// PHP 8.3-fpm · todas as extensões · Xdebug · Composer · Node.js · multi-DB
// ─────────────────────────────────────────────────────────────────────────────
pub fn dockerfile_dev(php_version: &str, node_version: u8, databases: &[String]) -> String {
    let sqlsrv_block = if databases.iter().any(|d| d == "sqlserver") {
        r#"
# ── Microsoft SQL Server ODBC driver ────────────────────────────────────────
RUN curl -sSL https://packages.microsoft.com/keys/microsoft.asc | gpg --dearmor \
        -o /usr/share/keyrings/microsoft-archive-keyring.gpg \
    && echo "deb [arch=amd64 signed-by=/usr/share/keyrings/microsoft-archive-keyring.gpg] \
        https://packages.microsoft.com/debian/12/prod bookworm main" \
        > /etc/apt/sources.list.d/mssql-release.list \
    && apt-get update \
    && ACCEPT_EULA=Y apt-get install -y --no-install-recommends \
        msodbcsql18 \
        mssql-tools18 \
        unixodbc-dev \
    && pecl install sqlsrv pdo_sqlsrv \
    && docker-php-ext-enable sqlsrv pdo_sqlsrv
"#
    } else {
        ""
    };

    let pgsql_ext = if databases.iter().any(|d| d == "pgsql") {
        "docker-php-ext-install pdo_pgsql pgsql \\\n    && "
    } else {
        ""
    };

    let mysql_ext = if databases.iter().any(|d| d == "mysql" || d == "mariadb") {
        "docker-php-ext-install pdo_mysql mysqli \\\n    && "
    } else {
        ""
    };

    format!(
        r#"# =============================================================================
# Dockerfile.dev — Laravel {php_version} DEV
# PHP {php_version}-fpm | Xdebug | Composer | Node {node_version} | Multi-DB
# =============================================================================
FROM php:{php_version}-fpm-bookworm AS dev

LABEL maintainer="Ng Development <caronte@example.com>"
LABEL description="Laravel DEV image - PHP {php_version} com todas as extensoes"

# ── Argumentos de build ───────────────────────────────────────────────────────
ARG USER_ID=1000
ARG GROUP_ID=1000
ARG NODE_VERSION={node_version}

# ── Dependencias de sistema ───────────────────────────────────────────────────
RUN apt-get update && apt-get install -y --no-install-recommends \
    # Build essentials
    build-essential \
    git \
    curl \
    wget \
    gnupg \
    ca-certificates \
    lsb-release \
    # Libs para extensoes PHP
    libpng-dev \
    libjpeg62-turbo-dev \
    libfreetype6-dev \
    libwebp-dev \
    libavif-dev \
    libzip-dev \
    libicu-dev \
    libxml2-dev \
    libxslt1-dev \
    libssl-dev \
    libreadline-dev \
    libbz2-dev \
    libgmp-dev \
    libmagickwand-dev \
    libmemcached-dev \
    libonig-dev \
    libtidy-dev \
    libldap2-dev \
    libsasl2-dev \
    # PostgreSQL client
    libpq-dev \
    # SQLite
    libsqlite3-dev \
    # Ferramentas
    unzip \
    zip \
    vim \
    nano \
    htop \
    supervisor \
    && rm -rf /var/lib/apt/lists/*

# ── Extensoes PHP core ────────────────────────────────────────────────────────
RUN docker-php-ext-configure gd \
        --with-freetype \
        --with-jpeg \
        --with-webp \
    && docker-php-ext-install -j"$(nproc)" \
        bcmath \
        bz2 \
        calendar \
        ctype \
        dba \
        dom \
        exif \
        fileinfo \
        ftp \
        gd \
        gettext \
        gmp \
        iconv \
        intl \
        mbstring \
        opcache \
        pcntl \
        pdo \
        pdo_sqlite \
        posix \
        simplexml \
        sockets \
        sysvmsg \
        sysvsem \
        sysvshm \
        tidy \
        tokenizer \
        xml \
        xmlreader \
        xmlwriter \
        xsl \
        zip

# ── Extensoes de banco de dados ───────────────────────────────────────────────
RUN {mysql_ext}{pgsql_ext}echo "DB extensions installed"
{sqlsrv_block}
# ── Extensoes via PECL ────────────────────────────────────────────────────────
RUN pecl channel-update pecl.php.net \
    && pecl install \
        redis \
        igbinary \
        imagick \
        memcached \
        uuid \
        xdebug \
    && docker-php-ext-enable \
        redis \
        igbinary \
        imagick \
        memcached \
        uuid \
        xdebug

# ── Composer ──────────────────────────────────────────────────────────────────
COPY --from=composer:latest /usr/bin/composer /usr/bin/composer
ENV COMPOSER_HOME=/tmp/composer
ENV COMPOSER_ALLOW_SUPERUSER=1

# ── Node.js (via NodeSource) ──────────────────────────────────────────────────
RUN curl -fsSL https://deb.nodesource.com/setup_${{NODE_VERSION}}.x | bash - \
    && apt-get install -y nodejs \
    && npm install -g npm@latest

# ── Configs PHP ───────────────────────────────────────────────────────────────
COPY docker/php/php-dev.ini      $PHP_INI_DIR/conf.d/99-app-dev.ini
COPY docker/php/xdebug.ini       $PHP_INI_DIR/conf.d/99-xdebug.ini
COPY docker/php/www.conf         /usr/local/etc/php-fpm.d/www.conf

# ── Usuário não-root ──────────────────────────────────────────────────────────
RUN groupmod -o -g "${{GROUP_ID}}" www-data \
    && usermod  -o -u "${{USER_ID}}" www-data \
    && mkdir -p /var/www/html/storage/logs \
    && chown -R www-data:www-data /var/www

WORKDIR /var/www/html
USER www-data

EXPOSE 9000
CMD ["php-fpm"]
"#,
        php_version = php_version,
        node_version = node_version,
        mysql_ext = mysql_ext,
        pgsql_ext = pgsql_ext,
        sqlsrv_block = sqlsrv_block,
    )
}

// ─────────────────────────────────────────────────────────────────────────────
// Dockerfile PROD — multi-stage, Alpine, sem devtools
// ─────────────────────────────────────────────────────────────────────────────
pub fn dockerfile_prod(php_version: &str, databases: &[String]) -> String {
    let sqlsrv_block = if databases.iter().any(|d| d == "sqlserver") {
        r#"
# ── SQL Server (ODBC) ────────────────────────────────────────────────────────
RUN curl -sSL https://packages.microsoft.com/keys/microsoft.asc | gpg --dearmor \
        -o /usr/share/keyrings/microsoft-archive-keyring.gpg \
    && echo "deb [arch=amd64 signed-by=/usr/share/keyrings/microsoft-archive-keyring.gpg] \
        https://packages.microsoft.com/debian/12/prod bookworm main" \
        > /etc/apt/sources.list.d/mssql-release.list \
    && apt-get update \
    && ACCEPT_EULA=Y apt-get install -y --no-install-recommends \
        msodbcsql18 \
        unixodbc-dev \
    && pecl install sqlsrv pdo_sqlsrv \
    && docker-php-ext-enable sqlsrv pdo_sqlsrv \
    && rm -rf /var/lib/apt/lists/*
"#
    } else {
        ""
    };

    let pgsql_ext = if databases.iter().any(|d| d == "pgsql") {
        "        pdo_pgsql \\\n        pgsql \\\n"
    } else {
        ""
    };

    let mysql_ext = if databases.iter().any(|d| d == "mysql" || d == "mariadb") {
        "        pdo_mysql \\\n        mysqli \\\n"
    } else {
        ""
    };

    format!(
        r#"# =============================================================================
# Dockerfile.prod — Laravel {php_version} PROD
# Multi-stage build: builder → runner Alpine
# =============================================================================

# ── Stage 1: Dependencias Composer ───────────────────────────────────────────
FROM composer:latest AS composer-deps

WORKDIR /app
COPY composer.json composer.lock ./
RUN composer install \
    --no-dev \
    --no-scripts \
    --no-autoloader \
    --no-interaction \
    --prefer-dist

COPY . .
RUN composer dump-autoload --optimize --no-dev

# ── Stage 2: Assets Node.js ───────────────────────────────────────────────────
FROM node:22-alpine AS node-assets

WORKDIR /app
COPY package.json package-lock.json* ./
RUN npm ci --omit=dev

COPY . .
RUN npm run build 2>/dev/null || echo "No build script defined, skipping."

# ── Stage 3: Runtime PHP (imagem final) ───────────────────────────────────────
FROM php:{php_version}-fpm-bookworm AS runtime

LABEL maintainer="Ng Development <caronte@example.com>"
LABEL description="Laravel PROD image - PHP {php_version} optimized"

# ── Dependencias de sistema (apenas producao) ────────────────────────────────
RUN apt-get update && apt-get install -y --no-install-recommends \
    # Libs de producao
    libpng-dev \
    libjpeg62-turbo-dev \
    libfreetype6-dev \
    libwebp-dev \
    libzip-dev \
    libicu-dev \
    libxml2-dev \
    libonig-dev \
    libpq-dev \
    libsqlite3-dev \
    libmagickwand-dev \
    libgmp-dev \
    # Runtime
    nginx \
    supervisor \
    curl \
    ca-certificates \
    unzip \
    && rm -rf /var/lib/apt/lists/*

# ── Extensoes PHP (producao) ──────────────────────────────────────────────────
RUN docker-php-ext-configure gd \
        --with-freetype \
        --with-jpeg \
        --with-webp \
    && docker-php-ext-install -j"$(nproc)" \
        bcmath \
        ctype \
        dom \
        exif \
        fileinfo \
        gd \
        gmp \
        iconv \
        intl \
        mbstring \
        opcache \
        pcntl \
        pdo \
        pdo_sqlite \
        posix \
        simplexml \
        sockets \
        tokenizer \
        xml \
        xmlreader \
        xmlwriter \
        xsl \
        zip \
{mysql_ext}{pgsql_ext}    && echo "Extensions installed"
{sqlsrv_block}
# ── Redis + Imagick via PECL ──────────────────────────────────────────────────
RUN pecl install redis igbinary imagick \
    && docker-php-ext-enable redis igbinary imagick

# ── Configs PHP e Nginx ───────────────────────────────────────────────────────
COPY docker/php/php-prod.ini      $PHP_INI_DIR/conf.d/99-app-prod.ini
COPY docker/php/www.conf          /usr/local/etc/php-fpm.d/www.conf
COPY docker/nginx/nginx.conf      /etc/nginx/nginx.conf
COPY docker/nginx/default.conf    /etc/nginx/conf.d/default.conf
COPY docker/supervisor/supervisord.prod.conf /etc/supervisor/conf.d/supervisord.conf

# ── Codigo da aplicacao ───────────────────────────────────────────────────────
WORKDIR /var/www/html

COPY --chown=www-data:www-data --from=composer-deps /app/vendor       ./vendor
COPY --chown=www-data:www-data --from=composer-deps /app              .
COPY --chown=www-data:www-data --from=node-assets   /app/public       ./public

# ── Bootstrap producao ────────────────────────────────────────────────────────
RUN php artisan config:cache \
    && php artisan route:cache \
    && php artisan view:cache \
    && php artisan event:cache \
    && chown -R www-data:www-data /var/www/html/storage \
    && chmod -R 775 /var/www/html/storage /var/www/html/bootstrap/cache

EXPOSE 80
CMD ["/usr/bin/supervisord", "-c", "/etc/supervisor/conf.d/supervisord.conf"]
"#,
        php_version = php_version,
        mysql_ext = mysql_ext,
        pgsql_ext = pgsql_ext,
        sqlsrv_block = sqlsrv_block,
    )
}

// ─────────────────────────────────────────────────────────────────────────────
// docker-compose.dev.yml
// ─────────────────────────────────────────────────────────────────────────────
pub fn compose_dev(app_name: &str, databases: &[String], with_redis: bool, with_mailpit: bool) -> String {
    let mut services = String::new();
    let mut depends = String::new();

    // ── MySQL ─────────────────────────────────────────────────────────────────
    if databases.iter().any(|d| d == "mysql") {
        services.push_str(r#"
  mysql:
    image: mysql:8.4
    restart: unless-stopped
    environment:
      MYSQL_ROOT_PASSWORD: "${DB_ROOT_PASSWORD:-secret_root}"
      MYSQL_DATABASE:      "${DB_DATABASE:-laravel}"
      MYSQL_USER:          "${DB_USERNAME:-laravel}"
      MYSQL_PASSWORD:      "${DB_PASSWORD:-secret}"
    ports:
      - "${FORWARD_DB_MYSQL_PORT:-3306}:3306"
    volumes:
      - mysql_data:/var/lib/mysql
      - ./docker/mysql/my.cnf:/etc/mysql/conf.d/my.cnf:ro
    networks:
      - app-network
    healthcheck:
      test: ["CMD", "mysqladmin", "ping", "-h", "localhost", "-u", "root", "-p${DB_ROOT_PASSWORD:-secret_root}"]
      interval: 10s
      timeout: 5s
      retries: 5
"#);
        depends.push_str("      mysql:\n        condition: service_healthy\n");
    }

    // ── MariaDB ───────────────────────────────────────────────────────────────
    if databases.iter().any(|d| d == "mariadb") {
        services.push_str(r#"
  mariadb:
    image: mariadb:11.4
    restart: unless-stopped
    environment:
      MARIADB_ROOT_PASSWORD: "${DB_ROOT_PASSWORD:-secret_root}"
      MARIADB_DATABASE:      "${DB_DATABASE:-laravel}"
      MARIADB_USER:          "${DB_USERNAME:-laravel}"
      MARIADB_PASSWORD:      "${DB_PASSWORD:-secret}"
    ports:
      - "${FORWARD_DB_MARIADB_PORT:-3307}:3306"
    volumes:
      - mariadb_data:/var/lib/mysql
    networks:
      - app-network
    healthcheck:
      test: ["CMD", "healthcheck.sh", "--connect", "--innodb_initialized"]
      interval: 10s
      timeout: 5s
      retries: 5
"#);
        depends.push_str("      mariadb:\n        condition: service_healthy\n");
    }

    // ── PostgreSQL ────────────────────────────────────────────────────────────
    if databases.iter().any(|d| d == "pgsql") {
        services.push_str(r#"
  pgsql:
    image: postgres:17-alpine
    restart: unless-stopped
    environment:
      POSTGRES_DB:       "${DB_DATABASE:-laravel}"
      POSTGRES_USER:     "${DB_USERNAME:-laravel}"
      POSTGRES_PASSWORD: "${DB_PASSWORD:-secret}"
    ports:
      - "${FORWARD_DB_PGSQL_PORT:-5432}:5432"
    volumes:
      - pgsql_data:/var/lib/postgresql/data
    networks:
      - app-network
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U ${DB_USERNAME:-laravel} -d ${DB_DATABASE:-laravel}"]
      interval: 10s
      timeout: 5s
      retries: 5
"#);
        depends.push_str("      pgsql:\n        condition: service_healthy\n");
    }

    // ── SQL Server ────────────────────────────────────────────────────────────
    if databases.iter().any(|d| d == "sqlserver") {
        services.push_str(r#"
  sqlserver:
    image: mcr.microsoft.com/mssql/server:2022-latest
    restart: unless-stopped
    environment:
      ACCEPT_EULA:        "Y"
      SA_PASSWORD:        "${DB_SA_PASSWORD:-SqlServer!123}"
      MSSQL_PID:          "Developer"
    ports:
      - "${FORWARD_DB_MSSQL_PORT:-1433}:1433"
    volumes:
      - sqlserver_data:/var/opt/mssql
    networks:
      - app-network
    healthcheck:
      test: ["CMD-SHELL", "/opt/mssql-tools18/bin/sqlcmd -S localhost -U sa -P ${DB_SA_PASSWORD:-SqlServer!123} -Q 'SELECT 1' -No || exit 1"]
      interval: 15s
      timeout: 10s
      retries: 5
"#);
        depends.push_str("      sqlserver:\n        condition: service_healthy\n");
    }

    // ── Redis ─────────────────────────────────────────────────────────────────
    if with_redis {
        services.push_str(r#"
  redis:
    image: redis:7.4-alpine
    restart: unless-stopped
    command: ["redis-server", "--appendonly", "yes", "--requirepass", "${REDIS_PASSWORD:-secret_redis}"]
    ports:
      - "${FORWARD_REDIS_PORT:-6379}:6379"
    volumes:
      - redis_data:/data
    networks:
      - app-network
    healthcheck:
      test: ["CMD", "redis-cli", "--no-auth-warning", "-a", "${REDIS_PASSWORD:-secret_redis}", "ping"]
      interval: 10s
      timeout: 5s
      retries: 5
"#);
        depends.push_str("      redis:\n        condition: service_healthy\n");
    }

    // ── Mailpit ───────────────────────────────────────────────────────────────
    if with_mailpit {
        services.push_str(r#"
  mailpit:
    image: axllent/mailpit:latest
    restart: unless-stopped
    ports:
      - "${FORWARD_MAILPIT_PORT:-8025}:8025"
      - "${FORWARD_MAILPIT_SMTP_PORT:-1025}:1025"
    networks:
      - app-network
"#);
    }

    // ── Volumes ───────────────────────────────────────────────────────────────
    let mut volumes = String::new();
    if databases.iter().any(|d| d == "mysql")     { volumes.push_str("  mysql_data:\n"); }
    if databases.iter().any(|d| d == "mariadb")   { volumes.push_str("  mariadb_data:\n"); }
    if databases.iter().any(|d| d == "pgsql")     { volumes.push_str("  pgsql_data:\n"); }
    if databases.iter().any(|d| d == "sqlserver") { volumes.push_str("  sqlserver_data:\n"); }
    if with_redis { volumes.push_str("  redis_data:\n"); }

    format!(
        r#"# =============================================================================
# docker-compose.dev.yml — {app_name} DEV
# =============================================================================
name: {app_name_lower}-dev

services:

  # ── Aplicacao PHP-FPM ────────────────────────────────────────────────────
  app:
    build:
      context: .
      dockerfile: Dockerfile.dev
      args:
        USER_ID:  "${{UID:-1000}}"
        GROUP_ID: "${{GID:-1000}}"
    restart: unless-stopped
    volumes:
      - .:/var/www/html
      - vendor_cache:/var/www/html/vendor
    networks:
      - app-network
    depends_on:
{depends}    environment:
      APP_ENV:      local
      APP_DEBUG:    "true"
      XDEBUG_MODE:  develop,debug,coverage
      XDEBUG_CLIENT_HOST: "${{XDEBUG_CLIENT_HOST:-host.docker.internal}}"
      XDEBUG_CLIENT_PORT: "${{XDEBUG_CLIENT_PORT:-9003}}"

  # ── Nginx ────────────────────────────────────────────────────────────────
  nginx:
    image: nginx:1.27-alpine
    restart: unless-stopped
    ports:
      - "${{APP_PORT:-80}}:80"
    volumes:
      - .:/var/www/html:ro
      - ./docker/nginx/nginx.conf:/etc/nginx/nginx.conf:ro
      - ./docker/nginx/default.conf:/etc/nginx/conf.d/default.conf:ro
    networks:
      - app-network
    depends_on:
      - app
{services}
networks:
  app-network:
    driver: bridge

volumes:
  vendor_cache:
{volumes}"#,
        app_name = app_name,
        app_name_lower = app_name.to_lowercase().replace(' ', "-"),
        depends = depends,
        services = services,
        volumes = volumes,
    )
}

// ─────────────────────────────────────────────────────────────────────────────
// docker-compose.prod.yml
// ─────────────────────────────────────────────────────────────────────────────
pub fn compose_prod(app_name: &str, databases: &[String], with_redis: bool) -> String {
    let mut services = String::new();
    let mut depends = String::new();
    let mut volumes = String::new();

    if databases.iter().any(|d| d == "mysql") {
        services.push_str(r#"
  mysql:
    image: mysql:8.4
    restart: always
    environment:
      MYSQL_ROOT_PASSWORD: "${DB_ROOT_PASSWORD}"
      MYSQL_DATABASE:      "${DB_DATABASE}"
      MYSQL_USER:          "${DB_USERNAME}"
      MYSQL_PASSWORD:      "${DB_PASSWORD}"
    volumes:
      - mysql_data:/var/lib/mysql
      - ./docker/mysql/my.cnf:/etc/mysql/conf.d/my.cnf:ro
    networks:
      - app-network
    healthcheck:
      test: ["CMD", "mysqladmin", "ping", "-h", "localhost"]
      interval: 30s
      timeout: 10s
      retries: 3
"#);
        depends.push_str("      mysql:\n        condition: service_healthy\n");
        volumes.push_str("  mysql_data:\n");
    }

    if databases.iter().any(|d| d == "mariadb") {
        services.push_str(r#"
  mariadb:
    image: mariadb:11.4
    restart: always
    environment:
      MARIADB_ROOT_PASSWORD: "${DB_ROOT_PASSWORD}"
      MARIADB_DATABASE:      "${DB_DATABASE}"
      MARIADB_USER:          "${DB_USERNAME}"
      MARIADB_PASSWORD:      "${DB_PASSWORD}"
    volumes:
      - mariadb_data:/var/lib/mysql
    networks:
      - app-network
    healthcheck:
      test: ["CMD", "healthcheck.sh", "--connect", "--innodb_initialized"]
      interval: 30s
      timeout: 10s
      retries: 3
"#);
        depends.push_str("      mariadb:\n        condition: service_healthy\n");
        volumes.push_str("  mariadb_data:\n");
    }

    if databases.iter().any(|d| d == "pgsql") {
        services.push_str(r#"
  pgsql:
    image: postgres:17-alpine
    restart: always
    environment:
      POSTGRES_DB:       "${DB_DATABASE}"
      POSTGRES_USER:     "${DB_USERNAME}"
      POSTGRES_PASSWORD: "${DB_PASSWORD}"
    volumes:
      - pgsql_data:/var/lib/postgresql/data
    networks:
      - app-network
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U ${DB_USERNAME} -d ${DB_DATABASE}"]
      interval: 30s
      timeout: 10s
      retries: 3
"#);
        depends.push_str("      pgsql:\n        condition: service_healthy\n");
        volumes.push_str("  pgsql_data:\n");
    }

    if databases.iter().any(|d| d == "sqlserver") {
        services.push_str(r#"
  sqlserver:
    image: mcr.microsoft.com/mssql/server:2022-latest
    restart: always
    environment:
      ACCEPT_EULA:   "Y"
      SA_PASSWORD:   "${DB_SA_PASSWORD}"
      MSSQL_PID:     "Standard"
    volumes:
      - sqlserver_data:/var/opt/mssql
    networks:
      - app-network
"#);
        volumes.push_str("  sqlserver_data:\n");
    }

    if with_redis {
        services.push_str(r#"
  redis:
    image: redis:7.4-alpine
    restart: always
    command: ["redis-server", "--appendonly", "yes", "--requirepass", "${REDIS_PASSWORD}"]
    volumes:
      - redis_data:/data
    networks:
      - app-network
    healthcheck:
      test: ["CMD", "redis-cli", "--no-auth-warning", "-a", "${REDIS_PASSWORD}", "ping"]
      interval: 30s
      timeout: 10s
      retries: 3
"#);
        depends.push_str("      redis:\n        condition: service_healthy\n");
        volumes.push_str("  redis_data:\n");
    }

    format!(
        r#"# =============================================================================
# docker-compose.prod.yml — {app_name} PROD
# ATENCAO: nunca commitar .env de producao!
# =============================================================================
name: {app_name_lower}-prod

services:

  # ── Aplicacao (PHP-FPM + Nginx via supervisor) ────────────────────────────
  app:
    build:
      context: .
      dockerfile: Dockerfile.prod
    restart: always
    networks:
      - app-network
    depends_on:
{depends}    environment:
      APP_ENV:   production
      APP_DEBUG: "false"
    deploy:
      resources:
        limits:
          cpus:   "2"
          memory: 512M
        reservations:
          memory: 256M
    healthcheck:
      test: ["CMD-SHELL", "curl -f http://localhost/health || exit 1"]
      interval: 30s
      timeout: 10s
      retries: 3

  # ── Horizon (queue worker) ───────────────────────────────────────────────
  horizon:
    build:
      context: .
      dockerfile: Dockerfile.prod
    restart: always
    command: ["php", "artisan", "horizon"]
    networks:
      - app-network
    depends_on:
{depends}    environment:
      APP_ENV:   production
      APP_DEBUG: "false"
{services}
networks:
  app-network:
    driver: bridge

volumes:
{volumes}"#,
        app_name = app_name,
        app_name_lower = app_name.to_lowercase().replace(' ', "-"),
        depends = depends,
        services = services,
        volumes = volumes,
    )
}

// ─────────────────────────────────────────────────────────────────────────────
// Nginx — nginx.conf (worker config)
// ─────────────────────────────────────────────────────────────────────────────
pub fn nginx_conf() -> &'static str {
    r#"user nginx;
worker_processes auto;
error_log /var/log/nginx/error.log warn;
pid       /var/run/nginx.pid;

events {
    worker_connections 2048;
    multi_accept       on;
    use                epoll;
}

http {
    include       /etc/nginx/mime.types;
    default_type  application/octet-stream;

    # ── Logging ──────────────────────────────────────────────────────────────
    log_format main '$remote_addr - $remote_user [$time_local] '
                    '"$request" $status $body_bytes_sent '
                    '"$http_referer" "$http_user_agent"';
    access_log /var/log/nginx/access.log main;

    # ── Performance ───────────────────────────────────────────────────────────
    sendfile           on;
    tcp_nopush         on;
    tcp_nodelay        on;
    keepalive_timeout  65;
    types_hash_max_size 2048;
    client_max_body_size 100M;

    # ── Seguranca ────────────────────────────────────────────────────────────
    server_tokens off;
    add_header X-Frame-Options         SAMEORIGIN;
    add_header X-Content-Type-Options  nosniff;
    add_header X-XSS-Protection        "1; mode=block";
    add_header Referrer-Policy         "strict-origin-when-cross-origin";

    # ── Gzip ─────────────────────────────────────────────────────────────────
    gzip             on;
    gzip_vary        on;
    gzip_comp_level  5;
    gzip_min_length  1024;
    gzip_types       text/plain text/css application/json application/javascript
                     text/xml application/xml application/xml+rss text/javascript
                     image/svg+xml;

    include /etc/nginx/conf.d/*.conf;
}
"#
}

// ─────────────────────────────────────────────────────────────────────────────
// Nginx — default.conf (Laravel server block)
// ─────────────────────────────────────────────────────────────────────────────
pub fn nginx_default_conf(server_name: &str) -> String {
    format!(
        r#"server {{
    listen 80;
    listen [::]:80;

    server_name {server_name};
    root /var/www/html/public;
    index index.php index.html;

    charset utf-8;

    # ── Healthcheck endpoint ─────────────────────────────────────────────────
    location /health {{
        access_log off;
        return 200 "OK\n";
        add_header Content-Type text/plain;
    }}

    # ── Laravel front controller ─────────────────────────────────────────────
    location / {{
        try_files $uri $uri/ /index.php?$query_string;
    }}

    # ── PHP-FPM ──────────────────────────────────────────────────────────────
    location ~ \.php$ {{
        fastcgi_pass   app:9000;
        fastcgi_index  index.php;
        fastcgi_param  SCRIPT_FILENAME $realpath_root$fastcgi_script_name;
        include        fastcgi_params;
        fastcgi_param  PHP_VALUE "upload_max_filesize=100M \n post_max_size=100M";
        fastcgi_read_timeout 300;
        fastcgi_buffers 16 16k;
        fastcgi_buffer_size 32k;
    }}

    # ── Assets estaticos ─────────────────────────────────────────────────────
    location ~* \.(jpg|jpeg|png|gif|ico|css|js|pdf|woff|woff2|ttf|svg|webp|avif)$ {{
        expires    30d;
        add_header Cache-Control "public, no-transform";
        access_log off;
    }}

    # ── Deny dotfiles ─────────────────────────────────────────────────────────
    location ~ /\. {{
        deny all;
        access_log off;
        log_not_found off;
    }}
}}
"#
    )
}

// ─────────────────────────────────────────────────────────────────────────────
// PHP ini — DEV
// ─────────────────────────────────────────────────────────────────────────────
pub fn php_ini_dev() -> &'static str {
    r#"; ─── PHP Dev Config ─────────────────────────────────────────────────────────

[PHP]
; Erros — mostrar tudo em DEV
display_errors          = On
display_startup_errors  = On
error_reporting         = E_ALL
log_errors              = On
error_log               = /var/log/php/error.log

; Limites
memory_limit            = 512M
max_execution_time      = 300
max_input_time          = 300
max_input_vars          = 10000
post_max_size           = 100M
upload_max_filesize     = 100M
default_charset         = UTF-8

; OPcache — desligado para DEV (hot reload)
[opcache]
opcache.enable          = 0
opcache.enable_cli      = 0

; Timezone
[Date]
date.timezone           = America/Sao_Paulo

; Session
[Session]
session.gc_maxlifetime  = 3600
"#
}

// ─────────────────────────────────────────────────────────────────────────────
// PHP ini — PROD
// ─────────────────────────────────────────────────────────────────────────────
pub fn php_ini_prod() -> &'static str {
    r#"; ─── PHP Prod Config ────────────────────────────────────────────────────────

[PHP]
; Erros — nunca exibir em PROD
display_errors          = Off
display_startup_errors  = Off
error_reporting         = E_ALL & ~E_DEPRECATED & ~E_STRICT
log_errors              = On
error_log               = /var/log/php/error.log

; Limites
memory_limit            = 256M
max_execution_time      = 60
max_input_time          = 60
max_input_vars          = 5000
post_max_size           = 50M
upload_max_filesize     = 50M
default_charset         = UTF-8

; OPcache — agressivo para PROD
[opcache]
opcache.enable                 = 1
opcache.enable_cli             = 0
opcache.memory_consumption     = 256
opcache.interned_strings_buffer= 16
opcache.max_accelerated_files  = 20000
opcache.validate_timestamps    = 0
opcache.revalidate_freq        = 0
opcache.fast_shutdown          = 1
opcache.jit_buffer_size        = 128M
opcache.jit                    = tracing

; Timezone
[Date]
date.timezone = America/Sao_Paulo

; Session
[Session]
session.cookie_secure    = 1
session.cookie_httponly  = 1
session.cookie_samesite  = Strict
session.gc_maxlifetime   = 7200
"#
}

// ─────────────────────────────────────────────────────────────────────────────
// Xdebug ini
// ─────────────────────────────────────────────────────────────────────────────
pub fn xdebug_ini() -> &'static str {
    r#"; ─── Xdebug Config ──────────────────────────────────────────────────────────
[xdebug]
xdebug.mode                = ${XDEBUG_MODE:-develop,debug}
xdebug.start_with_request  = yes
xdebug.client_host         = ${XDEBUG_CLIENT_HOST:-host.docker.internal}
xdebug.client_port         = ${XDEBUG_CLIENT_PORT:-9003}
xdebug.idekey              = VSCODE
xdebug.max_nesting_level   = 512
xdebug.log                 = /var/log/php/xdebug.log
xdebug.log_level           = 0
"#
}

// ─────────────────────────────────────────────────────────────────────────────
// PHP-FPM www.conf
// ─────────────────────────────────────────────────────────────────────────────
pub fn php_fpm_www_conf() -> &'static str {
    r#"; ─── PHP-FPM Pool — www ──────────────────────────────────────────────────────
[www]
user  = www-data
group = www-data

listen = 0.0.0.0:9000

; Gerenciamento de processos
pm                   = dynamic
pm.max_children      = 50
pm.start_servers     = 5
pm.min_spare_servers = 5
pm.max_spare_servers = 35
pm.max_requests      = 500

; Logs
access.log                 = /proc/self/fd/2
access.format              = "%R - %u %t \"%m %r%Q%q\" %s %f %{mili}d %{kilo}M %C%%"
catch_workers_output       = yes
decorate_workers_output    = no

; Variaveis de ambiente para PHP-FPM
clear_env = no
"#
}

// ─────────────────────────────────────────────────────────────────────────────
// MySQL my.cnf
// ─────────────────────────────────────────────────────────────────────────────
pub fn mysql_conf() -> &'static str {
    r#"[mysqld]
# Charset
character-set-server  = utf8mb4
collation-server      = utf8mb4_unicode_ci

# Performance
innodb_buffer_pool_size    = 256M
innodb_log_file_size       = 64M
innodb_flush_log_at_trx_commit = 1
innodb_flush_method        = O_DIRECT

# Conexoes
max_connections            = 200
wait_timeout               = 600
interactive_timeout        = 600

# Logs lentos (habilitado para dev, desabilitar em prod)
slow_query_log             = 1
slow_query_log_file        = /var/lib/mysql/slow.log
long_query_time            = 1

[client]
default-character-set = utf8mb4
"#
}

// ─────────────────────────────────────────────────────────────────────────────
// Supervisor — PROD (PHP-FPM + Nginx + Queue Worker)
// ─────────────────────────────────────────────────────────────────────────────
pub fn supervisord_prod() -> &'static str {
    r#"[supervisord]
nodaemon=true
user=root
logfile=/var/log/supervisor/supervisord.log
pidfile=/var/run/supervisord.pid

[program:nginx]
command=/usr/sbin/nginx -g "daemon off;"
autostart=true
autorestart=true
priority=10
stdout_logfile=/dev/stdout
stdout_logfile_maxbytes=0
stderr_logfile=/dev/stderr
stderr_logfile_maxbytes=0

[program:php-fpm]
command=/usr/local/sbin/php-fpm --nodaemonize
autostart=true
autorestart=true
priority=5
stdout_logfile=/dev/stdout
stdout_logfile_maxbytes=0
stderr_logfile=/dev/stderr
stderr_logfile_maxbytes=0

[program:laravel-queue]
command=php /var/www/html/artisan queue:work --sleep=3 --tries=3 --max-time=3600
user=www-data
autostart=true
autorestart=true
numprocs=2
process_name=%(program_name)s_%(process_num)02d
stdout_logfile=/dev/stdout
stdout_logfile_maxbytes=0
stderr_logfile=/dev/stderr
stderr_logfile_maxbytes=0
"#
}

// ─────────────────────────────────────────────────────────────────────────────
// .dockerignore
// ─────────────────────────────────────────────────────────────────────────────
pub fn dockerignore() -> &'static str {
    r#"# ── Git ──────────────────────────────────────────────────────────────────────
.git
.gitignore
.gitattributes

# ── Laravel ignorados ─────────────────────────────────────────────────────────
vendor/
node_modules/
storage/logs/*
storage/framework/cache/*
storage/framework/sessions/*
storage/framework/views/*
bootstrap/cache/*
.env
.env.*
!.env.example

# ── Docker ────────────────────────────────────────────────────────────────────
docker-compose*.yml
Dockerfile*
docker/

# ── Tests ─────────────────────────────────────────────────────────────────────
tests/
.phpunit*
phpunit.xml
coverage/

# ── Dev tools ─────────────────────────────────────────────────────────────────
.vscode/
.idea/
*.md
Makefile
*.sh
"#
}

// ─────────────────────────────────────────────────────────────────────────────
// .env.example — completo com todas as DBs
// ─────────────────────────────────────────────────────────────────────────────
pub fn env_example(app_name: &str, databases: &[String], with_redis: bool) -> String {
    let db_section = if databases.iter().any(|d| d == "mysql" || d == "mariadb") {
        format!(
            r#"DB_CONNECTION=mysql
DB_HOST=mysql
DB_PORT=3306
DB_DATABASE={app}
DB_USERNAME={app}
DB_PASSWORD=secret
DB_ROOT_PASSWORD=secret_root
"#,
            app = app_name.to_lowercase().replace(' ', "_")
        )
    } else if databases.iter().any(|d| d == "pgsql") {
        format!(
            r#"DB_CONNECTION=pgsql
DB_HOST=pgsql
DB_PORT=5432
DB_DATABASE={app}
DB_USERNAME={app}
DB_PASSWORD=secret
"#,
            app = app_name.to_lowercase().replace(' ', "_")
        )
    } else if databases.iter().any(|d| d == "sqlserver") {
        format!(
            r#"DB_CONNECTION=sqlsrv
DB_HOST=sqlserver
DB_PORT=1433
DB_DATABASE={app}
DB_USERNAME=sa
DB_SA_PASSWORD=SqlServer!123
"#,
            app = app_name.to_lowercase().replace(' ', "_")
        )
    } else {
        "DB_CONNECTION=sqlite\nDB_DATABASE=/var/www/html/database/database.sqlite\n".to_string()
    };

    let redis_section = if with_redis {
        r#"REDIS_HOST=redis
REDIS_PORT=6379
REDIS_PASSWORD=secret_redis

CACHE_STORE=redis
QUEUE_CONNECTION=redis
SESSION_DRIVER=redis
"#
    } else {
        r#"CACHE_STORE=file
QUEUE_CONNECTION=database
SESSION_DRIVER=file
"#
    };

    format!(
        r#"APP_NAME="{app_name}"
APP_ENV=local
APP_KEY=
APP_DEBUG=true
APP_URL=http://localhost

APP_PORT=80
FORWARD_DB_MYSQL_PORT=3306
FORWARD_DB_PGSQL_PORT=5432
FORWARD_REDIS_PORT=6379
FORWARD_MAILPIT_PORT=8025
FORWARD_MAILPIT_SMTP_PORT=1025

LOG_CHANNEL=stack
LOG_DEPRECATIONS_CHANNEL=null
LOG_LEVEL=debug

{db_section}
{redis_section}
MAIL_MAILER=smtp
MAIL_HOST=mailpit
MAIL_PORT=1025
MAIL_USERNAME=null
MAIL_PASSWORD=null
MAIL_ENCRYPTION=null
MAIL_FROM_ADDRESS="hello@example.com"
MAIL_FROM_NAME="${{APP_NAME}}"

BROADCAST_DRIVER=log
FILESYSTEM_DISK=local

XDEBUG_MODE=develop,debug
XDEBUG_CLIENT_HOST=host.docker.internal
XDEBUG_CLIENT_PORT=9003
"#,
        app_name = app_name,
        db_section = db_section,
        redis_section = redis_section,
    )
}

// ─────────────────────────────────────────────────────────────────────────────
// Makefile — atalhos dev
// ─────────────────────────────────────────────────────────────────────────────
pub fn makefile(app_name: &str) -> String {
    let _name = app_name.to_lowercase().replace(' ', "-");
    format!(
        r#".PHONY: help up down build shell artisan composer logs test

COMPOSE_DEV=docker compose -f docker-compose.dev.yml
COMPOSE_PROD=docker compose -f docker-compose.prod.yml

## ── DEV ─────────────────────────────────────────────────────────────────────

help:
	@echo ""
	@echo "  {app_name} — comandos Make"
	@echo "  ────────────────────────────────────────"
	@echo "  up        Sobe o ambiente DEV"
	@echo "  down      Derruba o ambiente DEV"
	@echo "  build     Reconstroi as imagens DEV"
	@echo "  shell     Abre shell no container app (DEV)"
	@echo "  artisan   Executa artisan  ex: make artisan cmd='migrate'"
	@echo "  composer  Executa composer ex: make composer cmd='install'"
	@echo "  logs      Exibe logs do app"
	@echo "  test      Roda PHPUnit no container"
	@echo "  prod-up   Sobe o ambiente PROD"
	@echo "  prod-down Derruba o ambiente PROD"
	@echo ""

up:
	${{COMPOSE_DEV}} up -d

down:
	${{COMPOSE_DEV}} down

build:
	${{COMPOSE_DEV}} build --no-cache

shell:
	${{COMPOSE_DEV}} exec app bash

artisan:
	${{COMPOSE_DEV}} exec app php artisan ${{cmd}}

composer:
	${{COMPOSE_DEV}} exec app composer ${{cmd}}

logs:
	${{COMPOSE_DEV}} logs -f app

test:
	${{COMPOSE_DEV}} exec app php artisan test

migrate:
	${{COMPOSE_DEV}} exec app php artisan migrate

fresh:
	${{COMPOSE_DEV}} exec app php artisan migrate:fresh --seed

## ── PROD ─────────────────────────────────────────────────────────────────────

prod-build:
	${{COMPOSE_PROD}} build --no-cache

prod-up:
	${{COMPOSE_PROD}} up -d

prod-down:
	${{COMPOSE_PROD}} down
"#,
        app_name = app_name,
    )
}
