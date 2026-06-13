// ─── PHP Templates for DDD Context Files ────────────────────────────────────

pub fn service_provider(ns: &str, name: &str, prefix: &str) -> String {
    format!(
        r#"<?php

declare(strict_types=1);

namespace {ns}\{name}\Infra\Providers;

use Illuminate\Foundation\Application;
use App\Contexts\Compartilhado\Base\Infra\Providers\CompartilhadoServiceProvider;

class {name}ServiceProvider extends CompartilhadoServiceProvider
{{
    public function __construct(Application $app)
    {{
        parent::__construct($app);
        $this->setPrefix('{prefix}');
        $this->setName('{prefix}');
        $this->setRoute(__DIR__ . '/../Presentation/Routes/api.php');
    }}
}}
"#
    )
}

pub fn routes(ns: &str, name: &str, operations: &[String]) -> String {
    let controller_use = format!(
        "use {ns}\\{name}\\Infra\\Presentation\\Http\\Controllers\\{name}Controller;",
    );

    let mut route_lines = Vec::new();
    for op in operations {
        let line = match op.as_str() {
            "consultar" => format!(
                "Route::get('/consultar', [{name}Controller::class, 'consultar'])->name('consultar');"
            ),
            "detalhar" => format!(
                "Route::get('/detalhar/{{id}}', [{name}Controller::class, 'detalhar'])->name('detalhar');"
            ),
            "criar" => format!(
                "Route::post('/criar', [{name}Controller::class, 'criar'])->name('criar');"
            ),
            "alterar" => format!(
                "Route::put('/alterar/{{id}}', [{name}Controller::class, 'alterar'])->name('alterar');"
            ),
            "deletar" => format!(
                "Route::delete('/deletar/{{id}}', [{name}Controller::class, 'deletar'])->name('deletar');"
            ),
            _ => continue,
        };
        route_lines.push(line);
    }

    let routes_body = route_lines.join("\n");

    format!(
        r#"<?php

declare(strict_types=1);

{controller_use}
use Illuminate\Support\Facades\Route;

{routes_body}
"#
    )
}

