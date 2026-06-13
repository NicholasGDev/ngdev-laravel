---
applyTo: 'back/app/Contexts/**'
---

# Arquitetura de Contexts — ngdev Laravel

> Regras obrigatórias para geração de código em `back/app/Contexts/`.
> Seguem DDD + Clean Architecture. Desvios devem ser aprovados explicitamente.

---

## Gerar um novo Context

```bash
php artisan make:context NomeContext
# opções: --prefix=rota-custom  --sem-entity  --sem-autorizacoes
```

O comando cria toda a estrutura de diretórios, ServiceProvider, rotas, controller, exception, error, entity, repository, query e usecase skeleton, e registra o provider em `bootstrap/providers.php`.

---

## Estrutura obrigatória de diretórios

```
Contexts/[Nome]/
├── Application/
│   ├── DTOs/Inputs/          # readonly class — parâmetros tipados
│   ├── DTOs/Outputs/         # class (não readonly quando usa PaginacaoOutput)
│   ├── Errors/               # extends BaseError — erros para o usuário
│   ├── Exceptions/           # extends BaseException — lançada pelo UseCase/Query
│   ├── Queries/              # apenas GET — nunca escrevem dados
│   ├── Services/             # lógica complexa de negócio
│   └── UseCases/             # apenas POST/PUT/DELETE
├── Domain/
│   ├── Entities/             # inicializar sempre com ::create() / ::update()
│   ├── Enums/
│   ├── Filters/
│   └── Autorizacoes/         # regras de autorização (quando aplicável)
└── Infra/
    ├── Persistence/Models/
    ├── Persistence/Repositories/
    ├── Presentation/Http/Controllers/
    ├── Presentation/Http/Requests/
    ├── Presentation/Routes/api.php
    └── Providers/[Nome]ServiceProvider.php
```

---

## ServiceProvider

```php
// Infra/Providers/[Nome]ServiceProvider.php
class [Nome]ServiceProvider extends CompartilhadoServiceProvider
{
    public function __construct(Application $app)
    {
        parent::__construct($app);
        $this->setPrefix('nome-em-kebab-case');
        $this->setName('nome-em-kebab-case');
        $this->setRoute(__DIR__ . '/../Presentation/Routes/api.php');
    }
}
```

Registrar em `bootstrap/providers.php` (o `make:context` faz isso automaticamente).

---

## Rotas — padrões obrigatórios

| HTTP   | Caminho             | Método do controller |
|--------|---------------------|----------------------|
| GET    | `/consultar`        | `consultar`          |
| GET    | `/detalhar/{id}`    | `detalhar`           |
| POST   | `/criar`            | `criar`              |
| PUT    | `/alterar/{id}`     | `alterar`            |
| DELETE | `/deletar/{id}`     | `deletar` (opcional) |

- Sempre minúsculo com hífen (`-`), nunca camelCase
- Sub-recursos: `/api/[context]/[sub-recurso]/[método]`

```php
// Infra/Presentation/Routes/api.php
Route::get('/consultar', [NomeController::class, 'consultar'])->name('consultar');
Route::get('/detalhar/{id}', [NomeController::class, 'detalhar'])->name('detalhar');
Route::post('/criar', [NomeController::class, 'criar'])->name('criar');
Route::put('/alterar/{id}', [NomeController::class, 'alterar'])->name('alterar');
```

---

## Fluxo de dados por tipo de operação

**GET → Query**
```
Controller → Query → Repository → Database
```

**POST / PUT / DELETE → UseCase**
```
Controller → Request → UseCase → (Service?) → Entity → Repository → Database
```

---

## DTOs de Input — `readonly class`

```php
declare(strict_types=1);

readonly class CriarClienteInput
{
    public function __construct(
        public string $nome,
        public EmailVO $email,
        public IdVO $codLoja,
    ) {}
}
```

---

## DTOs de Output — dois tipos

**Simples (readonly):**
```php
readonly class DetalharClienteOutput
{
    public function __construct(
        public IdVO $id,
        public string $nome,
    ) {}

    public function toArray(): array
    {
        return ['id' => $this->id->getValue(), 'nome' => $this->nome];
    }
}
```

