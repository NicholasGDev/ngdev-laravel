// ─── Migrations ──────────────────────────────────────────────────────────────
// Reutiliza as migrações do flow DDD (schema idêntico).

pub fn migrations() -> Vec<(&'static str, &'static str)> {
    use crate::flows::logistica_reversa::templates as t;
    vec![
        ("2026_01_01_000001_create_seguradoras_table.php",          t::migration_seguradoras()),
        ("2026_01_01_000002_create_transportadoras_table.php",      t::migration_transportadoras()),
        ("2026_01_01_000003_create_segurados_table.php",            t::migration_segurados()),
        ("2026_01_01_000004_create_apolices_table.php",             t::migration_apolices()),
        ("2026_01_01_000005_create_sinistros_table.php",            t::migration_sinistros()),
        ("2026_01_01_000006_create_itens_sinistrados_table.php",    t::migration_itens_sinistrados()),
        ("2026_01_01_000007_create_ordens_coleta_table.php",        t::migration_ordens_coleta()),
        ("2026_01_01_000008_create_movimentacoes_logisticas_table.php", t::migration_movimentacoes_logisticas()),
        ("2026_01_01_000009_create_recebimentos_cd_table.php",      t::migration_recebimentos_cd()),
        ("2026_01_01_000010_create_laudos_triagem_table.php",       t::migration_laudos_triagem()),
    ]
}

// ─── Models ──────────────────────────────────────────────────────────────────

pub fn models() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Seguradora.php",           MODEL_SEGURADORA),
        ("Transportadora.php",       MODEL_TRANSPORTADORA),
        ("Segurado.php",             MODEL_SEGURADO),
        ("Apolice.php",              MODEL_APOLICE),
        ("Sinistro.php",             MODEL_SINISTRO),
        ("ItemSinistrado.php",       MODEL_ITEM_SINISTRADO),
        ("OrdemColeta.php",          MODEL_ORDEM_COLETA),
        ("MovimentacaoLogistica.php",MODEL_MOVIMENTACAO_LOGISTICA),
        ("RecebimentoCd.php",        MODEL_RECEBIMENTO_CD),
        ("LaudoTriagem.php",         MODEL_LAUDO_TRIAGEM),
    ]
}

const MODEL_SEGURADORA: &str = r#"<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Database\Eloquent\Relations\HasMany;

class Seguradora extends Model
{
    use SoftDeletes;

    protected $table = 'seguradoras';

    protected $fillable = ['cnpj', 'razao_social', 'api_key', 'sla_coleta_horas', 'ativo'];

    protected $casts = [
        'sla_coleta_horas' => 'integer',
        'ativo'            => 'boolean',
    ];

    public function apolices(): HasMany
    {
        return $this->hasMany(Apolice::class, 'seguradora_id');
    }
}
"#;

const MODEL_TRANSPORTADORA: &str = r#"<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Database\Eloquent\Relations\HasMany;

class Transportadora extends Model
{
    use SoftDeletes;

    protected $table = 'transportadoras';

    protected $fillable = ['cnpj', 'nome', 'tipo_veiculo', 'regiao_cobertura', 'api_endpoint', 'ativo'];

    protected $casts = ['ativo' => 'boolean'];

    public function ordensColeta(): HasMany
    {
        return $this->hasMany(OrdemColeta::class, 'transportadora_id');
    }
}
"#;

const MODEL_SEGURADO: &str = r#"<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Database\Eloquent\Relations\HasMany;

class Segurado extends Model
{
    use SoftDeletes;

    protected $table = 'segurados';

    protected $fillable = ['cpf_cnpj', 'nome', 'telefone', 'email', 'endereco_coleta', 'cep_coleta'];

    public function apolices(): HasMany
    {
        return $this->hasMany(Apolice::class, 'segurado_id');
    }
}
"#;

const MODEL_APOLICE: &str = r#"<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\HasMany;

class Apolice extends Model
{
    use SoftDeletes;

    protected $table = 'apolices';

