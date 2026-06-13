// ── Substituição simples de placeholders (evita escaping de {} no format!) ────
fn s(template: &str, pairs: &[(&str, &str)]) -> String {
    let mut out = template.to_string();
    for (k, v) in pairs {
        out = out.replace(k, v);
    }
    out
}

// ── API pública ───────────────────────────────────────────────────────────────

pub fn html_shell(title: &str, theme: &str, body: &str) -> String {
    s(SHELL, &[("[TITLE]", title), ("[THEME]", theme), ("[BODY]", body)])
}

pub fn section_navbar(product: &str) -> String {
    NAVBAR.replace("[PRODUCT]", product)
}

pub fn section_hero(product: &str, tagline: &str) -> String {
    s(HERO, &[("[PRODUCT]", product), ("[TAGLINE]", tagline)])
}

pub fn section_logos() -> &'static str { LOGOS }
pub fn section_features_grid() -> &'static str { FEATURES_GRID }
pub fn section_features_tabs() -> &'static str { FEATURES_TABS }
pub fn section_stats() -> &'static str { STATS }
pub fn section_testimonials() -> &'static str { TESTIMONIALS }
pub fn section_pricing() -> &'static str { PRICING }
pub fn section_faq() -> &'static str { FAQ }

pub fn section_cta_bottom(product: &str) -> String {
    CTA_BOTTOM.replace("[PRODUCT]", product)
}

pub fn section_footer(company: &str) -> String {
    FOOTER.replace("[COMPANY]", company)
}

// ─────────────────────────────────────────────────────────────────────────────
// SHELL
// ─────────────────────────────────────────────────────────────────────────────
const SHELL: &str = r##"<!DOCTYPE html>
<html lang="pt-BR" data-theme="[THEME]">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>[TITLE]</title>
  <link href="https://cdn.jsdelivr.net/npm/daisyui@4.12.10/dist/full.min.css" rel="stylesheet" type="text/css" />
  <script src="https://cdn.tailwindcss.com"></script>
  <style>
    html { scroll-behavior: smooth; }
    @keyframes marquee {
      0%   { transform: translateX(0); }
      100% { transform: translateX(-50%); }
    }
    .marquee-track { animation: marquee 30s linear infinite; }
    .marquee-track:hover { animation-play-state: paused; }
    [data-panel] { display: none; }
    [data-panel].active { display: block; }
    [data-fade] { opacity: 0; transform: translateY(24px); transition: opacity .7s ease, transform .7s ease; }
    [data-fade].visible { opacity: 1; transform: none; }
  </style>
</head>
<body class="antialiased">
[BODY]
<script>
  // Sistema de tabs genérico
  document.querySelectorAll('[data-tab]').forEach(btn => {
    btn.addEventListener('click', () => {
      const grp = btn.dataset.tabGroup;
      document.querySelectorAll(`[data-tab-group="${grp}"]`).forEach(b => b.classList.remove('tab-active'));
      document.querySelectorAll(`[data-panel-group="${grp}"]`).forEach(p => p.classList.remove('active'));
      btn.classList.add('tab-active');
      document.querySelector(`[data-panel="${btn.dataset.tab}"][data-panel-group="${grp}"]`).classList.add('active');
    });
  });
  // Ativa primeira tab de cada grupo
  const seen = new Set();
  document.querySelectorAll('[data-tab]').forEach(btn => {
    const grp = btn.dataset.tabGroup;
    if (!seen.has(grp)) { seen.add(grp); btn.click(); }
  });
  // Scroll fade-in
  const io = new IntersectionObserver(entries => {
    entries.forEach(e => { if (e.isIntersecting) { e.target.classList.add('visible'); io.unobserve(e.target); } });
  }, { threshold: 0.1 });
  document.querySelectorAll('[data-fade]').forEach(el => io.observe(el));
</script>
</body>
</html>"##;

// ─────────────────────────────────────────────────────────────────────────────
// NAVBAR
// ─────────────────────────────────────────────────────────────────────────────
const NAVBAR: &str = r##"<header class="navbar bg-base-100/80 backdrop-blur-xl sticky top-0 z-50 border-b border-base-200 px-4">
  <div class="navbar-start">
    <div class="dropdown">
      <label tabindex="0" class="btn btn-ghost btn-square lg:hidden">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h8m-8 6h16" />
        </svg>
      </label>
      <ul tabindex="0" class="menu menu-sm dropdown-content mt-3 z-[1] p-2 shadow-xl bg-base-100 rounded-box w-52 border border-base-200">
        <li><a href="#features">Features</a></li>
        <li><a href="#pricing">Preços</a></li>
        <li><a href="#testimonials">Depoimentos</a></li>
        <li><a href="#faq">FAQ</a></li>
      </ul>
    </div>
    <a href="#" class="btn btn-ghost text-xl font-black text-primary">[PRODUCT]</a>
  </div>
  <div class="navbar-center hidden lg:flex">
    <ul class="menu menu-horizontal gap-1">
      <li><a href="#features" class="font-medium rounded-lg">Features</a></li>
      <li><a href="#pricing" class="font-medium rounded-lg">Preços</a></li>
      <li><a href="#testimonials" class="font-medium rounded-lg">Depoimentos</a></li>
      <li><a href="#faq" class="font-medium rounded-lg">FAQ</a></li>
    </ul>
  </div>
  <div class="navbar-end gap-2">
    <a href="#" class="btn btn-ghost btn-sm hidden sm:flex">Entrar</a>
    <a href="#pricing" class="btn btn-primary btn-sm">Começar grátis</a>
  </div>
