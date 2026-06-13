# ngdev-laravel

> **Gerador interativo de código Laravel com suporte a DDD, Clean Architecture e infraestrutura Docker.**  
> Escrito em Rust — roda via terminal e via painel gráfico (Tauri). Sem dependências externas.

```
##########################################################################
##########################################################################
##########################################################################
#################################################################****#####
###############################################################*=:..:+*###
##############################################################*-.    .+###
##############################################################*:      -###
##############################################################*-     .+###
##############################################################*: ...:=####
#####################*****###**+++**##############**+++***###*: -***######
####################=.....+*=:.    .-+*########*+-.     .-+*+:.-##########
##############*=-*##=     -.         .=#######+:          .:..=#+#########
############*=. :*##+                  =#####=.       ..    .=#-.*########
##########*-.   :*##+       :=++-.     .*###=     .-+***+:..+*- .*########
########+-.   .=*###+      =#%%%%*:    .+##*.    .+#%%%%%#=+*-  .*########
######+:    :=#%####+     :#####%%=     =##+.    =#%###%%%#*:   .*########
#####+.. .:+#%######+     -#######+     =##=    .*#########+    .*########
#####+...:*#########+.   .=#######+     =##=.   .*#########=    .*########
#####*:....-+#######+.  ..-#######+.    =##+.   .+#########:.   .*########
#######+:....:=*####+.....=#######+.....=###: ..=*+*#####*-......*########
######%%#*-....:=###+.....=#######+.....=##%+..+*-.:-=+=-.......:*########
########%%#*=...:###+.....=#######+.....=##%%+**-...............:*########
##########%%%#=::###+.....=#######+.....=#####*:..........--....:*########
###########%%%%#*###+.....=#######+.....=####*:.-+-:::::=*#=....:*########
##############%%%###*=----+#######*--=--*###+:.=#%######%##:....:#%%###%##
####################%%%%%%%######%%%%%#####+::=#*###%#####=:..:.=#%%%%%%%%
#####################%%%%%########%#######=.:+*-:-=+****+-:.::::#%%%%%%%%%
#####################################*=-:-::+#-::::::::::::::::*%%%%%%%%%%
####################################+::::::-#*-::::::::::::::-*%%%#%#%####
####################################-:::::::*##*+=-:::::::-+*%%%%%#%%%####
####################################=:::::::*%%%%%########%%%%%%%%#%%%####
####################################*-:::::+#%%%%%%%%%%%%%%%%%%%%%#%%%####
###################################%%#*+=+*#%%##%%%%%%%%%%%%%%%%%%%%%%####
###################################%%%%%%%%#######%%%%%%%%%#%%%%%%#%%%=###
####################################%%%%%%%%###%%%%%%###%%%#%%%%%%##%%+###
#####################################%%##%#####%##%%#%%#%%###%%##%#%%%%%%%
#########################################%####%%%%%%%%%%%%%%%%%%%%%%%%%%%%
########################################%%##########%%%%%%%%%%%#%%#%%%%%%%
######################################%%#######%#%#%%#%%%%%%%%%%%%%%%%%%%%
#######################################%%%%#%#%%%%%%%%%####%%%%%%%%%%%%%%%
#############%%%###################%##%%%%##%#%%%%%%%%%###%%#%%%%%%%%%#%%%
```
---

## Sumário

