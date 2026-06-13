// PDV (Ponto de Venda) — Laravel migration & model templates for PostgreSQL
// Migration order respects FK dependencies.

pub fn migrations() -> Vec<(&'static str, &'static str)> {
    vec![
        ("create_lojas_table", MIGRATION_LOJAS),
        ("create_categorias_table", MIGRATION_CATEGORIAS),
        ("create_usuarios_table", MIGRATION_USUARIOS),
        ("create_clientes_table", MIGRATION_CLIENTES),
        ("create_produtos_table", MIGRATION_PRODUTOS),
        ("create_turnos_caixa_table", MIGRATION_TURNOS_CAIXA),
        ("create_movimentacoes_caixa_table", MIGRATION_MOVIMENTACOES_CAIXA),
        ("create_vendas_table", MIGRATION_VENDAS),
        ("create_venda_items_table", MIGRATION_VENDA_ITEMS),
        ("create_pagamentos_table", MIGRATION_PAGAMENTOS),
    ]
}

pub fn models() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Loja", MODEL_LOJA),
        ("Categoria", MODEL_CATEGORIA),
        ("Usuario", MODEL_USUARIO),
        ("Cliente", MODEL_CLIENTE),
        ("Produto", MODEL_PRODUTO),
        ("TurnoCaixa", MODEL_TURNO_CAIXA),
        ("MovimentacaoCaixa", MODEL_MOVIMENTACAO_CAIXA),
        ("Venda", MODEL_VENDA),
        ("VendaItem", MODEL_VENDA_ITEM),
        ("Pagamento", MODEL_PAGAMENTO),
    ]
}

// ─── Migrations ──────────────────────────────────────────────────────────────

const MIGRATION_LOJAS: &str = r#"<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('lojas', function (Blueprint $table) {
            $table->uuid('id')->primary();
            $table->string('cnpj', 18)->unique();
            $table->string('razao_social');
            $table->text('endereco')->nullable();
            $table->json('configuracoes_fiscais')->nullable()->comment('Certificados e parâmetros fiscais (SEFAZ, NF-e)');
            $table->timestamps();
        });
    }

    public function down(): void
    {
        Schema::dropIfExists('lojas');
    }
};
"#;

const MIGRATION_CATEGORIAS: &str = r#"<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('categorias', function (Blueprint $table) {
            $table->uuid('id')->primary();
            $table->foreignUuid('loja_id')->constrained('lojas')->cascadeOnDelete();
            $table->string('nome');
            $table->timestamps();
        });
    }

    public function down(): void
    {
        Schema::dropIfExists('categorias');
    }
};
"#;

const MIGRATION_USUARIOS: &str = r#"<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('usuarios', function (Blueprint $table) {
            $table->uuid('id')->primary();
            $table->foreignUuid('loja_id')->constrained('lojas')->cascadeOnDelete();
            $table->string('nome');
            $table->string('email')->unique();
            $table->string('senha_hash');
            $table->string('pin', 6)->nullable()->comment('PIN numérico para login rápido no PDV');
            $table->enum('perfil', ['admin', 'gerente', 'operador'])->default('operador');
            $table->timestamps();
        });
    }

    public function down(): void
    {
        Schema::dropIfExists('usuarios');
    }
};
"#;

const MIGRATION_CLIENTES: &str = r#"<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('clientes', function (Blueprint $table) {
            $table->uuid('id')->primary();
            $table->foreignUuid('loja_id')->constrained('lojas')->cascadeOnDelete();
            $table->string('cpf_cnpj', 18)->nullable()->index();
            $table->string('nome');
            $table->string('email')->nullable();
            $table->string('telefone', 20)->nullable();
            $table->unsignedInteger('pontos_fidelidade')->default(0);
            $table->timestamps();
        });
    }

    public function down(): void
    {
        Schema::dropIfExists('clientes');
    }
};
"#;

