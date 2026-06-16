// ── Templates CI4 4.7 — baseados na estrutura real do appstarter ─────────────
// JWT implementado em PHP puro (HMAC-SHA256) — zero dependência externa.
// O gerador extrai CaronteCI4-base.zip e SOBRESCREVE apenas estes arquivos.

// ── Zip embarcado ─────────────────────────────────────────────────────────────
pub static CARONTE_CI4_BASE_ZIP: &[u8] = include_bytes!("../../../CaronteCI4-base.zip");

// ── Config/Database.php ───────────────────────────────────────────────────────
pub const CONFIG_DATABASE: &str = r#"<?php
declare(strict_types=1);

namespace Config;

use CodeIgniter\Database\Config;

class Database extends Config
{
    public string $filesPath    = APPPATH . 'Database' . DIRECTORY_SEPARATOR;
    public string $defaultGroup = 'default';

    public array $default = [
        'database'    => '',
        'DBDriver'    => 'SQLite3',
        'DBPrefix'    => '',
        'DBDebug'     => false,
        'swapPre'     => '',
        'failover'    => [],
        'foreignKeys' => true,
        'busyTimeout' => 1000,
        'synchronous' => null,
        'dateFormat'  => [
            'date'     => 'Y-m-d',
            'datetime' => 'Y-m-d H:i:s',
            'time'     => 'H:i:s',
        ],
    ];

    public array $tests = [
        'database'    => ':memory:',
        'DBDriver'    => 'SQLite3',
        'DBPrefix'    => 'db_',
        'DBDebug'     => true,
        'foreignKeys' => true,
        'busyTimeout' => 1000,
        'synchronous' => null,
        'dateFormat'  => [
            'date'     => 'Y-m-d',
            'datetime' => 'Y-m-d H:i:s',
            'time'     => 'H:i:s',
        ],
    ];

    public function __construct()
    {
        parent::__construct();
        $this->default['database'] = WRITEPATH . 'database/app.db';
    }
}
"#;

// ── Config/Filters.php ────────────────────────────────────────────────────────
pub const CONFIG_FILTERS: &str = r#"<?php
declare(strict_types=1);

namespace Config;

use App\Filters\CorsFilter;
use App\Filters\JwtFilter;
use CodeIgniter\Config\Filters as BaseFilters;
use CodeIgniter\Filters\CSRF;
use CodeIgniter\Filters\DebugToolbar;
use CodeIgniter\Filters\ForceHTTPS;
use CodeIgniter\Filters\Honeypot;
use CodeIgniter\Filters\InvalidChars;
use CodeIgniter\Filters\PageCache;
use CodeIgniter\Filters\PerformanceMetrics;
use CodeIgniter\Filters\SecureHeaders;

class Filters extends BaseFilters
{
    public array $aliases = [
        'csrf'          => CSRF::class,
        'toolbar'       => DebugToolbar::class,
        'honeypot'      => Honeypot::class,
        'invalidchars'  => InvalidChars::class,
        'secureheaders' => SecureHeaders::class,
        'forcehttps'    => ForceHTTPS::class,
        'pagecache'     => PageCache::class,
        'performance'   => PerformanceMetrics::class,
        'jwt'           => JwtFilter::class,
        'cors_custom'   => CorsFilter::class,
    ];

    public array $required = [
        'before' => ['forcehttps', 'pagecache'],
        'after'  => ['pagecache', 'performance', 'toolbar'],
    ];

    public array $globals = [
        'before' => ['cors_custom'],
        'after'  => [],
    ];

    public array $methods = [];
    public array $filters = [];
}
"#;

// ── Config/Routes.php ─────────────────────────────────────────────────────────
pub const CONFIG_ROUTES: &str = r#"<?php
declare(strict_types=1);

use CodeIgniter\Router\RouteCollection;

/** @var RouteCollection $routes */

$routes->post('auth/login', 'Auth::login');
$routes->post('contact',    'Contact::store');
$routes->get('setup/migrate', 'Setup::migrate');

$routes->group('', ['filter' => 'jwt'], static function (RouteCollection $routes): void {
    $routes->get('auth/me',              'Auth::me');
    $routes->get('export/tables',        'Export::tables');
    $routes->get('export/json/(:alpha)', 'Export::json/$1');
    $routes->get('export/csv/(:alpha)',  'Export::csv/$1');
});
"#;

