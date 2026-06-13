// ─── Migrations (11 tabelas) ─────────────────────────────────────────────────

pub fn migration_armazens() -> &'static str {
    r#"<?php

declare(strict_types=1);

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('armazens', function (Blueprint $table) {
            $table->id();
            $table->string('nome');
            $table->string('tipo')->comment('loja | deposito_central | terceiros');
            $table->text('endereco')->nullable();
            $table->boolean('ativo')->default(true);
            $table->timestamps();
            $table->softDeletes();
        });
    }

    public function down(): void
    {
        Schema::dropIfExists('armazens');
    }
};
"#
}

pub fn migration_posicoes_estoque() -> &'static str {
    r#"<?php

declare(strict_types=1);

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('posicoes_estoque', function (Blueprint $table) {
            $table->id();
            $table->foreignId('armazem_id')->constrained('armazens')->cascadeOnDelete();
            $table->string('corredor', 10)->nullable()->comment('Ex: A, B, C');
            $table->string('prateleira', 10)->nullable()->comment('Ex: 01, 02');
            $table->string('nivel', 10)->nullable()->comment('Ex: A, B (alto, baixo)');
            $table->string('status')->default('livre')
                ->comment('livre | ocupado | bloqueado');
            $table->timestamps();

            $table->unique(['armazem_id', 'corredor', 'prateleira', 'nivel'], 'posicao_unique');
            $table->index(['armazem_id', 'status']);
        });
    }

    public function down(): void
    {
        Schema::dropIfExists('posicoes_estoque');
    }
};
"#
}

pub fn migration_fornecedores() -> &'static str {
    r#"<?php

declare(strict_types=1);

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('fornecedores', function (Blueprint $table) {
            $table->id();
            $table->string('cnpj', 18)->unique()->comment('Formato: XX.XXX.XXX/XXXX-XX');
            $table->string('razao_social');
            $table->unsignedSmallInteger('prazo_entrega_dias')->default(7);
            $table->string('condicao_pagamento_padrao')->nullable()
                ->comment('Ex: 30/60/90, A_VISTA');
            $table->string('email_contato')->nullable();
            $table->string('telefone', 20)->nullable();
            $table->boolean('ativo')->default(true);
            $table->timestamps();
            $table->softDeletes();
        });
    }

    public function down(): void
    {
        Schema::dropIfExists('fornecedores');
    }
};
"#
}

pub fn migration_produtos() -> &'static str {
    r#"<?php

declare(strict_types=1);

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('produtos', function (Blueprint $table) {
            $table->id();
            $table->string('sku')->unique()->comment('Stock Keeping Unit');
            $table->string('nome');
            $table->string('unidade_medida', 5)->default('UN')
                ->comment('UN | KG | CX | LT | MT');
            $table->decimal('estoque_minimo', 12, 3)->default(0);
            $table->decimal('estoque_maximo', 12, 3)->nullable();
            $table->string('metodo_custo')->default('CUSTO_MEDIO')
                ->comment('PEPS | CUSTO_MEDIO');
            $table->decimal('custo_medio_atual', 14, 4)->default(0)
                ->comment('Atualizado a cada movimentacao de entrada');
            $table->boolean('controla_lote')->default(false);
            $table->boolean('ativo')->default(true);
            $table->timestamps();
            $table->softDeletes();

            $table->index('sku');
        });
    }

    public function down(): void
    {
        Schema::dropIfExists('produtos');
    }
};
"#
}

pub fn migration_lotes() -> &'static str {
    r#"<?php