const MIGRATION_PRODUTOS: &str = r#"<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('produtos', function (Blueprint $table) {
            $table->uuid('id')->primary();
            $table->foreignUuid('loja_id')->constrained('lojas')->cascadeOnDelete();
            $table->foreignUuid('categoria_id')->nullable()->constrained('categorias')->nullOnDelete();
            $table->string('codigo_barras')->nullable()->index()->comment('EAN-13 / EAN-8');
            $table->string('nome');
            $table->decimal('preco_venda', 10, 2);
            $table->decimal('preco_custo', 10, 2)->nullable();
            $table->unsignedInteger('estoque_atual')->default(0);
            $table->string('ncm', 8)->nullable()->comment('Nomenclatura Comum do Mercosul');
            $table->string('cest', 7)->nullable()->comment('Código Especificador da Substituição Tributária');
            $table->string('cfop_padrao', 4)->nullable()->comment('Código Fiscal de Operações e Prestações');
            $table->timestamps();
        });
    }

    public function down(): void
    {
        Schema::dropIfExists('produtos');
    }
};
"#;

const MIGRATION_TURNOS_CAIXA: &str = r#"<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('turnos_caixa', function (Blueprint $table) {
            $table->uuid('id')->primary();
            $table->foreignUuid('loja_id')->constrained('lojas')->cascadeOnDelete();
            $table->foreignUuid('usuario_id')->constrained('usuarios')->restrictOnDelete();
            $table->timestamp('data_abertura');
            $table->timestamp('data_fechamento')->nullable();
            $table->decimal('fundo_troco_inicial', 10, 2)->default(0);
            $table->decimal('saldo_final_esperado', 10, 2)->nullable();
            $table->decimal('saldo_final_informado', 10, 2)->nullable();
            $table->enum('status', ['aberto', 'fechado', 'divergente'])->default('aberto');
            $table->timestamps();
        });
    }

    public function down(): void
    {
        Schema::dropIfExists('turnos_caixa');
    }
};
"#;

const MIGRATION_MOVIMENTACOES_CAIXA: &str = r#"<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('movimentacoes_caixa', function (Blueprint $table) {
            $table->uuid('id')->primary();
            $table->foreignUuid('turno_caixa_id')->constrained('turnos_caixa')->cascadeOnDelete();
            $table->enum('tipo', ['suprimento', 'sangria']);
            $table->decimal('valor', 10, 2);
            $table->text('motivo')->nullable();
            $table->timestamp('data_hora')->useCurrent();
            $table->timestamps();
        });
    }

    public function down(): void
    {
        Schema::dropIfExists('movimentacoes_caixa');
    }
};
"#;

const MIGRATION_VENDAS: &str = r#"<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('vendas', function (Blueprint $table) {
            $table->uuid('id')->primary();
            $table->foreignUuid('turno_caixa_id')->constrained('turnos_caixa')->restrictOnDelete();
            $table->foreignUuid('cliente_id')->nullable()->constrained('clientes')->nullOnDelete();
            $table->timestamp('data_hora')->useCurrent()->index();
            $table->decimal('subtotal', 10, 2);
            $table->decimal('desconto', 10, 2)->default(0);
            $table->decimal('total', 10, 2);
            $table->enum('status', ['pendente', 'concluida', 'cancelada'])->default('pendente');
            $table->string('chave_nfe', 44)->nullable()->comment('Chave de acesso da NF-e/NFC-e (44 dígitos)');
            $table->enum('status_fiscal', ['pendente', 'autorizado', 'rejeitado', 'contingencia'])->default('pendente');
            $table->timestamps();
        });
    }

    public function down(): void
    {
        Schema::dropIfExists('vendas');
    }
};
"#;

const MIGRATION_VENDA_ITEMS: &str = r#"<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('venda_items', function (Blueprint $table) {
            $table->uuid('id')->primary();
            $table->foreignUuid('venda_id')->constrained('vendas')->cascadeOnDelete();
            $table->foreignUuid('produto_id')->constrained('produtos')->restrictOnDelete();
            $table->decimal('quantidade', 10, 3)->default(1)->comment('Suporta frações para produtos por peso');
            $table->decimal('preco_unitario', 10, 2);
            $table->decimal('subtotal', 10, 2);
            $table->timestamps();
        });
    }

    public function down(): void
    {
        Schema::dropIfExists('venda_items');
    }
};
"#;