    protected $fillable = [
        'seguradora_id', 'segurado_id', 'numero_apolice',
        'tipo_seguro', 'vigencia_inicio', 'vigencia_fim', 'ativa',
    ];

    protected $casts = [
        'vigencia_inicio' => 'date',
        'vigencia_fim'    => 'date',
        'ativa'           => 'boolean',
    ];

    public function seguradora(): BelongsTo { return $this->belongsTo(Seguradora::class, 'seguradora_id'); }
    public function segurado(): BelongsTo   { return $this->belongsTo(Segurado::class, 'segurado_id'); }

    public function sinistros(): HasMany
    {
        return $this->hasMany(Sinistro::class, 'apolice_id');
    }
}
"#;

const MODEL_SINISTRO: &str = r#"<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\HasMany;

class Sinistro extends Model
{
    use SoftDeletes;

    protected $table = 'sinistros';

    protected $fillable = [
        'apolice_id', 'numero_sinistro_seguradora',
        'data_abertura', 'status', 'observacoes',
    ];

    protected $casts = ['data_abertura' => 'datetime'];

    public function apolice(): BelongsTo
    {
        return $this->belongsTo(Apolice::class, 'apolice_id');
    }

    public function itens(): HasMany
    {
        return $this->hasMany(ItemSinistrado::class, 'sinistro_id');
    }

    public function ordensColeta(): HasMany
    {
        return $this->hasMany(OrdemColeta::class, 'sinistro_id');
    }
}
"#;

const MODEL_ITEM_SINISTRADO: &str = r#"<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\HasOne;

class ItemSinistrado extends Model
{
    protected $table = 'itens_sinistrados';

    protected $fillable = [
        'sinistro_id', 'categoria', 'marca',
        'modelo', 'identificador_unico', 'defeito_declarado',
    ];

    public function sinistro(): BelongsTo { return $this->belongsTo(Sinistro::class, 'sinistro_id'); }

    public function laudo(): HasOne
    {
        return $this->hasOne(LaudoTriagem::class, 'item_sinistrado_id');
    }
}
"#;

const MODEL_ORDEM_COLETA: &str = r#"<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\Relations\HasOne;

class OrdemColeta extends Model
{
    use SoftDeletes;

    protected $table = 'ordens_coleta';

    protected $fillable = [
        'sinistro_id', 'transportadora_id', 'data_agendamento',
        'data_efetivacao', 'codigo_rastreio', 'status_coleta',
        'tentativas_coleta', 'motivo_frustracao',
    ];

    protected $casts = [
        'data_agendamento'  => 'datetime',
        'data_efetivacao'   => 'datetime',
        'tentativas_coleta' => 'integer',
    ];

    public function sinistro(): BelongsTo       { return $this->belongsTo(Sinistro::class, 'sinistro_id'); }
    public function transportadora(): BelongsTo { return $this->belongsTo(Transportadora::class, 'transportadora_id'); }

    public function movimentacoes(): HasMany
    {
        return $this->hasMany(MovimentacaoLogistica::class, 'ordem_coleta_id')->orderBy('data_hora');
    }

    public function recebimento(): HasOne
    {
        return $this->hasOne(RecebimentoCd::class, 'ordem_coleta_id');
    }
}
"#;

const MODEL_MOVIMENTACAO_LOGISTICA: &str = r#"<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

/**
 * Tracking imutável — apenas inserção, sem update/delete.
 */
class MovimentacaoLogistica extends Model
{
    protected $table = 'movimentacoes_logisticas';

    protected $fillable = [
        'ordem_coleta_id', 'data_hora', 'localizacao_atual',
        'evento', 'descricao', 'metadata',
    ];

    protected $casts = [
        'data_hora' => 'datetime',
        'metadata'  => 'array',
    ];

    public function ordemColeta(): BelongsTo
    {
        return $this->belongsTo(OrdemColeta::class, 'ordem_coleta_id');
    }
}
"#;

const MODEL_RECEBIMENTO_CD: &str = r#"<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

class RecebimentoCd extends Model
{
    protected $table = 'recebimentos_cd';