pub fn controller(ns: &str, name: &str, operations: &[String]) -> String {
    let mut uses = vec![
        format!("use {ns}\\{name}\\Infra\\Presentation\\Http\\Controllers\\Controller;"),
        format!("use Illuminate\\Http\\JsonResponse;"),
        format!("use Illuminate\\Http\\Request;"),
    ];

    let has_consultar = operations.contains(&"consultar".to_string());
    let has_detalhar = operations.contains(&"detalhar".to_string());
    let has_criar = operations.contains(&"criar".to_string());
    let has_alterar = operations.contains(&"alterar".to_string());
    let has_deletar = operations.contains(&"deletar".to_string());

    if has_consultar {
        uses.push(format!("use {ns}\\{name}\\Application\\DTOs\\Inputs\\PaginacaoInput;"));
        uses.push(format!("use {ns}\\{name}\\Application\\Queries\\Consultar{name}sQuery;"));
    }
    if has_detalhar {
        uses.push(format!("use {ns}\\{name}\\Application\\Queries\\Detalhar{name}Query;"));
        uses.push(format!(
            "use App\\Contexts\\Compartilhado\\Base\\Domain\\VOs\\IdVO;"
        ));
    }
    if has_criar {
        uses.push(format!(
            "use {ns}\\{name}\\Application\\DTOs\\Inputs\\Criar{name}Input;"
        ));
        uses.push(format!(
            "use {ns}\\{name}\\Infra\\Presentation\\Http\\Requests\\Criar{name}Request;"
        ));
        uses.push(format!(
            "use {ns}\\{name}\\Application\\UseCases\\Criar{name}UseCase;"
        ));
    }
    if has_alterar {
        uses.push(format!(
            "use {ns}\\{name}\\Application\\DTOs\\Inputs\\Alterar{name}Input;"
        ));
        uses.push(format!(
            "use {ns}\\{name}\\Infra\\Presentation\\Http\\Requests\\Alterar{name}Request;"
        ));
        uses.push(format!(
            "use {ns}\\{name}\\Application\\UseCases\\Alterar{name}UseCase;"
        ));
        if !has_detalhar {
            uses.push(format!(
                "use App\\Contexts\\Compartilhado\\Base\\Domain\\VOs\\IdVO;"
            ));
        }
    }
    if has_deletar {
        uses.push(format!(
            "use {ns}\\{name}\\Application\\UseCases\\Deletar{name}UseCase;"
        ));
        if !has_detalhar && !has_alterar {
            uses.push(format!(
                "use App\\Contexts\\Compartilhado\\Base\\Domain\\VOs\\IdVO;"
            ));
        }
    }

    uses.sort();
    uses.dedup();
    let uses_block = uses.join("\n");

    let mut methods = Vec::new();

    if has_consultar {
        methods.push(format!(
            r#"    public function consultar(Request $request, Consultar{name}sQuery $query): JsonResponse
    {{
        $input  = new PaginacaoInput(page: $request->page(1), perPage: $request->perPage(20));
        $output = $query->executar($input);
        return $this->successResponse(data: $output->toArray());
    }}"#
        ));
    }

    if has_detalhar {
        methods.push(format!(
            r#"    public function detalhar(int $id, Detalhar{name}Query $query): JsonResponse
    {{
        $output = $query->executar(new IdVO($id));
        return $this->successResponse(data: $output->toArray());
    }}"#
        ));
    }

    if has_criar {
        methods.push(format!(
            r#"    public function criar(Criar{name}Request $request, Criar{name}UseCase $useCase): JsonResponse
    {{
        $input = new Criar{name}Input(
            // TODO: mapear campos do $request
        );
        $id = $useCase->executar($input);
        return $this->successResponse(data: ['id' => $id->getValue()], status: 201);
    }}"#
        ));
    }

    if has_alterar {
        methods.push(format!(
            r#"    public function alterar(int $id, Alterar{name}Request $request, Alterar{name}UseCase $useCase): JsonResponse
    {{
        $input = new Alterar{name}Input(
            id: new IdVO($id),
            // TODO: mapear campos do $request
        );
        $useCase->executar($input);
        return $this->successResponse();
    }}"#
        ));
    }

    if has_deletar {
        methods.push(format!(
            r#"    public function deletar(int $id, Deletar{name}UseCase $useCase): JsonResponse
    {{
        $useCase->executar(new IdVO($id));
        return $this->successResponse();
    }}"#
        ));
    }

    let methods_body = methods.join("\n\n");

    format!(
        r#"<?php

declare(strict_types=1);

namespace {ns}\{name}\Infra\Presentation\Http\Controllers;

{uses_block}

class {name}Controller extends Controller
{{
{methods_body}
}}
"#
    )
}

pub fn entity(ns: &str, name: &str) -> String {
    format!(
        r#"<?php

declare(strict_types=1);

namespace {ns}\{name}\Domain\Entities;

use App\Contexts\Compartilhado\Base\Domain\VOs\IdVO;

class {name}Entity
{{
    private function __construct(
        public readonly ?IdVO $id,
        // TODO: adicionar campos da entidade
    ) {{}}

    public static function create(
        // TODO: adicionar parametros
    ): self {{
        // TODO: adicionar validacoes
        return new self(
            id: null,
            // TODO: mapear campos
        );
    }}

    public static function update(
        IdVO $id,
        // TODO: adicionar parametros
    ): self {{
        return new self(
            id: $id,
            // TODO: mapear campos
        );
    }}
}}
"#
    )
}

pub fn eloquent_model(ns: &str, name: &str) -> String {
    let name_lower = heck_snake(name);
    let table = format!("{}s", name_lower);
    let pk = format!("cod_{}", name_lower);

    format!(
        r#"<?php

declare(strict_types=1);

namespace {ns}\{name}\Infra\Persistence\Models;

use Illuminate\Database\Eloquent\Model;

class {name} extends Model
{{
    protected $table = '{table}';
    protected $primaryKey = '{pk}';
    public $timestamps = true;

    protected $fillable = [
        // TODO: adicionar campos
    ];
}}
"#
    )
}