declare(strict_types=1);

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('lotes', function (Blueprint $table) {
            $table->id();
            $table->foreignId('produto_id')->constrained('produtos')->restrictOnDelete();
            $table->string('numero_lote');
            $table->date('data_fabricacao')->nullable();
            $table->date('data_validade')->nullable()
                ->comment('Null = produto sem validade');
            $table->string('status')->default('disponivel')
                ->comment('disponivel | quarentena | vencido | esgotado');
            $table->decimal('quantidade_disponivel', 12, 3)->default(0);
            $table->timestamps();

            $table->unique(['produto_id', 'numero_lote']);
            $table->index(['produto_id', 'status']);
            $table->index('data_validade');
        });
    }

    public function down(): void
    {
        Schema::dropIfExists('lotes');
    }
};
"#
}

pub fn migration_pedidos_compra() -> &'static str {
    r#"<?php

declare(strict_types=1);

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('pedidos_compra', function (Blueprint $table) {
            $table->id();
            $table->foreignId('fornecedor_id')->constrained('fornecedores')->restrictOnDelete();
            $table->string('numero_pedido')->unique()->nullable()
                ->comment('Numeracao interna do ERP');
            $table->dateTime('data_emissao');
            $table->date('data_prevista_entrega')->nullable();
            $table->string('status')->default('rascunho')
                ->comment('rascunho | emitido | recebido_parcial | concluido | cancelado');
            $table->decimal('valor_total', 14, 2)->default(0);
            $table->text('observacoes')->nullable();
            $table->timestamps();
            $table->softDeletes();

            $table->index(['fornecedor_id', 'status']);
            $table->index('data_emissao');
        });
    }

    public function down(): void
    {
        Schema::dropIfExists('pedidos_compra');
    }
};
"#
}

pub fn migration_itens_pedido_compra() -> &'static str {
    r#"<?php

declare(strict_types=1);

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('itens_pedido_compra', function (Blueprint $table) {
            $table->id();
            $table->foreignId('pedido_compra_id')->constrained('pedidos_compra')->cascadeOnDelete();
            $table->foreignId('produto_id')->constrained('produtos')->restrictOnDelete();
            $table->decimal('quantidade_solicitada', 12, 3);
            $table->decimal('quantidade_recebida', 12, 3)->default(0);
            $table->decimal('custo_unitario_previsto', 14, 4);
            $table->decimal('custo_unitario_real', 14, 4)->nullable()
                ->comment('Preenchido na NF de entrada');
            $table->timestamps();

            $table->index('pedido_compra_id');
        });
    }

    public function down(): void
    {
        Schema::dropIfExists('itens_pedido_compra');
    }
};
"#
}

pub fn migration_movimentacoes_estoque() -> &'static str {
    r#"<?php

declare(strict_types=1);

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('movimentacoes_estoque', function (Blueprint $table) {
            $table->id();
            $table->foreignId('produto_id')->constrained('produtos')->restrictOnDelete();
            $table->foreignId('lote_id')->nullable()->constrained('lotes')->nullOnDelete();
            $table->foreignId('armazem_origem_id')->nullable()->constrained('armazens')->nullOnDelete();
            $table->foreignId('armazem_destino_id')->nullable()->constrained('armazens')->nullOnDelete();
            $table->string('tipo_movimento')
                ->comment('entrada_compra | saida_venda | transferencia | ajuste_perda | ajuste_ganho | devolucao');
            $table->decimal('quantidade', 12, 3)
                ->comment('Usa decimal(12,3) para suportar KG, Litros, etc.');
            $table->decimal('custo_unitario_movimento', 14, 4)->default(0);
            $table->decimal('saldo_apos_movimento', 12, 3)
                ->comment('Kardex: saldo calculado apos esta movimentacao');
            $table->dateTime('data_hora');
            $table->unsignedBigInteger('usuario_id')->comment('FK para tabela de usuarios do sistema');
            $table->string('documento_referencia')->nullable()
                ->comment('NF, pedido_venda_id, ordem_transferencia_id, etc.');
            $table->text('observacao')->nullable();
            $table->timestamps();

            $table->index(['produto_id', 'data_hora']);
            $table->index(['produto_id', 'armazem_destino_id']);
            $table->index('tipo_movimento');
            $table->index('data_hora');
        });
    }

    public function down(): void
    {
        Schema::dropIfExists('movimentacoes_estoque');
    }
};
"#
}