const MIGRATION_PAGAMENTOS: &str = r#"<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('pagamentos', function (Blueprint $table) {
            $table->uuid('id')->primary();
            $table->foreignUuid('venda_id')->constrained('vendas')->cascadeOnDelete();
            $table->enum('metodo', ['dinheiro', 'pix', 'cartao_credito', 'cartao_debito']);
            $table->decimal('valor', 10, 2);
            $table->enum('status', ['pendente', 'aprovado', 'recusado'])->default('pendente');
            $table->string('nsu_transacao')->nullable()->comment('Comprovante único do TEF/Gateway de pagamento');
            $table->string('bandeira_cartao', 20)->nullable()->comment('Visa, Master, Elo, etc.');
            $table->timestamps();
        });
    }

    public function down(): void
    {
        Schema::dropIfExists('pagamentos');
    }
};
"#;

// ─── Models ──────────────────────────────────────────────────────────────────

const MODEL_LOJA: &str = r#"<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Concerns\HasUuids;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;

class Loja extends Model
{
    use HasFactory, HasUuids;

    protected $table = 'lojas';

    protected $fillable = [
        'cnpj',
        'razao_social',
        'endereco',
        'configuracoes_fiscais',
    ];

    protected $casts = [
        'configuracoes_fiscais' => 'array',
    ];

    public function usuarios()
    {
        return $this->hasMany(Usuario::class);
    }

    public function categorias()
    {
        return $this->hasMany(Categoria::class);
    }

    public function produtos()
    {
        return $this->hasMany(Produto::class);
    }

    public function clientes()
    {
        return $this->hasMany(Cliente::class);
    }

    public function turnosCaixa()
    {
        return $this->hasMany(TurnoCaixa::class);
    }
}
"#;

const MODEL_CATEGORIA: &str = r#"<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Concerns\HasUuids;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;

class Categoria extends Model
{
    use HasFactory, HasUuids;

    protected $table = 'categorias';

    protected $fillable = [
        'loja_id',
        'nome',
    ];

    public function loja()
    {
        return $this->belongsTo(Loja::class);
    }

    public function produtos()
    {
        return $this->hasMany(Produto::class);
    }
}
"#;

const MODEL_USUARIO: &str = r#"<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Concerns\HasUuids;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Foundation\Auth\User as Authenticatable;

class Usuario extends Authenticatable
{
    use HasFactory, HasUuids;

    protected $table = 'usuarios';

    protected $fillable = [
        'loja_id',
        'nome',
        'email',
        'senha_hash',
        'pin',
        'perfil',
    ];

    protected $hidden = [
        'senha_hash',
        'pin',
    ];

    public function loja()
    {
        return $this->belongsTo(Loja::class);
    }

    public function turnosCaixa()
    {
        return $this->hasMany(TurnoCaixa::class);
    }
}
"#;

const MODEL_CLIENTE: &str = r#"<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Concerns\HasUuids;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;

class Cliente extends Model
{
    use HasFactory, HasUuids;

    protected $table = 'clientes';

    protected $fillable = [
        'loja_id',
        'cpf_cnpj',
        'nome',
        'email',
        'telefone',
        'pontos_fidelidade',
    ];

    public function loja()
    {
        return $this->belongsTo(Loja::class);
    }

    public function vendas()
    {
        return $this->hasMany(Venda::class);
    }
}
"#;

const MODEL_PRODUTO: &str = r#"<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Concerns\HasUuids;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;

class Produto extends Model
{
    use HasFactory, HasUuids;

    protected $table = 'produtos';

    protected $fillable = [
        'loja_id',
        'categoria_id',
        'codigo_barras',
        'nome',
        'preco_venda',
        'preco_custo',
        'estoque_atual',
        'ncm',
        'cest',
        'cfop_padrao',
    ];

    protected $casts = [
        'preco_venda' => 'decimal:2',
        'preco_custo' => 'decimal:2',
    ];

