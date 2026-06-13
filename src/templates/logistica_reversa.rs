// ─── Migrations (10 tabelas) ─────────────────────────────────────────────────

pub fn migration_seguradoras() -> &'static str {
    r#"<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('seguradoras', function (Blueprint $table) {
            $table->id();
            $table->string('cnpj', 18)->unique()->comment('Formato: XX.XXX.XXX/XXXX-XX');
            $table->string('razao_social');
            $table->string('api_key')->nullable()->comment('Chave para integracao via webhook');
            $table->unsignedSmallInteger('sla_coleta_horas')->default(48)->comment('SLA maximo para coleta em horas');
            $table->boolean('ativo')->default(true);
            $table->timestamps();
            $table->softDeletes();
        });
    }

    public function down(): void
    {
        Schema::dropIfExists('seguradoras');
    }
};
"#
}

pub fn migration_transportadoras() -> &'static str {
    r#"<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('transportadoras', function (Blueprint $table) {
            $table->id();
            $table->string('cnpj', 18)->unique()->comment('Formato: XX.XXX.XXX/XXXX-XX');
            $table->string('nome');
            $table->string('tipo_veiculo')->comment('Ex: moto, van, caminhao');
            $table->text('regiao_cobertura')->nullable()->comment('JSON ou texto com regioes atendidas');
            $table->string('api_endpoint')->nullable()->comment('Endpoint da API Intelipost/carrier');
            $table->boolean('ativo')->default(true);
            $table->timestamps();
            $table->softDeletes();
        });
    }

    public function down(): void
    {
        Schema::dropIfExists('transportadoras');
    }
};
"#
}

pub fn migration_segurados() -> &'static str {
    r#"<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('segurados', function (Blueprint $table) {
            $table->id();
            $table->string('cpf_cnpj', 18)->unique()->comment('CPF ou CNPJ do segurado');
            $table->string('nome');
            $table->string('telefone', 20)->nullable();
            $table->string('email')->nullable();
            $table->text('endereco_coleta')->nullable()->comment('Endereco completo para coleta');
            $table->string('cep_coleta', 10)->nullable();
            $table->timestamps();
            $table->softDeletes();
        });
    }

    public function down(): void
    {
        Schema::dropIfExists('segurados');
    }
};
"#
}

pub fn migration_apolices() -> &'static str {
    r#"<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('apolices', function (Blueprint $table) {
            $table->id();
            $table->foreignId('seguradora_id')->constrained('seguradoras')->restrictOnDelete();
            $table->foreignId('segurado_id')->constrained('segurados')->restrictOnDelete();
            $table->string('numero_apolice')->unique();
            $table->string('tipo_seguro')->comment('auto | eletronicos | residencial');
            $table->date('vigencia_inicio')->nullable();
            $table->date('vigencia_fim')->nullable();
            $table->boolean('ativa')->default(true);
            $table->timestamps();
            $table->softDeletes();
        });
    }

    public function down(): void
    {
        Schema::dropIfExists('apolices');
    }
};
"#
}

pub fn migration_sinistros() -> &'static str {
    r#"<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('sinistros', function (Blueprint $table) {
            $table->id();
            $table->foreignId('apolice_id')->constrained('apolices')->restrictOnDelete();
            $table->string('numero_sinistro_seguradora')->unique()->comment('Referencia externa da seguradora');
            $table->dateTime('data_abertura');
            $table->string('status')->default('aguardando_coleta')
                ->comment('aguardando_coleta | em_transito | em_triagem | finalizado | frustrado');
            $table->text('observacoes')->nullable();
            $table->timestamps();
            $table->softDeletes();

            $table->index('status');
            $table->index('data_abertura');
        });
    }

    public function down(): void
    {
        Schema::dropIfExists('sinistros');
    }
};
"#
}