pub fn migration_inventarios() -> &'static str {
    r#"<?php

declare(strict_types=1);

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('inventarios', function (Blueprint $table) {
            $table->id();
            $table->foreignId('armazem_id')->constrained('armazens')->restrictOnDelete();
            $table->dateTime('data_inicio');
            $table->dateTime('data_fim')->nullable();
            $table->string('status')->default('em_andamento')
                ->comment('em_andamento | ajustado | cancelado');
            $table->unsignedBigInteger('usuario_responsavel_id');
            $table->text('observacoes')->nullable();
            $table->timestamps();

            $table->index(['armazem_id', 'status']);
        });
    }

    public function down(): void
    {
        Schema::dropIfExists('inventarios');
    }
};
"#
}

pub fn migration_contagens_inventario() -> &'static str {
    r#"<?php

declare(strict_types=1);

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('contagens_inventario', function (Blueprint $table) {
            $table->id();
            $table->foreignId('inventario_id')->constrained('inventarios')->cascadeOnDelete();
            $table->foreignId('produto_id')->constrained('produtos')->restrictOnDelete();
            $table->foreignId('lote_id')->nullable()->constrained('lotes')->nullOnDelete();
            $table->decimal('quantidade_sistema', 12, 3)
                ->comment('Saldo do sistema no momento do inventario');
            $table->decimal('quantidade_fisica', 12, 3)->nullable()
                ->comment('Contagem real do operador');
            $table->decimal('divergencia', 12, 3)->nullable()
                ->comment('quantidade_fisica - quantidade_sistema');
            $table->boolean('ajuste_aplicado')->default(false);
            $table->timestamps();

            $table->index(['inventario_id', 'produto_id']);
        });
    }

    public function down(): void
    {
        Schema::dropIfExists('contagens_inventario');
    }
};
"#
}

// ─── Eloquent Models ─────────────────────────────────────────────────────────

pub fn model_armazem(ns: &str) -> String {
    format!(
        r#"<?php

declare(strict_types=1);

namespace {ns}\Armazem\Infra\Persistence\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Database\Eloquent\Relations\HasMany;

class Armazem extends Model
{{
    use SoftDeletes;

    protected $table = 'armazens';

    protected $fillable = [
        'nome',
        'tipo',
        'endereco',
        'ativo',
    ];

    protected $casts = [
        'ativo' => 'boolean',
    ];

    public function posicoes(): HasMany
    {{
        return $this->hasMany(PosicaoEstoque::class, 'armazem_id');
    }}

    public function movimentacoesOrigem(): HasMany
    {{
        return $this->hasMany(
            \{ns}\MovimentacaoEstoque\Infra\Persistence\Models\MovimentacaoEstoque::class,
            'armazem_origem_id'
        );
    }}

    public function movimentacoesDestino(): HasMany
    {{
        return $this->hasMany(
            \{ns}\MovimentacaoEstoque\Infra\Persistence\Models\MovimentacaoEstoque::class,
            'armazem_destino_id'
        );
    }}
}}
"#
    )
}

pub fn model_posicao_estoque(ns: &str) -> String {
    format!(
        r#"<?php

declare(strict_types=1);

namespace {ns}\Armazem\Infra\Persistence\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

class PosicaoEstoque extends Model
{{
    protected $table = 'posicoes_estoque';

    protected $fillable = [
        'armazem_id',
        'corredor',
        'prateleira',
        'nivel',
        'status',
    ];

    public function armazem(): BelongsTo
    {{
        return $this->belongsTo(Armazem::class, 'armazem_id');
    }}
}}
"#
    )
}

