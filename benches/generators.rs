use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use ngdev_laravel::flows;
use std::path::PathBuf;

fn bench_landing_templates_only(c: &mut Criterion) {
    use ngdev_laravel::flows::landing_page::templates;

    let mut group = c.benchmark_group("landing_page_templates");

    group.bench_function("html_shell", |b| {
        b.iter(|| {
            black_box(templates::html_shell(
                black_box("AcmeCorp"),
                black_box("corporate"),
                black_box("<main>lorem</main>"),
            ))
        })
    });

    group.bench_function("section_hero", |b| {
        b.iter(|| {
            black_box(templates::section_hero(
                black_box("AcmeCorp"),
                black_box("Lorem ipsum dolor sit amet consectetur adipiscing elit."),
            ))
        })
    });

    group.bench_function("section_navbar", |b| {
        b.iter(|| black_box(templates::section_navbar(black_box("AcmeCorp"))))
    });

    group.bench_function("section_features_grid", |b| {
        b.iter(|| black_box(templates::section_features_grid()))
    });

    group.bench_function("section_features_tabs", |b| {
        b.iter(|| black_box(templates::section_features_tabs()))
    });

    group.bench_function("section_pricing", |b| {
        b.iter(|| black_box(templates::section_pricing()))
    });

    group.bench_function("section_testimonials", |b| {
        b.iter(|| black_box(templates::section_testimonials()))
    });

    group.bench_function("section_faq", |b| {
        b.iter(|| black_box(templates::section_faq()))
    });

    group.bench_function("section_stats", |b| {
        b.iter(|| black_box(templates::section_stats()))
    });

    group.bench_function("section_cta_bottom", |b| {
        b.iter(|| black_box(templates::section_cta_bottom(black_box("AcmeCorp"))))
    });

    group.bench_function("section_footer", |b| {
        b.iter(|| black_box(templates::section_footer(black_box("Acme Inc"))))
    });

    group.bench_function("all_sections_assembled", |b| {
        b.iter(|| {
            let mut body = String::new();
            body.push_str(&templates::section_navbar(black_box("AcmeCorp")));
            body.push_str(&templates::section_hero(
                black_box("AcmeCorp"),
                black_box("Lorem ipsum dolor sit amet."),
            ));
            body.push_str(templates::section_logos());
            body.push_str(templates::section_features_grid());
            body.push_str(templates::section_features_tabs());
            body.push_str(templates::section_stats());
            body.push_str(templates::section_testimonials());
            body.push_str(templates::section_pricing());
            body.push_str(templates::section_faq());
            body.push_str(&templates::section_cta_bottom(black_box("AcmeCorp")));
            body.push_str(&templates::section_footer(black_box("Acme Inc")));
            black_box(templates::html_shell(
                black_box("AcmeCorp"),
                black_box("corporate"),
                &body,
            ))
        })
    });

    group.finish();
}

fn bench_landing_page(c: &mut Criterion) {
    let all_sections: Vec<String> = vec![
        "logos", "features_grid", "features_tabs", "stats",
        "testimonials", "pricing", "faq", "cta_bottom",
    ]
    .into_iter()
    .map(String::from)
    .collect();

    let mut group = c.benchmark_group("landing_page");

    for n_sections in [1usize, 4, 8] {
        let sections = all_sections[..n_sections].to_vec();
        let tmp = tempfile_path("landing");

        group.bench_with_input(
            BenchmarkId::new("sections", n_sections),
            &(sections, tmp),
            |b, (secs, out)| {
                b.iter(|| {
                    flows::landing_page::generator::generate(black_box(
                        &flows::landing_page::generator::LandingPageOptions {
                            product_name: "AcmeCorp".into(),
                            tagline: "Lorem ipsum dolor sit amet.".into(),
                            company_name: "Acme Inc".into(),
                            theme: "corporate".into(),
                            layout: "generic".into(),
                            sections: secs.clone(),
                            output_dir: out.to_string_lossy().to_string(),
                        },
                    ))
                    .unwrap()
                })
            },
        );
    }

    group.finish();
}

