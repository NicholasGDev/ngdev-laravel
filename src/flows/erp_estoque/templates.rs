// ─── Migrations ──────────────────────────────────────────────────────────────
// Reutiliza as migrações do flow DDD (schema idêntico).

pub fn migrations() -> Vec<(&'static str, &'static str)> {
    use crate::flows::estoque::templates as t;
    vec![
        ("2026_01_01_000001_create_armazens_table.php",             t::migration_armazens()),
        ("2026_01_01_000002_create_posicoes_estoque_table.php",     t::migration_posicoes_estoque()),
        ("2026_01_01_000003_create_fornecedores_table.php",         t::migration_fornecedores()),
        ("2026_01_01_000004_create_produtos_table.php",             t::migration_produtos()),
        ("2026_01_01_000005_create_lotes_table.php",                t::migration_lotes()),
        ("2026_01_01_000006_create_pedidos_compra_table.php",       t::migration_pedidos_compra()),
        ("2026_01_01_000007_create_itens_pedido_compra_table.php",  t::migration_itens_pedido_compra()),
        ("2026_01_01_000008_create_movimentacoes_estoque_table.php",t::migration_movimentacoes_estoque()),
        ("2026_01_01_000009_create_inventarios_table.php",          t::migration_inventarios()),
        ("2026_01_01_000010_create_contagens_inventario_table.php", t::migration_contagens_inventario()),
    ]
}

// ─── Models ──────────────────────────────────────────────────────────────────

pub fn models() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Armazem.php",            MODEL_ARMAZEM),
        ("PosicaoEstoque.php",     MODEL_POSICAO_ESTOQUE),
        ("Fornecedor.php",         MODEL_FORNECEDOR),
        ("Produto.php",            MODEL_PRODUTO),
        ("Lote.php",               MODEL_LOTE),
        ("PedidoCompra.php",       MODEL_PEDIDO_COMPRA),
        ("ItemPedidoCompra.php",   MODEL_ITEM_PEDIDO_COMPRA),
        ("MovimentacaoEstoque.php",MODEL_MOVIMENTACAO_ESTOQUE),
        ("Inventario.php",         MODEL_INVENTARIO),
        ("ContagemInventario.php", MODEL_CONTAGEM_INVENTARIO),
    ]
}

const MODEL_ARMAZEM: &str = r#"<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Database\Eloquent\Relations\HasMany;

class Armazem extends Model
{
    use SoftDeletes;

    protected $table = 'armazens';

    protected $fillable = ['nome', 'tipo', 'endereco', 'ativo'];

    protected $casts = ['ativo' => 'boolean'];

    public function posicoes(): HasMany
    {
        return $this->hasMany(PosicaoEstoque::class, 'armazem_id');
    }

    public function movimentacoesOrigem(): HasMany
    {
        return $this->hasMany(MovimentacaoEstoque::class, 'armazem_origem_id');
    }

    public function movimentacoesDestino(): HasMany
    {
        return $this->hasMany(MovimentacaoEstoque::class, 'armazem_destino_id');
    }

    public function inventarios(): HasMany
    {
        return $this->hasMany(Inventario::class, 'armazem_id');
    }
}
"#;

const MODEL_POSICAO_ESTOQUE: &str = r#"<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

class PosicaoEstoque extends Model
{
    protected $table = 'posicoes_estoque';

    protected $fillable = ['armazem_id', 'corredor', 'prateleira', 'nivel', 'status'];

    public function armazem(): BelongsTo
    {
        return $this->belongsTo(Armazem::class, 'armazem_id');
    }
}
"#;

const MODEL_FORNECEDOR: &str = r#"<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Database\Eloquent\Relations\HasMany;

class Fornecedor extends Model
{
    use SoftDeletes;

    protected $table = 'fornecedores';

    protected $fillable = [
        'cnpj', 'razao_social', 'prazo_entrega_dias',
        'condicao_pagamento_padrao', 'email_contato', 'telefone', 'ativo',
    ];

    protected $casts = [
        'prazo_entrega_dias' => 'integer',
        'ativo'              => 'boolean',
    ];

    public function pedidosCompra(): HasMany
    {
        return $this->hasMany(PedidoCompra::class, 'fornecedor_id');
    }
}
"#;

