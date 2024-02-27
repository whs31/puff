#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Config
{
    remotes: ConfigRemote
}

#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct ConfigRemote
{
    pub registry_url: String,
    pub ci_url: String,
    pub artifactory_base_url: String
}

impl Default for Config
{
    fn default() -> Self {
        Self {
            remotes: ConfigRemote::default()
        }
    }
}

impl Default for ConfigRemote
{
    fn default() -> Self {
        Self {
            registry_url: String::from("http://uav.radar-mms.com/gitlab/test/essentials/poppy/poppy-registry.git"),
            ci_url: String::from("http://uav.radar-mms.com/gitlab/test/essentials/ci.git"),
            artifactory_base_url: String::from()
        }
    }
}