    protected $fillable = [
        'ordem_coleta_id', 'data_recebimento', 'usuario_recebedor_id',
        'doca', 'condicao_embalagem', 'observacoes_recebimento', 'foto_recebimento_hash',
    ];

    protected $casts = ['data_recebimento' => 'datetime'];

    public function ordemColeta(): BelongsTo
    {
        return $this->belongsTo(OrdemColeta::class, 'ordem_coleta_id');
    }
}
"#;

const MODEL_LAUDO_TRIAGEM: &str = r#"<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

class LaudoTriagem extends Model
{
    protected $table = 'laudos_triagem';

    protected $fillable = [
        'item_sinistrado_id', 'data_inspecao', 'tecnico_responsavel',
        'parecer_tecnico', 'classificacao_dano', 'destinacao_final', 'certificado_descarte',
    ];

    protected $casts = ['data_inspecao' => 'datetime'];

    public function item(): BelongsTo
    {
        return $this->belongsTo(ItemSinistrado::class, 'item_sinistrado_id');
    }
}
"#;

// ─── Services ────────────────────────────────────────────────────────────────

pub fn services() -> Vec<(&'static str, &'static str)> {
    vec![
        ("SeguradoraService.php",    SERVICE_SEGURADORA),
        ("TransportadoraService.php",SERVICE_TRANSPORTADORA),
        ("SeguradoService.php",      SERVICE_SEGURADO),
        ("ApoliceService.php",       SERVICE_APOLICE),
        ("SinistroService.php",      SERVICE_SINISTRO),
        ("OrdemColetaService.php",   SERVICE_ORDEM_COLETA),
        ("LaudoTriagemService.php",  SERVICE_LAUDO_TRIAGEM),
    ]
}

const SERVICE_SEGURADORA: &str = r#"<?php

declare(strict_types=1);

namespace App\Services;

use App\Models\Seguradora;
use Illuminate\Contracts\Pagination\LengthAwarePaginator;

class SeguradoraService
{
    public function __construct(protected Seguradora $model) {}

    public function index(): LengthAwarePaginator { return $this->model->paginate(); }
    public function show(int $id): Seguradora     { return $this->model->with('apolices')->findOrFail($id); }

    public function store(array $data): Seguradora
    {
        return $this->model->create($data);
    }

    public function update(int $id, array $data): Seguradora
    {
        $item = $this->model->findOrFail($id);
        $item->update($data);
        return $item;
    }

    public function destroy(int $id): void
    {
        $this->model->findOrFail($id)->delete();
    }
}
"#;

const SERVICE_TRANSPORTADORA: &str = r#"<?php

declare(strict_types=1);

namespace App\Services;

use App\Models\Transportadora;
use Illuminate\Contracts\Pagination\LengthAwarePaginator;

class TransportadoraService
{
    public function __construct(protected Transportadora $model) {}

    public function index(): LengthAwarePaginator  { return $this->model->paginate(); }
    public function show(int $id): Transportadora  { return $this->model->findOrFail($id); }

    public function store(array $data): Transportadora
    {
        return $this->model->create($data);
    }

    public function update(int $id, array $data): Transportadora
    {
        $item = $this->model->findOrFail($id);
        $item->update($data);
        return $item;
    }

    public function destroy(int $id): void
    {
        $this->model->findOrFail($id)->delete();
    }
}
"#;

const SERVICE_SEGURADO: &str = r#"<?php

declare(strict_types=1);

namespace App\Services;

use App\Models\Segurado;
use Illuminate\Contracts\Pagination\LengthAwarePaginator;

class SeguradoService
{
    public function __construct(protected Segurado $model) {}

    public function index(): LengthAwarePaginator { return $this->model->paginate(); }
    public function show(int $id): Segurado       { return $this->model->with('apolices')->findOrFail($id); }

    public function store(array $data): Segurado
    {
        return $this->model->create($data);
    }

    public function update(int $id, array $data): Segurado
    {
        $item = $this->model->findOrFail($id);
        $item->update($data);
        return $item;
    }

    public function destroy(int $id): void
    {
        $this->model->findOrFail($id)->delete();
    }
}
"#;