pub fn migration_itens_sinistrados() -> &'static str {
    r#"<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('itens_sinistrados', function (Blueprint $table) {
            $table->id();
            $table->foreignId('sinistro_id')->constrained('sinistros')->cascadeOnDelete();
            $table->string('categoria')->comment('smartphone | peca_automotiva | linha_branca');
            $table->string('marca')->nullable();
            $table->string('modelo')->nullable();
            $table->string('identificador_unico')->nullable()
                ->comment('IMEI (smartphone), Chassi (auto), N/S (outros)');
            $table->text('defeito_declarado')->nullable();
            $table->timestamps();
        });
    }

    public function down(): void
    {
        Schema::dropIfExists('itens_sinistrados');
    }
};
"#
}

pub fn migration_ordens_coleta() -> &'static str {
    r#"<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('ordens_coleta', function (Blueprint $table) {
            $table->id();
            $table->foreignId('sinistro_id')->constrained('sinistros')->restrictOnDelete();
            $table->foreignId('transportadora_id')->constrained('transportadoras')->restrictOnDelete();
            $table->dateTime('data_agendamento')->nullable();
            $table->dateTime('data_efetivacao')->nullable()->comment('Data em que a coleta foi confirmada');
            $table->string('codigo_rastreio')->unique()->nullable();
            $table->string('status_coleta')->default('pendente')
                ->comment('pendente | a_caminho | coletado | frustrado');
            $table->unsignedTinyInteger('tentativas_coleta')->default(0);
            $table->text('motivo_frustracao')->nullable();
            $table->timestamps();
            $table->softDeletes();

            $table->index('status_coleta');
            $table->index('data_agendamento');
        });
    }

    public function down(): void
    {
        Schema::dropIfExists('ordens_coleta');
    }
};
"#
}

pub fn migration_movimentacoes_logisticas() -> &'static str {
    r#"<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('movimentacoes_logisticas', function (Blueprint $table) {
            $table->id();
            $table->foreignId('ordem_coleta_id')->constrained('ordens_coleta')->cascadeOnDelete();
            $table->dateTime('data_hora');
            $table->string('localizacao_atual')->nullable()->comment('Cidade, bairro ou CD');
            $table->string('evento')->comment('saiu_para_coleta | recebido_cd | em_transferencia | entregue | frustrado');
            $table->text('descricao')->nullable();
            $table->json('metadata')->nullable()->comment('Dados extras do carrier (lat/lng, hash, etc)');
            $table->timestamps();

            $table->index(['ordem_coleta_id', 'data_hora']);
        });
    }

    public function down(): void
    {
        Schema::dropIfExists('movimentacoes_logisticas');
    }
};
"#
}

pub fn migration_recebimentos_cd() -> &'static str {
    r#"<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('recebimentos_cd', function (Blueprint $table) {
            $table->id();
            $table->foreignId('ordem_coleta_id')->unique()->constrained('ordens_coleta')->restrictOnDelete();
            $table->dateTime('data_recebimento');
            $table->unsignedBigInteger('usuario_recebedor_id')->comment('FK para tabela de usuarios do sistema');
            $table->string('doca')->nullable()->comment('Ex: DOCA-A1, DOCA-B3');
            $table->string('condicao_embalagem')
                ->comment('integra | danificada | violada | sem_embalagem');
            $table->text('observacoes_recebimento')->nullable();
            $table->string('foto_recebimento_hash')->nullable()->comment('Hash do arquivo de foto no storage');
            $table->timestamps();
        });
    }

    public function down(): void
    {
        Schema::dropIfExists('recebimentos_cd');
    }
};
"#
}

pub fn migration_laudos_triagem() -> &'static str {
    r#"<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('laudos_triagem', function (Blueprint $table) {
            $table->id();
            $table->foreignId('item_sinistrado_id')->unique()->constrained('itens_sinistrados')->restrictOnDelete();
            $table->dateTime('data_inspecao');
            $table->string('tecnico_responsavel');
            $table->text('parecer_tecnico');
            $table->string('classificacao_dano')
                ->comment('estetico | funcional | perda_total');
            $table->string('destinacao_final')
                ->comment('leilao_salvados | descarte_ecologico | reparo_autorizada | devolucao_segurado');
            $table->string('certificado_descarte')->nullable()
                ->comment('Hash/URL do documento ESG de descarte ecologico certificado');
            $table->timestamps();

            $table->index('classificacao_dano');
            $table->index('destinacao_final');
        });
    }

    public function down(): void
    {
        Schema::dropIfExists('laudos_triagem');
    }
};
"#
}

