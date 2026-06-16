use crate::flows::ci4_backend::generator::Ci4Options;
use crate::flows::landing_page::generator::{LandingPageOptions, generate as generate_lp};
use anyhow::Result;

/// Gera a landing page + injeta a seção de contato conectada ao backend CI4.
/// O backend CI4 em si é gerado pelo chamador (mod.rs) com generate_ci4().
pub fn generate_combined(lp_opts: &LandingPageOptions, _ci4_opts: &Ci4Options) -> Result<()> {
    // A landing page já carrega o api_url internamente via LandingPageOptions.
    // Basta delegar ao gerador padrão — ele já sabe injetar o formulário
    // quando api_url não está vazio.
    generate_lp(lp_opts)
}

// ── Template da seção de contato com fetch() ─────────────────────────────────
// Exportado para ser usado pelo generator.rs da landing_page.

pub fn section_contact(product: &str, api_url: &str) -> String {
    CONTACT_SECTION
        .replace("[PRODUCT]", product)
        .replace("[API_URL]", api_url)
}

const CONTACT_SECTION: &str = r##"<section id="contact" class="py-24 bg-base-100">
  <div class="mx-auto max-w-2xl px-6" data-fade>
    <div class="text-center mb-12">
      <h2 class="text-4xl font-black tracking-tight mb-4">Fale com a gente</h2>
      <p class="text-base-content/60 text-lg">Deixe seu contato — nossa equipe retorna em até 24h.</p>
    </div>

    <form id="contact-form" class="card bg-base-200 shadow-xl">
      <div class="card-body gap-4">

        <div class="form-control">
          <label class="label"><span class="label-text font-semibold">Nome *</span></label>
          <input id="cf-name" type="text" placeholder="Seu nome completo"
                 class="input input-bordered w-full" required minlength="2" maxlength="120" />
        </div>

        <div class="form-control">
          <label class="label"><span class="label-text font-semibold">E-mail *</span></label>
          <input id="cf-email" type="email" placeholder="seu@email.com"
                 class="input input-bordered w-full" required maxlength="120" />
        </div>

        <div class="form-control">
          <label class="label"><span class="label-text font-semibold">Telefone</span></label>
          <input id="cf-phone" type="tel" placeholder="(11) 9 0000-0000"
                 class="input input-bordered w-full" maxlength="30" />
        </div>

        <div class="form-control">
          <label class="label"><span class="label-text font-semibold">Mensagem</span></label>
          <textarea id="cf-message" placeholder="Como podemos ajudar?"
                    class="textarea textarea-bordered w-full h-28 resize-none" maxlength="1000"></textarea>
        </div>

        <div id="cf-feedback" class="hidden"></div>

        <div class="card-actions justify-end pt-2">
          <button id="cf-submit" type="submit" class="btn btn-primary px-10">
            Enviar mensagem
          </button>
        </div>

      </div>
    </form>
  </div>

  <script>
  (function () {
    const API_URL = '[API_URL]';

    const form     = document.getElementById('contact-form');
    const feedback = document.getElementById('cf-feedback');
    const btn      = document.getElementById('cf-submit');

    function showFeedback(msg, isError) {
      feedback.className = isError
        ? 'alert alert-error text-sm'
        : 'alert alert-success text-sm';
      feedback.textContent = msg;
      feedback.classList.remove('hidden');
    }

    form.addEventListener('submit', async function (e) {
      e.preventDefault();
      btn.disabled    = true;
      btn.textContent = 'Enviando…';
      feedback.classList.add('hidden');

      const payload = {
        name   : document.getElementById('cf-name').value.trim(),
        email  : document.getElementById('cf-email').value.trim(),
        phone  : document.getElementById('cf-phone').value.trim(),
        message: document.getElementById('cf-message').value.trim(),
        source : 'landing',
      };

      try {
        const res = await fetch(`${API_URL}/contact`, {
          method : 'POST',
          headers: { 'Content-Type': 'application/json' },
          body   : JSON.stringify(payload),
        });

        if (res.ok) {
          showFeedback('✔ Mensagem enviada! Retornaremos em breve.', false);
          form.reset();
        } else {
          const err = await res.json().catch(() => ({}));
          const msg = typeof err.error === 'string'
            ? err.error
            : (Object.values(err.error ?? {}).join(' ') || 'Erro ao enviar. Tente novamente.');
          showFeedback(msg, true);
        }
      } catch (_) {
        showFeedback('Falha de conexão. Verifique sua internet e tente novamente.', true);
      } finally {
        btn.disabled    = false;
        btn.textContent = 'Enviar mensagem';
      }
    });
  })();
  </script>
</section>"##;