pub fn model_fornecedor(ns: &str) -> String {
    format!(
        r#"<?php

declare(strict_types=1);

namespace {ns}\Fornecedor\Infra\Persistence\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Database\Eloquent\Relations\HasMany;

class Fornecedor extends Model
{{
    use SoftDeletes;

    protected $table = 'fornecedores';

    protected $fillable = [
        'cnpj',
        'razao_social',
        'prazo_entrega_dias',
        'condicao_pagamento_padrao',
        'email_contato',
        'telefone',
        'ativo',
    ];

    protected $casts = [
        'prazo_entrega_dias' => 'integer',
        'ativo'              => 'boolean',
    ];

    public function pedidosCompra(): HasMany
    {{
        return $this->hasMany(
            \{ns}\PedidoCompra\Infra\Persistence\Models\PedidoCompra::class,
            'fornecedor_id'
        );
    }}
}}
"#
    )
}

pub fn model_produto(ns: &str) -> String {
    format!(
        r#"<?php

declare(strict_types=1);

namespace {ns}\Produto\Infra\Persistence\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Database\Eloquent\Relations\HasMany;

class Produto extends Model
{{
    use SoftDeletes;

    protected $table = 'produtos';

    protected $fillable = [
        'sku',
        'nome',
        'unidade_medida',
        'estoque_minimo',
        'estoque_maximo',
        'metodo_custo',
        'custo_medio_atual',
        'controla_lote',
        'ativo',
    ];

    protected $casts = [
        'estoque_minimo'   => 'decimal:3',
        'estoque_maximo'   => 'decimal:3',
        'custo_medio_atual'=> 'decimal:4',
        'controla_lote'    => 'boolean',
        'ativo'            => 'boolean',
    ];

    public function lotes(): HasMany
    {{
        return $this->hasMany(Lote::class, 'produto_id');
    }}

    public function movimentacoes(): HasMany
    {{
        return $this->hasMany(
            \{ns}\MovimentacaoEstoque\Infra\Persistence\Models\MovimentacaoEstoque::class,
            'produto_id'
        );
    }}
}}
"#
    )
}

pub fn model_lote(ns: &str) -> String {
    format!(
        r#"<?php

declare(strict_types=1);

namespace {ns}\Produto\Infra\Persistence\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

class Lote extends Model
{{
    protected $table = 'lotes';

    protected $fillable = [
        'produto_id',
        'numero_lote',
        'data_fabricacao',
        'data_validade',
        'status',
        'quantidade_disponivel',
    ];

    protected $casts = [
        'data_fabricacao'      => 'date',
        'data_validade'        => 'date',
        'quantidade_disponivel'=> 'decimal:3',
    ];

    public function produto(): BelongsTo
    {{
        return $this->belongsTo(Produto::class, 'produto_id');
    }}
}}
"#
    )
}

pub fn model_pedido_compra(ns: &str) -> String {
    format!(
        r#"<?php

declare(strict_types=1);

namespace {ns}\PedidoCompra\Infra\Persistence\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\HasMany;

class PedidoCompra extends Model
{{
    use SoftDeletes;

    protected $table = 'pedidos_compra';

    protected $fillable = [
        'fornecedor_id',
        'numero_pedido',
        'data_emissao',
        'data_prevista_entrega',
        'status',
        'valor_total',
        'observacoes',
    ];

    protected $casts = [
        'data_emissao'          => 'datetime',
        'data_prevista_entrega' => 'date',
        'valor_total'           => 'decimal:2',
    ];

    public function fornecedor(): BelongsTo
    {{
        return $this->belongsTo(
            \{ns}\Fornecedor\Infra\Persistence\Models\Fornecedor::class,
            'fornecedor_id'
        );
    }}

    public function itens(): HasMany
    {{
        return $this->hasMany(ItemPedidoCompra::class, 'pedido_compra_id');
    }}
}}
"#
    )
}

