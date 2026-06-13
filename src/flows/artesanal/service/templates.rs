pub fn render(service_name: &str, model: &str) -> String {
    format!(
        r#"<?php

declare(strict_types=1);

namespace App\Services;

use App\Models\{model};
use Illuminate\Contracts\Pagination\LengthAwarePaginator;

class {service_name}Service
{{
    public function __construct(
        protected {model} $model,
    ) {{}}

    public function index(): LengthAwarePaginator
    {{
        return $this->model->paginate();
    }}

    public function show(int $id): {model}
    {{
        return $this->model->findOrFail($id);
    }}

    public function store(array $data): {model}
    {{
        return $this->model->create($data);
    }}

    public function update(int $id, array $data): {model}
    {{
        $item = $this->model->findOrFail($id);
        $item->update($data);
        return $item;
    }}

    public function destroy(int $id): void
    {{
        $this->model->findOrFail($id)->delete();
    }}
}}
"#,
        service_name = service_name,
        model = model,
    )
}
