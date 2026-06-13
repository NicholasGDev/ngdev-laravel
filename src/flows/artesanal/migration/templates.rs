use anyhow::Result;

pub fn render(_name: &str, table: &str) -> Result<String> {
    Ok(format!(
        r#"<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{{
    public function up(): void
    {{
        Schema::create('{table}', function (Blueprint $table) {{
            $table->id();
            $table->timestamps();
        }});
    }}

    public function down(): void
    {{
        Schema::dropIfExists('{table}');
    }}
}};
"#,
        table = table,
    ))
}