pub fn model_item_pedido_compra(ns: &str) -> String {
    format!(
        r#"<?php

declare(strict_types=1);

namespace {ns}\PedidoCompra\Infra\Persistence\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

class ItemPedidoCompra extends Model
{{
    protected $table = 'itens_pedido_compra';

    protected $fillable = [
        'pedido_compra_id',
        'produto_id',
        'quantidade_solicitada',
        'quantidade_recebida',
        'custo_unitario_previsto',
        'custo_unitario_real',
    ];

    protected $casts = [
        'quantidade_solicitada'   => 'decimal:3',
        'quantidade_recebida'     => 'decimal:3',
        'custo_unitario_previsto' => 'decimal:4',
        'custo_unitario_real'     => 'decimal:4',
    ];

    public function pedidoCompra(): BelongsTo
    {{
        return $this->belongsTo(PedidoCompra::class, 'pedido_compra_id');
    }}

    public function produto(): BelongsTo
    {{
        return $this->belongsTo(
            \{ns}\Produto\Infra\Persistence\Models\Produto::class,
            'produto_id'
        );
    }}
}}
"#
    )
}

pub fn model_movimentacao_estoque(ns: &str) -> String {
    format!(
        r#"<?php

declare(strict_types=1);

namespace {ns}\MovimentacaoEstoque\Infra\Persistence\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

class MovimentacaoEstoque extends Model
{{
    protected $table = 'movimentacoes_estoque';

    // Movimentacoes sao imutaveis (Kardex) — sem softDelete nem update
    protected $fillable = [
        'produto_id',
        'lote_id',
        'armazem_origem_id',
        'armazem_destino_id',
        'tipo_movimento',
        'quantidade',
        'custo_unitario_movimento',
        'saldo_apos_movimento',
        'data_hora',
        'usuario_id',
        'documento_referencia',
        'observacao',
    ];

    protected $casts = [
        'quantidade'               => 'decimal:3',
        'custo_unitario_movimento' => 'decimal:4',
        'saldo_apos_movimento'     => 'decimal:3',
        'data_hora'                => 'datetime',
    ];

    public function produto(): BelongsTo
    {{
        return $this->belongsTo(
            \{ns}\Produto\Infra\Persistence\Models\Produto::class,
            'produto_id'
        );
    }}

    public function lote(): BelongsTo
    {{
        return $this->belongsTo(
            \{ns}\Produto\Infra\Persistence\Models\Lote::class,
            'lote_id'
        );
    }}

    public function armazemOrigem(): BelongsTo
    {{
        return $this->belongsTo(
            \{ns}\Armazem\Infra\Persistence\Models\Armazem::class,
            'armazem_origem_id'
        );
    }}

    public function armazemDestino(): BelongsTo
    {{
        return $this->belongsTo(
            \{ns}\Armazem\Infra\Persistence\Models\Armazem::class,
            'armazem_destino_id'
        );
    }}
}}
"#
    )
}

pub fn model_inventario(ns: &str) -> String {
    format!(
        r#"<?php

declare(strict_types=1);

namespace {ns}\Inventario\Infra\Persistence\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\HasMany;

class Inventario extends Model
{{
    protected $table = 'inventarios';

    protected $fillable = [
        'armazem_id',
        'data_inicio',
        'data_fim',
        'status',
        'usuario_responsavel_id',
        'observacoes',
    ];

    protected $casts = [
        'data_inicio' => 'datetime',
        'data_fim'    => 'datetime',
    ];

    public function armazem(): BelongsTo
    {{
        return $this->belongsTo(
            \{ns}\Armazem\Infra\Persistence\Models\Armazem::class,
            'armazem_id'
        );
    }}

    public function contagens(): HasMany
    {{
        return $this->hasMany(ContagemInventario::class, 'inventario_id');
    }}
}}
"#
    )
}