// ── app/Libraries/Jwt.php — JWT puro PHP, zero dependência ───────────────────
pub const LIB_JWT: &str = r#"<?php
declare(strict_types=1);

namespace App\Libraries;

/**
 * JWT puro PHP (HS256) — sem dependência composer, FTP-ready.
 */
class Jwt
{
    public static function encode(array $payload, string $secret): string
    {
        $header = self::b64url(json_encode(['typ' => 'JWT', 'alg' => 'HS256'], JSON_THROW_ON_ERROR));
        $body   = self::b64url(json_encode($payload, JSON_THROW_ON_ERROR));
        $sig    = self::b64url(hash_hmac('sha256', "{$header}.{$body}", $secret, true));
        return "{$header}.{$body}.{$sig}";
    }

    /** @throws \RuntimeException */
    public static function decode(string $token, string $secret): array
    {
        $parts = explode('.', $token);
        if (count($parts) !== 3) {
            throw new \RuntimeException('Token malformado');
        }
        [$headerB64, $bodyB64, $sigB64] = $parts;

        $expected = self::b64url(hash_hmac('sha256', "{$headerB64}.{$bodyB64}", $secret, true));
        if (! hash_equals($expected, $sigB64)) {
            throw new \RuntimeException('Assinatura inválida');
        }

        $payload = json_decode(self::b64urlDecode($bodyB64), true, 512, JSON_THROW_ON_ERROR);

        if (isset($payload['exp']) && $payload['exp'] < time()) {
            throw new \RuntimeException('Token expirado');
        }
        return $payload;
    }

    private static function b64url(string $data): string
    {
        return rtrim(strtr(base64_encode($data), '+/', '-_'), '=');
    }

    private static function b64urlDecode(string $data): string
    {
        $decoded = base64_decode(strtr($data, '-_', '+/'), true);
        if ($decoded === false) {
            throw new \RuntimeException('Base64 inválido');
        }
        return $decoded;
    }
}
"#;

// ── app/Filters/JwtFilter.php ─────────────────────────────────────────────────
pub const FILTER_JWT: &str = r#"<?php
declare(strict_types=1);

namespace App\Filters;

use App\Libraries\Jwt;
use CodeIgniter\Filters\FilterInterface;
use CodeIgniter\HTTP\RequestInterface;
use CodeIgniter\HTTP\ResponseInterface;

class JwtFilter implements FilterInterface
{
    public function before(RequestInterface $request, $arguments = null)
    {
        $header = $request->getHeaderLine('Authorization');

        if (empty($header) || ! str_starts_with($header, 'Bearer ')) {
            return response()->setStatusCode(401)->setJSON(['error' => 'Token ausente']);
        }

        $token  = substr($header, 7);
        $secret = (string) env('JWT_SECRET', '');

        try {
            $request->user = (object) Jwt::decode($token, $secret);
        } catch (\RuntimeException $e) {
            return response()->setStatusCode(401)->setJSON(['error' => $e->getMessage()]);
        }
    }

    public function after(RequestInterface $request, ResponseInterface $response, $arguments = null): void {}
}
"#;

// ── app/Filters/CorsFilter.php ────────────────────────────────────────────────
pub const FILTER_CORS: &str = r#"<?php
declare(strict_types=1);

namespace App\Filters;

use CodeIgniter\Filters\FilterInterface;
use CodeIgniter\HTTP\RequestInterface;
use CodeIgniter\HTTP\ResponseInterface;

class CorsFilter implements FilterInterface
{
    public function before(RequestInterface $request, $arguments = null)
    {
        $origin = (string) env('CORS_ORIGIN', '*');

        if (strtolower($request->getMethod()) === 'options') {
            return response()
                ->setStatusCode(204)
                ->setHeader('Access-Control-Allow-Origin',  $origin)
                ->setHeader('Access-Control-Allow-Methods', 'GET, POST, PUT, PATCH, DELETE, OPTIONS')
                ->setHeader('Access-Control-Allow-Headers', 'Content-Type, Authorization, X-Requested-With')
                ->setHeader('Access-Control-Max-Age',       '86400');
        }
    }

