use anyhow::Result;

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
