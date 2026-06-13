use anyhow::Result;

pub fn render(name: &str, table: &str) -> Result<String> {
    Ok(format!(
        r#"<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;

class {name} extends Model
{{
    use HasFactory;

    protected $table = '{table}';

    protected $fillable = [
        //
    ];

    protected $hidden = [
        //
    ];
}}
"#,
        name = name,
        table = table,
    ))
}