// ─── Eloquent Models ─────────────────────────────────────────────────────────

pub fn model_seguradora(ns: &str) -> String {
    format!(
        r#"<?php

declare(strict_types=1);

namespace {ns}\Seguradora\Infra\Persistence\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Database\Eloquent\Relations\HasMany;

class Seguradora extends Model
{{
    use SoftDeletes;

    protected $table = 'seguradoras';

    protected $fillable = [
        'cnpj',
        'razao_social',
        'api_key',
        'sla_coleta_horas',
        'ativo',
    ];

    protected $casts = [
        'sla_coleta_horas' => 'integer',
        'ativo'            => 'boolean',
    ];

    public function apolices(): HasMany
    {{
        return $this->hasMany(\{ns}\Apolice\Infra\Persistence\Models\Apolice::class, 'seguradora_id');
    }}
}}
"#
    )
}

pub fn model_transportadora(ns: &str) -> String {
    format!(
        r#"<?php

declare(strict_types=1);

namespace {ns}\Transportadora\Infra\Persistence\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Database\Eloquent\Relations\HasMany;

class Transportadora extends Model
{{
    use SoftDeletes;

    protected $table = 'transportadoras';

    protected $fillable = [
        'cnpj',
        'nome',
        'tipo_veiculo',
        'regiao_cobertura',
        'api_endpoint',
        'ativo',
    ];

    protected $casts = [
        'ativo' => 'boolean',
    ];

    public function ordensColeta(): HasMany
    {{
        return $this->hasMany(\{ns}\OrdemColeta\Infra\Persistence\Models\OrdemColeta::class, 'transportadora_id');
    }}
}}
"#
    )
}

pub fn model_segurado(ns: &str) -> String {
    format!(
        r#"<?php

declare(strict_types=1);

namespace {ns}\Segurado\Infra\Persistence\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Database\Eloquent\Relations\HasMany;

class Segurado extends Model
{{
    use SoftDeletes;

    protected $table = 'segurados';

    protected $fillable = [
        'cpf_cnpj',
        'nome',
        'telefone',
        'email',
        'endereco_coleta',
        'cep_coleta',
    ];

    public function apolices(): HasMany
    {{
        return $this->hasMany(\{ns}\Apolice\Infra\Persistence\Models\Apolice::class, 'segurado_id');
    }}
}}
"#
    )
}

pub fn model_apolice(ns: &str) -> String {
    format!(
        r#"<?php

declare(strict_types=1);

namespace {ns}\Apolice\Infra\Persistence\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\HasMany;

class Apolice extends Model
{{
    use SoftDeletes;

    protected $table = 'apolices';

    protected $fillable = [
        'seguradora_id',
        'segurado_id',
        'numero_apolice',
        'tipo_seguro',
        'vigencia_inicio',
        'vigencia_fim',
        'ativa',
    ];

    protected $casts = [
        'vigencia_inicio' => 'date',
        'vigencia_fim'    => 'date',
        'ativa'           => 'boolean',
    ];

    public function seguradora(): BelongsTo
    {{
        return $this->belongsTo(\{ns}\Seguradora\Infra\Persistence\Models\Seguradora::class, 'seguradora_id');
    }}

    public function segurado(): BelongsTo
    {{
        return $this->belongsTo(\{ns}\Segurado\Infra\Persistence\Models\Segurado::class, 'segurado_id');
    }}

    public function sinistros(): HasMany
    {{
        return $this->hasMany(\{ns}\Sinistro\Infra\Persistence\Models\Sinistro::class, 'apolice_id');
    }}
}}
"#
    )
}