const SERVICE_APOLICE: &str = r#"<?php

declare(strict_types=1);

namespace App\Services;

use App\Models\Apolice;
use Illuminate\Contracts\Pagination\LengthAwarePaginator;

class ApoliceService
{
    public function __construct(protected Apolice $model) {}

    public function index(): LengthAwarePaginator { return $this->model->with(['seguradora', 'segurado'])->paginate(); }
    public function show(int $id): Apolice        { return $this->model->with(['seguradora', 'segurado', 'sinistros'])->findOrFail($id); }

    public function store(array $data): Apolice
    {
        return $this->model->create($data);
    }

    public function update(int $id, array $data): Apolice
    {
        $item = $this->model->findOrFail($id);
        $item->update($data);
        return $item;
    }

    public function destroy(int $id): void
    {
        $this->model->findOrFail($id)->delete();
    }
}
"#;

const SERVICE_SINISTRO: &str = r#"<?php

declare(strict_types=1);

namespace App\Services;

use App\Models\Sinistro;
use Illuminate\Contracts\Pagination\LengthAwarePaginator;

class SinistroService
{
    public function __construct(protected Sinistro $model) {}

    public function index(): LengthAwarePaginator { return $this->model->with('apolice')->paginate(); }
    public function show(int $id): Sinistro       { return $this->model->with(['apolice.segurado', 'itens', 'ordensColeta'])->findOrFail($id); }

    public function store(array $data): Sinistro
    {
        $itens = $data['itens'] ?? [];
        unset($data['itens']);

        $sinistro = $this->model->create($data);

        if (!empty($itens)) {
            $sinistro->itens()->createMany($itens);
        }

        return $sinistro->load(['apolice', 'itens']);
    }

    public function update(int $id, array $data): Sinistro
    {
        $item = $this->model->findOrFail($id);
        $item->update($data);
        return $item;
    }

    public function destroy(int $id): void
    {
        $this->model->findOrFail($id)->delete();
    }
}
"#;

const SERVICE_ORDEM_COLETA: &str = r#"<?php

declare(strict_types=1);

namespace App\Services;

use App\Models\OrdemColeta;
use Illuminate\Contracts\Pagination\LengthAwarePaginator;

class OrdemColetaService
{
    public function __construct(protected OrdemColeta $model) {}

    public function index(): LengthAwarePaginator { return $this->model->with(['sinistro', 'transportadora'])->paginate(); }
    public function show(int $id): OrdemColeta    { return $this->model->with(['sinistro.itens', 'transportadora', 'movimentacoes', 'recebimento'])->findOrFail($id); }

    public function store(array $data): OrdemColeta
    {
        return $this->model->create($data);
    }

    public function registrarMovimentacao(int $ordemId, array $data): OrdemColeta
    {
        $ordem = $this->model->findOrFail($ordemId);
        $ordem->movimentacoes()->create(array_merge($data, ['data_hora' => now()]));
        return $ordem->load('movimentacoes');
    }

    public function update(int $id, array $data): OrdemColeta
    {
        $item = $this->model->findOrFail($id);
        $item->update($data);
        return $item;
    }

    public function destroy(int $id): void
    {
        $this->model->findOrFail($id)->delete();
    }
}
"#;

const SERVICE_LAUDO_TRIAGEM: &str = r#"<?php

declare(strict_types=1);

namespace App\Services;

use App\Models\LaudoTriagem;
use Illuminate\Contracts\Pagination\LengthAwarePaginator;

class LaudoTriagemService
{
    public function __construct(protected LaudoTriagem $model) {}

    public function index(): LengthAwarePaginator { return $this->model->with('item.sinistro')->paginate(); }
    public function show(int $id): LaudoTriagem   { return $this->model->with('item.sinistro.apolice')->findOrFail($id); }

    public function store(array $data): LaudoTriagem
    {
        return $this->model->create($data);
    }

