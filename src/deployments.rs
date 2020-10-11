use std::collections::HashMap;
use std::fs;
use yaml_rust::YamlLoader;

#[derive(Debug)]
pub struct DeployConfig {
    pub stacks: HashMap<String, String>,
    pub deployments: Vec<Deployment>,
}

#[derive(Debug)]
pub struct Deployment {
    pub repo: String,
    pub image: String,
    pub action: String,
    pub stack: String,
}

pub fn read_config(path: &str) -> DeployConfig {
    let contents = fs::read_to_string(path).unwrap();
    let mut output = DeployConfig {
        stacks: HashMap::new(),
        deployments: Vec::new(),
    };

    let docs = YamlLoader::load_from_str(&contents).unwrap();

    let stacks = &docs[0]["stacks"];
    for (key, value) in stacks.as_hash().unwrap().iter() {
        output.stacks.insert(
            key.as_str().unwrap().to_string(),
            value.as_str().unwrap().to_string(),
        );
    }

    let watches = &docs[0]["watch"];
    for (_, config) in watches.as_hash().unwrap().iter() {
        let image = config["image"]
            .as_str()
            .expect("no image reference provided");
        let stack = config["stack"]
            .as_str()
            .expect("no stack provided for deployment");
        let repo = config["repo"].as_str().expect("no repo provided");
        let action = config["action"]
            .as_str()
            .expect("no action trigger provided");

        if output.stacks.contains_key(stack) {
            output.deployments.push(Deployment {
                repo: repo.to_string(),
                image: image.to_string(),
                action: action.to_string(),
                stack: stack.to_string(),
            })
        } else {
            log::info!("No stack entry found for: {}", stack);
        }
    }

    return output;
}