pub fn model_sinistro(ns: &str) -> String {
    format!(
        r#"<?php

declare(strict_types=1);

namespace {ns}\Sinistro\Infra\Persistence\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\HasMany;

class Sinistro extends Model
{{
    use SoftDeletes;

    protected $table = 'sinistros';

    protected $fillable = [
        'apolice_id',
        'numero_sinistro_seguradora',
        'data_abertura',
        'status',
        'observacoes',
    ];

    protected $casts = [
        'data_abertura' => 'datetime',
    ];

    public function apolice(): BelongsTo
    {{
        return $this->belongsTo(\{ns}\Apolice\Infra\Persistence\Models\Apolice::class, 'apolice_id');
    }}

    public function itens(): HasMany
    {{
        return $this->hasMany(ItemSinistrado::class, 'sinistro_id');
    }}

    public function ordensColeta(): HasMany
    {{
        return $this->hasMany(\{ns}\OrdemColeta\Infra\Persistence\Models\OrdemColeta::class, 'sinistro_id');
    }}
}}
"#
    )
}

pub fn model_item_sinistrado(ns: &str) -> String {
    format!(
        r#"<?php

declare(strict_types=1);

namespace {ns}\Sinistro\Infra\Persistence\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\HasOne;

class ItemSinistrado extends Model
{{
    protected $table = 'itens_sinistrados';

    protected $fillable = [
        'sinistro_id',
        'categoria',
        'marca',
        'modelo',
        'identificador_unico',
        'defeito_declarado',
    ];

    public function sinistro(): BelongsTo
    {{
        return $this->belongsTo(Sinistro::class, 'sinistro_id');
    }}

    public function laudo(): HasOne
    {{
        return $this->hasOne(\{ns}\Triagem\Infra\Persistence\Models\LaudoTriagem::class, 'item_sinistrado_id');
    }}
}}
"#
    )
}

pub fn model_ordem_coleta(ns: &str) -> String {
    format!(
        r#"<?php

declare(strict_types=1);

namespace {ns}\OrdemColeta\Infra\Persistence\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\Relations\HasOne;

class OrdemColeta extends Model
{{
    use SoftDeletes;

    protected $table = 'ordens_coleta';

    protected $fillable = [
        'sinistro_id',
        'transportadora_id',
        'data_agendamento',
        'data_efetivacao',
        'codigo_rastreio',
        'status_coleta',
        'tentativas_coleta',
        'motivo_frustracao',
    ];

    protected $casts = [
        'data_agendamento' => 'datetime',
        'data_efetivacao'  => 'datetime',
        'tentativas_coleta' => 'integer',
    ];

    public function sinistro(): BelongsTo
    {{
        return $this->belongsTo(\{ns}\Sinistro\Infra\Persistence\Models\Sinistro::class, 'sinistro_id');
    }}

    public function transportadora(): BelongsTo
    {{
        return $this->belongsTo(\{ns}\Transportadora\Infra\Persistence\Models\Transportadora::class, 'transportadora_id');
    }}

    public function movimentacoes(): HasMany
    {{
        return $this->hasMany(MovimentacaoLogistica::class, 'ordem_coleta_id')->orderBy('data_hora');
    }}

    public function recebimento(): HasOne
    {{
        return $this->hasOne(RecebimentoCd::class, 'ordem_coleta_id');
    }}
}}
"#
    )
}

pub fn model_movimentacao_logistica(ns: &str) -> String {
    format!(
        r#"<?php

declare(strict_types=1);

namespace {ns}\OrdemColeta\Infra\Persistence\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

class MovimentacaoLogistica extends Model
{{
    protected $table = 'movimentacoes_logisticas';

    protected $fillable = [
        'ordem_coleta_id',
        'data_hora',
        'localizacao_atual',
        'evento',
        'descricao',
        'metadata',
    ];

    protected $casts = [
        'data_hora' => 'datetime',
        'metadata'  => 'array',
    ];

    public function ordemColeta(): BelongsTo
    {{
        return $this->belongsTo(OrdemColeta::class, 'ordem_coleta_id');
    }}
}}
"#
    )
}

pub fn model_recebimento_cd(ns: &str) -> String {
    format!(
        r#"<?php

declare(strict_types=1);

namespace {ns}\OrdemColeta\Infra\Persistence\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

class RecebimentoCd extends Model
{{
    protected $table = 'recebimentos_cd';

    protected $fillable = [
        'ordem_coleta_id',
        'data_recebimento',
        'usuario_recebedor_id',
        'doca',
        'condicao_embalagem',
        'observacoes_recebimento',
        'foto_recebimento_hash',
    ];

    protected $casts = [
        'data_recebimento' => 'datetime',
    ];

    public function ordemColeta(): BelongsTo
    {{
        return $this->belongsTo(OrdemColeta::class, 'ordem_coleta_id');
    }}
}}
"#
    )
}