    public function after(RequestInterface $request, ResponseInterface $response, $arguments = null)
    {
        $origin = (string) env('CORS_ORIGIN', '*');
        return $response
            ->setHeader('Access-Control-Allow-Origin',  $origin)
            ->setHeader('Access-Control-Allow-Methods', 'GET, POST, PUT, PATCH, DELETE, OPTIONS')
            ->setHeader('Access-Control-Allow-Headers', 'Content-Type, Authorization, X-Requested-With');
    }
}
"#;

// ── Controllers ───────────────────────────────────────────────────────────────
pub const CTRL_BASE: &str = r#"<?php
declare(strict_types=1);

namespace App\Controllers;

use CodeIgniter\Controller;
use CodeIgniter\HTTP\CLIRequest;
use CodeIgniter\HTTP\IncomingRequest;
use CodeIgniter\HTTP\RequestInterface;
use CodeIgniter\HTTP\ResponseInterface;
use Psr\Log\LoggerInterface;

abstract class BaseController extends Controller
{
    protected IncomingRequest|CLIRequest $request;
    protected $helpers = ['url', 'form'];

    public function initController(
        RequestInterface $request,
        ResponseInterface $response,
        LoggerInterface $logger
    ): void {
        parent::initController($request, $response, $logger);
    }
}
"#;

pub const CTRL_AUTH: &str = r#"<?php
declare(strict_types=1);

namespace App\Controllers;

use App\Libraries\Jwt;
use App\Models\UserModel;

class Auth extends BaseController
{
    public function login(): \CodeIgniter\HTTP\ResponseInterface
    {
        $json     = $this->request->getJSON(true) ?? [];
        $email    = trim((string) ($json['email']    ?? ''));
        $password = trim((string) ($json['password'] ?? ''));

        if ($email === '' || $password === '') {
            return $this->response->setStatusCode(422)->setJSON(['error' => 'Email e senha são obrigatórios']);
        }

        $user = (new UserModel())->where('email', $email)->first();

        if (! $user || ! password_verify($password, $user['password'])) {
            return $this->response->setStatusCode(401)->setJSON(['error' => 'Credenciais inválidas']);
        }

        $secret = (string) env('JWT_SECRET', 'default-secret');
        $ttl    = (int)    env('JWT_TTL', 3600);

        $token = Jwt::encode([
            'iss' => base_url(), 'iat' => time(), 'exp' => time() + $ttl,
            'sub' => $user['id'], 'email' => $user['email'],
            'name' => $user['name'], 'role' => $user['role'] ?? 'user',
        ], $secret);

        return $this->response->setJSON([
            'token'      => $token,
            'expires_in' => $ttl,
            'user'       => ['id' => $user['id'], 'email' => $user['email'],
                             'name' => $user['name'], 'role' => $user['role'] ?? 'user'],
        ]);
    }

    public function me(): \CodeIgniter\HTTP\ResponseInterface
    {
        return $this->response->setJSON(['user' => (array) $this->request->user]);
    }
}
"#;

pub const CTRL_EXPORT: &str = r#"<?php
declare(strict_types=1);

namespace App\Controllers;

class Export extends BaseController
{
    private const ALLOWED = ['leads', 'users'];

    public function tables(): \CodeIgniter\HTTP\ResponseInterface
    {
        return $this->response->setJSON(['tables' => self::ALLOWED]);
    }

    public function json(string $table): \CodeIgniter\HTTP\ResponseInterface
    {
        if (! in_array($table, self::ALLOWED, true)) {
            return $this->response->setStatusCode(403)->setJSON(['error' => "Tabela '{$table}' não permitida"]);
        }
        $rows = db_connect()->table($table)->get()->getResultArray();
        return $this->response->setJSON([
            'table' => $table, 'total' => count($rows),
            'exported_at' => date('Y-m-d H:i:s'), 'data' => $rows,
        ]);
    }