    public function update(int $id, array $data): LaudoTriagem
    {
        $item = $this->model->findOrFail($id);
        $item->update($data);
        return $item;
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
        ("Logistica/SeguradoraController.php",    CONTROLLER_SEGURADORA),
        ("Logistica/TransportadoraController.php",CONTROLLER_TRANSPORTADORA),
        ("Logistica/SeguradoController.php",      CONTROLLER_SEGURADO),
        ("Logistica/ApoliceController.php",       CONTROLLER_APOLICE),
        ("Logistica/SinistroController.php",      CONTROLLER_SINISTRO),
        ("Logistica/OrdemColetaController.php",   CONTROLLER_ORDEM_COLETA),
        ("Logistica/LaudoTriagemController.php",  CONTROLLER_LAUDO_TRIAGEM),
    ]
}

const CONTROLLER_SEGURADORA: &str = r#"<?php

declare(strict_types=1);

namespace App\Http\Controllers\Logistica;

use App\Http\Controllers\Controller;
use App\Services\SeguradoraService;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;

class SeguradoraController extends Controller
{
    public function __construct(protected SeguradoraService $service) {}

    public function index(): JsonResponse { return response()->json($this->service->index()); }
    public function show(int $id): JsonResponse { return response()->json($this->service->show($id)); }

    public function store(Request $request): JsonResponse
    {
        $item = $this->service->store($request->validate([
            'cnpj'              => 'required|string|size:18|unique:seguradoras,cnpj',
            'razao_social'      => 'required|string|max:255',
            'api_key'           => 'nullable|string',
            'sla_coleta_horas'  => 'integer|min:1',
            'ativo'             => 'boolean',
        ]));
        return response()->json($item, 201);
    }