pub fn model_contagem_inventario(ns: &str) -> String {
    format!(
        r#"<?php

declare(strict_types=1);

namespace {ns}\Inventario\Infra\Persistence\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

class ContagemInventario extends Model
{{
    protected $table = 'contagens_inventario';

    protected $fillable = [
        'inventario_id',
        'produto_id',
        'lote_id',
        'quantidade_sistema',
        'quantidade_fisica',
        'divergencia',
        'ajuste_aplicado',
    ];

    protected $casts = [
        'quantidade_sistema' => 'decimal:3',
        'quantidade_fisica'  => 'decimal:3',
        'divergencia'        => 'decimal:3',
        'ajuste_aplicado'    => 'boolean',
    ];

    public function inventario(): BelongsTo
    {{
        return $this->belongsTo(Inventario::class, 'inventario_id');
    }}

    public function produto(): BelongsTo
    {{
        return $this->belongsTo(
            \{ns}\Produto\Infra\Persistence\Models\Produto::class,
            'produto_id'
        );
    }}
}}
"#
    )
}

// ─── UseCases especiais para o Kardex ────────────────────────────────────────

pub fn usecase_registrar_movimentacao(ns: &str) -> String {
    format!(
        r#"<?php

declare(strict_types=1);

namespace {ns}\MovimentacaoEstoque\Application\UseCases;

use {ns}\MovimentacaoEstoque\Application\DTOs\Inputs\RegistrarMovimentacaoInput;
use {ns}\MovimentacaoEstoque\Application\Errors\EstoqueInsuficienteError;
use {ns}\MovimentacaoEstoque\Application\Errors\EstoqueNegativoNaoPermitidoError;
use {ns}\MovimentacaoEstoque\Application\Exceptions\MovimentacaoEstoqueException;
use {ns}\MovimentacaoEstoque\Domain\Entities\MovimentacaoEstoqueEntity;
use {ns}\MovimentacaoEstoque\Infra\Persistence\Repositories\MovimentacaoEstoqueRepository;
use {ns}\Produto\Infra\Persistence\Repositories\ProdutoRepository;
use App\Contexts\Compartilhado\Base\Domain\VOs\IdVO;

class RegistrarMovimentacaoUseCase
{{
    public function __construct(
        private MovimentacaoEstoqueRepository $movimentacaoRepository,
        private ProdutoRepository             $produtoRepository,
    ) {{}}

    public function executar(RegistrarMovimentacaoInput $input): IdVO
    {{
        $produto = $this->produtoRepository->obterPorId($input->produtoId);

        if (! $produto) {{
            throw new MovimentacaoEstoqueException(new EstoqueInsuficienteError());
        }}

        $saldoAtual = $this->movimentacaoRepository->obterSaldoAtual($input->produtoId, $input->armazemDestinoId);

        $isSaida = in_array($input->tipoMovimento, ['saida_venda', 'ajuste_perda', 'transferencia'], true);

        if ($isSaida && $saldoAtual < $input->quantidade) {{
            throw new MovimentacaoEstoqueException(new EstoqueInsuficienteError());
        }}

        $saldoApos = $isSaida
            ? $saldoAtual - $input->quantidade
            : $saldoAtual + $input->quantidade;

        // Validacao: estoque negativo nao permitido (configuravel no Manager JSON)
        if ($saldoApos < 0) {{
            throw new MovimentacaoEstoqueException(new EstoqueNegativoNaoPermitidoError());
        }}

        $entity = MovimentacaoEstoqueEntity::create(
            produtoId:              $input->produtoId,
            loteId:                 $input->loteId,
            armazemOrigemId:        $input->armazemOrigemId,
            armazemDestinoId:       $input->armazemDestinoId,
            tipoMovimento:          $input->tipoMovimento,
            quantidade:             $input->quantidade,
            custoUnitario:          $input->custoUnitario,
            saldoAposMovimentacao:  $saldoApos,
            usuarioId:              $input->usuarioId,
            documentoReferencia:    $input->documentoReferencia,
        );

        return $this->movimentacaoRepository->registrar($entity);
    }}
}}
"#
    )
}