fn bench_landing_themes(c: &mut Criterion) {
    let sections: Vec<String> = vec!["features_grid", "pricing", "cta_bottom"]
        .into_iter()
        .map(String::from)
        .collect();

    let mut group = c.benchmark_group("landing_page_themes");

    for theme in ["light", "dark", "corporate", "cyberpunk", "luxury"] {
        let tmp = tempfile_path(&format!("theme_{theme}"));

        group.bench_with_input(
            BenchmarkId::new("theme", theme),
            &(theme, tmp),
            |b, (t, out)| {
                b.iter(|| {
                    flows::landing_page::generator::generate(black_box(
                        &flows::landing_page::generator::LandingPageOptions {
                            product_name: "Demo".into(),
                            tagline: "Lorem ipsum.".into(),
                            company_name: "Demo Corp".into(),
                            theme: t.to_string(),
                            layout: "generic".into(),
                            sections: sections.clone(),
                            output_dir: out.to_string_lossy().to_string(),
                        },
                    ))
                    .unwrap()
                })
            },
        );
    }

    group.finish();
}

fn bench_context_generator(c: &mut Criterion) {
    use ngdev_laravel::flows::context::generator::{generate, ContextOptions};

    let mut group = c.benchmark_group("context_ddd");

    group.bench_function("minimal_2ops", |b| {
        b.iter(|| {
            let tmp = tempdir_path("ctx_minimal");
            generate(black_box(&ContextOptions {
                nome: "Produto".into(),
                base_path: tmp.to_string_lossy().to_string(),
                prefixo: "produtos".into(),
                namespace_base: "App\\Contexts".into(),
                com_entity: false,
                com_autorizacoes: false,
                operacoes: vec!["consultar".into(), "detalhar".into()],
            }))
            .unwrap()
        })
    });

    group.bench_function("full_5ops_entity_auth", |b| {
        b.iter(|| {
            let tmp = tempdir_path("ctx_full");
            generate(black_box(&ContextOptions {
                nome: "Produto".into(),
                base_path: tmp.to_string_lossy().to_string(),
                prefixo: "produtos".into(),
                namespace_base: "App\\Contexts".into(),
                com_entity: true,
                com_autorizacoes: true,
                operacoes: vec![
                    "consultar".into(), "detalhar".into(), "criar".into(),
                    "alterar".into(), "deletar".into(),
                ],
            }))
            .unwrap()
        })
    });

    group.finish();
}

fn bench_docker_generator(c: &mut Criterion) {
    use ngdev_laravel::flows::docker::generator::{generate, DockerOptions};

    let mut group = c.benchmark_group("docker_infra");

    group.bench_function("mysql_only", |b| {
        b.iter(|| {
            let tmp = tempdir_path("docker_mysql");
            generate(black_box(&DockerOptions {
                app_name: "MyApp".into(),
                php_version: "8.3".into(),
                node_version: 22,
                output_path: tmp.to_string_lossy().to_string(),
                server_name: "localhost".into(),
                databases: vec!["mysql".into()],
                with_redis: false,
                with_mailpit: false,
            }))
            .unwrap()
        })
    });

    group.bench_function("all_dbs_redis_mailpit", |b| {
        b.iter(|| {
            let tmp = tempdir_path("docker_full");
            generate(black_box(&DockerOptions {
                app_name: "MyApp".into(),
                php_version: "8.3".into(),
                node_version: 22,
                output_path: tmp.to_string_lossy().to_string(),
                server_name: "app.local".into(),
                databases: vec![
                    "mysql".into(), "pgsql".into(),
                    "mariadb".into(), "sqlserver".into(),
                ],
                with_redis: true,
                with_mailpit: true,
            }))
            .unwrap()
        })
    });

    group.finish();
}

fn tempfile_path(prefix: &str) -> PathBuf {
    let dir = std::env::temp_dir().join("ngdev_bench");
    std::fs::create_dir_all(&dir).ok();
    dir.join(format!(
        "{prefix}_{}.html",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .subsec_nanos()
    ))
}

fn tempdir_path(prefix: &str) -> PathBuf {
    let dir = std::env::temp_dir()
        .join("ngdev_bench")
        .join(format!(
            "{}_{}",
            prefix,
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .subsec_nanos()
        ));
    std::fs::create_dir_all(&dir).ok();
    dir
}

criterion_group!(
    benches,
    bench_landing_templates_only,
    bench_landing_page,
    bench_landing_themes,
    bench_context_generator,
    bench_docker_generator,
);
criterion_main!(benches);