    public function csv(string $table): \CodeIgniter\HTTP\ResponseInterface
    {
        if (! in_array($table, self::ALLOWED, true)) {
            return $this->response->setStatusCode(403)->setJSON(['error' => "Tabela '{$table}' não permitida"]);
        }
        $rows = db_connect()->table($table)->get()->getResultArray();
        if (empty($rows)) {
            return $this->response->setStatusCode(204);
        }
        $temp = fopen('php://temp', 'r+');
        fputcsv($temp, array_keys($rows[0]));
        foreach ($rows as $row) { fputcsv($temp, array_map('strval', $row)); }
        rewind($temp);
        $csv = stream_get_contents($temp);
        fclose($temp);
        $filename = $table . '_' . date('Ymd_His') . '.csv';
        return $this->response
            ->setHeader('Content-Type', 'text/csv; charset=utf-8')
            ->setHeader('Content-Disposition', "attachment; filename=\"{$filename}\"")
            ->setBody($csv);
    }
}
"#;

pub const CTRL_SETUP: &str = r#"<?php
declare(strict_types=1);

namespace App\Controllers;

class Setup extends BaseController
{
    public function migrate(): \CodeIgniter\HTTP\ResponseInterface
    {
        $token      = (string) ($this->request->getGet('token') ?? '');
        $setupToken = (string) env('SETUP_TOKEN', '');

        if ($setupToken === '' || ! hash_equals($setupToken, $token)) {
            return $this->response->setStatusCode(403)->setJSON(['error' => 'Token inválido']);
        }

        try {
            \Config\Services::migrations()->latest();
            return $this->response->setJSON([
                'message' => 'Migrations executadas com sucesso.',
                'note'    => 'Remova a rota /setup/migrate em Config/Routes.php.',
            ]);
        } catch (\Throwable $e) {
            return $this->response->setStatusCode(500)->setJSON(['error' => $e->getMessage()]);
        }
    }
}
"#;

// ── Models ────────────────────────────────────────────────────────────────────
pub const MODEL_USER: &str = r#"<?php
declare(strict_types=1);

namespace App\Models;

use CodeIgniter\Model;

class UserModel extends Model
{
    protected $table         = 'users';
    protected $returnType    = 'array';
    protected $allowedFields = ['name', 'email', 'password', 'role'];
    protected $useTimestamps = true;
    protected $beforeInsert  = ['hashPassword'];
    protected $beforeUpdate  = ['hashPassword'];

    protected function hashPassword(array $data): array
    {
        if (! empty($data['data']['password'])) {
            $data['data']['password'] = password_hash($data['data']['password'], PASSWORD_BCRYPT);
        }
        return $data;
    }
}
"#;

pub const MODEL_LEAD: &str = r#"<?php
declare(strict_types=1);

namespace App\Models;

use CodeIgniter\Model;

class LeadModel extends Model
{
    protected $table         = 'leads';
    protected $returnType    = 'array';
    protected $allowedFields = ['name', 'email', 'phone', 'message', 'source'];
    protected $useTimestamps = true;
}
"#;

// ── Migrations ────────────────────────────────────────────────────────────────
pub const MIG_USERS: &str = r#"<?php
declare(strict_types=1);

namespace App\Database\Migrations;

use CodeIgniter\Database\Migration;

class CreateUsersTable extends Migration
{
    public function up(): void
    {
        $this->forge->addField([
            'id'         => ['type' => 'INTEGER', 'auto_increment' => true],
            'name'       => ['type' => 'VARCHAR', 'constraint' => 120],
            'email'      => ['type' => 'VARCHAR', 'constraint' => 120, 'unique' => true],
            'password'   => ['type' => 'VARCHAR', 'constraint' => 255],
            'role'       => ['type' => 'VARCHAR', 'constraint' => 20, 'default' => 'user'],
            'created_at' => ['type' => 'DATETIME', 'null' => true],
            'updated_at' => ['type' => 'DATETIME', 'null' => true],
        ]);
        $this->forge->addPrimaryKey('id');
        $this->forge->createTable('users', true);

        $this->db->table('users')->insert([
            'name'       => 'Admin',
            'email'      => 'admin@caronte.local',
            'password'   => password_hash('caronte@2026', PASSWORD_BCRYPT),
            'role'       => 'admin',
            'created_at' => date('Y-m-d H:i:s'),
            'updated_at' => date('Y-m-d H:i:s'),
        ]);
    }

    public function down(): void
    {
        $this->forge->dropTable('users', true);
    }
}
"#;

pub const MIG_LEADS: &str = r#"<?php
declare(strict_types=1);

namespace App\Database\Migrations;

use CodeIgniter\Database\Migration;

