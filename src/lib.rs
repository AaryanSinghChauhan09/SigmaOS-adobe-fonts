//! SigmaOS-adobe-fonts — comprehensive Adobe fonts component.
//!
//! Provides font management for Adobe fonts, including font cataloging,
//! metadata extraction, and font rendering support.

use std::collections::HashMap;

/// Font format
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FontFormat {
    /// TrueType font
    TrueType,
    /// OpenType font
    OpenType,
    /// PostScript Type 1 font
    PostScript,
    /// Adobe Font Metrics
    AFM,
    /// Multiple Master font
    MultipleMaster,
    /// Unknown format
    Unknown,
}

/// Font style
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FontStyle {
    /// Regular
    Regular,
    /// Bold
    Bold,
    /// Italic
    Italic,
    /// Bold Italic
    BoldItalic,
    /// Light
    Light,
    /// Medium
    Medium,
    /// Black
    Black,
}

/// Font metadata
#[derive(Debug, Clone)]
pub struct FontMetadata {
    /// Font family name
    pub family_name: String,
    /// Font subfamily name
    pub subfamily_name: String,
    /// Full font name
    pub full_name: String,
    /// Font version
    pub version: String,
    /// Copyright notice
    pub copyright: String,
    /// Trademark notice
    pub trademark: String,
    /// Font vendor
    pub vendor: String,
    /// Font designer
    pub designer: String,
    /// Font description
    pub description: String,
    /// Font format
    pub format: FontFormat,
    /// Font style
    pub style: FontStyle,
    /// Units per em
    pub units_per_em: u16,
    /// Ascent
    pub ascent: i16,
    /// Descent
    pub descent: i16,
    /// Line gap
    pub line_gap: i16,
    /// Cap height
    pub cap_height: i16,
    /// X height
    pub x_height: i16,
    /// Italic angle
    pub italic_angle: i16,
    /// Weight class
    pub weight_class: u16,
    /// Width class
    pub width_class: u16,
    /// Character set
    pub charset: String,
    /// Encoding
    pub encoding: String,
    /// PostScript name
    pub postscript_name: String,
}

/// Font file information
#[derive(Debug, Clone)]
pub struct FontFile {
    /// File path
    pub path: String,
    /// File size
    pub size: u64,
    /// File format
    pub format: FontFormat,
    /// Font metadata
    pub metadata: FontMetadata,
    /// Checksum
    pub checksum: u32,
    /// Last modified timestamp
    pub modified_at: u64,
}

/// Font collection
#[derive(Debug, Clone)]
pub struct FontCollection {
    /// Collection name
    pub name: String,
    /// Font files
    pub fonts: Vec<FontFile>,
    /// Collection description
    pub description: String,
    /// Collection version
    pub version: String,
}

/// Font manager
#[derive(Debug, Clone)]
pub struct FontManager {
    /// Font collections
    collections: HashMap<String, FontCollection>,
    /// Font files by path
    font_files: HashMap<String, FontFile>,
    /// Font families
    families: HashMap<String, Vec<String>>,
    /// Statistics
    total_fonts: u64,
    /// Total collections
    total_collections: u64,
}

impl Default for FontManager {
    fn default() -> Self {
        Self {
            collections: HashMap::new(),
            font_files: HashMap::new(),
            families: HashMap::new(),
            total_fonts: 0,
            total_collections: 0,
        }
    }
}