const MODEL_PRODUTO: &str = r#"<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Database\Eloquent\Relations\HasMany;

class Produto extends Model
{
    use SoftDeletes;

    protected $table = 'produtos';

    protected $fillable = [
        'sku', 'nome', 'unidade_medida', 'estoque_minimo', 'estoque_maximo',
        'metodo_custo', 'custo_medio_atual', 'controla_lote', 'ativo',
    ];

    protected $casts = [
        'estoque_minimo'    => 'decimal:3',
        'estoque_maximo'    => 'decimal:3',
        'custo_medio_atual' => 'decimal:4',
        'controla_lote'     => 'boolean',
        'ativo'             => 'boolean',
    ];

    public function lotes(): HasMany
    {
        return $this->hasMany(Lote::class, 'produto_id');
    }

    public function movimentacoes(): HasMany
    {
        return $this->hasMany(MovimentacaoEstoque::class, 'produto_id');
    }
}
"#;

const MODEL_LOTE: &str = r#"<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

class Lote extends Model
{
    protected $table = 'lotes';

    protected $fillable = [
        'produto_id', 'numero_lote', 'data_fabricacao',
        'data_validade', 'status', 'quantidade_disponivel',
    ];

    protected $casts = [
        'data_fabricacao'       => 'date',
        'data_validade'         => 'date',
        'quantidade_disponivel' => 'decimal:3',
    ];

    public function produto(): BelongsTo
    {
        return $this->belongsTo(Produto::class, 'produto_id');
    }
}
"#;

const MODEL_PEDIDO_COMPRA: &str = r#"<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\HasMany;

class PedidoCompra extends Model
{
    use SoftDeletes;

    protected $table = 'pedidos_compra';

    protected $fillable = [
        'fornecedor_id', 'numero_pedido', 'data_emissao',
        'data_prevista_entrega', 'status', 'valor_total', 'observacoes',
    ];

    protected $casts = [
        'data_emissao'          => 'datetime',
        'data_prevista_entrega' => 'date',
        'valor_total'           => 'decimal:2',
    ];

    public function fornecedor(): BelongsTo
    {
        return $this->belongsTo(Fornecedor::class, 'fornecedor_id');
    }

    public function itens(): HasMany
    {
        return $this->hasMany(ItemPedidoCompra::class, 'pedido_compra_id');
    }
}
"#;

const MODEL_ITEM_PEDIDO_COMPRA: &str = r#"<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

class ItemPedidoCompra extends Model
{
    protected $table = 'itens_pedido_compra';

    protected $fillable = [
        'pedido_compra_id', 'produto_id', 'quantidade_solicitada',
        'quantidade_recebida', 'custo_unitario_previsto', 'custo_unitario_real',
    ];

    protected $casts = [
        'quantidade_solicitada'   => 'decimal:3',
        'quantidade_recebida'     => 'decimal:3',
        'custo_unitario_previsto' => 'decimal:4',
        'custo_unitario_real'     => 'decimal:4',
    ];

    public function pedidoCompra(): BelongsTo
    {
        return $this->belongsTo(PedidoCompra::class, 'pedido_compra_id');
    }

    public function produto(): BelongsTo
    {
        return $this->belongsTo(Produto::class, 'produto_id');
    }
}
"#;

const MODEL_MOVIMENTACAO_ESTOQUE: &str = r#"<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

/**
 * Movimentações são imutáveis (Kardex) — nunca usar update() ou delete().
 */
class MovimentacaoEstoque extends Model
{
    protected $table = 'movimentacoes_estoque';

    protected $fillable = [
        'produto_id', 'lote_id', 'armazem_origem_id', 'armazem_destino_id',
        'tipo_movimento', 'quantidade', 'custo_unitario_movimento',
        'saldo_apos_movimento', 'data_hora', 'usuario_id',
        'documento_referencia', 'observacao',
    ];

    protected $casts = [
        'quantidade'               => 'decimal:3',
        'custo_unitario_movimento' => 'decimal:4',
        'saldo_apos_movimento'     => 'decimal:3',
        'data_hora'                => 'datetime',
    ];

    public function produto(): BelongsTo    { return $this->belongsTo(Produto::class, 'produto_id'); }
    public function lote(): BelongsTo       { return $this->belongsTo(Lote::class, 'lote_id'); }
    public function armazemOrigem(): BelongsTo  { return $this->belongsTo(Armazem::class, 'armazem_origem_id'); }
    public function armazemDestino(): BelongsTo { return $this->belongsTo(Armazem::class, 'armazem_destino_id'); }
}
"#;