pub fn repository(ns: &str, name: &str) -> String {
    let name_lower = heck_snake(name);
    let table = format!("{}s", name_lower);

    format!(
        r#"<?php

declare(strict_types=1);

namespace {ns}\{name}\Infra\Persistence\Repositories;

use {ns}\{name}\Domain\Entities\{name}Entity;
use {ns}\{name}\Infra\Persistence\Models\{name};
use App\Contexts\Compartilhado\Base\Domain\VOs\IdVO;
use App\Contexts\Compartilhado\Base\Application\DTOs\Inputs\PaginacaoInput;
use Illuminate\Pagination\LengthAwarePaginator;

class {name}Repository
{{
    public function obterPorId(IdVO $id): ?object
    {{
        return {name}::find($id->getValue());
    }}

    public function obterListaPorPaginacao(PaginacaoInput $input): LengthAwarePaginator
    {{
        return {name}::select([
            // TODO: listar colunas
        ])->paginate(perPage: $input->perPage, page: $input->page);
    }}

    public function criar({name}Entity $entity): IdVO
    {{
        $model = {name}::create([
            // TODO: mapear campos da entity
        ]);
        return new IdVO($model->getKey());
    }}

    public function alterar({name}Entity $entity): void
    {{
        {name}::where('{table}', $entity->id->getValue())->update([
            // TODO: mapear campos da entity
        ]);
    }}

    public function deletar(IdVO $id): void
    {{
        {name}::destroy($id->getValue());
    }}
}}
"#,
        ns = ns,
        name = name,
        table = table,
    )
}

pub fn exception(ns: &str, name: &str) -> String {
    format!(
        r#"<?php

declare(strict_types=1);

namespace {ns}\{name}\Application\Exceptions;

use App\Contexts\Compartilhado\Base\Application\Exceptions\BaseException;

class {name}Exception extends BaseException {{}}
"#
    )
}

pub fn error_nao_encontrado(ns: &str, name: &str) -> String {
    format!(
        r#"<?php

declare(strict_types=1);

namespace {ns}\{name}\Application\Errors;

use App\Contexts\Compartilhado\Base\Application\Errors\BaseError;

class {name}NaoEncontradoError extends BaseError
{{
    protected int $code = 404;
    protected string $message = '{name} nao encontrado';
}}
"#
    )
}

pub fn query_consultar(ns: &str, name: &str) -> String {
    format!(
        r#"<?php

declare(strict_types=1);

namespace {ns}\{name}\Application\Queries;

use {ns}\{name}\Application\DTOs\Outputs\Consultar{name}sOutput;
use {ns}\{name}\Infra\Persistence\Repositories\{name}Repository;
use App\Contexts\Compartilhado\Base\Application\DTOs\Inputs\PaginacaoInput;

class Consultar{name}sQuery
{{
    public function __construct(
        private {name}Repository ${var}Repository,
    ) {{}}

    public function executar(PaginacaoInput $input): Consultar{name}sOutput
    {{
        $output    = new Consultar{name}sOutput();
        $registros = $this->{var}Repository->obterListaPorPaginacao($input);

        foreach ($registros as $registro) {{
            $output->adicionarRegistro(
                // TODO: mapear campos do $registro
            );
        }}

        $output->page     = $registros->currentPage();
        $output->lastPage = $registros->lastPage();
        $output->total    = $registros->total();

        return $output;
    }}
}}
"#,
        ns = ns,
        name = name,
        var = heck_camel(name),
    )
}