impl FontManager {
    /// Create new font manager
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Initialize with default collections
    pub fn initialize(&mut self) {
        // Add default Adobe fonts collection
        let mut adobe_fonts = FontCollection {
            name: "Adobe Fonts".into(),
            fonts: vec![],
            description: "Adobe Creative Cloud fonts".into(),
            version: "1.0".into(),
        };
        
        // Add some sample fonts
        let sample_fonts = vec![
            ("Myriad Pro", FontStyle::Regular),
            ("Myriad Pro", FontStyle::Bold),
            ("Myriad Pro", FontStyle::Italic),
            ("Minion Pro", FontStyle::Regular),
            ("Minion Pro", FontStyle::Bold),
            ("Adobe Clean", FontStyle::Regular),
        ];
        
        for (family, style) in sample_fonts {
            let font_file = FontFile {
                path: format!("/usr/share/fonts/adobe/{}_{}.otf", family.to_lowercase().replace(' ', "_"), style_to_string(&style).to_lowercase().replace(' ', "_")),
                size: 1024 * 1024, // 1MB sample
                format: FontFormat::OpenType,
                metadata: FontMetadata {
                    family_name: family.to_string(),
                    subfamily_name: style_to_string(&style),
                    full_name: format!("{} {}", family, style_to_string(&style)),
                    version: "1.0".into(),
                    copyright: "Copyright Adobe Systems".into(),
                    trademark: "Adobe is a trademark".into(),
                    vendor: "Adobe Systems".into(),
                    designer: "Adobe Type".into(),
                    description: format!("{} {} font", family, style_to_string(&style)),
                    format: FontFormat::OpenType,
                    style: style.clone(),
                    units_per_em: 1000,
                    ascent: 800,
                    descent: -200,
                    line_gap: 0,
                    cap_height: 700,
                    x_height: 500,
                    italic_angle: 0,
                    weight_class: match style {
                        FontStyle::Regular => 400,
                        FontStyle::Bold => 700,
                        FontStyle::Light => 300,
                        FontStyle::Medium => 500,
                        FontStyle::Black => 900,
                        _ => 400,
                    },
                    width_class: 5,
                    charset: "Unicode".into(),
                    encoding: "Unicode".into(),
                    postscript_name: format!("{}-{}", family.replace(' ', ""), style_to_string(&style)),
                },
                checksum: 0,
                modified_at: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            };
            
            // Add to collection and font_files map
            adobe_fonts.fonts.push(font_file.clone());
            self.font_files.insert(font_file.path.clone(), font_file.clone());
            self.total_fonts += 1;
            
            // Update family index
            let family = font_file.metadata.family_name.clone();
            self.families.entry(family).or_insert_with(Vec::new).push(font_file.path.clone());
        }
        
        self.add_collection(adobe_fonts);
    }
    
    /// Add font collection
    pub fn add_collection(&mut self, collection: FontCollection) -> String {
        let name = collection.name.clone();
        self.collections.insert(name.clone(), collection);
        self.total_collections += 1;
        name
    }
    
    /// Remove font collection
    pub fn remove_collection(&mut self, name: &str) -> bool {
        if let Some(collection) = self.collections.remove(name) {
            self.total_collections = self.total_collections.saturating_sub(1);
            self.total_fonts = self.total_fonts.saturating_sub(collection.fonts.len() as u64);
            return true;
        }
        false
    }
    
    /// Get font collection
    pub fn get_collection(&self, name: &str) -> Option<&FontCollection> {
        self.collections.get(name)
    }
    
    /// Get all collections
    pub fn get_all_collections(&self) -> Vec<&FontCollection> {
        self.collections.values().collect()
    }
    
    /// Add font file
    pub fn add_font_file(&mut self, collection_name: &str, font_file: FontFile) -> bool {
        if let Some(collection) = self.collections.get_mut(collection_name) {
            let path = font_file.path.clone();
            let family = font_file.metadata.family_name.clone();
            
            collection.fonts.push(font_file.clone());
            self.font_files.insert(path.clone(), font_file);
            self.total_fonts += 1;
            
            // Update family index
            self.families.entry(family).or_insert_with(Vec::new).push(path);
            
            return true;
        }
        false
    }
    
