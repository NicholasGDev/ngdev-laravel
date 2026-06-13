use anyhow::Result;

/// Controller com injeção de Service (padrão MVCS).
pub fn render_resource_with_service(controller_name: &str, base_name: &str) -> String {
    format!(
        r#"<?php

declare(strict_types=1);

namespace App\Http\Controllers;

use App\Services\{base_name}Service;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;

class {controller_name} extends Controller
{{
    public function __construct(
        protected {base_name}Service $service,
    ) {{}}

    public function index(): JsonResponse
    {{
        return response()->json($this->service->index());
    }}

    public function store(Request $request): JsonResponse
    {{
        $item = $this->service->store($request->validated());
        return response()->json($item, 201);
    }}

    public function show(int $id): JsonResponse
    {{
        return response()->json($this->service->show($id));
    }}

    public function update(Request $request, int $id): JsonResponse
    {{
        return response()->json($this->service->update($id, $request->validated()));
    }}

    public function destroy(int $id): JsonResponse
    {{
        $this->service->destroy($id);
        return response()->json(null, 204);
    }}
}}
"#,
        controller_name = controller_name,
        base_name = base_name,
    )
}

pub fn render_resource(name: &str, model: Option<&str>) -> Result<String> {
    let use_model = model
        .map(|m| format!("use App\\Models\\{};\n", m))
        .unwrap_or_default();

    let model_type = model.unwrap_or("mixed");

    Ok(format!(
        r#"<?php

namespace App\Http\Controllers;

{use_model}use Illuminate\Http\Request;

class {name} extends Controller
{{
    public function index()
    {{
        //
    }}

    public function create()
    {{
        //
    }}

    public function store(Request $request)
    {{
        //
    }}

    public function show({model_type} ${var})
    {{
        //
    }}

    public function edit({model_type} ${var})
    {{
        //
    }}

    public function update(Request $request, {model_type} ${var})
    {{
        //
    }}

    public function destroy({model_type} ${var})
    {{
        //
    }}
}}
"#,
        use_model = use_model,
        name = name,
        model_type = model_type,
        var = model_type.to_lowercase(),
    ))
}

pub fn render_plain(name: &str) -> Result<String> {
    Ok(format!(
        r#"<?php

namespace App\Http\Controllers;

use Illuminate\Http\Request;

class {name} extends Controller
{{
    //
}}
"#,
        name = name,
    ))
}