pub fn query_detalhar(ns: &str, name: &str) -> String {
    format!(
        r#"<?php

declare(strict_types=1);

namespace {ns}\{name}\Application\Queries;

use {ns}\{name}\Application\DTOs\Outputs\Detalhar{name}Output;
use {ns}\{name}\Application\Exceptions\{name}Exception;
use {ns}\{name}\Application\Errors\{name}NaoEncontradoError;
use {ns}\{name}\Infra\Persistence\Repositories\{name}Repository;
use App\Contexts\Compartilhado\Base\Domain\VOs\IdVO;

class Detalhar{name}Query
{{
    public function __construct(
        private {name}Repository ${var}Repository,
    ) {{}}

    public function executar(IdVO $id): Detalhar{name}Output
    {{
        $registro = $this->{var}Repository->obterPorId($id);

        if (! $registro) {{
            throw new {name}Exception(new {name}NaoEncontradoError());
        }}

        return new Detalhar{name}Output(
            // TODO: mapear campos do $registro
        );
    }}
}}
"#,
        ns = ns,
        name = name,
        var = heck_camel(name),
    )
}

pub fn usecase_criar(ns: &str, name: &str) -> String {
    format!(
        r#"<?php

declare(strict_types=1);

namespace {ns}\{name}\Application\UseCases;

use {ns}\{name}\Application\DTOs\Inputs\Criar{name}Input;
use {ns}\{name}\Domain\Entities\{name}Entity;
use {ns}\{name}\Infra\Persistence\Repositories\{name}Repository;
use App\Contexts\Compartilhado\Base\Domain\VOs\IdVO;

class Criar{name}UseCase
{{
    public function __construct(
        private {name}Repository ${var}Repository,
    ) {{}}

    public function executar(Criar{name}Input $input): IdVO
    {{
        $entity = {name}Entity::create(
            // TODO: mapear campos do $input
        );

        return $this->{var}Repository->criar($entity);
    }}
}}
"#,
        ns = ns,
        name = name,
        var = heck_camel(name),
    )
}

pub fn usecase_alterar(ns: &str, name: &str) -> String {
    format!(
        r#"<?php

declare(strict_types=1);

namespace {ns}\{name}\Application\UseCases;

use {ns}\{name}\Application\DTOs\Inputs\Alterar{name}Input;
use {ns}\{name}\Application\Exceptions\{name}Exception;
use {ns}\{name}\Application\Errors\{name}NaoEncontradoError;
use {ns}\{name}\Domain\Entities\{name}Entity;
use {ns}\{name}\Infra\Persistence\Repositories\{name}Repository;

class Alterar{name}UseCase
{{
    public function __construct(
        private {name}Repository ${var}Repository,
    ) {{}}

    public function executar(Alterar{name}Input $input): void
    {{
        $existente = $this->{var}Repository->obterPorId($input->id);

        if (! $existente) {{
            throw new {name}Exception(new {name}NaoEncontradoError());
        }}

        $entity = {name}Entity::update(
            id: $input->id,
            // TODO: mapear campos do $input
        );

        $this->{var}Repository->alterar($entity);
    }}
}}
"#,
        ns = ns,
        name = name,
        var = heck_camel(name),
    )
}

pub fn usecase_deletar(ns: &str, name: &str) -> String {
    format!(
        r#"<?php

declare(strict_types=1);

namespace {ns}\{name}\Application\UseCases;

use {ns}\{name}\Application\Exceptions\{name}Exception;
use {ns}\{name}\Application\Errors\{name}NaoEncontradoError;
use {ns}\{name}\Infra\Persistence\Repositories\{name}Repository;
use App\Contexts\Compartilhado\Base\Domain\VOs\IdVO;

class Deletar{name}UseCase
{{
    public function __construct(
        private {name}Repository ${var}Repository,
    ) {{}}

    public function executar(IdVO $id): void
    {{
        $existente = $this->{var}Repository->obterPorId($id);

        if (! $existente) {{
            throw new {name}Exception(new {name}NaoEncontradoError());
        }}

        $this->{var}Repository->deletar($id);
    }}
}}
"#,
        ns = ns,
        name = name,
        var = heck_camel(name),
    )
}