</header>"##;

// ─────────────────────────────────────────────────────────────────────────────
// HERO
// ─────────────────────────────────────────────────────────────────────────────
const HERO: &str = r##"<section id="hero" class="relative overflow-hidden bg-base-100 py-24 md:py-40">
  <div class="pointer-events-none absolute -top-1/4 right-0 h-[800px] w-[800px] rounded-full bg-primary/5 blur-3xl"></div>
  <div class="pointer-events-none absolute bottom-0 left-0 h-[600px] w-[600px] rounded-full bg-secondary/5 blur-3xl"></div>
  <div class="relative mx-auto max-w-5xl px-6 text-center">
    <div class="mb-8 inline-flex items-center gap-2 rounded-full border border-primary/20 bg-primary/5 px-4 py-2 text-sm font-medium text-primary" data-fade>
      <span class="flex h-2 w-2 rounded-full bg-primary animate-pulse"></span>
      Novo — agora disponível para todos
    </div>
    <h1 class="mb-6 text-6xl font-black leading-none tracking-tight md:text-8xl" data-fade>
      [PRODUCT]<br />
      <span class="bg-gradient-to-r from-primary to-secondary bg-clip-text text-transparent">done right.</span>
    </h1>
    <p class="mx-auto mb-12 max-w-2xl text-xl leading-relaxed text-base-content/60 md:text-2xl" data-fade>
      [TAGLINE]
    </p>
    <div class="flex flex-wrap items-center justify-center gap-4 mb-24" data-fade>
      <a href="#pricing" class="btn btn-primary btn-lg px-10 shadow-xl shadow-primary/20">
        Começar grátis
        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6" />
        </svg>
      </a>
      <a href="#features" class="btn btn-ghost btn-lg px-10">Ver como funciona →</a>
    </div>
    <div class="stats stats-vertical shadow-xl border border-base-200 bg-base-100 sm:stats-horizontal" data-fade>
      <div class="stat place-items-center px-10 py-6">
        <div class="stat-title text-xs font-semibold uppercase tracking-widest">Usuários ativos</div>
        <div class="stat-value text-4xl font-black text-primary">21M+</div>
        <div class="stat-desc">↗︎ 22% esse mês</div>
      </div>
      <div class="stat place-items-center px-10 py-6">
        <div class="stat-title text-xs font-semibold uppercase tracking-widest">Uptime SLA</div>
        <div class="stat-value text-4xl font-black text-secondary">99.9%</div>
        <div class="stat-desc">Confiabilidade enterprise</div>
      </div>
      <div class="stat place-items-center px-10 py-6">
        <div class="stat-title text-xs font-semibold uppercase tracking-widest">Integrações</div>
        <div class="stat-value text-4xl font-black text-accent">300+</div>
        <div class="stat-desc">Conecta com seu stack</div>
      </div>
    </div>
  </div>
</section>"##;

// ─────────────────────────────────────────────────────────────────────────────
// LOGOS
// ─────────────────────────────────────────────────────────────────────────────
const LOGOS: &str = r##"<section class="overflow-hidden border-y border-base-200 bg-base-200/40 py-12">
  <p class="mb-8 text-center text-xs font-semibold uppercase tracking-[0.2em] text-base-content/40">
    Confiado por equipes de empresas líderes
  </p>
  <div class="overflow-hidden">
    <div class="marquee-track flex w-max gap-20 px-10">
      <span class="text-2xl font-black text-base-content/20">Acme Corp</span>
      <span class="text-2xl font-black text-base-content/20">Globex Inc</span>
      <span class="text-2xl font-black text-base-content/20">Initech</span>
      <span class="text-2xl font-black text-base-content/20">Umbrella Co</span>
      <span class="text-2xl font-black text-base-content/20">Hooli</span>
      <span class="text-2xl font-black text-base-content/20">Pied Piper</span>
      <span class="text-2xl font-black text-base-content/20">Soylent Corp</span>
      <span class="text-2xl font-black text-base-content/20">Stark Industries</span>
      <span class="text-2xl font-black text-base-content/20">Wayne Ent.</span>
      <span class="text-2xl font-black text-base-content/20">Acme Corp</span>
      <span class="text-2xl font-black text-base-content/20">Globex Inc</span>
      <span class="text-2xl font-black text-base-content/20">Initech</span>
      <span class="text-2xl font-black text-base-content/20">Umbrella Co</span>
      <span class="text-2xl font-black text-base-content/20">Hooli</span>
      <span class="text-2xl font-black text-base-content/20">Pied Piper</span>
      <span class="text-2xl font-black text-base-content/20">Soylent Corp</span>
      <span class="text-2xl font-black text-base-content/20">Stark Industries</span>
      <span class="text-2xl font-black text-base-content/20">Wayne Ent.</span>
    </div>
  </div>
