/// Layout "SaaS / Serviço" — inspirado no padrão Contabilizei:
/// navbar branca sticky · hero com jornadas · barra de prova social ·
/// tabela comparativa vs mercado · seletor de jornada com imagem ·
/// slider de benefícios · grid de artigos · depoimentos com foto ·
/// FAQ accordion · footer colunar com redes sociais

fn s(tmpl: &str, pairs: &[(&str, &str)]) -> String {
    let mut o = tmpl.to_string();
    for (k, v) in pairs { o = o.replace(k, v); }
    o
}

// ── Shell (sempre branco — ignora tema DaisyUI para o layout saas) ────────────
const SHELL_SAAS: &str = r##"<!DOCTYPE html>
<html lang="pt-BR">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>[TITLE]</title>
  <link href="https://cdn.jsdelivr.net/npm/daisyui@4.12.10/dist/full.min.css" rel="stylesheet"/>
  <script src="https://cdn.tailwindcss.com"></script>
  <style>
    :root { --brand:#00C86A; --brand-dark:#00A357; }
    html  { scroll-behavior:smooth; font-family:'Inter',system-ui,sans-serif; background:#fff; }
    body  { color:#1a1a2e; }
    .btn-brand { background:var(--brand); color:#fff; border:none; }
    .btn-brand:hover { background:var(--brand-dark); color:#fff; }
    .text-brand  { color:var(--brand); }
    .border-brand{ border-color:var(--brand); }
    .bg-brand    { background:var(--brand); }
    /* Sticky nav shadow on scroll */
    .nav-scrolled { box-shadow:0 2px 12px rgba(0,0,0,.08); }
    /* Journey tabs */
    .journey-tab.active  { border-bottom:3px solid var(--brand); color:var(--brand); font-weight:700; }
    .journey-panel       { display:none; }
    .journey-panel.active{ display:flex; }
    /* Benefit slider */
    .benefit-track { display:flex; gap:1.5rem; overflow-x:auto; scroll-snap-type:x mandatory; scrollbar-width:none; }
    .benefit-track::-webkit-scrollbar { display:none; }
    .benefit-card  { min-width:280px; scroll-snap-align:start; }
    /* Fade in */
    [data-fade]{ opacity:0; transform:translateY(20px); transition:opacity .6s ease,transform .6s ease; }
    [data-fade].visible{ opacity:1; transform:none; }
  </style>
</head>
<body class="antialiased">
[BODY]
<script>
  /* sticky nav */
  const nav = document.getElementById('saas-nav');
  window.addEventListener('scroll', () => nav?.classList.toggle('nav-scrolled', scrollY > 10));

  /* journey tabs */
  document.querySelectorAll('.journey-tab').forEach(tab => {
    tab.addEventListener('click', () => {
      document.querySelectorAll('.journey-tab').forEach(t => t.classList.remove('active'));
      document.querySelectorAll('.journey-panel').forEach(p => p.classList.remove('active'));
      tab.classList.add('active');
      document.getElementById('panel-' + tab.dataset.journey)?.classList.add('active');
    });
  });
  document.querySelector('.journey-tab')?.click();

  /* fade-in observer */
  const observer = new IntersectionObserver(entries => {
    entries.forEach(e => { if(e.isIntersecting) e.target.classList.add('visible'); });
  }, { threshold: 0.1 });
  document.querySelectorAll('[data-fade]').forEach(el => observer.observe(el));
</script>
</body>
</html>
"##;

pub fn html_shell_saas(title: &str, body: &str) -> String {
    SHELL_SAAS.replace("[TITLE]", title).replace("[BODY]", body)
}

// ── Navbar ────────────────────────────────────────────────────────────────────
const NAVBAR_SAAS: &str = r##"<header id="saas-nav" class="fixed top-0 left-0 right-0 z-50 bg-white transition-shadow duration-300">
  <div class="max-w-7xl mx-auto px-6 h-16 flex items-center justify-between">
    <a href="#" class="text-2xl font-black text-brand">[PRODUCT]</a>
    <nav class="hidden md:flex items-center gap-8 text-sm font-medium text-gray-600">
      <a href="#solucoes" class="hover:text-brand transition-colors">Soluções</a>
      <a href="#beneficios" class="hover:text-brand transition-colors">Benefícios</a>
      <a href="#precos"    class="hover:text-brand transition-colors">Preços</a>
      <a href="#blog"      class="hover:text-brand transition-colors">Blog</a>
      <a href="#faq"       class="hover:text-brand transition-colors">FAQ</a>
    </nav>
    <div class="flex items-center gap-3">
      <a href="#login"  class="hidden md:inline text-sm font-medium text-gray-600 hover:text-brand transition-colors">Entrar</a>
      <a href="#contato" class="btn btn-brand btn-sm rounded-full px-5 text-sm font-semibold">Começar grátis</a>
    </div>
  </div>
</header>
"##;

pub fn section_navbar_saas(product: &str) -> String {
    NAVBAR_SAAS.replace("[PRODUCT]", product)
}

// ── Hero ─────────────────────────────────────────────────────────────────────
const HERO_SAAS: &str = r##"<section class="pt-28 pb-16 bg-white" id="inicio">
  <div class="max-w-7xl mx-auto px-6">
    <div class="text-center max-w-3xl mx-auto mb-12" data-fade>
      <span class="inline-block bg-green-50 text-brand text-xs font-bold px-3 py-1 rounded-full mb-4 uppercase tracking-wider">Novidade</span>
      <h1 class="text-4xl md:text-5xl lg:text-6xl font-black text-gray-900 leading-tight mb-4">
        [HEADLINE]
      </h1>
      <p class="text-lg text-gray-500 mb-6">[TAGLINE]</p>
      <p class="text-sm font-semibold text-gray-700">Planos a partir de <span class="text-brand text-xl font-black">R$ 99/mês</span></p>
    </div>

    <!-- 3 jornadas do usuário -->
    <div class="grid grid-cols-1 md:grid-cols-3 gap-6 max-w-5xl mx-auto" data-fade>

      <div class="card bg-base-100 shadow-lg border border-gray-100 hover:border-brand transition-colors cursor-pointer group p-6">
        <div class="w-10 h-10 bg-green-50 rounded-xl flex items-center justify-center mb-4 group-hover:bg-brand group-hover:text-white transition-colors text-brand">
          <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"/></svg>
        </div>
        <h3 class="font-bold text-gray-900 mb-2">Abrir minha empresa</h3>
        <p class="text-sm text-gray-500 mb-4">Saia da informalidade e abra seu CNPJ de forma online, rápida e sem burocracia.</p>
        <a href="#contato" class="text-brand text-sm font-semibold hover:underline flex items-center gap-1">
          Começar agora <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/></svg>
        </a>
      </div>

      <div class="card bg-base-100 shadow-lg border border-gray-100 hover:border-brand transition-colors cursor-pointer group p-6 md:ring-2 md:ring-brand">
        <div class="w-10 h-10 bg-green-50 rounded-xl flex items-center justify-center mb-4 group-hover:bg-brand group-hover:text-white transition-colors text-brand">
          <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/></svg>
        </div>
        <span class="inline-block bg-brand text-white text-xs px-2 py-0.5 rounded-full mb-2 font-bold">Mais popular</span>
        <h3 class="font-bold text-gray-900 mb-2">Assessoria completa</h3>
        <p class="text-sm text-gray-500 mb-4">Migre do MEI, troque de contador ou formalize sua PF com suporte especializado.</p>
        <a href="#contato" class="text-brand text-sm font-semibold hover:underline flex items-center gap-1">
          Fale com especialista <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/></svg>
        </a>
      </div>

      <div class="card bg-base-100 shadow-lg border border-gray-100 hover:border-brand transition-colors cursor-pointer group p-6">
        <div class="w-10 h-10 bg-green-50 rounded-xl flex items-center justify-center mb-4 group-hover:bg-brand group-hover:text-white transition-colors text-brand">
          <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"/></svg>
        </div>
        <h3 class="font-bold text-gray-900 mb-2">Educação financeira</h3>
        <p class="text-sm text-gray-500 mb-4">Cursos gratuitos para organizar finanças, aprender sobre IA e impulsionar seu negócio.</p>
        <a href="#blog" class="text-brand text-sm font-semibold hover:underline flex items-center gap-1">
          Acesse grátis <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/></svg>
        </a>
      </div>

    </div>
  </div>
</section>
"##;

pub fn section_hero_saas(product: &str, tagline: &str) -> String {
    s(HERO_SAAS, &[("[HEADLINE]", product), ("[TAGLINE]", tagline)])
}

// ── Barra de Prova Social ────────────────────────────────────────────────────
const SOCIAL_PROOF: &str = r##"<section class="bg-gray-50 border-y border-gray-100 py-5">
  <div class="max-w-7xl mx-auto px-6 flex flex-wrap items-center justify-center gap-8" data-fade>

    <div class="flex items-center gap-2 text-sm font-medium text-gray-600">
      <span class="w-8 h-8 bg-brand rounded-full flex items-center justify-center text-white text-xs font-black">★</span>
      <span>RA1000 — Empresa verificada</span>
    </div>

    <div class="flex items-center gap-2 text-sm font-medium text-gray-600">
      <svg class="w-5 h-5" viewBox="0 0 24 24"><path fill="#4285F4" d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z"/><path fill="#34A853" d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z"/><path fill="#FBBC05" d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z"/><path fill="#EA4335" d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z"/></svg>
      <span><strong>4.7</strong> no Google · +2.000 avaliações</span>
    </div>

    <div class="flex items-center gap-2 text-sm font-medium text-gray-600">
      <svg class="w-5 h-5 text-brand" fill="currentColor" viewBox="0 0 20 20"><path d="M13 6a3 3 0 11-6 0 3 3 0 016 0zM18 8a2 2 0 11-4 0 2 2 0 014 0zM14 15a4 4 0 00-8 0v3h8v-3zM6 8a2 2 0 11-4 0 2 2 0 014 0zM16 18v-3a5.972 5.972 0 00-.75-2.906A3.005 3.005 0 0119 15v3h-3zM4.75 12.094A5.973 5.973 0 004 15v3H1v-3a3 3 0 013.75-2.906z"/></svg>
      <span>+80 mil clientes ativos</span>
    </div>

    <div class="flex items-center gap-2 text-sm font-medium text-gray-600">
      <svg class="w-5 h-5 text-brand" fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M6.267 3.455a3.066 3.066 0 001.745-.723 3.066 3.066 0 013.976 0 3.066 3.066 0 001.745.723 3.066 3.066 0 012.812 2.812c.051.643.304 1.254.723 1.745a3.066 3.066 0 010 3.976 3.066 3.066 0 00-.723 1.745 3.066 3.066 0 01-2.812 2.812 3.066 3.066 0 00-1.745.723 3.066 3.066 0 01-3.976 0 3.066 3.066 0 00-1.745-.723 3.066 3.066 0 01-2.812-2.812 3.066 3.066 0 00-.723-1.745 3.066 3.066 0 010-3.976 3.066 3.066 0 00.723-1.745 3.066 3.066 0 012.812-2.812zm7.44 5.252a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/></svg>
      <span>Desde 2013 — 10+ anos de mercado</span>
    </div>

  </div>
</section>
"##;

pub fn section_social_proof() -> &'static str { SOCIAL_PROOF }

// ── Tabela Comparativa vs Mercado ────────────────────────────────────────────
const COMPARISON_TABLE: &str = r##"<section class="py-20 bg-white" id="comparativo">
  <div class="max-w-4xl mx-auto px-6" data-fade>
    <h2 class="text-3xl md:text-4xl font-black text-gray-900 text-center mb-3">
      Contrate o plano anual e <span class="text-brand">economize até R$ 4 mil</span>
    </h2>
    <p class="text-center text-gray-500 mb-10">Todos os custos de abertura por nossa conta no plano anual.</p>

    <div class="overflow-x-auto rounded-2xl shadow-lg border border-gray-100">
      <table class="w-full text-sm">
        <thead>
          <tr class="bg-gray-50 border-b border-gray-100">
            <th class="text-left px-6 py-4 font-semibold text-gray-500 w-1/2">Item</th>
            <th class="text-center px-6 py-4 font-semibold text-gray-500">Média do mercado</th>
            <th class="text-center px-6 py-4 font-black text-brand">[PRODUCT]</th>
          </tr>
        </thead>
        <tbody>
          <tr class="border-b border-gray-50">
            <td class="px-6 py-4 text-gray-700">Certificado digital e-CPF</td>
            <td class="px-6 py-4 text-center text-gray-400 line-through">R$ 120–200</td>
            <td class="px-6 py-4 text-center font-bold text-brand">Custo Zero</td>
          </tr>
          <tr class="border-b border-gray-50 bg-gray-50/50">
            <td class="px-6 py-4 text-gray-700">Consultoria e documentação</td>
            <td class="px-6 py-4 text-center text-gray-400 line-through">R$ 500–600</td>
            <td class="px-6 py-4 text-center font-bold text-brand">Custo Zero</td>
          </tr>
          <tr class="border-b border-gray-50">
            <td class="px-6 py-4 text-gray-700">Taxas governamentais de abertura</td>
            <td class="px-6 py-4 text-center text-gray-400 line-through">R$ 70–150</td>
            <td class="px-6 py-4 text-center font-bold text-brand">Custo Zero</td>
          </tr>
          <tr class="bg-green-50/60">
            <td class="px-6 py-4 font-bold text-gray-900">Economia total estimada</td>
            <td class="px-6 py-4 text-center text-gray-400 line-through">Até R$ 950</td>
            <td class="px-6 py-4 text-center font-black text-brand text-base">Até R$ 4.000</td>
          </tr>
        </tbody>
      </table>
    </div>

    <div class="text-center mt-8">
      <a href="#contato" class="btn btn-brand btn-lg rounded-full px-10 font-bold">Quero economizar</a>
    </div>
  </div>
</section>
"##;

pub fn section_comparison_table(product: &str) -> String {
    COMPARISON_TABLE.replace("[PRODUCT]", product)
}

// ── Seletor de Jornada ───────────────────────────────────────────────────────
const JOURNEY_SELECTOR: &str = r##"<section class="py-20 bg-gray-50" id="solucoes">
  <div class="max-w-7xl mx-auto px-6" data-fade>
    <h2 class="text-3xl md:text-4xl font-black text-gray-900 text-center mb-2">
      Soluções para cada etapa da sua jornada
    </h2>
    <p class="text-center text-gray-500 mb-10">Comece do zero, migre do MEI ou troque de contador sem complicações.</p>

    <!-- Tabs -->
    <div class="flex justify-center gap-0 border-b border-gray-200 mb-10">
      <button class="journey-tab px-6 py-3 text-sm font-medium text-gray-500 hover:text-brand transition-colors" data-journey="pf">
        Pessoa Física / PJ nova
      </button>
      <button class="journey-tab px-6 py-3 text-sm font-medium text-gray-500 hover:text-brand transition-colors" data-journey="mei">
        Migrar do MEI
      </button>
      <button class="journey-tab px-6 py-3 text-sm font-medium text-gray-500 hover:text-brand transition-colors" data-journey="troca">
        Trocar de contador
      </button>
    </div>

    <!-- Painel PF -->
    <div id="panel-pf" class="journey-panel flex-col md:flex-row items-center gap-10">
      <div class="flex-1 order-2 md:order-1">
        <span class="inline-block bg-green-50 text-brand text-xs font-bold px-3 py-1 rounded-full mb-4 uppercase">Para quem ainda é PF</span>
        <h3 class="text-2xl font-black text-gray-900 mb-4">Saia da informalidade e pague menos imposto</h3>
        <ul class="space-y-3 mb-6">
          <li class="flex items-start gap-3 text-sm text-gray-600"><span class="w-5 h-5 bg-brand text-white rounded-full flex items-center justify-center flex-shrink-0 text-xs font-bold">✓</span>Abertura de CNPJ 100% online e sem custo no plano anual</li>
          <li class="flex items-start gap-3 text-sm text-gray-600"><span class="w-5 h-5 bg-brand text-white rounded-full flex items-center justify-center flex-shrink-0 text-xs font-bold">✓</span>Escolha do regime tributário ideal para seu faturamento</li>
          <li class="flex items-start gap-3 text-sm text-gray-600"><span class="w-5 h-5 bg-brand text-white rounded-full flex items-center justify-center flex-shrink-0 text-xs font-bold">✓</span>Certificado digital e-CPF incluído no plano anual</li>
          <li class="flex items-start gap-3 text-sm text-gray-600"><span class="w-5 h-5 bg-brand text-white rounded-full flex items-center justify-center flex-shrink-0 text-xs font-bold">✓</span>Contador dedicado e suporte via chat e videochamada</li>
        </ul>
        <div class="flex gap-3">
          <a href="#contato" class="btn btn-brand rounded-full font-bold">Quero economizar</a>
          <a href="#faq" class="btn btn-ghost rounded-full">Ainda tenho dúvidas</a>
        </div>
      </div>
      <div class="flex-1 order-1 md:order-2">
        <div class="bg-gray-200 rounded-2xl h-64 md:h-80 flex items-center justify-center text-gray-400 text-sm">
          [Ilustração — Pessoa trabalhando no laptop]
        </div>
      </div>
    </div>

    <!-- Painel MEI -->
    <div id="panel-mei" class="journey-panel flex-col md:flex-row items-center gap-10">
      <div class="flex-1 order-2 md:order-1">
        <span class="inline-block bg-green-50 text-brand text-xs font-bold px-3 py-1 rounded-full mb-4 uppercase">Para MEIs que cresceram</span>
        <h3 class="text-2xl font-black text-gray-900 mb-4">Migre do MEI para ME com segurança</h3>
        <ul class="space-y-3 mb-6">
          <li class="flex items-start gap-3 text-sm text-gray-600"><span class="w-5 h-5 bg-brand text-white rounded-full flex items-center justify-center flex-shrink-0 text-xs font-bold">✓</span>Análise gratuita de quando e como fazer a migração</li>
          <li class="flex items-start gap-3 text-sm text-gray-600"><span class="w-5 h-5 bg-brand text-white rounded-full flex items-center justify-center flex-shrink-0 text-xs font-bold">✓</span>Desenquadramento do MEI feito pelos nossos especialistas</li>
          <li class="flex items-start gap-3 text-sm text-gray-600"><span class="w-5 h-5 bg-brand text-white rounded-full flex items-center justify-center flex-shrink-0 text-xs font-bold">✓</span>Nenhuma multa ou problema com o Fisco</li>
          <li class="flex items-start gap-3 text-sm text-gray-600"><span class="w-5 h-5 bg-brand text-white rounded-full flex items-center justify-center flex-shrink-0 text-xs font-bold">✓</span>Notas fiscais ilimitadas após a migração</li>
        </ul>
        <div class="flex gap-3">
          <a href="#contato" class="btn btn-brand rounded-full font-bold">Quero migrar para ME</a>
          <a href="#faq" class="btn btn-ghost rounded-full">Ainda tenho dúvidas</a>
        </div>
      </div>
      <div class="flex-1 order-1 md:order-2">
        <div class="bg-gray-200 rounded-2xl h-64 md:h-80 flex items-center justify-center text-gray-400 text-sm">
          [Ilustração — Contador no escritório]
        </div>
      </div>
    </div>

    <!-- Painel Troca -->
    <div id="panel-troca" class="journey-panel flex-col md:flex-row items-center gap-10">
      <div class="flex-1 order-2 md:order-1">
        <span class="inline-block bg-green-50 text-brand text-xs font-bold px-3 py-1 rounded-full mb-4 uppercase">Para quem já tem empresa</span>
        <h3 class="text-2xl font-black text-gray-900 mb-4">Troque de contador sem dor de cabeça</h3>
        <ul class="space-y-3 mb-6">
          <li class="flex items-start gap-3 text-sm text-gray-600"><span class="w-5 h-5 bg-brand text-white rounded-full flex items-center justify-center flex-shrink-0 text-xs font-bold">✓</span>Transferência de contabilidade 100% feita por nós</li>
          <li class="flex items-start gap-3 text-sm text-gray-600"><span class="w-5 h-5 bg-brand text-white rounded-full flex items-center justify-center flex-shrink-0 text-xs font-bold">✓</span>Revisão fiscal retroativa dos últimos meses sem custo</li>
          <li class="flex items-start gap-3 text-sm text-gray-600"><span class="w-5 h-5 bg-brand text-white rounded-full flex items-center justify-center flex-shrink-0 text-xs font-bold">✓</span>Plataforma online com acesso 24/7 aos documentos</li>
          <li class="flex items-start gap-3 text-sm text-gray-600"><span class="w-5 h-5 bg-brand text-white rounded-full flex items-center justify-center flex-shrink-0 text-xs font-bold">✓</span>Atendimento especializado no seu segmento de negócio</li>
        </ul>
        <div class="flex gap-3">
          <a href="#contato" class="btn btn-brand rounded-full font-bold">Quero economizar</a>
          <a href="#faq" class="btn btn-ghost rounded-full">Ainda tenho dúvidas</a>
        </div>
      </div>
      <div class="flex-1 order-1 md:order-2">
        <div class="bg-gray-200 rounded-2xl h-64 md:h-80 flex items-center justify-center text-gray-400 text-sm">
          [Ilustração — Handshake negócios]
        </div>
      </div>
    </div>

  </div>
</section>
"##;

pub fn section_journey_selector() -> &'static str { JOURNEY_SELECTOR }

// ── Slider de Benefícios ─────────────────────────────────────────────────────
const BENEFITS_SLIDER: &str = r##"<section class="py-20 bg-white" id="beneficios">
  <div class="max-w-7xl mx-auto px-6" data-fade>
    <h2 class="text-3xl md:text-4xl font-black text-gray-900 mb-2">
      [PRODUCT] · <span class="text-brand">muito mais que contabilidade</span>
    </h2>
    <p class="text-gray-500 mb-8">Tudo que você precisa para crescer, numa única plataforma.</p>

    <div class="benefit-track pb-4">

      <div class="benefit-card card bg-base-100 shadow-md border border-gray-100 p-6">
        <div class="w-10 h-10 bg-green-50 text-brand rounded-xl flex items-center justify-center mb-4">
          <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 10h18M7 15h1m4 0h1m-7 4h12a3 3 0 003-3V8a3 3 0 00-3-3H6a3 3 0 00-3 3v8a3 3 0 003 3z"/></svg>
        </div>
        <h4 class="font-bold text-gray-900 mb-2">Conta bancária PJ digital</h4>
        <p class="text-sm text-gray-500">Conta PJ gratuita integrada à contabilidade, com Pix ilimitados e sem tarifas.</p>
      </div>

      <div class="benefit-card card bg-base-100 shadow-md border border-gray-100 p-6">
        <div class="w-10 h-10 bg-green-50 text-brand rounded-xl flex items-center justify-center mb-4">
          <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/></svg>
        </div>
        <h4 class="font-bold text-gray-900 mb-2">Emissor de Notas Fiscais</h4>
        <p class="text-sm text-gray-500">Emita NF-e, NFS-e e NFC-e ilimitadas. Replique notas recorrentes com 1 clique.</p>
      </div>

      <div class="benefit-card card bg-base-100 shadow-md border border-gray-100 p-6">
        <div class="w-10 h-10 bg-green-50 text-brand rounded-xl flex items-center justify-center mb-4">
          <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"/></svg>
        </div>
        <h4 class="font-bold text-gray-900 mb-2">Controle de Impostos</h4>
        <p class="text-sm text-gray-500">Alertas automáticos de guias a vencer. Jamais pague multa por atraso.</p>
      </div>

      <div class="benefit-card card bg-base-100 shadow-md border border-gray-100 p-6">
        <div class="w-10 h-10 bg-green-50 text-brand rounded-xl flex items-center justify-center mb-4">
          <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 9V7a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2m2 4h10a2 2 0 002-2v-6a2 2 0 00-2-2H9a2 2 0 00-2 2v6a2 2 0 002 2zm7-5a2 2 0 11-4 0 2 2 0 014 0z"/></svg>
        </div>
        <h4 class="font-bold text-gray-900 mb-2">Cobrança Online de Clientes</h4>
        <p class="text-sm text-gray-500">Parcele serviços em até 12x. Receba em 2 dias úteis, sem maquininha.</p>
      </div>

      <div class="benefit-card card bg-base-100 shadow-md border border-gray-100 p-6">
        <div class="w-10 h-10 bg-green-50 text-brand rounded-xl flex items-center justify-center mb-4">
          <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"/></svg>
        </div>
        <h4 class="font-bold text-gray-900 mb-2">Benefícios de Saúde</h4>
        <p class="text-sm text-gray-500">Psicólogos, nutricionistas, plano odontológico e seguro de vida com o seu CNPJ.</p>
      </div>

    </div>

    <div class="text-center mt-8">
      <a href="#precos" class="btn btn-outline border-brand text-brand hover:bg-brand hover:text-white rounded-full font-bold">Ver todos os planos</a>
    </div>
  </div>
</section>
"##;

pub fn section_benefits_slider(product: &str) -> String {
    BENEFITS_SLIDER.replace("[PRODUCT]", product)
}

// ── Grid de Conteúdo / Blog ──────────────────────────────────────────────────
const CONTENT_GRID: &str = r##"<section class="py-20 bg-gray-50" id="blog">
  <div class="max-w-7xl mx-auto px-6" data-fade>
    <h2 class="text-3xl md:text-4xl font-black text-gray-900 mb-2">
      Tudo que você precisa saber sobre<br class="hidden md:block"/> <span class="text-brand">abrir e gerir sua empresa</span>
    </h2>
    <p class="text-gray-500 mb-10">Conteúdo gratuito criado por nossos especialistas.</p>

    <div class="grid grid-cols-1 md:grid-cols-3 gap-8">

      <article class="card bg-white shadow-sm border border-gray-100 overflow-hidden hover:shadow-md transition-shadow">
        <div class="bg-gray-200 h-44 flex items-center justify-center text-gray-400 text-sm">[Imagem do artigo 1]</div>
        <div class="card-body p-5">
          <span class="text-xs font-bold text-brand uppercase">Abertura de empresa</span>
          <h3 class="font-bold text-gray-900 mt-1 mb-2 leading-tight">5 motivos para contratar uma contabilidade online e modernizar sua empresa</h3>
          <p class="text-sm text-gray-500 mb-4">Já imaginou os benefícios de uma contabilidade digital para sua empresa? Descubra por que cada vez mais empreendedores estão migrando.</p>
          <a href="#" class="text-brand text-sm font-semibold hover:underline flex items-center gap-1">
            Leia mais <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/></svg>
          </a>
        </div>
      </article>

      <article class="card bg-white shadow-sm border border-gray-100 overflow-hidden hover:shadow-md transition-shadow">
        <div class="bg-gray-200 h-44 flex items-center justify-center text-gray-400 text-sm">[Imagem do artigo 2]</div>
        <div class="card-body p-5">
          <span class="text-xs font-bold text-brand uppercase">Gestão financeira</span>
          <h3 class="font-bold text-gray-900 mt-1 mb-2 leading-tight">O que é contabilidade online e como ela ajuda as Pessoas Jurídicas?</h3>
          <p class="text-sm text-gray-500 mb-4">Sabia que você pode trabalhar como pessoa jurídica com uma contabilidade 100% digital? Entenda como funciona.</p>
          <a href="#" class="text-brand text-sm font-semibold hover:underline flex items-center gap-1">
            Leia mais <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/></svg>
          </a>
        </div>
      </article>

      <article class="card bg-white shadow-sm border border-gray-100 overflow-hidden hover:shadow-md transition-shadow">
        <div class="bg-gray-200 h-44 flex items-center justify-center text-gray-400 text-sm">[Imagem do artigo 3]</div>
        <div class="card-body p-5">
          <span class="text-xs font-bold text-brand uppercase">CNPJ</span>
          <h3 class="font-bold text-gray-900 mt-1 mb-2 leading-tight">Como abrir empresa: o passo a passo completo para tirar seu CNPJ</h3>
          <p class="text-sm text-gray-500 mb-4">Guia prático com tudo o que você precisa saber para abrir seu CNPJ sem burocracia e sem sair de casa.</p>
          <a href="#" class="text-brand text-sm font-semibold hover:underline flex items-center gap-1">
            Leia mais <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/></svg>
          </a>
        </div>
      </article>

    </div>

    <div class="text-center mt-10">
      <a href="#" class="btn btn-ghost text-brand border border-brand hover:bg-brand hover:text-white rounded-full font-semibold">Ver mais conteúdos</a>
    </div>
  </div>
</section>
"##;

pub fn section_content_grid() -> &'static str { CONTENT_GRID }

// ── Depoimentos com Foto ─────────────────────────────────────────────────────
const TESTIMONIALS_PHOTO: &str = r##"<section class="py-20 bg-white" id="depoimentos">
  <div class="max-w-7xl mx-auto px-6" data-fade>
    <div class="flex items-center justify-between mb-10">
      <h2 class="text-3xl md:text-4xl font-black text-gray-900">O que nossos clientes falam</h2>
      <div class="hidden md:flex items-center gap-2 text-sm text-gray-500">
        <span class="font-bold text-gray-700">4.7</span>
        <div class="flex text-yellow-400">★★★★★</div>
        <span>+2.000 avaliações</span>
      </div>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-3 gap-6">

      <div class="card bg-gray-50 border border-gray-100 p-6">
        <div class="flex items-center gap-3 mb-4">
          <div class="w-12 h-12 bg-gray-300 rounded-full flex items-center justify-center text-gray-500 font-bold text-lg">A</div>
          <div>
            <p class="font-bold text-gray-900 text-sm">Ana Luisa Garcia</p>
            <p class="text-xs text-gray-500">Médica · cliente desde 2017</p>
          </div>
        </div>
        <div class="flex text-yellow-400 text-xs mb-3">★★★★★</div>
        <p class="text-sm text-gray-600 leading-relaxed">"Abri minha empresa sem sair de casa e de forma simples e prática. Resolvo dúvidas via e-mail e videochamada com agilidade."</p>
      </div>

      <div class="card bg-gray-50 border border-gray-100 p-6">
        <div class="flex items-center gap-3 mb-4">
          <div class="w-12 h-12 bg-gray-300 rounded-full flex items-center justify-center text-gray-500 font-bold text-lg">R</div>
          <div>
            <p class="font-bold text-gray-900 text-sm">Raphael Ligmanovski</p>
            <p class="text-xs text-gray-500">Consultor de sistemas · cliente desde 2022</p>
          </div>
        </div>
        <div class="flex text-yellow-400 text-xs mb-3">★★★★★</div>
        <p class="text-sm text-gray-600 leading-relaxed">"Consigo fazer a gestão do CNPJ em poucos passos e otimizo meu tempo. Até pagar os impostos do meu CNPJ ficou fácil."</p>
      </div>

      <div class="card bg-gray-50 border border-gray-100 p-6">
        <div class="flex items-center gap-3 mb-4">
          <div class="w-12 h-12 bg-gray-300 rounded-full flex items-center justify-center text-gray-500 font-bold text-lg">B</div>
          <div>
            <p class="font-bold text-gray-900 text-sm">Bruna Ricci</p>
            <p class="text-xs text-gray-500">Analista administrativa · cliente desde 2022</p>
          </div>
        </div>
        <div class="flex text-yellow-400 text-xs mb-3">★★★★★</div>
        <p class="text-sm text-gray-600 leading-relaxed">"A abertura da minha empresa foi simples, online e com um atendimento completo e humano. Recomendo muito."</p>
      </div>

      <div class="card bg-gray-50 border border-gray-100 p-6">
        <div class="flex items-center gap-3 mb-4">
          <div class="w-12 h-12 bg-gray-300 rounded-full flex items-center justify-center text-gray-500 font-bold text-lg">F</div>
          <div>
            <p class="font-bold text-gray-900 text-sm">Felipe Bernardo</p>
            <p class="text-xs text-gray-500">Arquiteto · cliente desde 2021</p>
          </div>
        </div>
        <div class="flex text-yellow-400 text-xs mb-3">★★★★★</div>
        <p class="text-sm text-gray-600 leading-relaxed">"Me ajudou a deixar a gestão do CNPJ mais rápida. Sobra muito mais tempo para focar no meu negócio e nos clientes."</p>
      </div>

      <div class="card bg-gray-50 border border-gray-100 p-6">
        <div class="flex items-center gap-3 mb-4">
          <div class="w-12 h-12 bg-gray-300 rounded-full flex items-center justify-center text-gray-500 font-bold text-lg">V</div>
          <div>
            <p class="font-bold text-gray-900 text-sm">Vitor de Sousa</p>
            <p class="text-xs text-gray-500">Consultor · cliente desde 2023</p>
          </div>
        </div>
        <div class="flex text-yellow-400 text-xs mb-3">★★★★★</div>
        <p class="text-sm text-gray-600 leading-relaxed">"Não preciso me preocupar com a parte burocrática de ser PJ porque a equipe cuida de tudo isso para mim."</p>
      </div>

      <div class="card bg-gray-50 border border-gray-100 p-6">
        <div class="flex items-center gap-3 mb-4">
          <div class="w-12 h-12 bg-gray-300 rounded-full flex items-center justify-center text-gray-500 font-bold text-lg">T</div>
          <div>
            <p class="font-bold text-gray-900 text-sm">Telma Sturlini</p>
            <p class="text-xs text-gray-500">Psicóloga · cliente desde 2021</p>
          </div>
        </div>
        <div class="flex text-yellow-400 text-xs mb-3">★★★★★</div>
        <p class="text-sm text-gray-600 leading-relaxed">"A facilidade e a forma amistosa de interação com quem está abrindo empresa são os principais motivos de eu ser cliente."</p>
      </div>

    </div>
  </div>
</section>
"##;

pub fn section_testimonials_photo() -> &'static str { TESTIMONIALS_PHOTO }

// ── FAQ (saas — fundo branco, mais clean) ────────────────────────────────────
const FAQ_SAAS: &str = r##"<section class="py-20 bg-gray-50" id="faq">
  <div class="max-w-3xl mx-auto px-6" data-fade>
    <h2 class="text-3xl md:text-4xl font-black text-gray-900 text-center mb-10">Perguntas frequentes</h2>
    <div class="space-y-3">

      <details class="bg-white border border-gray-100 rounded-xl px-6 py-4 cursor-pointer group">
        <summary class="flex items-center justify-between font-semibold text-gray-800 list-none">
          O que está incluso na mensalidade?
          <svg class="w-5 h-5 text-gray-400 group-open:rotate-180 transition-transform" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/></svg>
        </summary>
        <p class="mt-3 text-sm text-gray-500 leading-relaxed">Todos os nossos planos incluem contabilidade completa, emissão de guias de impostos, suporte especializado, acesso à plataforma 24/7 e emissor de notas fiscais ilimitadas.</p>
      </details>

      <details class="bg-white border border-gray-100 rounded-xl px-6 py-4 cursor-pointer group">
        <summary class="flex items-center justify-between font-semibold text-gray-800 list-none">
          A equipe tem contadores de verdade?
          <svg class="w-5 h-5 text-gray-400 group-open:rotate-180 transition-transform" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/></svg>
        </summary>
        <p class="mt-3 text-sm text-gray-500 leading-relaxed">Sim! Contamos com uma equipe de contadores registrados no CRC com especialização nos principais segmentos de mercado. Você terá um profissional dedicado à sua empresa.</p>
      </details>

      <details class="bg-white border border-gray-100 rounded-xl px-6 py-4 cursor-pointer group">
        <summary class="flex items-center justify-between font-semibold text-gray-800 list-none">
          Como funciona a abertura de empresa sem custo?
          <svg class="w-5 h-5 text-gray-400 group-open:rotate-180 transition-transform" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/></svg>
        </summary>
        <p class="mt-3 text-sm text-gray-500 leading-relaxed">No plano anual, assumimos todos os custos de abertura: certificado digital, taxas governamentais e consultoria. Você paga apenas a mensalidade e já começa com empresa aberta.</p>
      </details>

      <details class="bg-white border border-gray-100 rounded-xl px-6 py-4 cursor-pointer group">
        <summary class="flex items-center justify-between font-semibold text-gray-800 list-none">
          Posso trocar de contador sem perder histórico?
          <svg class="w-5 h-5 text-gray-400 group-open:rotate-180 transition-transform" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/></svg>
        </summary>
        <p class="mt-3 text-sm text-gray-500 leading-relaxed">Com toda a certeza. Nossa equipe faz toda a migração da sua contabilidade, incluindo histórico e documentos fiscais. Sem burocracia para você.</p>
      </details>

      <details class="bg-white border border-gray-100 rounded-xl px-6 py-4 cursor-pointer group">
        <summary class="flex items-center justify-between font-semibold text-gray-800 list-none">
          Quais são os planos e valores disponíveis?
          <svg class="w-5 h-5 text-gray-400 group-open:rotate-180 transition-transform" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/></svg>
        </summary>
        <p class="mt-3 text-sm text-gray-500 leading-relaxed">Os planos partem de R$ 99/mês no plano anual. Cada plano é customizado de acordo com o regime tributário e segmento da sua empresa. Consulte nossa tabela de preços.</p>
      </details>

    </div>
  </div>
</section>
"##;

pub fn section_faq_saas() -> &'static str { FAQ_SAAS }

// ── Footer Colunar ───────────────────────────────────────────────────────────
const FOOTER_SAAS: &str = r##"<footer class="bg-gray-900 text-gray-400 pt-16 pb-8">
  <div class="max-w-7xl mx-auto px-6">
    <div class="grid grid-cols-2 md:grid-cols-5 gap-8 mb-12">

      <!-- Marca -->
      <div class="col-span-2">
        <p class="text-white text-2xl font-black mb-2">[COMPANY]</p>
        <p class="text-sm mb-4">Simplificando a vida de quem empreende.<br/>Desde [YEAR].</p>
        <!-- Redes sociais -->
        <div class="flex gap-3">
          <a href="#" class="w-8 h-8 bg-gray-800 hover:bg-brand rounded-lg flex items-center justify-center transition-colors" aria-label="YouTube">
            <svg class="w-4 h-4 fill-current" viewBox="0 0 24 24"><path d="M23.498 6.186a3.016 3.016 0 00-2.122-2.136C19.505 3.545 12 3.545 12 3.545s-7.505 0-9.377.505A3.017 3.017 0 00.502 6.186C0 8.07 0 12 0 12s0 3.93.502 5.814a3.016 3.016 0 002.122 2.136c1.871.505 9.376.505 9.376.505s7.505 0 9.377-.505a3.015 3.015 0 002.122-2.136C24 15.93 24 12 24 12s0-3.93-.502-5.814zM9.545 15.568V8.432L15.818 12l-6.273 3.568z"/></svg>
          </a>
          <a href="#" class="w-8 h-8 bg-gray-800 hover:bg-brand rounded-lg flex items-center justify-center transition-colors" aria-label="LinkedIn">
            <svg class="w-4 h-4 fill-current" viewBox="0 0 24 24"><path d="M20.447 20.452h-3.554v-5.569c0-1.328-.027-3.037-1.852-3.037-1.853 0-2.136 1.445-2.136 2.939v5.667H9.351V9h3.414v1.561h.046c.477-.9 1.637-1.85 3.37-1.85 3.601 0 4.267 2.37 4.267 5.455v6.286zM5.337 7.433a2.062 2.062 0 01-2.063-2.065 2.064 2.064 0 112.063 2.065zm1.782 13.019H3.555V9h3.564v11.452zM22.225 0H1.771C.792 0 0 .774 0 1.729v20.542C0 23.227.792 24 1.771 24h20.451C23.2 24 24 23.227 24 22.271V1.729C24 .774 23.2 0 22.222 0h.003z"/></svg>
          </a>
          <a href="#" class="w-8 h-8 bg-gray-800 hover:bg-brand rounded-lg flex items-center justify-center transition-colors" aria-label="Instagram">
            <svg class="w-4 h-4 fill-current" viewBox="0 0 24 24"><path d="M12 2.163c3.204 0 3.584.012 4.85.07 3.252.148 4.771 1.691 4.919 4.919.058 1.265.069 1.645.069 4.849 0 3.205-.012 3.584-.069 4.849-.149 3.225-1.664 4.771-4.919 4.919-1.266.058-1.644.07-4.85.07-3.204 0-3.584-.012-4.849-.07-3.26-.149-4.771-1.699-4.919-4.92-.058-1.265-.07-1.644-.07-4.849 0-3.204.013-3.583.07-4.849.149-3.227 1.664-4.771 4.919-4.919 1.266-.057 1.645-.069 4.849-.069zM12 0C8.741 0 8.333.014 7.053.072 2.695.272.273 2.69.073 7.052.014 8.333 0 8.741 0 12c0 3.259.014 3.668.072 4.948.2 4.358 2.618 6.78 6.98 6.98C8.333 23.986 8.741 24 12 24c3.259 0 3.668-.014 4.948-.072 4.354-.2 6.782-2.618 6.979-6.98.059-1.28.073-1.689.073-4.948 0-3.259-.014-3.667-.072-4.947-.196-4.354-2.617-6.78-6.979-6.98C15.668.014 15.259 0 12 0zm0 5.838a6.162 6.162 0 100 12.324 6.162 6.162 0 000-12.324zM12 16a4 4 0 110-8 4 4 0 010 8zm6.406-11.845a1.44 1.44 0 100 2.881 1.44 1.44 0 000-2.881z"/></svg>
          </a>
        </div>
      </div>

      <!-- Coluna 1 -->
      <div>
        <p class="text-white font-semibold text-sm mb-3 uppercase tracking-wider">A empresa</p>
        <ul class="space-y-2 text-sm">
          <li><a href="#" class="hover:text-white transition-colors">Sobre nós</a></li>
          <li><a href="#" class="hover:text-white transition-colors">Blog</a></li>
          <li><a href="#" class="hover:text-white transition-colors">Carreiras</a></li>
          <li><a href="#" class="hover:text-white transition-colors">Imprensa</a></li>
        </ul>
      </div>

      <!-- Coluna 2 -->
      <div>
        <p class="text-white font-semibold text-sm mb-3 uppercase tracking-wider">Serviços</p>
        <ul class="space-y-2 text-sm">
          <li><a href="#" class="hover:text-white transition-colors">Abrir empresa</a></li>
          <li><a href="#" class="hover:text-white transition-colors">Migrar do MEI</a></li>
          <li><a href="#" class="hover:text-white transition-colors">Trocar de contador</a></li>
          <li><a href="#" class="hover:text-white transition-colors">Notas fiscais</a></li>
        </ul>
      </div>

      <!-- Coluna 3 -->
      <div>
        <p class="text-white font-semibold text-sm mb-3 uppercase tracking-wider">Legal</p>
        <ul class="space-y-2 text-sm">
          <li><a href="#" class="hover:text-white transition-colors">Termos de uso</a></li>
          <li><a href="#" class="hover:text-white transition-colors">Privacidade</a></li>
          <li><a href="#" class="hover:text-white transition-colors">Cookies</a></li>
          <li><a href="#" class="hover:text-white transition-colors">Canal de ética</a></li>
        </ul>
      </div>

    </div>

    <div class="border-t border-gray-800 pt-6 flex flex-col md:flex-row items-center justify-between gap-3 text-xs">
      <p>© [YEAR] [COMPANY]. Todos os direitos reservados.</p>
      <p>Simplificando a vida de quem empreende no Brasil.</p>
    </div>
  </div>
</footer>
"##;

pub fn section_footer_saas(company: &str) -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let year = 2024 + (SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs() / 31_536_000 - 54);
    FOOTER_SAAS
        .replace("[COMPANY]", company)
        .replace("[YEAR]", &year.to_string())
}