pub fn input_criar(ns: &str, name: &str) -> String {
    format!(
        r#"<?php

declare(strict_types=1);

namespace {ns}\{name}\Application\DTOs\Inputs;

readonly class Criar{name}Input
{{
    public function __construct(
        // TODO: adicionar campos tipados
    ) {{}}
}}
"#
    )
}

pub fn input_alterar(ns: &str, name: &str) -> String {
    format!(
        r#"<?php

declare(strict_types=1);

namespace {ns}\{name}\Application\DTOs\Inputs;

use App\Contexts\Compartilhado\Base\Domain\VOs\IdVO;

readonly class Alterar{name}Input
{{
    public function __construct(
        public IdVO $id,
        // TODO: adicionar campos tipados
    ) {{}}
}}
"#
    )
}

pub fn output_consultar(ns: &str, name: &str) -> String {
    format!(
        r#"<?php

declare(strict_types=1);

namespace {ns}\{name}\Application\DTOs\Outputs;

use App\Contexts\Compartilhado\Base\Application\DTOs\Outputs\PaginacaoOutput;
use App\Contexts\Compartilhado\Base\Domain\VOs\IdVO;

class Consultar{name}sOutput extends PaginacaoOutput
{{
    public function adicionarRegistro(
        IdVO $id,
        // TODO: adicionar campos
    ): void {{
        $this->data[] = [
            'id' => $id->getValue(),
            // TODO: mapear campos
        ];
    }}
}}
"#
    )
}

pub fn output_detalhar(ns: &str, name: &str) -> String {
    format!(
        r#"<?php

declare(strict_types=1);

namespace {ns}\{name}\Application\DTOs\Outputs;

use App\Contexts\Compartilhado\Base\Domain\VOs\IdVO;

readonly class Detalhar{name}Output
{{
    public function __construct(
        public IdVO $id,
        // TODO: adicionar campos
    ) {{}}

    public function toArray(): array
    {{
        return [
            'id' => $this->id->getValue(),
            // TODO: mapear campos
        ];
    }}
}}
"#
    )
}

pub fn request_criar(ns: &str, name: &str) -> String {
    format!(
        r#"<?php

declare(strict_types=1);

namespace {ns}\{name}\Infra\Presentation\Http\Requests;

use Illuminate\Foundation\Http\FormRequest;

class Criar{name}Request extends FormRequest
{{
    public function authorize(): bool
    {{
        return true;
    }}

    public function rules(): array
    {{
        return [
            // TODO: adicionar regras de validacao
        ];
    }}
}}
"#
    )
}

pub fn request_alterar(ns: &str, name: &str) -> String {
    format!(
        r#"<?php

declare(strict_types=1);

namespace {ns}\{name}\Infra\Presentation\Http\Requests;

use Illuminate\Foundation\Http\FormRequest;

class Alterar{name}Request extends FormRequest
{{
    public function authorize(): bool
    {{
        return true;
    }}

    public function rules(): array
    {{
        return [
            // TODO: adicionar regras de validacao
        ];
    }}
}}
"#
    )
}

pub fn autorizacoes(ns: &str, name: &str) -> String {
    format!(
        r#"<?php

declare(strict_types=1);

namespace {ns}\{name}\Domain\Autorizacoes;

class {name}Autorizacoes
{{
    public const CONSULTAR = '{name}.consultar';
    public const DETALHAR  = '{name}.detalhar';
    public const CRIAR     = '{name}.criar';
    public const ALTERAR   = '{name}.alterar';
    public const DELETAR   = '{name}.deletar';
}}
"#
    )
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

/// PascalCase "ClientePedido" → snake_case "cliente_pedido"
fn heck_snake(s: &str) -> String {
    use heck::ToSnakeCase;
    s.to_snake_case()
}

/// PascalCase "ClientePedido" → camelCase "clientePedido"
fn heck_camel(s: &str) -> String {
    use heck::ToLowerCamelCase;
    s.to_lower_camel_case()
}