pub fn model_laudo_triagem(ns: &str) -> String {
    format!(
        r#"<?php

declare(strict_types=1);

namespace {ns}\Triagem\Infra\Persistence\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

class LaudoTriagem extends Model
{{
    protected $table = 'laudos_triagem';

    protected $fillable = [
        'item_sinistrado_id',
        'data_inspecao',
        'tecnico_responsavel',
        'parecer_tecnico',
        'classificacao_dano',
        'destinacao_final',
        'certificado_descarte',
    ];

    protected $casts = [
        'data_inspecao' => 'datetime',
    ];

    public function itemSinistrado(): BelongsTo
    {{
        return $this->belongsTo(\{ns}\Sinistro\Infra\Persistence\Models\ItemSinistrado::class, 'item_sinistrado_id');
    }}
}}
"#
    )
}

// ─── Manager JSON ─────────────────────────────────────────────────────────────

pub fn manager_json(erp_id: &str, company_name: &str, warehouse_id: &str) -> String {
    format!(
        r#"{{
  "logistica_reversa_manager": {{
    "version": "2.1.0",
    "tenant_config": {{
      "erp_id": "{erp_id}",
      "company_name": "{company_name}",
      "default_warehouse_id": "{warehouse_id}",
      "timezone": "America/Sao_Paulo"
    }},
    "business_rules": {{
      "sla_definitions": {{
        "max_hours_to_collect": 48,
        "max_hours_to_triage": 24,
        "max_collection_attempts": 3
      }},
      "salvage_destinations": [
        "LEILAO",
        "DESCARTE_ECOLOGICO_CERTIFICADO",
        "REPARO_AUTORIZADA",
        "DEVOLUCAO_CLIENTE"
      ]
    }},
    "integrations": {{
      "insurance_webhooks": {{
        "enabled": true,
        "endpoints": [
          {{
            "event": "COLETA_EFETIVADA",
            "url": "https://api.seguradora.com.br/v2/webhooks/reversa/status",
            "retry_policy": "exponential_backoff"
          }},
          {{
            "event": "LAUDO_CONCLUIDO",
            "url": "https://api.seguradora.com.br/v2/webhooks/reversa/laudos",
            "retry_policy": "exponential_backoff"
          }}
        ]
      }},
      "carriers_api": {{
        "provider": "Intelipost_Reverse",
        "auto_dispatch": true
      }}
    }},
    "database_schema": {{
      "tables": [
        {{
          "name": "ordens_coleta",
          "description": "Gerenciamento do transporte do bem sinistrado",
          "fields": [
            {{ "name": "id", "type": "bigint", "primary_key": true }},
            {{ "name": "sinistro_id", "type": "bigint", "foreign_key": "sinistros.id" }},
            {{ "name": "transportadora_id", "type": "bigint", "foreign_key": "transportadoras.id" }},
            {{ "name": "codigo_rastreio", "type": "string", "unique": true }},
            {{ "name": "status_coleta", "type": "enum", "values": ["pendente", "a_caminho", "coletado", "frustrado"] }}
          ]
        }},
        {{
          "name": "laudos_triagem",
          "description": "Inspecao tecnica e destinacao do bem",
          "fields": [
            {{ "name": "id", "type": "bigint", "primary_key": true }},
            {{ "name": "item_sinistrado_id", "type": "bigint", "foreign_key": "itens_sinistrados.id" }},
            {{ "name": "classificacao_dano", "type": "string", "required": true }},
            {{ "name": "destinacao_final", "type": "string", "required": true }},
            {{ "name": "certificado_descarte", "type": "string", "description": "Hash do documento ESG, se aplicavel", "nullable": true }}
          ]
        }}
      ]
    }}
  }}
}}
"#
    )
}
