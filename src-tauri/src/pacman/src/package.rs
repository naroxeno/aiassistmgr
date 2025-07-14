/// Represents the category of a package
#[derive(Debug)]
pub enum PkgCategory {
    /// Plugin for Obsidian
    ObsidianPlugin,
    /// Learning Materials
    LearningMaterials,
    /// AI prompt templates and collections
    AiPrompt,
    /// Meta-package that groups other packages
    MetaPackage
}

/// Represents a resource package with metadata
pub struct Package {
    /// Name of the package
    name: String,
    /// Version string (e.g. "1.0.0")
    version: String,
    /// Author or maintainer of the package
    author: String,
    /// List of files included in the package
    files: Vec<String>,
    /// Optional list of package dependencies
    dependencies: Option<Vec<String>>,
    /// Links to update this package, we automatically distinguish zsync-file links from package-file links
    download_link: Option<Vec<String>>,
    /// Optional description of the package
    description: Option<String>,
    /// Category this package belongs to
    category: PkgCategory
}

impl Package {
    /// Creates a new Package instance
    ///
    /// # Arguments
    /// * `name` - Package name
    /// * `version` - Version string
    /// * `author` - Author name
    /// * `files` - List of files in package
    /// * `dependencies` - Optional dependencies
    /// * `link` - Optional related links
    /// * `description` - Optional description
    /// * `category` - Package category
    pub fn new(
        name: String,
        version: String,
        author: String,
        files: Vec<String>,
        dependencies: Option<Vec<String>>,
        download_link: Option<Vec<String>>,
        description: Option<String>,
        category: PkgCategory  // Added missing category parameter
    ) -> Self {
        Package {
            name,
            version,
            author,
            files,
            dependencies,
            download_link,
            description,
            category
        }
    }

    /// 将下载链接分为普通文件和zsync文件两组
    ///
    /// 使用字符串匹配检查是否包含".zsync"来区分链接类型（未使用正则表达式以避免额外依赖）
    /// 返回元组 `(普通文件链接, zsync文件链接)`
    ///
    /// # 错误
    /// - 当`download_link`字段为`None`时返回"没有下载链接"
    /// - 当普通文件链接数组为空时返回"没有找到普通下载链接"
    /// - 当zsync文件链接数组为空时返回"没有找到zsync下载链接"
    pub fn separate_download_links(&self) -> Result<(Vec<String>, Vec<String>), String> {
        // 获取下载链接，若为None直接返回错误
        let links = match &self.download_link {
            Some(links) => links,
            None => return Err("没有下载链接".to_string()),
        };

        let mut regular_links = Vec::new();   // 存储普通文件链接
        let mut zsync_links = Vec::new();     // 存储zsync文件链接

        // 遍历所有链接并进行分类
        for link in links {
            if link.contains(".zsync") {
                zsync_links.push(link.clone());
            } else {
                regular_links.push(link.clone());
            }
        }

        // 检查普通文件链接是否为空
        if regular_links.is_empty() {
            return Err("No regular download links found".to_string());
        }

        // 检查zsync文件链接是否为空
        if zsync_links.is_empty() {
            return Err("No zsync download links found".to_string());
        }

        Ok((regular_links, zsync_links))
    }
}
