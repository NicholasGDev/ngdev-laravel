import { invoke } from "@tauri-apps/api/core";
import "./styles.css";

// ── Tipos de retorno ──────────────────────────────────────────────────────────

type InvokeResult = string;

// ── Utilitários de UI ─────────────────────────────────────────────────────────

function setContent(html: string) {
  document.getElementById("main-content")!.innerHTML = html;
}

function showResult(msg: string, isError = false) {
  const el = document.getElementById("result-box");
  if (!el) return;
  el.className = `alert mt-4 ${isError ? "alert-error" : "alert-success"}`;
  el.innerHTML = `<span>${msg}</span>`;
  el.classList.remove("hidden");
}

function resultBox() {
  return `<div id="result-box" class="alert hidden"></div>`;
}

// ── Páginas ───────────────────────────────────────────────────────────────────

const pages: Record<string, () => string> = {
  // Dashboard inicial
  home: () => `
    <div class="hero min-h-[60vh]">
      <div class="hero-content text-center">
        <div>
          <h1 class="text-4xl font-bold text-primary">ngdev Manager</h1>
          <p class="py-4 text-base-content/70 max-w-md">
            Gerador de código Laravel com DDD. Selecione um gerador no menu lateral para começar.
          </p>
          <div class="stats shadow mt-4">
            <div class="stat"><div class="stat-title">Geradores</div><div class="stat-value text-primary">9</div></div>
            <div class="stat"><div class="stat-title">Plataformas</div><div class="stat-value text-secondary">2</div><div class="stat-desc">Windows · Linux</div></div>
          </div>
        </div>
      </div>
    </div>`,

  // Context DDD
  context: () => `
    <h2 class="text-2xl font-bold mb-4">🏗 Gerar Context DDD</h2>
    <div class="card bg-base-200 shadow-xl">
      <div class="card-body grid grid-cols-1 md:grid-cols-2 gap-4">
        <label class="form-control">
          <span class="label-text">Nome do Context</span>
          <input id="ctx-nome" type="text" placeholder="Ex: Produto" class="input input-bordered" />
        </label>
        <label class="form-control">
          <span class="label-text">Namespace Base</span>
          <input id="ctx-ns" type="text" placeholder="App\\Contexts" class="input input-bordered" />
        </label>
        <label class="form-control">
          <span class="label-text">Base Path</span>
          <input id="ctx-path" type="text" placeholder="back/app/Contexts" class="input input-bordered" />
        </label>
        <label class="form-control">
          <span class="label-text">Prefixo de Rota</span>
          <input id="ctx-prefix" type="text" placeholder="produtos" class="input input-bordered" />
        </label>

        <div class="md:col-span-2">
          <p class="label-text mb-2">Operações</p>
          <div class="flex flex-wrap gap-3">
            ${["criar","alterar","deletar","consultar","detalhar"].map(op => `
              <label class="label cursor-pointer gap-2">
                <input type="checkbox" class="checkbox checkbox-primary ctx-op" value="${op}" />
                <span class="label-text">${op}</span>
              </label>`).join("")}
          </div>
        </div>

        <div class="flex gap-4 md:col-span-2">
          <label class="label cursor-pointer gap-2">
            <input id="ctx-entity" type="checkbox" class="checkbox checkbox-secondary" checked />
            <span class="label-text">Com Entity</span>
          </label>
          <label class="label cursor-pointer gap-2">
            <input id="ctx-auth" type="checkbox" class="checkbox checkbox-secondary" />
            <span class="label-text">Com Autorizações</span>
          </label>
        </div>

        <div class="md:col-span-2">
          <button id="btn-context" class="btn btn-primary w-full">Gerar Context</button>
        </div>
      </div>
    </div>
    ${resultBox()}`,

  // Docker
  docker: () => `
    <h2 class="text-2xl font-bold mb-4">🐳 Gerar Infraestrutura Docker</h2>
    <div class="card bg-base-200 shadow-xl">
      <div class="card-body grid grid-cols-1 md:grid-cols-2 gap-4">
        <label class="form-control">
          <span class="label-text">Nome da Aplicação</span>
          <input id="dock-app" type="text" placeholder="zeus-retail" class="input input-bordered" />
        </label>
        <label class="form-control">
          <span class="label-text">Server Name</span>
          <input id="dock-server" type="text" placeholder="zeus.local" class="input input-bordered" />
        </label>
        <label class="form-control">
          <span class="label-text">Versão PHP</span>
          <input id="dock-php" type="text" value="8.3" class="input input-bordered" />
        </label>
        <label class="form-control">
          <span class="label-text">Versão Node</span>
          <input id="dock-node" type="number" value="20" class="input input-bordered" />
        </label>
        <label class="form-control md:col-span-2">
          <span class="label-text">Output Path</span>
          <input id="dock-out" type="text" placeholder="." class="input input-bordered" />
        </label>

        <div class="md:col-span-2">
          <p class="label-text mb-2">Bancos de Dados</p>
          <div class="flex flex-wrap gap-3">
            ${["mysql","pgsql","sqlite","sqlsrv"].map(db => `
              <label class="label cursor-pointer gap-2">
                <input type="checkbox" class="checkbox checkbox-primary dock-db" value="${db}" />
                <span class="label-text">${db}</span>
              </label>`).join("")}
          </div>
        </div>

        <div class="flex gap-4 md:col-span-2">
          <label class="label cursor-pointer gap-2">
            <input id="dock-redis" type="checkbox" class="checkbox checkbox-secondary" />
            <span class="label-text">Redis</span>
          </label>
          <label class="label cursor-pointer gap-2">
            <input id="dock-mailpit" type="checkbox" class="checkbox checkbox-secondary" />
            <span class="label-text">Mailpit</span>
          </label>
        </div>

        <div class="md:col-span-2">
          <button id="btn-docker" class="btn btn-primary w-full">Gerar Docker</button>
        </div>
      </div>
    </div>
    ${resultBox()}`,

  // PDV
  pdv: () => `
    <h2 class="text-2xl font-bold mb-4">🧾 Gerar Módulo PDV</h2>
    <div class="card bg-base-200 shadow-xl max-w-md">
      <div class="card-body gap-4">
        <label class="form-control">
          <span class="label-text">Diretório raiz do projeto Laravel</span>
          <input id="pdv-root" type="text" placeholder="/caminho/para/meu-projeto" class="input input-bordered" />
        </label>
        <div class="flex gap-4">
          <label class="label cursor-pointer gap-2">
            <input id="pdv-mig" type="checkbox" class="checkbox checkbox-primary" />
            <span class="label-text">Apenas Migrations</span>
          </label>
          <label class="label cursor-pointer gap-2">
            <input id="pdv-mod" type="checkbox" class="checkbox checkbox-primary" />
            <span class="label-text">Apenas Models</span>
          </label>
        </div>
        <button id="btn-pdv" class="btn btn-primary w-full">Gerar PDV</button>
      </div>
    </div>
    ${resultBox()}`,

  // Estoque
  estoque: () => `
    <h2 class="text-2xl font-bold mb-4">📦 Gerar Módulo Estoque</h2>
    <div class="card bg-base-200 shadow-xl">
      <div class="card-body grid grid-cols-1 md:grid-cols-2 gap-4">
        <label class="form-control">
          <span class="label-text">Base Path</span>
          <input id="est-path" type="text" placeholder="back/app/Contexts" class="input input-bordered" />
        </label>
        <label class="form-control">
          <span class="label-text">Namespace Base</span>
          <input id="est-ns" type="text" placeholder="App\\Contexts" class="input input-bordered" />
        </label>
        <label class="form-control">
          <span class="label-text">Migration Path</span>
          <input id="est-mig" type="text" placeholder="database/migrations" class="input input-bordered" />
        </label>
        <label class="form-control">
          <span class="label-text">Tenant ID</span>
          <input id="est-tenant" type="text" placeholder="tenant_1" class="input input-bordered" />
        </label>
        <label class="form-control md:col-span-2">
          <span class="label-text">Método de Valoração</span>
          <select id="est-val" class="select select-bordered">
            <option value="fifo">FIFO</option>
            <option value="lifo">LIFO</option>
            <option value="average">Custo Médio</option>
          </select>
        </label>
        <div class="md:col-span-2">
          <button id="btn-estoque" class="btn btn-primary w-full">Gerar Estoque</button>
        </div>
      </div>
    </div>
    ${resultBox()}`,

  // Logística Reversa
  logistica: () => `
    <h2 class="text-2xl font-bold mb-4">🔄 Gerar Logística Reversa</h2>
    <div class="card bg-base-200 shadow-xl">
      <div class="card-body grid grid-cols-1 md:grid-cols-2 gap-4">
        <label class="form-control">
          <span class="label-text">Base Path</span>
          <input id="log-path" type="text" placeholder="back/app/Contexts" class="input input-bordered" />
        </label>
        <label class="form-control">
          <span class="label-text">Namespace Base</span>
          <input id="log-ns" type="text" placeholder="App\\Contexts" class="input input-bordered" />
        </label>
        <label class="form-control">
          <span class="label-text">Migration Path</span>
          <input id="log-mig" type="text" placeholder="database/migrations" class="input input-bordered" />
        </label>
        <label class="form-control">
          <span class="label-text">ERP ID</span>
          <input id="log-erp" type="text" placeholder="erp_001" class="input input-bordered" />
        </label>
        <label class="form-control">
          <span class="label-text">Company Name</span>
          <input id="log-company" type="text" placeholder="Zeus Retail" class="input input-bordered" />
        </label>
        <label class="form-control">
          <span class="label-text">Warehouse ID</span>
          <input id="log-wh" type="text" placeholder="wh_main" class="input input-bordered" />
        </label>
        <div class="md:col-span-2">
          <button id="btn-logistica" class="btn btn-primary w-full">Gerar Logística Reversa</button>
        </div>
      </div>
    </div>
    ${resultBox()}`,

  // Landing Page
  landing: () => `
    <h2 class="text-2xl font-bold mb-4">🌐 Gerar Landing Page</h2>
    <div class="card bg-base-200 shadow-xl">
      <div class="card-body grid grid-cols-1 md:grid-cols-2 gap-4">

        <label class="form-control md:col-span-2">
          <span class="label-text font-semibold">Layout Base</span>
          <select id="lp-layout" class="select select-bordered" onchange="window._lpLayoutChange(this.value)">
            <option value="generic">Generic — DaisyUI · produto/SaaS clássico</option>
            <option value="saas">SaaS — Contabilizei-style · fundo branco · serviços</option>
          </select>
        </label>

        <label class="form-control">
          <span class="label-text">Nome do Produto</span>
          <input id="lp-prod" type="text" placeholder="Zeus PDV" class="input input-bordered" />
        </label>
        <label class="form-control">
          <span class="label-text">Tagline</span>
          <input id="lp-tag" type="text" placeholder="O melhor PDV do mercado" class="input input-bordered" />
        </label>
        <label class="form-control">
          <span class="label-text">Empresa</span>
          <input id="lp-company" type="text" placeholder="Zeus Retail" class="input input-bordered" />
        </label>
        <label class="form-control">
          <span class="label-text">Diretório de Saída</span>
          <input id="lp-out" type="text" placeholder="landing" class="input input-bordered" />
        </label>

        <!-- Tema (só para generic) -->
        <label class="form-control md:col-span-2" id="lp-theme-row">
          <span class="label-text">Tema DaisyUI</span>
          <select id="lp-theme" class="select select-bordered">
            <option value="dark">dark</option>
            <option value="light">light</option>
            <option value="corporate">corporate</option>
            <option value="retro">retro</option>
            <option value="cyberpunk">cyberpunk</option>
          </select>
        </label>

        <!-- Seções Generic -->
        <div class="md:col-span-2" id="lp-sec-generic">
          <p class="label-text mb-2 font-semibold">Seções — Generic</p>
          <div class="flex flex-wrap gap-3">
            ${["logos","features_grid","features_tabs","stats","testimonials","pricing","faq","cta_bottom"].map(s => `
              <label class="label cursor-pointer gap-2">
                <input type="checkbox" class="checkbox checkbox-primary lp-sec" value="${s}" checked />
                <span class="label-text text-xs">${s}</span>
              </label>`).join("")}
          </div>
        </div>

        <!-- Seções SaaS -->
        <div class="md:col-span-2 hidden" id="lp-sec-saas">
          <p class="label-text mb-2 font-semibold">Seções — SaaS</p>
          <div class="flex flex-wrap gap-3">
            ${["social_proof","comparison_table","journey_selector","benefits_slider","content_grid","testimonials_photo","faq"].map(s => `
              <label class="label cursor-pointer gap-2">
                <input type="checkbox" class="checkbox checkbox-secondary lp-sec-saas-chk" value="${s}" checked />
                <span class="label-text text-xs">${s}</span>
              </label>`).join("")}
          </div>
        </div>

        <div class="md:col-span-2">
          <button id="btn-landing" class="btn btn-primary w-full">Gerar Landing Page</button>
        </div>
      </div>
    </div>
    ${resultBox()}
    <script>
      window._lpLayoutChange = (v) => {
        document.getElementById('lp-theme-row')?.classList.toggle('hidden', v === 'saas');
        document.getElementById('lp-sec-generic')?.classList.toggle('hidden', v === 'saas');
        document.getElementById('lp-sec-saas')?.classList.toggle('hidden', v !== 'saas');
      };
    </script>`,

  // Controller
  controller: () => `
    <h2 class="text-2xl font-bold mb-4">🎮 Gerar Controller</h2>
    <div class="card bg-base-200 shadow-xl max-w-md">
      <div class="card-body gap-4">
        <label class="form-control">
          <span class="label-text">Diretório raiz do projeto Laravel</span>
          <input id="ctrl-root" type="text" placeholder="/caminho/para/meu-projeto" class="input input-bordered" />
        </label>
        <label class="form-control">
          <span class="label-text">Nome</span>
          <input id="ctrl-name" type="text" placeholder="ProdutoController" class="input input-bordered" />
        </label>
        <label class="form-control">
          <span class="label-text">Model (opcional)</span>
          <input id="ctrl-model" type="text" placeholder="Produto" class="input input-bordered" />
        </label>
        <label class="label cursor-pointer gap-2">
          <input id="ctrl-resource" type="checkbox" class="checkbox checkbox-primary" checked />
          <span class="label-text">Resource Controller</span>
        </label>
        <button id="btn-controller" class="btn btn-primary w-full">Gerar Controller</button>
      </div>
    </div>
    ${resultBox()}`,

  // Model
  model: () => `
    <h2 class="text-2xl font-bold mb-4">🗃 Gerar Model</h2>
    <div class="card bg-base-200 shadow-xl max-w-md">
      <div class="card-body gap-4">
        <label class="form-control">
          <span class="label-text">Diretório raiz do projeto Laravel</span>
          <input id="mod-root" type="text" placeholder="/caminho/para/meu-projeto" class="input input-bordered" />
        </label>
        <label class="form-control">
          <span class="label-text">Nome</span>
          <input id="mod-name" type="text" placeholder="Produto" class="input input-bordered" />
        </label>
        <div class="flex gap-4">
          <label class="label cursor-pointer gap-2">
            <input id="mod-mig" type="checkbox" class="checkbox checkbox-secondary" />
            <span class="label-text">+ Migration</span>
          </label>
          <label class="label cursor-pointer gap-2">
            <input id="mod-ctrl" type="checkbox" class="checkbox checkbox-secondary" />
            <span class="label-text">+ Controller</span>
          </label>
        </div>
        <button id="btn-model" class="btn btn-primary w-full">Gerar Model</button>
      </div>
    </div>
    ${resultBox()}`,

  // Migration
  migration: () => `
    <h2 class="text-2xl font-bold mb-4">📋 Gerar Migration</h2>
    <div class="card bg-base-200 shadow-xl max-w-md">
      <div class="card-body gap-4">
        <label class="form-control">
          <span class="label-text">Diretório raiz do projeto Laravel</span>
          <input id="mig-root" type="text" placeholder="/caminho/para/meu-projeto" class="input input-bordered" />
        </label>
        <label class="form-control">
          <span class="label-text">Nome</span>
          <input id="mig-name" type="text" placeholder="create_produtos_table" class="input input-bordered" />
        </label>
        <label class="form-control">
          <span class="label-text">Table (opcional)</span>
          <input id="mig-table" type="text" placeholder="produtos" class="input input-bordered" />
        </label>
        <button id="btn-migration" class="btn btn-primary w-full">Gerar Migration</button>
      </div>
    </div>
    ${resultBox()}`,
};