const MODEL_INVENTARIO: &str = r#"<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\HasMany;

class Inventario extends Model
{
    protected $table = 'inventarios';

    protected $fillable = [
        'armazem_id', 'data_inicio', 'data_fim',
        'status', 'usuario_responsavel_id', 'observacoes',
    ];

    protected $casts = [
        'data_inicio' => 'datetime',
        'data_fim'    => 'datetime',
    ];

    public function armazem(): BelongsTo
    {
        return $this->belongsTo(Armazem::class, 'armazem_id');
    }

    public function contagens(): HasMany
    {
        return $this->hasMany(ContagemInventario::class, 'inventario_id');
    }
}
"#;

const MODEL_CONTAGEM_INVENTARIO: &str = r#"<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

class ContagemInventario extends Model
{
    protected $table = 'contagens_inventario';

    protected $fillable = [
        'inventario_id', 'produto_id', 'lote_id',
        'quantidade_sistema', 'quantidade_fisica', 'divergencia', 'ajuste_aplicado',
    ];

    protected $casts = [
        'quantidade_sistema' => 'decimal:3',
        'quantidade_fisica'  => 'decimal:3',
        'divergencia'        => 'decimal:3',
        'ajuste_aplicado'    => 'boolean',
    ];

    public function inventario(): BelongsTo { return $this->belongsTo(Inventario::class, 'inventario_id'); }
    public function produto(): BelongsTo    { return $this->belongsTo(Produto::class, 'produto_id'); }
    public function lote(): BelongsTo       { return $this->belongsTo(Lote::class, 'lote_id'); }
}
"#;

// ─── Services ────────────────────────────────────────────────────────────────

pub fn services() -> Vec<(&'static str, &'static str)> {
    vec![
        ("ArmazemService.php",            SERVICE_ARMAZEM),
        ("FornecedorService.php",         SERVICE_FORNECEDOR),
        ("ProdutoService.php",            SERVICE_PRODUTO),
        ("PedidoCompraService.php",       SERVICE_PEDIDO_COMPRA),
        ("MovimentacaoEstoqueService.php",SERVICE_MOVIMENTACAO_ESTOQUE),
        ("InventarioService.php",         SERVICE_INVENTARIO),
    ]
}

const SERVICE_ARMAZEM: &str = r#"<?php

declare(strict_types=1);

namespace App\Services;

use App\Models\Armazem;
use Illuminate\Contracts\Pagination\LengthAwarePaginator;

class ArmazemService
{
    public function __construct(protected Armazem $model) {}

    public function index(): LengthAwarePaginator
    {
        return $this->model->with('posicoes')->paginate();
    }

    public function show(int $id): Armazem
    {
        return $this->model->with('posicoes')->findOrFail($id);
    }

    public function store(array $data): Armazem
    {
        return $this->model->create($data);
    }

    public function update(int $id, array $data): Armazem
    {
        $armazem = $this->model->findOrFail($id);
        $armazem->update($data);
        return $armazem;
    }

    public function destroy(int $id): void
    {
        $this->model->findOrFail($id)->delete();
    }
}
"#;

const SERVICE_FORNECEDOR: &str = r#"<?php

declare(strict_types=1);

namespace App\Services;

use App\Models\Fornecedor;
use Illuminate\Contracts\Pagination\LengthAwarePaginator;

class FornecedorService
{
    public function __construct(protected Fornecedor $model) {}

    public function index(): LengthAwarePaginator
    {
        return $this->model->paginate();
    }

    public function show(int $id): Fornecedor
    {
        return $this->model->with('pedidosCompra')->findOrFail($id);
    }

    public function store(array $data): Fornecedor
    {
        return $this->model->create($data);
    }

    public function update(int $id, array $data): Fornecedor
    {
        $fornecedor = $this->model->findOrFail($id);
        $fornecedor->update($data);
        return $fornecedor;
    }

    public function destroy(int $id): void
    {
        $this->model->findOrFail($id)->delete();
    }
}
"#;

const SERVICE_PRODUTO: &str = r#"<?php

declare(strict_types=1);