**Listagem paginada (extends PaginacaoOutput):**
```php
class ConsultarClientesOutput extends PaginacaoOutput
{
    public function adicionarRegistro(   // nome livre: adicionarItem, adicionarCliente, etc.
        IdVO $id,
        string $nome,
        Carbon $dataCadastro,
    ): void {
        $this->data[] = [
            'id'            => $id->getValue(),
            'nome'          => $nome,
            'data_cadastro' => $dataCadastro->format('Y-m-d'),
        ];
    }
}
```

---

## Query (GET)

```php
class ConsultarClientesQuery
{
    public function __construct(
        private ClienteRepository $clienteRepository,
    ) {}

    public function executar(PaginacaoInput $input): ConsultarClientesOutput
    {
        $output    = new ConsultarClientesOutput();
        $registros = $this->clienteRepository->obterListaPorPaginacao($input);

        foreach ($registros as $registro) {
            $output->adicionarRegistro(
                id:           new IdVO((int) $registro->cod_cliente),
                nome:         $registro->des_cliente,
                dataCadastro: Carbon::parse($registro->data_cadastro),
            );
        }

        $output->page     = $registros->currentPage();
        $output->lastPage = $registros->lastPage();
        $output->total    = $registros->total();

        return $output;
    }
}
```

---

## UseCase (POST/PUT/DELETE)

```php
class CriarClienteUseCase
{
    public function __construct(
        private ClienteRepository $clienteRepository,
    ) {}

    public function executar(CriarClienteInput $input): IdVO
    {
        $existente = $this->clienteRepository->obterPorEmail($input->email);
        if ($existente) {
            throw new ClienteException(new ClienteJaExisteError());
        }

        $entity = ClienteEntity::create(nome: $input->nome, email: $input->email);
        return $this->clienteRepository->criar($entity);
    }
}
```

---

## Entity

```php
class ClienteEntity
{
    private function __construct(
        public readonly ?IdVO $id,
        public readonly string $nome,
        public readonly EmailVO $email,
    ) {}

    public static function create(string $nome, EmailVO $email): self
    {
        // validações aqui
        return new self(id: null, nome: $nome, email: $email);
    }

    public static function update(IdVO $id, string $nome): self
    {
        return new self(id: $id, nome: $nome, email: new EmailVO(''));
    }
}
```

**Nunca instanciar Entity com `new` diretamente.** Sempre usar `::create()` ou `::update()`.

---

## Repository

```php
class ClienteRepository
{
    // ✅ Retorna OBJETO Eloquent, nunca ->toArray()
    public function obterPorId(IdVO $id): ?object
    {
        return Cliente::find($id->getValue());
    }

    public function obterListaPorPaginacao(PaginacaoInput $input): LengthAwarePaginator
    {
        return Cliente::select(['cod_cliente', 'des_cliente', 'data_cadastro'])
            ->paginate(perPage: $input->perPage, page: $input->page);
    }

    public function criar(ClienteEntity $entity): IdVO
    {
        $model = Cliente::create(['des_cliente' => $entity->nome]);
        return new IdVO($model->cod_cliente);
    }
}
```

---

## Errors e Exceptions

**Exception do context (um por context):**
```php
// Application/Exceptions/ClienteException.php
class ClienteException extends BaseException {}
```

**Error específico:**
```php
// Application/Errors/ClienteNaoEncontradoError.php
class ClienteNaoEncontradoError extends BaseError
{
    protected int $code = 404;
    protected string $message = 'Cliente não encontrado';
}
```

**Códigos HTTP:** 400 Bad Request · 401 Unauthorized · 403 Forbidden · 404 Not Found · 409 Conflict · 422 Unprocessable · 500 Internal Error

**Uso no UseCase:**
```php
throw new ClienteException(new ClienteNaoEncontradoError());
```

---

## Controller