</section>"##;

// ─────────────────────────────────────────────────────────────────────────────
// FEATURES GRID
// ─────────────────────────────────────────────────────────────────────────────
const FEATURES_GRID: &str = r##"<section id="features" class="py-28 bg-base-100">
  <div class="mx-auto max-w-7xl px-6">
    <div class="mx-auto mb-16 max-w-2xl text-center" data-fade>
      <div class="badge badge-outline badge-primary mb-4">Features</div>
      <h2 class="text-4xl font-bold tracking-tight md:text-5xl mb-4">
        Tudo que você precisa.<br />Nada do que não.
      </h2>
      <p class="text-lg text-base-content/60">
        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor
        incididunt ut labore et dolore magna aliqua ut enim ad minim veniam.
      </p>
    </div>
    <div class="grid gap-6 sm:grid-cols-2 lg:grid-cols-4">
      <div class="card border border-base-200 bg-base-100 transition-all duration-300 hover:-translate-y-2 hover:shadow-xl" data-fade>
        <div class="card-body">
          <div class="mb-4 flex h-12 w-12 items-center justify-center rounded-xl bg-primary/10">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 text-primary" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
            </svg>
          </div>
          <h3 class="card-title text-lg">Velocidade</h3>
          <p class="text-sm text-base-content/60">Lorem ipsum dolor sit amet consectetur adipiscing elit. Quisque vel diam vitae urna fermentum imperdiet.</p>
        </div>
      </div>
      <div class="card border border-base-200 bg-base-100 transition-all duration-300 hover:-translate-y-2 hover:shadow-xl" data-fade>
        <div class="card-body">
          <div class="mb-4 flex h-12 w-12 items-center justify-center rounded-xl bg-secondary/10">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 text-secondary" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
            </svg>
          </div>
          <h3 class="card-title text-lg">Segurança</h3>
          <p class="text-sm text-base-content/60">Ut enim ad minim veniam quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo.</p>
        </div>
      </div>
      <div class="card border border-base-200 bg-base-100 transition-all duration-300 hover:-translate-y-2 hover:shadow-xl" data-fade>
        <div class="card-body">
          <div class="mb-4 flex h-12 w-12 items-center justify-center rounded-xl bg-accent/10">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 text-accent" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
            </svg>
          </div>
          <h3 class="card-title text-lg">Analytics</h3>
          <p class="text-sm text-base-content/60">Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla.</p>
        </div>
      </div>
      <div class="card border border-base-200 bg-base-100 transition-all duration-300 hover:-translate-y-2 hover:shadow-xl" data-fade>
        <div class="card-body">
          <div class="mb-4 flex h-12 w-12 items-center justify-center rounded-xl bg-success/10">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 text-success" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0z" />
            </svg>
          </div>
          <h3 class="card-title text-lg">Colaboração</h3>
          <p class="text-sm text-base-content/60">Excepteur sint occaecat cupidatat non proident sunt in culpa qui officia deserunt mollit anim.</p>
        </div>
      </div>
    </div>
  </div>
</section>"##;