namespace App\Services;

use App\Models\Produto;
use Illuminate\Contracts\Pagination\LengthAwarePaginator;

class ProdutoService
{
    public function __construct(protected Produto $model) {}

    public function index(): LengthAwarePaginator
    {
        return $this->model->paginate();
    }

    public function show(int $id): Produto
    {
        return $this->model->with(['lotes', 'movimentacoes'])->findOrFail($id);
    }

    public function store(array $data): Produto
    {
        return $this->model->create($data);
    }

    public function update(int $id, array $data): Produto
    {
        $produto = $this->model->findOrFail($id);
        $produto->update($data);
        return $produto;
    }

    public function destroy(int $id): void
    {
        $this->model->findOrFail($id)->delete();
    }
}
"#;

const SERVICE_PEDIDO_COMPRA: &str = r#"<?php

declare(strict_types=1);

namespace App\Services;

use App\Models\PedidoCompra;
use Illuminate\Contracts\Pagination\LengthAwarePaginator;

class PedidoCompraService
{
    public function __construct(protected PedidoCompra $model) {}

    public function index(): LengthAwarePaginator
    {
        return $this->model->with('fornecedor')->paginate();
    }

    public function show(int $id): PedidoCompra
    {
        return $this->model->with(['fornecedor', 'itens.produto'])->findOrFail($id);
    }

    public function store(array $data): PedidoCompra
    {
        $itens = $data['itens'] ?? [];
        unset($data['itens']);

        $pedido = $this->model->create($data);

        if (!empty($itens)) {
            $pedido->itens()->createMany($itens);
        }

        return $pedido->load(['fornecedor', 'itens.produto']);
    }

    public function update(int $id, array $data): PedidoCompra
    {
        $pedido = $this->model->findOrFail($id);
        $pedido->update($data);
        return $pedido;
    }

    public function destroy(int $id): void
    {
        $this->model->findOrFail($id)->delete();
    }
}
"#;

const SERVICE_MOVIMENTACAO_ESTOQUE: &str = r#"<?php

declare(strict_types=1);

namespace App\Services;

use App\Models\MovimentacaoEstoque;
use App\Models\Produto;
use Illuminate\Contracts\Pagination\LengthAwarePaginator;
use Illuminate\Support\Facades\DB;

/**
 * Kardex — movimentações são imutáveis.
 * Nunca expõe update() ou destroy().
 */
class MovimentacaoEstoqueService
{
    public function __construct(
        protected MovimentacaoEstoque $model,
        protected Produto $produto,
    ) {}

    public function index(): LengthAwarePaginator
    {
        return $this->model
            ->with(['produto', 'armazemOrigem', 'armazemDestino'])
            ->orderByDesc('data_hora')
            ->paginate();
    }

    public function porProduto(int $produtoId): LengthAwarePaginator
    {
        return $this->model
            ->where('produto_id', $produtoId)
            ->orderByDesc('data_hora')
            ->paginate();
    }

    public function show(int $id): MovimentacaoEstoque
    {
        return $this->model
            ->with(['produto', 'lote', 'armazemOrigem', 'armazemDestino'])
            ->findOrFail($id);
    }

    public function registrar(array $data): MovimentacaoEstoque
    {
        return DB::transaction(function () use ($data) {
            $produto = $this->produto->lockForUpdate()->findOrFail($data['produto_id']);

            $saldo = $this->calcularSaldo($produto->id, $data['tipo_movimento'], (float) $data['quantidade']);

            $data['saldo_apos_movimento'] = $saldo;
            $data['data_hora']            ??= now();

            // Atualiza custo médio em entradas
            if (str_starts_with($data['tipo_movimento'], 'entrada')) {
                $custo = (float) ($data['custo_unitario_movimento'] ?? $produto->custo_medio_atual);
                $produto->update(['custo_medio_atual' => $custo]);
            }

            return $this->model->create($data);
        });
    }

    private function calcularSaldo(int $produtoId, string $tipo, float $quantidade): float
    {
        $ultima = $this->model
            ->where('produto_id', $produtoId)
            ->orderByDesc('data_hora')
            ->value('saldo_apos_movimento') ?? 0.0;

        $saidas = ['saida_venda', 'ajuste_perda'];

        return in_array($tipo, $saidas, true)
            ? $ultima - $quantidade
            : $ultima + $quantidade;
    }
}
"#;