    public function update(Request $request, int $id): JsonResponse
    {
        $item = $this->service->update($id, $request->validate([
            'razao_social'     => 'sometimes|string|max:255',
            'api_key'          => 'nullable|string',
            'sla_coleta_horas' => 'sometimes|integer|min:1',
            'ativo'            => 'boolean',
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

const CONTROLLER_TRANSPORTADORA: &str = r#"<?php

declare(strict_types=1);

namespace App\Http\Controllers\Logistica;

use App\Http\Controllers\Controller;
use App\Services\TransportadoraService;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;

class TransportadoraController extends Controller
{
    public function __construct(protected TransportadoraService $service) {}

    public function index(): JsonResponse { return response()->json($this->service->index()); }
    public function show(int $id): JsonResponse { return response()->json($this->service->show($id)); }

    public function store(Request $request): JsonResponse
    {
        $item = $this->service->store($request->validate([
            'cnpj'              => 'required|string|size:18|unique:transportadoras,cnpj',
            'nome'              => 'required|string|max:255',
            'tipo_veiculo'      => 'required|string',
            'regiao_cobertura'  => 'nullable|string',
            'api_endpoint'      => 'nullable|url',
            'ativo'             => 'boolean',
        ]));
        return response()->json($item, 201);
    }

    public function update(Request $request, int $id): JsonResponse
    {
        $item = $this->service->update($id, $request->validate([
            'nome'             => 'sometimes|string|max:255',
            'tipo_veiculo'     => 'sometimes|string',
            'regiao_cobertura' => 'nullable|string',
            'api_endpoint'     => 'nullable|url',
            'ativo'            => 'boolean',
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

const CONTROLLER_SEGURADO: &str = r#"<?php

declare(strict_types=1);

namespace App\Http\Controllers\Logistica;

use App\Http\Controllers\Controller;
use App\Services\SeguradoService;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;

class SeguradoController extends Controller
{
    public function __construct(protected SeguradoService $service) {}

    public function index(): JsonResponse { return response()->json($this->service->index()); }
    public function show(int $id): JsonResponse { return response()->json($this->service->show($id)); }

    public function store(Request $request): JsonResponse
    {
        $item = $this->service->store($request->validate([
            'cpf_cnpj'        => 'required|string|max:18|unique:segurados,cpf_cnpj',
            'nome'            => 'required|string|max:255',
            'telefone'        => 'nullable|string|max:20',
            'email'           => 'nullable|email',
            'endereco_coleta' => 'nullable|string',
            'cep_coleta'      => 'nullable|string|max:10',
        ]));
        return response()->json($item, 201);
    }

    public function update(Request $request, int $id): JsonResponse
    {
        $item = $this->service->update($id, $request->validate([
            'nome'            => 'sometimes|string|max:255',
            'telefone'        => 'nullable|string|max:20',
            'email'           => 'nullable|email',
            'endereco_coleta' => 'nullable|string',
            'cep_coleta'      => 'nullable|string|max:10',
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

const CONTROLLER_APOLICE: &str = r#"<?php

declare(strict_types=1);

namespace App\Http\Controllers\Logistica;

use App\Http\Controllers\Controller;
use App\Services\ApoliceService;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;

class ApoliceController extends Controller
{
    public function __construct(protected ApoliceService $service) {}

    public function index(): JsonResponse { return response()->json($this->service->index()); }
    public function show(int $id): JsonResponse { return response()->json($this->service->show($id)); }

    public function store(Request $request): JsonResponse
    {
        $item = $this->service->store($request->validate([
            'seguradora_id'  => 'required|exists:seguradoras,id',
            'segurado_id'    => 'required|exists:segurados,id',
            'numero_apolice' => 'required|string|unique:apolices,numero_apolice',
            'tipo_seguro'    => 'required|string|in:auto,eletronicos,residencial',
            'vigencia_inicio'=> 'nullable|date',
            'vigencia_fim'   => 'nullable|date|after:vigencia_inicio',
            'ativa'          => 'boolean',
        ]));
        return response()->json($item, 201);
    }

    public function update(Request $request, int $id): JsonResponse
    {
        $item = $this->service->update($id, $request->validate([
            'vigencia_fim' => 'nullable|date',
            'ativa'        => 'boolean',
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

const CONTROLLER_SINISTRO: &str = r#"<?php

declare(strict_types=1);

namespace App\Http\Controllers\Logistica;

use App\Http\Controllers\Controller;
use App\Services\SinistroService;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;

class SinistroController extends Controller
{
    public function __construct(protected SinistroService $service) {}

    public function index(): JsonResponse { return response()->json($this->service->index()); }
    public function show(int $id): JsonResponse { return response()->json($this->service->show($id)); }

    public function store(Request $request): JsonResponse
    {
        $item = $this->service->store($request->validate([
            'apolice_id'                    => 'required|exists:apolices,id',
            'numero_sinistro_seguradora'    => 'required|string|unique:sinistros,numero_sinistro_seguradora',
            'data_abertura'                 => 'required|date',
            'observacoes'                   => 'nullable|string',
            'itens'                         => 'array',
            'itens.*.categoria'             => 'required|string',
            'itens.*.marca'                 => 'nullable|string',
            'itens.*.modelo'                => 'nullable|string',
            'itens.*.identificador_unico'   => 'nullable|string',
            'itens.*.defeito_declarado'     => 'nullable|string',
        ]));
        return response()->json($item, 201);
    }

    public function update(Request $request, int $id): JsonResponse
    {
        $item = $this->service->update($id, $request->validate([
            'status'      => 'sometimes|string|in:aguardando_coleta,em_transito,em_triagem,finalizado,frustrado',
            'observacoes' => 'nullable|string',
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

const CONTROLLER_ORDEM_COLETA: &str = r#"<?php

declare(strict_types=1);

namespace App\Http\Controllers\Logistica;

use App\Http\Controllers\Controller;
use App\Services\OrdemColetaService;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;

class OrdemColetaController extends Controller
{
    public function __construct(protected OrdemColetaService $service) {}

    public function index(): JsonResponse { return response()->json($this->service->index()); }
    public function show(int $id): JsonResponse { return response()->json($this->service->show($id)); }

    public function store(Request $request): JsonResponse
    {
        $item = $this->service->store($request->validate([
            'sinistro_id'        => 'required|exists:sinistros,id',
            'transportadora_id'  => 'required|exists:transportadoras,id',
            'data_agendamento'   => 'nullable|date',
            'codigo_rastreio'    => 'nullable|string|unique:ordens_coleta,codigo_rastreio',
        ]));
        return response()->json($item, 201);
    }

    public function update(Request $request, int $id): JsonResponse
    {
        $item = $this->service->update($id, $request->validate([
            'status_coleta'      => 'sometimes|string|in:pendente,a_caminho,coletado,frustrado',
            'data_agendamento'   => 'nullable|date',
            'data_efetivacao'    => 'nullable|date',
            'codigo_rastreio'    => 'nullable|string',
            'motivo_frustracao'  => 'nullable|string',
        ]));
        return response()->json($item);
    }

    public function registrarMovimentacao(Request $request, int $id): JsonResponse
    {
        $ordem = $this->service->registrarMovimentacao($id, $request->validate([
            'localizacao_atual' => 'nullable|string',
            'evento'            => 'required|string|in:saiu_para_coleta,recebido_cd,em_transferencia,entregue,frustrado',
            'descricao'         => 'nullable|string',
            'metadata'          => 'nullable|array',
        ]));
        return response()->json($ordem, 201);
    }

    public function destroy(int $id): JsonResponse
    {
        $this->service->destroy($id);
        return response()->json(null, 204);
    }
}
"#;

const CONTROLLER_LAUDO_TRIAGEM: &str = r#"<?php

declare(strict_types=1);

namespace App\Http\Controllers\Logistica;

use App\Http\Controllers\Controller;
use App\Services\LaudoTriagemService;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;

class LaudoTriagemController extends Controller
{
    public function __construct(protected LaudoTriagemService $service) {}

    public function index(): JsonResponse { return response()->json($this->service->index()); }
    public function show(int $id): JsonResponse { return response()->json($this->service->show($id)); }

    public function store(Request $request): JsonResponse
    {
        $item = $this->service->store($request->validate([
            'item_sinistrado_id'  => 'required|exists:itens_sinistrados,id|unique:laudos_triagem,item_sinistrado_id',
            'data_inspecao'       => 'required|date',
            'tecnico_responsavel' => 'required|string|max:255',
            'parecer_tecnico'     => 'required|string',
            'classificacao_dano'  => 'required|string|in:estetico,funcional,perda_total',
            'destinacao_final'    => 'required|string|in:leilao_salvados,descarte_ecologico,reparo_autorizada,devolucao_segurado',
            'certificado_descarte'=> 'nullable|string',
        ]));
        return response()->json($item, 201);
    }

    public function update(Request $request, int $id): JsonResponse
    {
        $item = $this->service->update($id, $request->validate([
            'parecer_tecnico'     => 'sometimes|string',
            'classificacao_dano'  => 'sometimes|string|in:estetico,funcional,perda_total',
            'destinacao_final'    => 'sometimes|string|in:leilao_salvados,descarte_ecologico,reparo_autorizada,devolucao_segurado',
            'certificado_descarte'=> 'nullable|string',
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

use App\Http\Controllers\Logistica\SeguradoraController;
use App\Http\Controllers\Logistica\TransportadoraController;
use App\Http\Controllers\Logistica\SeguradoController;
use App\Http\Controllers\Logistica\ApoliceController;
use App\Http\Controllers\Logistica\SinistroController;
use App\Http\Controllers\Logistica\OrdemColetaController;
use App\Http\Controllers\Logistica\LaudoTriagemController;
use Illuminate\Support\Facades\Route;

Route::prefix('logistica')->group(function () {
    Route::apiResource('seguradoras',    SeguradoraController::class);
    Route::apiResource('transportadoras',TransportadoraController::class);
    Route::apiResource('segurados',      SeguradoController::class);
    Route::apiResource('apolices',       ApoliceController::class);
    Route::apiResource('sinistros',      SinistroController::class);
    Route::apiResource('laudos-triagem', LaudoTriagemController::class);

    // Ordens de coleta + tracking
    Route::apiResource('ordens-coleta', OrdemColetaController::class);
    Route::post('ordens-coleta/{id}/movimentacoes', [OrdemColetaController::class, 'registrarMovimentacao']);
});
"#;
