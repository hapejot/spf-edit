use crate::json_model::PanelType;
use crate::parser::RawSections;

/// Classify a panel into a PanelType and category directory name.
/// Uses both the filename prefix and content heuristics.
pub fn categorize(filename: &str, sections: &RawSections) -> (PanelType, &'static str) {
    let upper = filename.to_uppercase();
    let stem = upper.trim_end_matches(".PAN");

    // Content-based heuristics (take priority)
    let has_model = sections.model.is_some();
    let has_trans = sections.proc_section.as_ref()
        .map(|p| p.to_uppercase().contains("TRANS(") || p.to_uppercase().contains("TRANS ("))
        .unwrap_or(false);
    let has_ver = sections.proc_section.as_ref()
        .map(|p| p.to_uppercase().contains("VER"))
        .unwrap_or(false);
    let has_body_only = sections.attr.is_none()
        && sections.model.is_none()
        && sections.init.as_ref().map(|s| s.trim().is_empty()).unwrap_or(true)
        && sections.proc_section.as_ref().map(|s| s.trim().is_empty()).unwrap_or(true);

    // Name-based patterns
    if stem.starts_with("S2RT") {
        return (PanelType::Tutorial, "tutorial");
    }
    if stem.starts_with("PHONE") {
        return (PanelType::Sample, "sample");
    }
    if stem.starts_with("SAMPHELP") || stem.starts_with("SPDTEST") {
        return (PanelType::Sample, "sample");
    }
    if stem == "SEDITPAN" || stem.starts_with("S2DEDIT") {
        return (PanelType::Edit, "edit");
    }
    if stem == "SBROWPAN" || stem.starts_with("S2DBROW") || stem == "S2SCBROW" {
        return (PanelType::Browse, "browse");
    }
    if stem.starts_with("S2CHNG") || stem == "S2CH18P2" {
        return (PanelType::Changelog, "changelog");
    }

    // Menu: has TRANS navigation
    if has_trans {
        return (PanelType::Menu, "menu");
    }
    if stem.starts_with("S2M") {
        return (PanelType::Menu, "menu");
    }

    // List: has MODEL section
    if has_model {
        return (PanelType::List, "list");
    }

    // Help: body-only panels, or S2R*/S2P* prefixes with no inputs
    if has_body_only {
        return (PanelType::Help, "help");
    }
    if (stem.starts_with("S2R") || stem.starts_with("S2P") || stem.starts_with("S2PTF"))
        && !has_ver && !has_model
    {
        return (PanelType::Help, "help");
    }

    // Form: has VER validation with INPUT fields
    if has_ver {
        return (PanelType::Form, "form");
    }

    // Dialog: S2D* prefix
    if stem.starts_with("S2D") {
        return (PanelType::Dialog, "dialog");
    }

    // Default: help for R/P panels, dialog for everything else
    if stem.starts_with("S2R") || stem.starts_with("S2P") {
        (PanelType::Help, "help")
    } else {
        (PanelType::Dialog, "dialog")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_sections() -> RawSections {
        RawSections::default()
    }

    #[test]
    fn test_categorize_edit_panel() {
        let (pt, dir) = categorize("SEDITPAN.PAN", &empty_sections());
        assert_eq!(pt, PanelType::Edit);
        assert_eq!(dir, "edit");
    }

    #[test]
    fn test_categorize_changelog() {
        let (pt, dir) = categorize("S2CHNG05.PAN", &empty_sections());
        assert_eq!(pt, PanelType::Changelog);
        assert_eq!(dir, "changelog");
    }

    #[test]
    fn test_categorize_list_by_model() {
        let mut s = empty_sections();
        s.model = Some("_Z+~COL1 ~COL2".to_string());
        let (pt, dir) = categorize("S2DCOLOR.PAN", &s);
        assert_eq!(pt, PanelType::List);
        assert_eq!(dir, "list");
    }

    #[test]
    fn test_categorize_menu_by_trans() {
        let mut s = empty_sections();
        s.proc_section = Some("&ZSEL = TRANS( &ZCMD, 1, 'PANEL(X)')".to_string());
        let (pt, dir) = categorize("S2CHNG00.PAN", &s);
        // S2CHNG prefix wins over TRANS heuristic
        assert_eq!(pt, PanelType::Changelog);
        assert_eq!(dir, "changelog");
    }

    #[test]
    fn test_categorize_help_panel() {
        let (pt, dir) = categorize("S2R3B001.PAN", &empty_sections());
        assert_eq!(pt, PanelType::Help);
        assert_eq!(dir, "help");
    }

    #[test]
    fn test_categorize_form_with_ver() {
        let mut s = empty_sections();
        s.proc_section = Some("VER ( &FNAME, NB )".to_string());
        let (pt, dir) = categorize("S2DCMD.PAN", &s);
        assert_eq!(pt, PanelType::Form);
        assert_eq!(dir, "form");
    }

    #[test]
    fn test_categorize_sample() {
        let (pt, dir) = categorize("PHONEMNU.PAN", &empty_sections());
        assert_eq!(pt, PanelType::Sample);
        assert_eq!(dir, "sample");
    }
}
