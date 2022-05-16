use super::*;

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectOptions {
    /// If this is specified then any target that doesn't have an PRODUCT_BUNDLE_IDENTIFIER (via
    /// all levels of build settings) will get an autogenerated one by combining bundleIdPrefix and
    /// the target name: bundleIdPrefix.name. The target name will be stripped of all characters
    /// that aren't alphanumerics, hyphens, or periods. Underscores will be replaced with hyphens.
    ///
    /// Note: This is used to launch apps
    pub bundle_id_prefix: Option<String>,

    /// The path to the carthage build directory. Defaults to Carthage/Build. This is used when
    /// specifying target carthage dependencies
    ///
    #[serde(default = "carthage_build_path_default")]
    pub carthage_build_path: String,

    /// The path to the carthage executable. Defaults to carthage.
    pub carthage_executable_path: Option<String>,

    /// When this is set to true, all the invididual frameworks for Carthage framework dependencies
    /// will automatically be found. This property can be overriden individually for each carthage
    /// dependency - for more details see See findFrameworks in the Dependency section. Defaults to
    /// false.
    #[serde(default)]
    pub find_carthage_frameworks: bool,

    /// This controls the settings that are automatically applied to the project and its targets.
    /// These are the same build settings that Xcode would add when creating a new project. Project
    /// settings are applied by config type. Target settings are applied by the product type and
    /// platform. By default this is set to all
    ///
    /// - all: project and target settings
    /// - project: only project settings
    /// - targets: only target settings
    /// - none: no settings are automatically applied
    #[serde(default = "setting_presets_default")]
    pub setting_presets: String,

    /// A project wide deployment target can be specified for each platform otherwise the default
    /// SDK version in Xcode will be used. This will be overridden by any custom build settings
    /// that set the deployment target eg IPHONEOS_DEPLOYMENT_TARGET. Target specific deployment
    /// targets can also be set with Target.deploymentTarget.
    #[serde(default)]
    pub deployment_target: HashMap<Platform, String>,

    /// The default configuration for command line builds from Xcode. If the configuration provided
    /// here doesn't match one in your configs key, XcodeGen will fail. If you don't set this, the
    /// first configuration alphabetically will be chosen.
    pub default_config: Option<String>,

    /// If this is false and your project does not include resources located in a Base.lproj
    /// directory then Base will not be included in the projects 'known regions'. The default value
    /// is true.
    #[serde(default = "use_base_internationalization_default")]
    pub use_base_internationalization: bool,
}

const fn use_base_internationalization_default() -> bool {
    true
}

fn carthage_build_path_default() -> String {
    String::from("Carthage/Build")
}

fn setting_presets_default() -> String {
    String::from("all")
}