    /// Remove font file
    pub fn remove_font_file(&mut self, path: &str) -> bool {
        if let Some(font_file) = self.font_files.remove(path) {
            self.total_fonts = self.total_fonts.saturating_sub(1);
            
            // Remove from collection
            for collection in self.collections.values_mut() {
                if let Some(pos) = collection.fonts.iter().position(|f| f.path == path) {
                    collection.fonts.remove(pos);
                    break;
                }
            }
            
            // Remove from family index
            let family = font_file.metadata.family_name.clone();
            if let Some(family_fonts) = self.families.get_mut(&family) {
                family_fonts.retain(|p| p != path);
                if family_fonts.is_empty() {
                    self.families.remove(&family);
                }
            }
            
            return true;
        }
        false
    }
    
    /// Get font file
    pub fn get_font_file(&self, path: &str) -> Option<&FontFile> {
        self.font_files.get(path)
    }
    
    /// Get all font files
    pub fn get_all_font_files(&self) -> Vec<&FontFile> {
        self.font_files.values().collect()
    }
    
    /// Get fonts by family
    pub fn get_fonts_by_family(&self, family: &str) -> Vec<&FontFile> {
        self.families.get(family)
            .map(|paths| {
                paths.iter()
                    .filter_map(|path| self.font_files.get(path))
                    .collect()
            })
            .unwrap_or_default()
    }
    
    /// Get fonts by style
    pub fn get_fonts_by_style(&self, style: &FontStyle) -> Vec<&FontFile> {
        self.font_files.values()
            .filter(|f| f.metadata.style == *style)
            .collect()
    }
    
    /// Get fonts by format
    pub fn get_fonts_by_format(&self, format: &FontFormat) -> Vec<&FontFile> {
        self.font_files.values()
            .filter(|f| f.format == *format)
            .collect()
    }
    
    /// Search fonts
    pub fn search_fonts(&self, query: &str) -> Vec<&FontFile> {
        let query_lower = query.to_lowercase();
        self.font_files.values()
            .filter(|f| {
                f.metadata.family_name.to_lowercase().contains(&query_lower) ||
                f.metadata.full_name.to_lowercase().contains(&query_lower) ||
                f.metadata.vendor.to_lowercase().contains(&query_lower) ||
                f.metadata.designer.to_lowercase().contains(&query_lower)
            })
            .collect()
    }
    
    /// Get font statistics
    pub fn get_statistics(&self) -> FontStatistics {
        FontStatistics {
            total_fonts: self.total_fonts,
            total_collections: self.total_collections,
            total_families: self.families.len() as u64,
            total_size: self.font_files.values().map(|f| f.size).sum(),
            formats: self.get_format_breakdown(),
            styles: self.get_style_breakdown(),
        }
    }
    
    /// Get format breakdown
    fn get_format_breakdown(&self) -> HashMap<FontFormat, u64> {
        let mut breakdown = HashMap::new();
        for font_file in self.font_files.values() {
            *breakdown.entry(font_file.format.clone()).or_insert(0) += 1;
        }
        breakdown
    }
    
    /// Get style breakdown
    fn get_style_breakdown(&self) -> HashMap<FontStyle, u64> {
        let mut breakdown = HashMap::new();
        for font_file in self.font_files.values() {
            *breakdown.entry(font_file.metadata.style.clone()).or_insert(0) += 1;
        }
        breakdown
    }
    
    /// Export font data
    pub fn export_data(&self) -> FontExport {
        FontExport {
            collections: self.collections.values().cloned().collect(),
            font_files: self.font_files.values().cloned().collect(),
        }
    }
    
    /// Import font data
    pub fn import_data(&mut self, export: FontExport) {
        self.collections.clear();
        self.font_files.clear();
        self.families.clear();
        self.total_fonts = 0;
        self.total_collections = 0;
        
        for collection in export.collections {
            self.collections.insert(collection.name.clone(), collection);
            self.total_collections += 1;
        }
        
        for font_file in export.font_files {
            self.font_files.insert(font_file.path.clone(), font_file.clone());
            self.total_fonts += 1;
            
            // Update family index
            let family = font_file.metadata.family_name.clone();
            self.families.entry(family).or_insert_with(Vec::new).push(font_file.path.clone());
        }
    }
}