pub fn usecase_fechar_inventario(ns: &str) -> String {
    format!(
        r#"<?php

declare(strict_types=1);

namespace {ns}\Inventario\Application\UseCases;

use {ns}\Inventario\Application\DTOs\Inputs\FecharInventarioInput;
use {ns}\Inventario\Application\Errors\InventarioNaoEncontradoError;
use {ns}\Inventario\Application\Errors\InventarioJaFinalizadoError;
use {ns}\Inventario\Application\Exceptions\InventarioException;
use {ns}\Inventario\Infra\Persistence\Repositories\InventarioRepository;
use {ns}\MovimentacaoEstoque\Application\UseCases\RegistrarMovimentacaoUseCase;
use {ns}\MovimentacaoEstoque\Application\DTOs\Inputs\RegistrarMovimentacaoInput;
use App\Contexts\Compartilhado\Base\Domain\VOs\IdVO;

class FecharInventarioUseCase
{{
    public function __construct(
        private InventarioRepository        $inventarioRepository,
        private RegistrarMovimentacaoUseCase $registrarMovimentacao,
    ) {{}}

    public function executar(FecharInventarioInput $input): void
    {{
        $inventario = $this->inventarioRepository->obterPorId($input->inventarioId);

        if (! $inventario) {{
            throw new InventarioException(new InventarioNaoEncontradoError());
        }}

        if ($inventario->status !== 'em_andamento') {{
            throw new InventarioException(new InventarioJaFinalizadoError());
        }}

        $contagens = $this->inventarioRepository->obterContagensPendentes($input->inventarioId);

        // Para cada divergencia, gera movimentacao de ajuste no Kardex
        foreach ($contagens as $contagem) {{
            if ($contagem->divergencia === null || $contagem->divergencia == 0) {{
                continue;
            }}

            $tipoAjuste = $contagem->divergencia > 0 ? 'ajuste_ganho' : 'ajuste_perda';

            $this->registrarMovimentacao->executar(new RegistrarMovimentacaoInput(
                produtoId:           new IdVO((int) $contagem->produto_id),
                loteId:              $contagem->lote_id ? new IdVO((int) $contagem->lote_id) : null,
                armazemOrigemId:     null,
                armazemDestinoId:    new IdVO((int) $inventario->armazem_id),
                tipoMovimento:       $tipoAjuste,
                quantidade:          abs((float) $contagem->divergencia),
                custoUnitario:       0.0,
                usuarioId:           $input->usuarioId,
                documentoReferencia: 'inventario_' . $inventario->id,
            ));
        }}

        $this->inventarioRepository->fechar($input->inventarioId);
    }}
}}
"#
    )
}

// ─── Errors especiais de Estoque ─────────────────────────────────────────────

pub fn error_estoque_insuficiente(ns: &str) -> String {
    format!(
        r#"<?php

declare(strict_types=1);

namespace {ns}\MovimentacaoEstoque\Application\Errors;

use App\Contexts\Compartilhado\Base\Application\Errors\BaseError;

class EstoqueInsuficienteError extends BaseError
{{
    protected int $code = 422;
    protected string $message = 'Estoque insuficiente para realizar a operacao';
}}
"#
    )
}

pub fn error_estoque_negativo(ns: &str) -> String {
    format!(
        r#"<?php

declare(strict_types=1);

namespace {ns}\MovimentacaoEstoque\Application\Errors;

use App\Contexts\Compartilhado\Base\Application\Errors\BaseError;

class EstoqueNegativoNaoPermitidoError extends BaseError
{{
    protected int $code = 422;
    protected string $message = 'Estoque negativo nao permitido para este produto';
}}
"#
    )
}