    public function loja()
    {
        return $this->belongsTo(Loja::class);
    }

    public function categoria()
    {
        return $this->belongsTo(Categoria::class);
    }

    public function vendaItems()
    {
        return $this->hasMany(VendaItem::class);
    }
}
"#;

const MODEL_TURNO_CAIXA: &str = r#"<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Concerns\HasUuids;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;

class TurnoCaixa extends Model
{
    use HasFactory, HasUuids;

    protected $table = 'turnos_caixa';

    protected $fillable = [
        'loja_id',
        'usuario_id',
        'data_abertura',
        'data_fechamento',
        'fundo_troco_inicial',
        'saldo_final_esperado',
        'saldo_final_informado',
        'status',
    ];

    protected $casts = [
        'data_abertura'  => 'datetime',
        'data_fechamento' => 'datetime',
    ];

    public function loja()
    {
        return $this->belongsTo(Loja::class);
    }

    public function usuario()
    {
        return $this->belongsTo(Usuario::class);
    }

    public function vendas()
    {
        return $this->hasMany(Venda::class);
    }

    public function movimentacoes()
    {
        return $this->hasMany(MovimentacaoCaixa::class);
    }
}
"#;

const MODEL_MOVIMENTACAO_CAIXA: &str = r#"<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Concerns\HasUuids;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;

class MovimentacaoCaixa extends Model
{
    use HasFactory, HasUuids;

    protected $table = 'movimentacoes_caixa';

    protected $fillable = [
        'turno_caixa_id',
        'tipo',
        'valor',
        'motivo',
        'data_hora',
    ];

    protected $casts = [
        'data_hora' => 'datetime',
        'valor'     => 'decimal:2',
    ];

    public function turnoCaixa()
    {
        return $this->belongsTo(TurnoCaixa::class);
    }
}
"#;

const MODEL_VENDA: &str = r#"<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Concerns\HasUuids;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;

class Venda extends Model
{
    use HasFactory, HasUuids;

    protected $table = 'vendas';

    protected $fillable = [
        'turno_caixa_id',
        'cliente_id',
        'data_hora',
        'subtotal',
        'desconto',
        'total',
        'status',
        'chave_nfe',
        'status_fiscal',
    ];

    protected $casts = [
        'data_hora' => 'datetime',
        'subtotal'  => 'decimal:2',
        'desconto'  => 'decimal:2',
        'total'     => 'decimal:2',
    ];

    public function turnoCaixa()
    {
        return $this->belongsTo(TurnoCaixa::class);
    }

    public function cliente()
    {
        return $this->belongsTo(Cliente::class);
    }

    public function items()
    {
        return $this->hasMany(VendaItem::class);
    }

    public function pagamentos()
    {
        return $this->hasMany(Pagamento::class);
    }
}
"#;

const MODEL_VENDA_ITEM: &str = r#"<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Concerns\HasUuids;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;

class VendaItem extends Model
{
    use HasFactory, HasUuids;

    protected $table = 'venda_items';

    protected $fillable = [
        'venda_id',
        'produto_id',
        'quantidade',
        'preco_unitario',
        'subtotal',
    ];

    protected $casts = [
        'quantidade'     => 'decimal:3',
        'preco_unitario' => 'decimal:2',
        'subtotal'       => 'decimal:2',
    ];

    public function venda()
    {
        return $this->belongsTo(Venda::class);
    }

    public function produto()
    {
        return $this->belongsTo(Produto::class);
    }
}
"#;

const MODEL_PAGAMENTO: &str = r#"<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Concerns\HasUuids;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;

class Pagamento extends Model
{
    use HasFactory, HasUuids;

    protected $table = 'pagamentos';

    protected $fillable = [
        'venda_id',
        'metodo',
        'valor',
        'status',
        'nsu_transacao',
        'bandeira_cartao',
    ];

    protected $casts = [
        'valor' => 'decimal:2',
    ];

    public function venda()
    {
        return $this->belongsTo(Venda::class);
    }
}
"#;