// ─────────────────────────────────────────────────────────────────────────────
// FEATURES TABS  (mock-browser + checklist)
// ─────────────────────────────────────────────────────────────────────────────
const FEATURES_TABS: &str = r##"<section class="py-28 bg-base-200/30">
  <div class="mx-auto max-w-7xl px-6">
    <div class="mx-auto mb-16 max-w-2xl text-center" data-fade>
      <div class="badge badge-outline mb-4">Como funciona</div>
      <h2 class="text-4xl font-bold tracking-tight md:text-5xl mb-4">Crie. Otimize. Escale.</h2>
      <p class="text-lg text-base-content/60">Lorem ipsum dolor sit amet consectetur adipiscing elit. Sed do eiusmod tempor incididunt.</p>
    </div>
    <div class="tabs tabs-boxed justify-center bg-base-200 p-1 w-fit mx-auto mb-12" data-fade>
      <button class="tab" data-tab="criar" data-tab-group="features">Criar</button>
      <button class="tab" data-tab="otimizar" data-tab-group="features">Otimizar</button>
      <button class="tab" data-tab="escalar" data-tab-group="features">Escalar</button>
    </div>

    <div data-panel="criar" data-panel-group="features">
      <div class="grid gap-12 lg:grid-cols-2 items-center">
        <div data-fade>
          <div class="badge badge-primary mb-4">Criar</div>
          <h3 class="text-3xl font-bold mb-4">Comece em minutos, não em dias</h3>
          <p class="text-base-content/60 mb-6 leading-relaxed">Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam quis nostrud exercitation.</p>
          <ul class="space-y-3">
            <li class="flex items-center gap-3 text-sm"><span class="flex h-6 w-6 shrink-0 items-center justify-center rounded-full bg-primary/10 text-primary">✓</span>Templates profissionais prontos para usar</li>
            <li class="flex items-center gap-3 text-sm"><span class="flex h-6 w-6 shrink-0 items-center justify-center rounded-full bg-primary/10 text-primary">✓</span>Drag-and-drop sem código</li>
            <li class="flex items-center gap-3 text-sm"><span class="flex h-6 w-6 shrink-0 items-center justify-center rounded-full bg-primary/10 text-primary">✓</span>Publicação com 1 clique</li>
          </ul>
        </div>
        <div class="rounded-2xl border border-base-300 overflow-hidden shadow-2xl" data-fade>
          <div class="flex items-center gap-1.5 bg-base-300 px-4 py-3">
            <div class="h-3 w-3 rounded-full bg-error/60"></div>
            <div class="h-3 w-3 rounded-full bg-warning/60"></div>
            <div class="h-3 w-3 rounded-full bg-success/60"></div>
            <div class="ml-3 h-5 flex-1 rounded-full bg-base-200"></div>
          </div>
          <div class="flex min-h-80 items-center justify-center bg-gradient-to-br from-primary/10 to-secondary/5 p-10">
            <div class="text-center opacity-20">
              <svg xmlns="http://www.w3.org/2000/svg" class="mx-auto mb-4 h-20 w-20" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1" d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
              </svg>
              <p class="text-sm font-bold">Editor Preview</p>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div data-panel="otimizar" data-panel-group="features">
      <div class="grid gap-12 lg:grid-cols-2 items-center">
        <div data-fade>
          <div class="badge badge-secondary mb-4">Otimizar</div>
          <h3 class="text-3xl font-bold mb-4">Performance que faz diferença</h3>
          <p class="text-base-content/60 mb-6 leading-relaxed">Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit.</p>
          <ul class="space-y-3">
            <li class="flex items-center gap-3 text-sm"><span class="flex h-6 w-6 shrink-0 items-center justify-center rounded-full bg-secondary/10 text-secondary">✓</span>Compressão automática de imagens</li>
            <li class="flex items-center gap-3 text-sm"><span class="flex h-6 w-6 shrink-0 items-center justify-center rounded-full bg-secondary/10 text-secondary">✓</span>Core Web Vitals otimizados</li>
            <li class="flex items-center gap-3 text-sm"><span class="flex h-6 w-6 shrink-0 items-center justify-center rounded-full bg-secondary/10 text-secondary">✓</span>CDN global com 300+ edge nodes</li>
          </ul>
        </div>
        <div class="rounded-2xl border border-base-300 overflow-hidden shadow-2xl" data-fade>
          <div class="flex items-center gap-1.5 bg-base-300 px-4 py-3">
            <div class="h-3 w-3 rounded-full bg-error/60"></div>
            <div class="h-3 w-3 rounded-full bg-warning/60"></div>
            <div class="h-3 w-3 rounded-full bg-success/60"></div>
            <div class="ml-3 h-5 flex-1 rounded-full bg-base-200"></div>
          </div>
          <div class="flex min-h-80 items-center justify-center bg-gradient-to-br from-secondary/10 to-accent/5 p-10">
            <div class="text-center opacity-20">
              <svg xmlns="http://www.w3.org/2000/svg" class="mx-auto mb-4 h-20 w-20" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
              </svg>
              <p class="text-sm font-bold">Analytics Preview</p>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div data-panel="escalar" data-panel-group="features">
      <div class="grid gap-12 lg:grid-cols-2 items-center">
        <div data-fade>
          <div class="badge badge-accent mb-4">Escalar</div>
          <h3 class="text-3xl font-bold mb-4">Crescimento sem limites</h3>
          <p class="text-base-content/60 mb-6 leading-relaxed">Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum. Sed ut perspiciatis unde omnis iste natus.</p>
          <ul class="space-y-3">
            <li class="flex items-center gap-3 text-sm"><span class="flex h-6 w-6 shrink-0 items-center justify-center rounded-full bg-accent/10 text-accent">✓</span>Infraestrutura auto-escalável</li>
            <li class="flex items-center gap-3 text-sm"><span class="flex h-6 w-6 shrink-0 items-center justify-center rounded-full bg-accent/10 text-accent">✓</span>Multi-região e multi-cloud</li>
            <li class="flex items-center gap-3 text-sm"><span class="flex h-6 w-6 shrink-0 items-center justify-center rounded-full bg-accent/10 text-accent">✓</span>SLA 99.99% com suporte 24/7</li>
          </ul>
        </div>
        <div class="rounded-2xl border border-base-300 overflow-hidden shadow-2xl" data-fade>
          <div class="flex items-center gap-1.5 bg-base-300 px-4 py-3">
            <div class="h-3 w-3 rounded-full bg-error/60"></div>
            <div class="h-3 w-3 rounded-full bg-warning/60"></div>
            <div class="h-3 w-3 rounded-full bg-success/60"></div>
            <div class="ml-3 h-5 flex-1 rounded-full bg-base-200"></div>
          </div>
          <div class="flex min-h-80 items-center justify-center bg-gradient-to-br from-accent/10 to-primary/5 p-10">
            <div class="text-center opacity-20">
              <svg xmlns="http://www.w3.org/2000/svg" class="mx-auto mb-4 h-20 w-20" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1" d="M3.055 11H5a2 2 0 012 2v1a2 2 0 002 2 2 2 0 012 2v2.945M8 3.935V5.5A2.5 2.5 0 0010.5 8h.5a2 2 0 012 2 2 2 0 104 0 2 2 0 012-2h1.064M15 20.488V18a2 2 0 012-2h3.064M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
              <p class="text-sm font-bold">Infrastructure Preview</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</section>"##;