pub fn error_inventario_ja_finalizado(ns: &str) -> String {
    format!(
        r#"<?php

declare(strict_types=1);

namespace {ns}\Inventario\Application\Errors;

use App\Contexts\Compartilhado\Base\Application\Errors\BaseError;

class InventarioJaFinalizadoError extends BaseError
{{
    protected int $code = 409;
    protected string $message = 'Inventario ja foi finalizado ou cancelado';
}}
"#
    )
}

// ─── Input DTO especial para movimentacao ────────────────────────────────────

pub fn input_registrar_movimentacao(ns: &str) -> String {
    format!(
        r#"<?php

declare(strict_types=1);

namespace {ns}\MovimentacaoEstoque\Application\DTOs\Inputs;

use App\Contexts\Compartilhado\Base\Domain\VOs\IdVO;

readonly class RegistrarMovimentacaoInput
{{
    public function __construct(
        public IdVO   $produtoId,
        public ?IdVO  $loteId,
        public ?IdVO  $armazemOrigemId,
        public ?IdVO  $armazemDestinoId,
        public string $tipoMovimento,
        public float  $quantidade,
        public float  $custoUnitario,
        public IdVO   $usuarioId,
        public ?string $documentoReferencia = null,
    ) {{}}
}}
"#
    )
}

pub fn input_fechar_inventario(ns: &str) -> String {
    format!(
        r#"<?php

declare(strict_types=1);

namespace {ns}\Inventario\Application\DTOs\Inputs;

use App\Contexts\Compartilhado\Base\Domain\VOs\IdVO;

readonly class FecharInventarioInput
{{
    public function __construct(
        public IdVO $inventarioId,
        public IdVO $usuarioId,
    ) {{}}
}}
"#
    )
}

// ─── Manager JSON ─────────────────────────────────────────────────────────────

pub fn manager_json(tenant_id: &str, valuation_method: &str) -> String {
    format!(
        r#"{{
  "estoque_manager": {{
    "version": "3.0.0",
    "company_config": {{
      "tenant_id": "{tenant_id}",
      "valuation_method": "{valuation_method}",
      "allow_negative_stock": false
    }},
    "automation_rules": {{
      "replenishment": {{
        "auto_generate_purchase_orders": true,
        "trigger": "BELOW_MINIMUM_STOCK"
      }},
      "alerts": {{
        "batch_expiration_warning_days": 30,
        "stock_out_warning": true
      }}
    }},
    "integrations": {{
      "ecommerce": {{
        "sync_stock_levels": true,
        "sync_interval_seconds": 300
      }},
      "accounting": {{
        "export_kardex_monthly": true,
        "endpoint": "https://api.contabilidade.com/v1/kardex"
      }}
    }},
    "database_schema": {{
      "tables": [
        {{
          "name": "movimentacoes_estoque",
          "description": "Tabela Kardex - Fonte da verdade para saldo de estoque",
          "fields": [
            {{ "name": "id", "type": "bigint", "primary_key": true }},
            {{ "name": "produto_id", "type": "bigint", "foreign_key": "produtos.id" }},
            {{ "name": "tipo_movimento", "type": "enum", "values": ["entrada_compra", "saida_venda", "transferencia", "ajuste_perda", "ajuste_ganho", "devolucao"] }},
            {{ "name": "quantidade", "type": "decimal(12,3)", "description": "Suporta KG, Litros, Metros" }},
            {{ "name": "saldo_apos_movimento", "type": "decimal(12,3)", "required": true, "description": "Kardex imutavel" }}
          ]
        }},
        {{
          "name": "lotes",
          "description": "Controle FEFO/PEPS e rastreabilidade",
          "fields": [
            {{ "name": "id", "type": "bigint", "primary_key": true }},
            {{ "name": "produto_id", "type": "bigint", "foreign_key": "produtos.id" }},
            {{ "name": "data_validade", "type": "date", "nullable": true }},
            {{ "name": "status", "type": "enum", "values": ["disponivel", "quarentena", "vencido", "esgotado"] }}
          ]
        }}
      ]
    }}
  }}
}}
"#
    )
}