const SERVICE_INVENTARIO: &str = r#"<?php

declare(strict_types=1);

namespace App\Services;

use App\Models\Inventario;
use Illuminate\Contracts\Pagination\LengthAwarePaginator;

class InventarioService
{
    public function __construct(protected Inventario $model) {}

    public function index(): LengthAwarePaginator
    {
        return $this->model->with('armazem')->paginate();
    }

    public function show(int $id): Inventario
    {
        return $this->model->with(['armazem', 'contagens.produto'])->findOrFail($id);
    }

    public function store(array $data): Inventario
    {
        $contagens = $data['contagens'] ?? [];
        unset($data['contagens']);

        $inventario = $this->model->create($data);

        if (!empty($contagens)) {
            $inventario->contagens()->createMany($contagens);
        }

        return $inventario->load(['armazem', 'contagens']);
    }

    public function update(int $id, array $data): Inventario
    {
        $inventario = $this->model->findOrFail($id);
        $inventario->update($data);
        return $inventario;
    }

    public function destroy(int $id): void
    {
        $this->model->findOrFail($id)->delete();
    }
}
"#;

// ─── Controllers ─────────────────────────────────────────────────────────────

pub fn controllers() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Estoque/ArmazemController.php",             CONTROLLER_ARMAZEM),
        ("Estoque/FornecedorController.php",          CONTROLLER_FORNECEDOR),
        ("Estoque/ProdutoController.php",             CONTROLLER_PRODUTO),
        ("Estoque/PedidoCompraController.php",        CONTROLLER_PEDIDO_COMPRA),
        ("Estoque/MovimentacaoEstoqueController.php", CONTROLLER_MOVIMENTACAO_ESTOQUE),
        ("Estoque/InventarioController.php",          CONTROLLER_INVENTARIO),
    ]
}

const CONTROLLER_ARMAZEM: &str = r#"<?php

declare(strict_types=1);

namespace App\Http\Controllers\Estoque;

use App\Http\Controllers\Controller;
use App\Services\ArmazemService;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;

class ArmazemController extends Controller
{
    public function __construct(protected ArmazemService $service) {}

    public function index(): JsonResponse
    {
        return response()->json($this->service->index());
    }

    public function store(Request $request): JsonResponse
    {
        $item = $this->service->store($request->validate([
            'nome'     => 'required|string|max:255',
            'tipo'     => 'required|string|in:loja,deposito_central,terceiros',
            'endereco' => 'nullable|string',
            'ativo'    => 'boolean',
        ]));
        return response()->json($item, 201);
    }

    public function show(int $id): JsonResponse
    {
        return response()->json($this->service->show($id));
    }

    public function update(Request $request, int $id): JsonResponse
    {
        $item = $this->service->update($id, $request->validate([
            'nome'     => 'sometimes|string|max:255',
            'tipo'     => 'sometimes|string|in:loja,deposito_central,terceiros',
            'endereco' => 'nullable|string',
            'ativo'    => 'boolean',
        ]));
        return response()->json($item);
    }

    public function destroy(int $id): JsonResponse
    {
        $this->service->destroy($id);
        return response()->json(null, 204);
    }
}
"#;

const CONTROLLER_FORNECEDOR: &str = r#"<?php

declare(strict_types=1);

namespace App\Http\Controllers\Estoque;

use App\Http\Controllers\Controller;
use App\Services\FornecedorService;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;

class FornecedorController extends Controller
{
    public function __construct(protected FornecedorService $service) {}

    public function index(): JsonResponse
    {
        return response()->json($this->service->index());
    }

    public function store(Request $request): JsonResponse
    {
        $item = $this->service->store($request->validate([
            'cnpj'                       => 'required|string|size:18|unique:fornecedores,cnpj',
            'razao_social'               => 'required|string|max:255',
            'prazo_entrega_dias'         => 'integer|min:0',
            'condicao_pagamento_padrao'  => 'nullable|string',
            'email_contato'              => 'nullable|email',
            'telefone'                   => 'nullable|string|max:20',
            'ativo'                      => 'boolean',
        ]));
        return response()->json($item, 201);
    }

    public function show(int $id): JsonResponse
    {
        return response()->json($this->service->show($id));
    }