// ─────────────────────────────────────────────────────────────────────────────
// STATS  (fundo colorido, 4 números grandes)
// ─────────────────────────────────────────────────────────────────────────────
const STATS: &str = r##"<section class="py-24 bg-primary text-primary-content">
  <div class="mx-auto max-w-7xl px-6">
    <div class="mx-auto mb-14 max-w-xl text-center" data-fade>
      <h2 class="text-4xl font-bold tracking-tight md:text-5xl mb-4">Números que falam por si</h2>
      <p class="text-primary-content/70">Lorem ipsum dolor sit amet consectetur adipiscing elit. Sed do eiusmod.</p>
    </div>
    <div class="grid grid-cols-2 gap-10 lg:grid-cols-4">
      <div class="text-center" data-fade>
        <div class="text-5xl md:text-6xl font-black mb-2">21M+</div>
        <div class="text-xs font-semibold text-primary-content/70 uppercase tracking-widest">Usuários ativos</div>
      </div>
      <div class="text-center" data-fade>
        <div class="text-5xl md:text-6xl font-black mb-2">300+</div>
        <div class="text-xs font-semibold text-primary-content/70 uppercase tracking-widest">Integrações</div>
      </div>
      <div class="text-center" data-fade>
        <div class="text-5xl md:text-6xl font-black mb-2">99.9%</div>
        <div class="text-xs font-semibold text-primary-content/70 uppercase tracking-widest">Uptime SLA</div>
      </div>
      <div class="text-center" data-fade>
        <div class="text-5xl md:text-6xl font-black mb-2">4.9★</div>
        <div class="text-xs font-semibold text-primary-content/70 uppercase tracking-widest">Avaliação média</div>
      </div>
    </div>
  </div>
</section>"##;

// ─────────────────────────────────────────────────────────────────────────────
// TESTIMONIALS
// ─────────────────────────────────────────────────────────────────────────────
const TESTIMONIALS: &str = r##"<section id="testimonials" class="py-28 bg-base-100">
  <div class="mx-auto max-w-7xl px-6">
    <div class="mx-auto mb-16 max-w-2xl text-center" data-fade>
      <div class="badge badge-outline mb-4">Depoimentos</div>
      <h2 class="text-4xl font-bold tracking-tight md:text-5xl mb-4">Amado por quem constrói a web</h2>
      <p class="text-lg text-base-content/60">Lorem ipsum dolor sit amet, consectetur adipiscing elit.</p>
    </div>
    <div class="grid gap-6 md:grid-cols-3">
      <div class="card border border-base-200 bg-base-100 shadow-sm" data-fade>
        <div class="card-body">
          <div class="text-warning text-lg mb-4">★★★★★</div>
          <p class="text-base-content/70 italic mb-6 leading-relaxed text-sm">
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam quis nostrud exercitation."
          </p>
          <div class="flex items-center gap-3">
            <div class="avatar placeholder">
              <div class="bg-primary text-primary-content rounded-full w-10">
                <span class="text-xs font-bold">NA</span>
              </div>
            </div>
            <div>
              <p class="font-semibold text-sm">Nadine Araujo</p>
              <p class="text-xs text-base-content/50">Fundadora & Diretora Criativa, Studio NA</p>
            </div>
          </div>
        </div>
      </div>
      <div class="card border-2 border-primary bg-primary/5 shadow-sm" data-fade>
        <div class="card-body">
          <div class="text-warning text-lg mb-4">★★★★★</div>
          <p class="text-base-content/70 italic mb-6 leading-relaxed text-sm">
            "Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate."
          </p>
          <div class="flex items-center gap-3">
            <div class="avatar placeholder">
              <div class="bg-secondary text-secondary-content rounded-full w-10">
                <span class="text-xs font-bold">RB</span>
              </div>
            </div>
            <div>
              <p class="font-semibold text-sm">Rafael Brava</p>
              <p class="text-xs text-base-content/50">Diretor, Brava Design Agency</p>
            </div>
          </div>
        </div>
      </div>
      <div class="card border border-base-200 bg-base-100 shadow-sm" data-fade>
        <div class="card-body">
          <div class="text-warning text-lg mb-4">★★★★★</div>
          <p class="text-base-content/70 italic mb-6 leading-relaxed text-sm">
            "Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum. Sed ut perspiciatis unde omnis iste natus error."
          </p>
          <div class="flex items-center gap-3">
            <div class="avatar placeholder">
              <div class="bg-accent text-accent-content rounded-full w-10">
                <span class="text-xs font-bold">MS</span>
              </div>
            </div>
            <div>
              <p class="font-semibold text-sm">Marina Santos</p>
              <p class="text-xs text-base-content/50">Web Creator & Designer, MS Studio</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</section>"##;