/// Convert style to string
fn style_to_string(style: &FontStyle) -> String {
    match style {
        FontStyle::Regular => "Regular".into(),
        FontStyle::Bold => "Bold".into(),
        FontStyle::Italic => "Italic".into(),
        FontStyle::BoldItalic => "Bold Italic".into(),
        FontStyle::Light => "Light".into(),
        FontStyle::Medium => "Medium".into(),
        FontStyle::Black => "Black".into(),
    }
}

/// Font statistics
#[derive(Debug, Clone)]
pub struct FontStatistics {
    /// Total fonts
    pub total_fonts: u64,
    /// Total collections
    pub total_collections: u64,
    /// Total families
    pub total_families: u64,
    /// Total size in bytes
    pub total_size: u64,
    /// Format breakdown
    pub formats: HashMap<FontFormat, u64>,
    /// Style breakdown
    pub styles: HashMap<FontStyle, u64>,
}

/// Font export data
#[derive(Debug, Clone)]
pub struct FontExport {
    /// Collections
    pub collections: Vec<FontCollection>,
    /// Font files
    pub font_files: Vec<FontFile>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_font_format() {
        let formats = vec![
            FontFormat::TrueType,
            FontFormat::OpenType,
            FontFormat::PostScript,
            FontFormat::AFM,
            FontFormat::MultipleMaster,
            FontFormat::Unknown,
        ];
        assert_eq!(formats.len(), 6);
    }
    
    #[test]
    fn test_font_style() {
        let styles = vec![
            FontStyle::Regular,
            FontStyle::Bold,
            FontStyle::Italic,
            FontStyle::BoldItalic,
            FontStyle::Light,
            FontStyle::Medium,
            FontStyle::Black,
        ];
        assert_eq!(styles.len(), 7);
    }
    
    #[test]
    fn test_font_metadata_creation() {
        let metadata = FontMetadata {
            family_name: "Test Font".into(),
            subfamily_name: "Regular".into(),
            full_name: "Test Font Regular".into(),
            version: "1.0".into(),
            copyright: "Copyright Test".into(),
            trademark: "Test is a trademark".into(),
            vendor: "Test Vendor".into(),
            designer: "Test Designer".into(),
            description: "Test font description".into(),
            format: FontFormat::OpenType,
            style: FontStyle::Regular,
            units_per_em: 1000,
            ascent: 800,
            descent: -200,
            line_gap: 0,
            cap_height: 700,
            x_height: 500,
            italic_angle: 0,
            weight_class: 400,
            width_class: 5,
            charset: "Unicode".into(),
            encoding: "Unicode".into(),
            postscript_name: "TestFont-Regular".into(),
        };
        
        assert_eq!(metadata.family_name, "Test Font");
        assert_eq!(metadata.format, FontFormat::OpenType);
    }
    
    #[test]
    fn test_font_file_creation() {
        let font_file = FontFile {
            path: "/usr/share/fonts/test.otf".into(),
            size: 1024 * 1024,
            format: FontFormat::OpenType,
            metadata: FontMetadata {
                family_name: "Test Font".into(),
                subfamily_name: "Regular".into(),
                full_name: "Test Font Regular".into(),
                version: "1.0".into(),
                copyright: "Copyright Test".into(),
                trademark: "Test".into(),
                vendor: "Test".into(),
                designer: "Test".into(),
                description: "Test".into(),
                format: FontFormat::OpenType,
                style: FontStyle::Regular,
                units_per_em: 1000,
                ascent: 800,
                descent: -200,
                line_gap: 0,
                cap_height: 700,
                x_height: 500,
                italic_angle: 0,
                weight_class: 400,
                width_class: 5,
                charset: "Unicode".into(),
                encoding: "Unicode".into(),
                postscript_name: "TestFont-Regular".into(),
            },
            checksum: 12345,
            modified_at: 1234567890,
        };
        
        assert_eq!(font_file.path, "/usr/share/fonts/test.otf");
        assert_eq!(font_file.size, 1024 * 1024);
    }
    