    public function update(Request $request, int $id): JsonResponse
    {
        $item = $this->service->update($id, $request->validate([
            'razao_social'              => 'sometimes|string|max:255',
            'prazo_entrega_dias'        => 'sometimes|integer|min:0',
            'condicao_pagamento_padrao' => 'nullable|string',
            'email_contato'             => 'nullable|email',
            'telefone'                  => 'nullable|string|max:20',
            'ativo'                     => 'boolean',
        ]));
        return response()->json($item);
    }

    public function destroy(int $id): JsonResponse
    {
        $this->service->destroy($id);
        return response()->json(null, 204);
    }
}
"#;

const CONTROLLER_PRODUTO: &str = r#"<?php

declare(strict_types=1);

namespace App\Http\Controllers\Estoque;

use App\Http\Controllers\Controller;
use App\Services\ProdutoService;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;

class ProdutoController extends Controller
{
    public function __construct(protected ProdutoService $service) {}

    public function index(): JsonResponse
    {
        return response()->json($this->service->index());
    }

    public function store(Request $request): JsonResponse
    {
        $item = $this->service->store($request->validate([
            'sku'            => 'required|string|max:100|unique:produtos,sku',
            'nome'           => 'required|string|max:255',
            'unidade_medida' => 'required|string|in:UN,KG,CX,LT,MT',
            'estoque_minimo' => 'numeric|min:0',
            'estoque_maximo' => 'nullable|numeric|min:0',
            'metodo_custo'   => 'in:PEPS,CUSTO_MEDIO',
            'controla_lote'  => 'boolean',
            'ativo'          => 'boolean',
        ]));
        return response()->json($item, 201);
    }

    public function show(int $id): JsonResponse
    {
        return response()->json($this->service->show($id));
    }

    public function update(Request $request, int $id): JsonResponse
    {
        $item = $this->service->update($id, $request->validate([
            'nome'           => 'sometimes|string|max:255',
            'unidade_medida' => 'sometimes|string|in:UN,KG,CX,LT,MT',
            'estoque_minimo' => 'sometimes|numeric|min:0',
            'estoque_maximo' => 'nullable|numeric|min:0',
            'controla_lote'  => 'boolean',
            'ativo'          => 'boolean',
        ]));
        return response()->json($item);
    }

    public function destroy(int $id): JsonResponse
    {
        $this->service->destroy($id);
        return response()->json(null, 204);
    }
}
"#;

const CONTROLLER_PEDIDO_COMPRA: &str = r#"<?php

declare(strict_types=1);

namespace App\Http\Controllers\Estoque;

use App\Http\Controllers\Controller;
use App\Services\PedidoCompraService;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;

class PedidoCompraController extends Controller
{
    public function __construct(protected PedidoCompraService $service) {}

    public function index(): JsonResponse
    {
        return response()->json($this->service->index());
    }

    public function store(Request $request): JsonResponse
    {
        $item = $this->service->store($request->validate([
            'fornecedor_id'         => 'required|exists:fornecedores,id',
            'numero_pedido'         => 'nullable|string|unique:pedidos_compra,numero_pedido',
            'data_emissao'          => 'required|date',
            'data_prevista_entrega' => 'nullable|date|after:data_emissao',
            'observacoes'           => 'nullable|string',
            'itens'                 => 'array',
            'itens.*.produto_id'               => 'required|exists:produtos,id',
            'itens.*.quantidade_solicitada'    => 'required|numeric|min:0.001',
            'itens.*.custo_unitario_previsto'  => 'required|numeric|min:0',
        ]));
        return response()->json($item, 201);
    }

    public function show(int $id): JsonResponse
    {
        return response()->json($this->service->show($id));
    }

    public function update(Request $request, int $id): JsonResponse
    {
        $item = $this->service->update($id, $request->validate([
            'status'                => 'sometimes|string|in:rascunho,emitido,recebido_parcial,concluido,cancelado',
            'data_prevista_entrega' => 'nullable|date',
            'observacoes'           => 'nullable|string',
        ]));
        return response()->json($item);
    }

    public function destroy(int $id): JsonResponse
    {
        $this->service->destroy($id);
        return response()->json(null, 204);
    }
}
"#;

const CONTROLLER_MOVIMENTACAO_ESTOQUE: &str = r#"<?php