```php
class ClienteController extends Controller
{
    public function consultar(Request $request, ConsultarClientesQuery $query): JsonResponse
    {
        $input  = new PaginacaoInput(page: $request->page(1), perPage: $request->perPage(20));
        $output = $query->executar($input);
        return $this->successResponse(data: $output->toArray());
    }

    public function criar(CriarClienteRequest $request, CriarClienteUseCase $useCase): JsonResponse
    {
        $input = new CriarClienteInput(nome: $request->nome, email: new EmailVO($request->email));
        $id    = $useCase->executar($input);
        return $this->successResponse(data: ['id' => $id->getValue()], status: 201);
    }
}
```

---

## Value Objects disponíveis

```php
// Base
use App\Contexts\Compartilhado\Base\Domain\VOs\IdVO;        // IDs numéricos
use App\Contexts\Compartilhado\Base\Domain\VOs\EmailVO;
use App\Contexts\Compartilhado\Base\Domain\VOs\PhoneVO;
use App\Contexts\Compartilhado\Base\Domain\VOs\CepVO;
use App\Contexts\Compartilhado\Base\Domain\VOs\MoneyVO;
use App\Contexts\Compartilhado\Base\Domain\VOs\DecimalVO;
use App\Contexts\Compartilhado\Base\Domain\VOs\DocumentVO;  // CPF/CNPJ automático

// Documentos específicos
use App\Contexts\Compartilhado\Base\Domain\VOs\Documents\CPFVO;
use App\Contexts\Compartilhado\Base\Domain\VOs\Documents\CNPJVO;
use App\Contexts\Compartilhado\Base\Domain\VOs\Documents\RGVO;
use App\Contexts\Compartilhado\Base\Domain\VOs\Documents\RNEVO;

// Enums compartilhados
use App\Contexts\Compartilhado\Base\Domain\Enums\TipoPessoaEnum;
use App\Contexts\Compartilhado\Base\Domain\Enums\UFEnum;
```

Procurar VOs existentes antes de criar novos.

---

## Regras absolutas — Arrays Associativos

| Local                          | Array associativo permitido? |
|-------------------------------|------------------------------|
| DTO Output `toArray()`         | ✅ SIM                        |
| DTO Output métodos `adicionar*`| ✅ SIM                        |
| Repository                     | ❌ NÃO — retornar objeto       |
| Query / UseCase                | ❌ NÃO                        |
| Entity / Service               | ❌ NÃO                        |

```php
// ❌ PROIBIDO em Repository
return $model->toArray();

// ❌ PROIBIDO em Query/UseCase
$dados = ['campo' => $valor];
$item  = (object) ['cod' => $cod];

// ❌ PROIBIDO — acessar Eloquent como array
$nome = $model['des_cliente'];

// ✅ CORRETO — Repository retorna objeto
return $model;

// ✅ CORRETO — Query usa atributos
$registro->des_cliente;

// ✅ CORRETO — passa diretamente para o método tipado
$output->adicionarRegistro(id: new IdVO((int) $registro->cod_cliente), nome: $registro->des_cliente);
```

---

## Regras de nomenclatura

- `declare(strict_types=1)` em TODOS os arquivos PHP
- **UseCases** → apenas POST/PUT/DELETE
- **Queries** → apenas GET
- **Repository** → único com SQL direto
- **Entity** → nunca `new Entity()`, sempre `::create()` / `::update()`
- Rotas sempre em **minúsculo com hífen**, nunca camelCase

---

## Checklist antes de finalizar código

- [ ] Repository retorna objeto Eloquent (`return $model`)
- [ ] Nenhum `->toArray()` no Repository
- [ ] Nenhum array associativo fora de DTO Output
- [ ] Nenhum `(object) [...]` fora de DTO Output
- [ ] DTOs Input são `readonly class`
- [ ] Entity inicializada via método estático
- [ ] Erros de usuário usam Exception + Error
- [ ] Rotas em minúsculo com hífen
- [ ] `declare(strict_types=1)` presente
- [ ] UseCase ≠ GET, Query ≠ POST/PUT/DELETE