    #[test]
    fn test_font_manager_creation() {
        let manager = FontManager::new();
        assert_eq!(manager.get_statistics().total_fonts, 0);
        assert_eq!(manager.get_statistics().total_collections, 0);
    }
    
    #[test]
    fn test_initialize() {
        let mut manager = FontManager::new();
        manager.initialize();
        
        assert!(manager.get_collection("Adobe Fonts").is_some());
        assert!(manager.get_statistics().total_fonts > 0);
    }
    
    #[test]
    fn test_add_collection() {
        let mut manager = FontManager::new();
        
        let collection = FontCollection {
            name: "Test Collection".into(),
            fonts: vec![],
            description: "Test collection".into(),
            version: "1.0".into(),
        };
        
        manager.add_collection(collection);
        
        assert!(manager.get_collection("Test Collection").is_some());
        assert_eq!(manager.get_statistics().total_collections, 1);
    }
    
    #[test]
    fn test_add_font_file() {
        let mut manager = FontManager::new();
        manager.initialize();
        
        let font_file = FontFile {
            path: "/usr/share/fonts/test.otf".into(),
            size: 1024,
            format: FontFormat::OpenType,
            metadata: FontMetadata {
                family_name: "Test Font".into(),
                subfamily_name: "Regular".into(),
                full_name: "Test Font Regular".into(),
                version: "1.0".into(),
                copyright: "Copyright Test".into(),
                trademark: "Test".into(),
                vendor: "Test".into(),
                designer: "Test".into(),
                description: "Test".into(),
                format: FontFormat::OpenType,
                style: FontStyle::Regular,
                units_per_em: 1000,
                ascent: 800,
                descent: -200,
                line_gap: 0,
                cap_height: 700,
                x_height: 500,
                italic_angle: 0,
                weight_class: 400,
                width_class: 5,
                charset: "Unicode".into(),
                encoding: "Unicode".into(),
                postscript_name: "TestFont-Regular".into(),
            },
            checksum: 0,
            modified_at: 0,
        };
        
        assert!(manager.add_font_file("Adobe Fonts", font_file));
        assert!(manager.get_font_file("/usr/share/fonts/test.otf").is_some());
    }
    
    #[test]
    fn test_get_fonts_by_family() {
        let mut manager = FontManager::new();
        manager.initialize();
        
        let fonts = manager.get_fonts_by_family("Myriad Pro");
        assert!(!fonts.is_empty());
    }
    
    #[test]
    fn test_get_fonts_by_style() {
        let mut manager = FontManager::new();
        manager.initialize();
        
        let fonts = manager.get_fonts_by_style(&FontStyle::Regular);
        assert!(!fonts.is_empty());
    }
    
    #[test]
    fn test_search_fonts() {
        let mut manager = FontManager::new();
        manager.initialize();
        
        let results = manager.search_fonts("Myriad");
        assert!(!results.is_empty());
    }
    
    #[test]
    fn test_font_statistics() {
        let mut manager = FontManager::new();
        manager.initialize();
        
        let stats = manager.get_statistics();
        assert!(stats.total_fonts > 0);
        assert!(stats.total_collections > 0);
        assert!(stats.total_families > 0);
        assert!(stats.total_size > 0);
    }
    
    #[test]
    fn test_export_import() {
        let mut manager = FontManager::new();
        manager.initialize();
        
        let export = manager.export_data();
        
        let mut manager2 = FontManager::new();
        manager2.import_data(export);
        
        assert_eq!(manager2.get_statistics().total_fonts, manager.get_statistics().total_fonts);
    }
}