// ─────────────────────────────────────────────────────────────────────────────
// PRICING
// ─────────────────────────────────────────────────────────────────────────────
const PRICING: &str = r##"<section id="pricing" class="py-28 bg-base-200/30">
  <div class="mx-auto max-w-7xl px-6">
    <div class="mx-auto mb-16 max-w-2xl text-center" data-fade>
      <div class="badge badge-outline mb-4">Preços</div>
      <h2 class="text-4xl font-bold tracking-tight md:text-5xl mb-4">
        Comece grátis. Escale conforme cresce.
      </h2>
      <p class="text-lg text-base-content/60">Sem surpresas no cartão. Cancele quando quiser.</p>
    </div>
    <div class="grid gap-8 lg:grid-cols-3 items-stretch">
      <!-- Free -->
      <div class="card border border-base-200 bg-base-100" data-fade>
        <div class="card-body p-8 flex flex-col">
          <div class="badge badge-ghost mb-4 w-fit">Grátis</div>
          <div class="mb-6">
            <span class="text-5xl font-black">R$0</span>
            <span class="text-base-content/50 text-sm">/mês</span>
          </div>
          <p class="text-sm text-base-content/60 mb-6">Lorem ipsum dolor sit amet consectetur adipiscing.</p>
          <ul class="space-y-2.5 mb-8 flex-1">
            <li class="flex items-center gap-2 text-sm"><span class="text-success font-bold">✓</span> 3 projetos ativos</li>
            <li class="flex items-center gap-2 text-sm"><span class="text-success font-bold">✓</span> 1 GB de armazenamento</li>
            <li class="flex items-center gap-2 text-sm"><span class="text-success font-bold">✓</span> Suporte via comunidade</li>
            <li class="flex items-center gap-2 text-sm text-base-content/30"><span>✕</span> Analytics avançado</li>
            <li class="flex items-center gap-2 text-sm text-base-content/30"><span>✕</span> Custom domains</li>
          </ul>
          <a href="#" class="btn btn-outline w-full">Começar grátis</a>
        </div>
      </div>
      <!-- Pro -->
      <div class="card border-2 border-primary bg-base-100 shadow-xl shadow-primary/10 relative" data-fade>
        <div class="absolute -top-4 left-1/2 -translate-x-1/2 whitespace-nowrap">
          <div class="badge badge-primary px-4 py-3 text-xs font-semibold shadow-lg">✦ Mais popular</div>
        </div>
        <div class="card-body p-8 flex flex-col">
          <div class="badge badge-primary mb-4 w-fit">Pro</div>
          <div class="mb-6">
            <span class="text-5xl font-black">R$97</span>
            <span class="text-base-content/50 text-sm">/mês</span>
          </div>
          <p class="text-sm text-base-content/60 mb-6">Ut enim ad minim veniam quis nostrud exercitation.</p>
          <ul class="space-y-2.5 mb-8 flex-1">
            <li class="flex items-center gap-2 text-sm"><span class="text-primary font-bold">✓</span> Projetos ilimitados</li>
            <li class="flex items-center gap-2 text-sm"><span class="text-primary font-bold">✓</span> 50 GB de armazenamento</li>
            <li class="flex items-center gap-2 text-sm"><span class="text-primary font-bold">✓</span> Suporte prioritário</li>
            <li class="flex items-center gap-2 text-sm"><span class="text-primary font-bold">✓</span> Analytics avançado</li>
            <li class="flex items-center gap-2 text-sm"><span class="text-primary font-bold">✓</span> Custom domains</li>
          </ul>
          <a href="#" class="btn btn-primary w-full">Assinar Pro</a>
        </div>
      </div>
      <!-- Enterprise -->
      <div class="card border border-base-200 bg-base-100" data-fade>
        <div class="card-body p-8 flex flex-col">
          <div class="badge badge-ghost mb-4 w-fit">Enterprise</div>
          <div class="mb-6">
            <span class="text-5xl font-black">Custom</span>
          </div>
          <p class="text-sm text-base-content/60 mb-6">Duis aute irure dolor in reprehenderit in voluptate.</p>
          <ul class="space-y-2.5 mb-8 flex-1">
            <li class="flex items-center gap-2 text-sm"><span class="text-success font-bold">✓</span> Tudo no Pro</li>
            <li class="flex items-center gap-2 text-sm"><span class="text-success font-bold">✓</span> SSO e SAML</li>
            <li class="flex items-center gap-2 text-sm"><span class="text-success font-bold">✓</span> SLA 99.99%</li>
            <li class="flex items-center gap-2 text-sm"><span class="text-success font-bold">✓</span> Gestor de conta dedicado</li>
            <li class="flex items-center gap-2 text-sm"><span class="text-success font-bold">✓</span> Treinamento & onboarding</li>
          </ul>
          <a href="#" class="btn btn-outline w-full">Falar com vendas</a>
        </div>
      </div>
    </div>
  </div>