- [Visão Geral](#visão-geral)
- [Estrutura do Projeto](#estrutura-do-projeto)
- [Fluxo 1 — Menu Principal](#fluxo-1--menu-principal)
- [Fluxo 2 — Context DDD](#fluxo-2--context-ddd)
- [Fluxo 3 — Scaffolds Completos](#fluxo-3--scaffolds-completos-logística--estoque--pdv)
- [Fluxo 4 — Infra Docker](#fluxo-4--infra-docker)
- [Fluxo 5 — Landing Page](#fluxo-5--landing-page)
- [Fluxo 6 — Geradores Avulsos](#fluxo-6--geradores-avulsos)
- [Geradores em Detalhe](#geradores-em-detalhe)
- [Painel Gráfico — ngdev Manager](#painel-gráfico--ngdev-manager-tauri)
- [Instalação](#instalação-e-uso)
- [Licença](#licença)

---

## Visão Geral

O `ngdev` gera scaffolding completo para projetos Laravel com DDD e Clean Architecture. Cada gerador faz perguntas interativas e salva os arquivos no **caminho absoluto informado pelo usuário** — nunca dentro do próprio projeto `ngdev`.

| Modo | Como rodar |
|---|---|
| **CLI interativo** | `ngdev` no terminal |
| **Painel desktop** | `tauri dev` (dev) ou binário `ngdev-manager` (prod) |

**9 geradores disponíveis:**

| # | Gerador | O que entrega |
|---|---|---|
| 1 | Context DDD | Bounded context completo (Application + Domain + Infra) |
| 2 | Logística Reversa | 7 Contexts + 10 Migrations + 10 Models + Manager JSON |
| 3 | ERP Estoque | 6 Contexts + Kardex + UseCases + Manager JSON |
| 4 | Infra Docker | Dockerfile dev/prod + compose + Nginx + Makefile |
| 5 | Landing Page | HTML + Tailwind + DaisyUI (generic) ou Contabilizei-style (SaaS) |
| 6 | Model | Eloquent + Migration + Controller opcionais |
| 7 | Controller | Plain ou Resource com Model |
| 8 | Migration | Schema::create com strict_types |
| 9 | PDV | Scaffold completo Ponto de Venda |

---

## Estrutura do Projeto

```
ngdev-laravel/
├── src/
│   ├── main.rs                  ← CLI: menu interativo + ASCII logo
│   ├── cli.rs                   ← Structs de args compartilhados
│   └── flows/
│       ├── context/             ← Gerador Context DDD
│       ├── docker/              ← Gerador Infra Docker
│       ├── estoque/             ← Scaffold ERP Estoque
│       ├── landing_page/        ← Gerador Landing Page (generic + saas)
│       │   ├── templates.rs     ← Layout DaisyUI generic
│       │   └── templates_saas.rs← Layout Contabilizei-style
│       ├── logistica_reversa/   ← Scaffold Logística Reversa
│       ├── pdv/                 ← Scaffold PDV
│       └── artesanal/
│           ├── controller/      ← Gerador Controller
│           ├── model/           ← Gerador Model
│           └── migration/       ← Gerador Migration
├── manager/                     ← Crate Tauri (painel desktop)
│   ├── src/
│   │   ├── lib.rs               ← Tauri app builder
│   │   └── commands.rs          ← Bridge frontend → geradores Rust
│   └── tauri.conf.json
├── frontend-installer/          ← SPA Vite + TypeScript + Tailwind + DaisyUI
│   ├── index.html               ← Layout drawer (sidebar + main)
│   └── src/main.ts              ← Páginas e handlers dos 9 geradores
├── Cargo.toml                   ← Workspace (ngdev-laravel + manager)
└── install.sh                   ← Instala ngdev em /usr/local/bin
```

---

## Fluxo 1 — Menu Principal

```mermaid
flowchart TD
    START([▶ ngdev]) --> LOGO[Exibe ASCII Logo\nNg Development · Laravel]
    LOGO --> MENU{Selecione o gerador}

    MENU --> G1[🏗 Context DDD]
    MENU --> G2[🚚 Logística Reversa]
    MENU --> G3[📦 ERP Estoque]
    MENU --> G4[🐳 Infra Docker]
    MENU --> G5[🌐 Landing Page]
    MENU --> G6[🗃 Model]
    MENU --> G7[🎮 Controller]
    MENU --> G8[📋 Migration]
    MENU --> G9[🧾 PDV]
    MENU --> EXIT([Sair])

    G1 --> FLOW_CTX([→ ver Fluxo 2])
    G2 & G3 & G9 --> FLOW_SC([→ ver Fluxo 3])
    G4 --> FLOW_DOC([→ ver Fluxo 4])
    G5 --> FLOW_LP([→ ver Fluxo 5])
    G6 & G7 & G8 --> FLOW_ART([→ ver Fluxo 6])
```

---

## Fluxo 2 — Context DDD

```mermaid
flowchart TD
    IN([Gerador Context DDD]) --> W1[Nome do Context\nex: Produto → PascalCase automático]
    W1 --> W2[Prefixo de rota\nkebab-case auto-sugerido]
    W2 --> W3[Namespace base PHP\nex: App\\\\Contexts]
    W3 --> W4[Diretório base dos Contexts\nex: back/app/Contexts]
    W4 --> W5{Gerar Domain Entity?}
    W5 --> W6{Gerar Autorizações?}
    W6 --> W7[MultiSelect: operações\nconsultar · detalhar · criar · alterar · deletar]
    W7 --> W8[Caminho absoluto do projeto Laravel\nex: /home/user/meu-projeto]

    W8 --> APP & DOM & INF

    subgraph APP [Application Layer]
        A1[DTOs Input — readonly class]
        A2[DTOs Output — simples e paginado]
        A3[Queries — GET consultar / detalhar]
        A4[UseCases — POST criar · PUT alterar · DELETE deletar]
        A5[Errors · Exceptions]
    end

    subgraph DOM [Domain Layer]
        D1[Entity — ::create e ::update]
        D2[Enums de status]
        D3[Autorizações — se selecionado]
    end

    subgraph INF [Infra Layer]
        I1[Eloquent Model]
        I2[Repository]
        I3[Controller + FormRequests]
        I4[Routes · ServiceProvider]
    end

    APP & DOM & INF --> OK([✅ Context gerado\nem /projeto/base_path/NomeContext/])
```

---

## Fluxo 3 — Scaffolds Completos (Logística · Estoque · PDV)

```mermaid
flowchart TD
    subgraph LOG [🚚 Logística Reversa de Sinistros]
        L1[paths · namespace · ERP ID\ncompany · warehouse] --> L2[Caminho absoluto do projeto]
        L2 --> LA[7 Contexts DDD\nSeguradora · Transportadora · Segurado\nApólice · Sinistro · OrdemColeta · LaudoTriagem]
        L2 --> LB[10 Migrations ordenadas por FK\ndeclare strict + índices + comentários]
        L2 --> LC[10 Eloquent Models\n+ ItemSinistrado · MovimentacaoLogistica · RecebimentoCd]
        L2 --> LD[Manager JSON\nSLA · Webhooks exponential_backoff · Intelipost Reverse]
        LA & LB & LC & LD --> LOK([✅ Logística gerada])
    end

    subgraph EST [📦 ERP Estoque]
        E1[paths · namespace\nmétodo custeio PEPS / Custo Médio] --> E2[Caminho absoluto do projeto]
        E2 --> EA[6 Contexts DDD\nArmazem · Fornecedor · Produto\nPedidoCompra · MovimentacaoEstoque · Inventario]
        E2 --> EB[10 Migrations decimal 12-3\nmulti-armazém · lote · inventário]
        E2 --> EC[10 Models + 4 sub-entidades\nPosicaoEstoque · Lote\nItemPedidoCompra · ContagemInventario]
        E2 --> ED[UseCases com regra de negócio\nRegistrarMovimentacao — Kardex imutável\nFecharInventario — ajuste_ganho / ajuste_perda]
        E2 --> EE[Manager JSON\nPEPS · Custo Médio · alerta vencimento · ressuprimento]
        EA & EB & EC & ED & EE --> EOK([✅ ERP Estoque gerado])
    end

    subgraph PDV [🧾 Scaffold PDV]
        P1[Modo: Tudo / Só Migrations / Só Models] --> P2[Caminho absoluto do projeto]
        P2 --> PA[Migrations Ponto de Venda]
        P2 --> PB[Eloquent Models PDV]
        PA & PB --> POK([✅ PDV gerado])
    end
```

---

## Fluxo 4 — Infra Docker

```mermaid
flowchart TD
    IN([Gerador Docker]) --> W1[Nome da aplicação]
    W1 --> W2[Output path — raiz do projeto Laravel]
    W2 --> W3[server_name Nginx\nex: localhost ou app.local]
    W3 --> W4[Versão PHP — 8.3 · 8.2 · 8.1]
    W4 --> W5[Versão Node — 22 LTS · 20 LTS]
    W5 --> W6[MultiSelect: bancos\nMySQL · MariaDB · PostgreSQL · SQL Server · SQLite]
    W6 --> W7{Redis?}
    W7 --> W8{Mailpit?}

    W8 --> DEV[Dockerfile.dev\nPHP-fpm + Xdebug + Composer + Node + 30 ext]
    W8 --> PROD[Dockerfile.prod\nMulti-stage: composer → node → runtime]
    W8 --> COMPOSE[docker-compose.dev.yml\ndocker-compose.prod.yml]
    W8 --> CONF[php-dev.ini · php-prod.ini · xdebug.ini · www.conf\nnginx.conf · default.conf · my.cnf · supervisord.conf]
    W8 --> EXTRA[.dockerignore · .env.example\nMakefile — up / down / build / shell / artisan / migrate]

    DEV & PROD & COMPOSE & CONF & EXTRA --> OK([✅ Infra Docker gerada])
```

---

## Fluxo 5 — Landing Page

```mermaid
flowchart TD
    IN([Gerador Landing Page]) --> L1{Layout base}

    L1 -- Generic DaisyUI --> GEN[produto · tagline · empresa]
    L1 -- SaaS Contabilizei-style --> SAAS[produto · tagline · empresa]

    GEN --> GT[Tema DaisyUI\nlight · dark · corporate · lofi · business\ncupcake · cyberpunk · forest · luxury · night]
    GT --> GS[MultiSelect seções\nlogos · features_grid · features_tabs · stats\ntestimonials · pricing · faq · cta_bottom]

    SAAS --> SS[MultiSelect seções\nsocial_proof · comparison_table · journey_selector\nbenefits_slider · content_grid · testimonials_photo · faq]

    GS --> OUT[Caminho absoluto de saída\nex: /home/user/projetos/minha-landing]
    SS --> OUT

    OUT --> IDX[index.html — página completa montada]
    OUT --> NAV[sections/navbar/index.html]
    OUT --> HERO[sections/hero/index.html]
    OUT --> SEC[sections/nome_secao/index.html\npara cada seção selecionada]
    OUT --> FOOT[sections/footer/index.html]

    IDX & NAV & HERO & SEC & FOOT --> OK([✅ Landing Page gerada])
```

---

## Fluxo 6 — Geradores Avulsos

```mermaid
flowchart TD
    subgraph MOD [🗃 Model]
        M1[Nome — ex: Produto] --> M2{+ Migration?}
        M2 --> M3{+ Controller?}
        M3 --> M4[Caminho absoluto do projeto]
        M4 --> MA[app/Models/Produto.php\nfillable · hidden · table]
        M4 --> MB[database/migrations/..._create_produtos_table.php]
        M4 --> MC[app/Http/Controllers/ProdutoController.php]
        MA & MB & MC --> MOK([✅ Model gerado])
    end

    subgraph CTRL [🎮 Controller]
        C1[Nome — ex: ProdutoController] --> C2{Resource Controller?}
        C2 --> C3[Model opcional — type-hint automático]
        C3 --> C4[Caminho absoluto do projeto]
        C4 --> CA[app/Http/Controllers/ProdutoController.php\nindex · create · store · show · edit · update · destroy]
        CA --> COK([✅ Controller gerado])
    end

    subgraph MIG [📋 Migration]
        G1[Nome — ex: create_produtos_table] --> G2[Table opcional\nauto-inferida do nome]
        G2 --> G3[Caminho absoluto do projeto]
        G3 --> GA[database/migrations/YYYY_MM_DD_HHMMSS_nome.php\ndeclare strict + Schema::create + timestamps]
        GA --> GOK([✅ Migration gerada])
    end
```

---

## Geradores em Detalhe

### 🏗️ Context DDD

| Camada | O que gera |
|---|---|
| **Application** | DTOs Input (`readonly class`) · DTOs Output (simples e paginado) · Queries (GET) · UseCases (POST/PUT/DELETE) · Errors · Exceptions |
| **Domain** | Entity (com `::create()` / `::update()`) · Enums · Autorizações |
| **Infra** | Eloquent Model · Repository · Controller · FormRequests · Routes · ServiceProvider |

---

### 🚚 Logística Reversa de Sinistros

- **7 Contexts DDD** — Seguradora, Transportadora, Segurado, Apólice, Sinistro, OrdemColeta, LaudoTriagem
- **3 sub-entidades** — ItemSinistrado, MovimentacaoLogistica, RecebimentoCd
- **10 Migrations** em ordem de FK com `declare(strict_types=1)`, índices e comentários
- **10 Eloquent Models** com relacionamentos cross-context via FQN
- **Manager JSON** — SLA, webhooks com `exponential_backoff`, Intelipost Reverse

---

### 📦 ERP de Estoque

- **6 Contexts DDD** — Armazem, Fornecedor, Produto, PedidoCompra, MovimentacaoEstoque *(Kardex imutável — sem PUT/DELETE)*, Inventario
- **4 sub-entidades** — PosicaoEstoque, Lote, ItemPedidoCompra, ContagemInventario
- **UseCases especiais:**
  - `RegistrarMovimentacaoUseCase` — calcula `saldo_apos_movimento`, bloqueia estoque negativo
  - `FecharInventarioUseCase` — gera ajustes `ajuste_ganho`/`ajuste_perda` no Kardex automaticamente
- **10 Migrations** com `decimal(12,3)` para suporte a KG/Litros
- **Manager JSON** — PEPS / Custo Médio, alerta de vencimento, ressuprimento automático

---

### 🐳 Infra Docker (DEV + PROD)

| Arquivo | Descrição |
|---|---|
| `Dockerfile.dev` | PHP 8.3-fpm, +30 extensões, Xdebug, Composer, Node.js |
| `Dockerfile.prod` | Multi-stage: composer-deps → node-assets → runtime otimizado |
| `docker-compose.dev.yml` | App + Nginx + DBs + Redis + Mailpit com healthchecks |
| `docker-compose.prod.yml` | App + Horizon worker, sem devtools |
| `docker/php/php-dev.ini` | Erros visíveis, OPcache desligado |
| `docker/php/php-prod.ini` | OPcache agressivo + JIT tracing |
| `docker/php/xdebug.ini` | Modo develop/debug/coverage, porta 9003 |
| `docker/php/www.conf` | PHP-FPM pool: pm=dynamic, max_children=50 |
| `docker/nginx/nginx.conf` | Worker config, gzip, headers de segurança |
| `docker/nginx/default.conf` | Server block Laravel com healthcheck |
| `docker/mysql/my.cnf` | utf8mb4, InnoDB tuning, slow query log |
| `docker/supervisor/supervisord.prod.conf` | Nginx + PHP-FPM + 2 queue workers |
| `.dockerignore` | Exclui vendor, node_modules, .env, tests |
| `.env.example` | Pré-configurado com os serviços selecionados |
| `Makefile` | `make up/down/build/shell/artisan/migrate/test/prod-*` |

**Bancos suportados:** MySQL 8.4 · MariaDB 11.4 · PostgreSQL 17 · SQL Server 2022 · SQLite

---

### 🌐 Landing Page

Dois layouts disponíveis — cada seção em **diretório próprio**:

**Generic (DaisyUI)** — 10 temas:
`logos` · `features_grid` · `features_tabs` · `stats` · `testimonials` · `pricing` · `faq` · `cta_bottom`

**SaaS (Contabilizei-style)** — fundo branco, verde:
`social_proof` · `comparison_table` · `journey_selector` · `benefits_slider` · `content_grid` · `testimonials_photo` · `faq`

Estrutura de saída:
```
minha-landing/
  index.html
  sections/
    navbar/index.html
    hero/index.html
    features_grid/index.html
    pricing/index.html
    footer/index.html
    ...
```

---

## Painel Gráfico — ngdev Manager (Tauri)

Interface desktop construída com **Tauri v2 + Vite + TypeScript + Tailwind CSS + DaisyUI**.

```
manager/              ← crate Rust (Tauri backend)
frontend-installer/   ← SPA com 9 telas, uma por gerador
```

Para rodar em modo de desenvolvimento:

```bash
cd manager
tauri dev
# O Vite frontend inicia automaticamente em http://localhost:1420
```

Para build de produção:

```bash
cd manager
tauri build
```

---

## Instalação e Uso

### CLI

```bash
# Compilar e instalar globalmente em /usr/local/bin
./install.sh --build

# Executar
ngdev

# Ou sem instalar
cargo run
```

### Desinstalar

```bash
./install.sh --uninstall
```

### Compilar binário estático (musl)

```bash
rustup target add x86_64-unknown-linux-musl
RUSTFLAGS="-C target-feature=+crt-static" \
  cargo build --release --target x86_64-unknown-linux-musl
```

---

## Licença

```
MIT License — Copyright (c) 2026 Ng Development
```
