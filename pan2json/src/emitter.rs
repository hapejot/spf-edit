use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde::Serialize;
use crate::json_model::Panel;

/// Write a Panel as pretty-printed JSON to the categorized output directory.
pub fn write_panel(panel: &Panel, output_dir: &Path) -> std::io::Result<()> {
    let category_dir = output_dir.join(&panel.metadata.category);
    fs::create_dir_all(&category_dir)?;

    let filename = format!("{}.json", panel.id);
    let filepath = category_dir.join(&filename);

    let json = serde_json::to_string_pretty(panel)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    fs::write(&filepath, json)?;

    Ok(())
}

/// Manifest entry for a single panel
#[derive(Debug, Serialize)]
pub struct ManifestEntry {
    pub id: String,
    pub category: String,
    #[serde(rename = "type")]
    pub panel_type: String,
    pub source_file: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub help_panel: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nav_targets: Vec<String>,
    pub has_warnings: bool,
}

/// Manifest for all panels
#[derive(Debug, Serialize)]
pub struct Manifest {
    pub total_panels: usize,
    pub categories: HashMap<String, usize>,
    pub panels: Vec<ManifestEntry>,
}

impl Manifest {
    pub fn new() -> Self {
        Manifest {
            total_panels: 0,
            categories: HashMap::new(),
            panels: Vec::new(),
        }
    }

    pub fn add_panel(&mut self, panel: &Panel) {
        let category = panel.metadata.category.clone();
        *self.categories.entry(category.clone()).or_insert(0) += 1;
        self.total_panels += 1;

        let help_panel = panel.init.as_ref().and_then(|i| i.help_panel.clone());

        let nav_targets: Vec<String> = panel.proc_section.as_ref()
            .and_then(|p| p.navigation.as_ref())
            .map(|nav| {
                nav.routes.iter().filter_map(|r| {
                    match &r.action {
                        crate::json_model::NavAction::Panel { target } => Some(target.clone()),
                        crate::json_model::NavAction::List { targets } => {
                            Some(targets.join(", "))
                        }
                        _ => None,
                    }
                }).collect()
            })
            .unwrap_or_default();

        self.panels.push(ManifestEntry {
            id: panel.id.clone(),
            category,
            panel_type: format!("{:?}", panel.panel_type).to_lowercase(),
            source_file: panel.metadata.source_file.clone(),
            help_panel,
            nav_targets,
            has_warnings: !panel.metadata.parse_warnings.is_empty(),
        });
    }

    /// Write the manifest to manifest.json in the output directory.
    pub fn write(&self, output_dir: &Path) -> std::io::Result<()> {
        let filepath = output_dir.join("manifest.json");
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        fs::write(&filepath, json)?;
        Ok(())
    }
}

/// Statistics collected during conversion
pub struct ConversionStats {
    pub total_files: usize,
    pub successful: usize,
    pub with_warnings: usize,
    pub failed: usize,
    pub by_category: HashMap<String, usize>,
}

impl ConversionStats {
    pub fn new() -> Self {
        ConversionStats {
            total_files: 0,
            successful: 0,
            with_warnings: 0,
            failed: 0,
            by_category: HashMap::new(),
        }
    }

    pub fn report(&self) {
        println!("\n--- Conversion Summary ---");
        println!("Total files:    {}", self.total_files);
        println!("Successful:     {}", self.successful);
        println!("With warnings:  {}", self.with_warnings);
        println!("Failed:         {}", self.failed);
        println!("\nBy category:");
        let mut cats: Vec<_> = self.by_category.iter().collect();
        cats.sort_by_key(|(_, v)| std::cmp::Reverse(**v));
        for (cat, count) in cats {
            println!("  {:15} {}", cat, count);
        }
    }
}