</section>"##;

// ─────────────────────────────────────────────────────────────────────────────
// FAQ
// ─────────────────────────────────────────────────────────────────────────────
const FAQ: &str = r##"<section id="faq" class="py-28 bg-base-100">
  <div class="mx-auto max-w-3xl px-6">
    <div class="mb-16 text-center" data-fade>
      <div class="badge badge-outline mb-4">FAQ</div>
      <h2 class="text-4xl font-bold tracking-tight md:text-5xl mb-4">Perguntas frequentes</h2>
      <p class="text-lg text-base-content/60">Não encontrou o que procura? <a href="#" class="link link-primary">Entre em contato.</a></p>
    </div>
    <div class="space-y-4" data-fade>
      <div class="collapse collapse-plus border border-base-200 bg-base-100 rounded-xl">
        <input type="radio" name="faq-acc" checked />
        <div class="collapse-title font-semibold">Lorem ipsum dolor sit amet consectetur adipiscing elit?</div>
        <div class="collapse-content text-base-content/60 leading-relaxed text-sm">
          Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.
        </div>
      </div>
      <div class="collapse collapse-plus border border-base-200 bg-base-100 rounded-xl">
        <input type="radio" name="faq-acc" />
        <div class="collapse-title font-semibold">Ut enim ad minim veniam quis nostrud exercitation ullamco?</div>
        <div class="collapse-content text-base-content/60 leading-relaxed text-sm">
          Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident sunt in culpa qui officia deserunt mollit anim.
        </div>
      </div>
      <div class="collapse collapse-plus border border-base-200 bg-base-100 rounded-xl">
        <input type="radio" name="faq-acc" />
        <div class="collapse-title font-semibold">Duis aute irure dolor in reprehenderit in voluptate?</div>
        <div class="collapse-content text-base-content/60 leading-relaxed text-sm">
          Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium totam rem aperiam eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta.
        </div>
      </div>
      <div class="collapse collapse-plus border border-base-200 bg-base-100 rounded-xl">
        <input type="radio" name="faq-acc" />
        <div class="collapse-title font-semibold">Nemo enim ipsam voluptatem quia voluptas sit aspernatur?</div>
        <div class="collapse-content text-base-content/60 leading-relaxed text-sm">
          At vero eos et accusamus et iusto odio dignissimos ducimus qui blanditiis praesentium voluptatum deleniti atque corrupti quos dolores et quas molestias excepturi sint occaecati cupiditate.
        </div>
      </div>
      <div class="collapse collapse-plus border border-base-200 bg-base-100 rounded-xl">
        <input type="radio" name="faq-acc" />
        <div class="collapse-title font-semibold">Quis autem vel eum iure reprehenderit qui in ea voluptate?</div>
        <div class="collapse-content text-base-content/60 leading-relaxed text-sm">
          Nam libero tempore cum soluta nobis est eligendi optio cumque nihil impedit quo minus id quod maxime placeat facere possimus omnis voluptas assumenda est omnis dolor repellendus.
        </div>
      </div>
    </div>
  </div>
</section>"##;

// ─────────────────────────────────────────────────────────────────────────────
// CTA BOTTOM
// ─────────────────────────────────────────────────────────────────────────────
const CTA_BOTTOM: &str = r##"<section class="py-28 bg-gradient-to-br from-primary to-secondary relative overflow-hidden">
  <div class="pointer-events-none absolute inset-0 opacity-10" style="background-image: radial-gradient(circle at 1px 1px, white 1px, transparent 0); background-size: 40px 40px;"></div>
  <div class="relative mx-auto max-w-3xl px-6 text-center text-primary-content" data-fade>
    <h2 class="text-4xl font-black tracking-tight md:text-6xl mb-6">
      Pronto para começar<br />com [PRODUCT]?
    </h2>
    <p class="text-xl text-primary-content/80 mb-10 max-w-xl mx-auto leading-relaxed">
      Junte-se a mais de 21 milhões de usuários que já constroem o futuro. 30 dias grátis, sem cartão.
    </p>
    <div class="flex flex-wrap gap-4 justify-center">
      <a href="#pricing" class="btn btn-lg bg-white text-primary hover:bg-white/90 border-0 px-10 shadow-2xl">
        Começar agora — é grátis
      </a>
      <a href="#features" class="btn btn-lg btn-ghost text-white border-white/30 hover:bg-white/10 px-10">
        Saiba mais →
      </a>
    </div>
    <p class="mt-8 text-sm text-primary-content/60">
      ✓ Sem cartão de crédito &nbsp;&nbsp; ✓ Cancele quando quiser &nbsp;&nbsp; ✓ Suporte gratuito
    </p>
  </div>