// ── Handlers dos botões ───────────────────────────────────────────────────────

function val(id: string): string {
  return (document.getElementById(id) as HTMLInputElement)?.value.trim() ?? "";
}
function checked(id: string): boolean {
  return (document.getElementById(id) as HTMLInputElement)?.checked ?? false;
}
function checkedValues(cls: string): string[] {
  return Array.from(document.querySelectorAll<HTMLInputElement>(`.${cls}:checked`)).map(el => el.value);
}

async function runCommand(command: string, payload: unknown) {
  try {
    const result = await invoke<InvokeResult>(command, { input: payload });
    showResult(result, false);
  } catch (err) {
    showResult(String(err), true);
  }
}

function attachHandlers(page: string) {
  switch (page) {
    case "context":
      document.getElementById("btn-context")?.addEventListener("click", () =>
        runCommand("generate_context", {
          nome: val("ctx-nome"),
          base_path: val("ctx-path"),
          prefixo: val("ctx-prefix"),
          namespace_base: val("ctx-ns"),
          com_entity: checked("ctx-entity"),
          com_autorizacoes: checked("ctx-auth"),
          operacoes: checkedValues("ctx-op"),
        })
      );
      break;

    case "docker":
      document.getElementById("btn-docker")?.addEventListener("click", () =>
        runCommand("generate_docker", {
          app_name: val("dock-app"),
          php_version: val("dock-php"),
          node_version: parseInt(val("dock-node"), 10) || 20,
          output_path: val("dock-out") || ".",
          server_name: val("dock-server"),
          databases: checkedValues("dock-db"),
          with_redis: checked("dock-redis"),
          with_mailpit: checked("dock-mailpit"),
        })
      );
      break;

    case "pdv":
      document.getElementById("btn-pdv")?.addEventListener("click", () =>
        runCommand("generate_pdv", {
          migrations_only: checked("pdv-mig"),
          models_only: checked("pdv-mod"),
          project_root: val("pdv-root") || ".",
        })
      );
      break;

    case "estoque":
      document.getElementById("btn-estoque")?.addEventListener("click", () =>
        runCommand("generate_estoque", {
          base_path: val("est-path"),
          namespace_base: val("est-ns"),
          migration_path: val("est-mig"),
          tenant_id: val("est-tenant"),
          valuation_method: val("est-val"),
        })
      );
      break;

    case "logistica":
      document.getElementById("btn-logistica")?.addEventListener("click", () =>
        runCommand("generate_logistica_reversa", {
          base_path: val("log-path"),
          namespace_base: val("log-ns"),
          migration_path: val("log-mig"),
          erp_id: val("log-erp"),
          company_name: val("log-company"),
          warehouse_id: val("log-wh"),
        })
      );
      break;

    case "landing":
      document.getElementById("btn-landing")?.addEventListener("click", () => {
        const layout = val("lp-layout");
        const isSaas = layout === "saas";
        runCommand("generate_landing_page", {
          product_name: val("lp-prod"),
          tagline: val("lp-tag"),
          company_name: val("lp-company"),
          theme: isSaas ? "light" : val("lp-theme"),
          layout,
          sections: isSaas ? checkedValues("lp-sec-saas-chk") : checkedValues("lp-sec"),
          output_dir: val("lp-out") || "landing",
        });
      });
      break;

    case "controller":
      document.getElementById("btn-controller")?.addEventListener("click", () => {
        const model = val("ctrl-model");
        runCommand("generate_controller", {
          name: val("ctrl-name"),
          resource: checked("ctrl-resource"),
          model: model || null,
          project_root: val("ctrl-root") || ".",
        });
      });
      break;

    case "model":
      document.getElementById("btn-model")?.addEventListener("click", () =>
        runCommand("generate_model", {
          name: val("mod-name"),
          migration: checked("mod-mig"),
          controller: checked("mod-ctrl"),
          project_root: val("mod-root") || ".",
        })
      );
      break;

    case "migration":
      document.getElementById("btn-migration")?.addEventListener("click", () => {
        const table = val("mig-table");
        runCommand("generate_migration", {
          name: val("mig-name"),
          table: table || null,
          project_root: val("mig-root") || ".",
        });
      });
      break;
  }
}

// ── Roteamento de páginas ─────────────────────────────────────────────────────

function navigate(page: string) {
  // Atualiza estado ativo no sidebar
  document.querySelectorAll<HTMLElement>(".nav-link").forEach(el => {
    el.classList.toggle("active", el.dataset.page === page);
  });

  const render = pages[page] ?? pages.home;
  setContent(render());
  attachHandlers(page);
}

// ── Bootstrap ─────────────────────────────────────────────────────────────────

document.addEventListener("DOMContentLoaded", () => {
  // Delegação de cliques no sidebar
  document.getElementById("sidebar-nav")?.addEventListener("click", (e) => {
    const target = (e.target as HTMLElement).closest<HTMLElement>("[data-page]");
    if (target?.dataset.page) {
      navigate(target.dataset.page);
      // Fecha sidebar no mobile
      (document.getElementById("sidebar-toggle") as HTMLInputElement).checked = false;
    }
  });

  navigate("home");
});