class CreateLeadsTable extends Migration
{
    public function up(): void
    {
        $this->forge->addField([
            'id'         => ['type' => 'INTEGER', 'auto_increment' => true],
            'name'       => ['type' => 'VARCHAR', 'constraint' => 120],
            'email'      => ['type' => 'VARCHAR', 'constraint' => 120],
            'phone'      => ['type' => 'VARCHAR', 'constraint' => 30,  'null' => true],
            'message'    => ['type' => 'TEXT',                          'null' => true],
            'source'     => ['type' => 'VARCHAR', 'constraint' => 60,  'default' => 'landing'],
            'created_at' => ['type' => 'DATETIME', 'null' => true],
            'updated_at' => ['type' => 'DATETIME', 'null' => true],
        ]);
        $this->forge->addPrimaryKey('id');
        $this->forge->createTable('leads', true);
    }

    public function down(): void
    {
        $this->forge->dropTable('leads', true);
    }
}
"#;

// ── Funções dinâmicas ─────────────────────────────────────────────────────────

pub fn config_app(base_url: &str) -> String {
    format!(
        r#"<?php
declare(strict_types=1);

namespace Config;

use CodeIgniter\Config\BaseConfig;

class App extends BaseConfig
{{
    public string $baseURL     = '{base_url}';
    public string $indexPage   = '';
    public string $appTimezone = 'America/Sao_Paulo';
    public string $defaultLocale = 'pt-BR';
    public string $charset     = 'UTF-8';
    public bool   $forceGlobalSecureRequests = false;
    public array  $allowedHostnames = [];
}}
"#
    )
}

pub fn env_template(base_url: &str, cors_origin: &str) -> String {
    format!(
        r#"#--------------------------------------------------------------------
# ENVIRONMENT
#--------------------------------------------------------------------
CI_ENVIRONMENT = production

app.baseURL   = '{base_url}'
app.indexPage = ''

#--------------------------------------------------------------------
# DATABASE — SQLite (writable/database/app.db)
#--------------------------------------------------------------------
database.default.DBDriver    = SQLite3
database.default.database    = WRITEPATH/database/app.db
database.default.foreignKeys = true
database.default.DBDebug     = false

#--------------------------------------------------------------------
# JWT — TROQUE antes do deploy!
#--------------------------------------------------------------------
JWT_SECRET = change-me-to-a-random-secret-string-32-chars
JWT_TTL    = 3600

#--------------------------------------------------------------------
# CORS
#--------------------------------------------------------------------
CORS_ORIGIN = {cors_origin}

#--------------------------------------------------------------------
# SETUP — remova a rota após o primeiro uso!
#--------------------------------------------------------------------
SETUP_TOKEN = change-me-before-first-deploy
"#
    )
}

pub fn ctrl_contact(company_name: &str) -> String {
    format!(
        r#"<?php
declare(strict_types=1);

namespace App\Controllers;

use App\Models\LeadModel;

/** Contact — recebe leads da landing page. Empresa: {company_name} */
class Contact extends BaseController
{{
    public function store(): \CodeIgniter\HTTP\ResponseInterface
    {{
        $json  = $this->request->getJSON(true) ?? $this->request->getPost() ?? [];
        $rules = [
            'name'  => 'required|min_length[2]|max_length[120]',
            'email' => 'required|valid_email|max_length[120]',
        ];

        if (! $this->validate($rules, $json)) {{
            return $this->response->setStatusCode(422)->setJSON(['error' => $this->validator->getErrors()]);
        }}

        $id = (new LeadModel())->insert([
            'name'    => htmlspecialchars(trim((string) ($json['name']    ?? '')), ENT_QUOTES, 'UTF-8'),
            'email'   => strtolower(trim((string) ($json['email']         ?? ''))),
            'phone'   => htmlspecialchars(trim((string) ($json['phone']   ?? '')), ENT_QUOTES, 'UTF-8'),
            'message' => htmlspecialchars(trim((string) ($json['message'] ?? '')), ENT_QUOTES, 'UTF-8'),
            'source'  => htmlspecialchars(trim((string) ($json['source']  ?? 'landing')), ENT_QUOTES, 'UTF-8'),
        ]);

        return $this->response->setStatusCode(201)->setJSON(['message' => 'Contato registrado', 'id' => $id]);
    }}
}}
"#
    )
}