declare(strict_types=1);

namespace App\Http\Controllers\Estoque;

use App\Http\Controllers\Controller;
use App\Services\MovimentacaoEstoqueService;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;

/**
 * Kardex — apenas index, show e store. Sem update ou destroy.
 */
class MovimentacaoEstoqueController extends Controller
{
    public function __construct(protected MovimentacaoEstoqueService $service) {}

    public function index(): JsonResponse
    {
        return response()->json($this->service->index());
    }

    public function show(int $id): JsonResponse
    {
        return response()->json($this->service->show($id));
    }

    public function store(Request $request): JsonResponse
    {
        $item = $this->service->registrar($request->validate([
            'produto_id'               => 'required|exists:produtos,id',
            'lote_id'                  => 'nullable|exists:lotes,id',
            'armazem_origem_id'        => 'nullable|exists:armazens,id',
            'armazem_destino_id'       => 'nullable|exists:armazens,id',
            'tipo_movimento'           => 'required|string|in:entrada_compra,saida_venda,transferencia,ajuste_perda,ajuste_ganho,devolucao',
            'quantidade'               => 'required|numeric|min:0.001',
            'custo_unitario_movimento' => 'numeric|min:0',
            'usuario_id'               => 'required|integer',
            'documento_referencia'     => 'nullable|string',
            'observacao'               => 'nullable|string',
        ]));
        return response()->json($item, 201);
    }
}
"#;

const CONTROLLER_INVENTARIO: &str = r#"<?php

declare(strict_types=1);

namespace App\Http\Controllers\Estoque;

use App\Http\Controllers\Controller;
use App\Services\InventarioService;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;

class InventarioController extends Controller
{
    public function __construct(protected InventarioService $service) {}

    public function index(): JsonResponse
    {
        return response()->json($this->service->index());
    }

    public function store(Request $request): JsonResponse
    {
        $item = $this->service->store($request->validate([
            'armazem_id'             => 'required|exists:armazens,id',
            'data_inicio'            => 'required|date',
            'usuario_responsavel_id' => 'required|integer',
            'observacoes'            => 'nullable|string',
            'contagens'              => 'array',
            'contagens.*.produto_id'        => 'required|exists:produtos,id',
            'contagens.*.lote_id'           => 'nullable|exists:lotes,id',
            'contagens.*.quantidade_sistema'=> 'required|numeric|min:0',
        ]));
        return response()->json($item, 201);
    }

    public function show(int $id): JsonResponse
    {
        return response()->json($this->service->show($id));
    }

    public function update(Request $request, int $id): JsonResponse
    {
        $item = $this->service->update($id, $request->validate([
            'status'     => 'sometimes|string|in:em_andamento,ajustado,cancelado',
            'data_fim'   => 'nullable|date',
            'observacoes'=> 'nullable|string',
        ]));
        return response()->json($item);
    }

    public function destroy(int $id): JsonResponse
    {
        $this->service->destroy($id);
        return response()->json(null, 204);
    }
}
"#;

// ─── Routes ──────────────────────────────────────────────────────────────────

pub const ROUTES: &str = r#"<?php

declare(strict_types=1);

use App\Http\Controllers\Estoque\ArmazemController;
use App\Http\Controllers\Estoque\FornecedorController;
use App\Http\Controllers\Estoque\ProdutoController;
use App\Http\Controllers\Estoque\PedidoCompraController;
use App\Http\Controllers\Estoque\MovimentacaoEstoqueController;
use App\Http\Controllers\Estoque\InventarioController;
use Illuminate\Support\Facades\Route;

Route::prefix('estoque')->group(function () {
    Route::apiResource('armazens',              ArmazemController::class);
    Route::apiResource('fornecedores',          FornecedorController::class);
    Route::apiResource('produtos',              ProdutoController::class);
    Route::apiResource('pedidos-compra',        PedidoCompraController::class);
    Route::apiResource('inventarios',           InventarioController::class);

    // Kardex — sem PUT/DELETE
    Route::get ('movimentacoes',     [MovimentacaoEstoqueController::class, 'index']);
    Route::post('movimentacoes',     [MovimentacaoEstoqueController::class, 'store']);
    Route::get ('movimentacoes/{id}',[MovimentacaoEstoqueController::class, 'show']);
});
"#;