</section>"##;

// ─────────────────────────────────────────────────────────────────────────────
// FOOTER
// ─────────────────────────────────────────────────────────────────────────────
const FOOTER: &str = r##"<footer class="bg-base-200 pt-16 pb-8">
  <div class="mx-auto max-w-7xl px-6">
    <div class="grid gap-8 sm:grid-cols-2 lg:grid-cols-5 mb-12">
      <div class="lg:col-span-2">
        <a href="#" class="text-2xl font-black text-primary">[COMPANY]</a>
        <p class="mt-4 text-sm text-base-content/60 max-w-xs leading-relaxed">
          Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
        </p>
        <div class="mt-6 flex gap-3">
          <a href="#" class="btn btn-ghost btn-sm btn-square" aria-label="Twitter/X">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="currentColor" viewBox="0 0 24 24">
              <path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.084 4.126H5.117z"/>
            </svg>
          </a>
          <a href="#" class="btn btn-ghost btn-sm btn-square" aria-label="LinkedIn">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="currentColor" viewBox="0 0 24 24">
              <path d="M20.447 20.452h-3.554v-5.569c0-1.328-.027-3.037-1.852-3.037-1.853 0-2.136 1.445-2.136 2.939v5.667H9.351V9h3.414v1.561h.046c.477-.9 1.637-1.85 3.37-1.85 3.601 0 4.267 2.37 4.267 5.455v6.286zM5.337 7.433a2.062 2.062 0 01-2.063-2.065 2.064 2.064 0 112.063 2.065zm1.782 13.019H3.555V9h3.564v11.452zM22.225 0H1.771C.792 0 0 .774 0 1.729v20.542C0 23.227.792 24 1.771 24h20.451C23.2 24 24 23.227 24 22.271V1.729C24 .774 23.2 0 22.222 0h.003z"/>
            </svg>
          </a>
          <a href="#" class="btn btn-ghost btn-sm btn-square" aria-label="GitHub">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="currentColor" viewBox="0 0 24 24">
              <path d="M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.399 3-.405 1.02.006 2.04.138 3 .405 2.28-1.552 3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.42.36.81 1.096.81 2.22 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12"/>
            </svg>
          </a>
        </div>
      </div>
      <div>
        <h3 class="font-semibold mb-4 text-xs uppercase tracking-widest text-base-content/60">Produto</h3>
        <ul class="space-y-3">
          <li><a href="#features" class="text-sm text-base-content/60 hover:text-primary transition-colors">Features</a></li>
          <li><a href="#pricing" class="text-sm text-base-content/60 hover:text-primary transition-colors">Preços</a></li>
          <li><a href="#" class="text-sm text-base-content/60 hover:text-primary transition-colors">Changelog</a></li>
          <li><a href="#" class="text-sm text-base-content/60 hover:text-primary transition-colors">Roadmap</a></li>
        </ul>
      </div>
      <div>
        <h3 class="font-semibold mb-4 text-xs uppercase tracking-widest text-base-content/60">Empresa</h3>
        <ul class="space-y-3">
          <li><a href="#" class="text-sm text-base-content/60 hover:text-primary transition-colors">Sobre nós</a></li>
          <li><a href="#" class="text-sm text-base-content/60 hover:text-primary transition-colors">Blog</a></li>
          <li><a href="#" class="text-sm text-base-content/60 hover:text-primary transition-colors">Carreiras</a></li>
          <li><a href="#" class="text-sm text-base-content/60 hover:text-primary transition-colors">Contato</a></li>
        </ul>
      </div>
      <div>
        <h3 class="font-semibold mb-4 text-xs uppercase tracking-widest text-base-content/60">Legal</h3>
        <ul class="space-y-3">
          <li><a href="#" class="text-sm text-base-content/60 hover:text-primary transition-colors">Termos de uso</a></li>
          <li><a href="#" class="text-sm text-base-content/60 hover:text-primary transition-colors">Privacidade</a></li>
          <li><a href="#" class="text-sm text-base-content/60 hover:text-primary transition-colors">Cookies</a></li>
        </ul>
      </div>
    </div>
    <div class="border-t border-base-300 pt-8 flex flex-col sm:flex-row justify-between items-center gap-4">
      <p class="text-sm text-base-content/40">© 2025 [COMPANY]. Todos os direitos reservados.</p>
      <p class="text-sm text-base-content/30">Feito com ♥ usando Tailwind CSS + DaisyUI</p>
    </div>
  </div>
</footer>"##;
