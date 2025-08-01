use anyhow::Result;

use crate::cmd::meta::{KubieInfo, KubieInfoKind};
use crate::kubeconfig;
use crate::vars;

pub fn info(info: KubieInfo) -> Result<()> {
    match info.kind {
        KubieInfoKind::Context => {
            let conf = if vars::is_kubie_active() {
                kubeconfig::get_current_config()?
            } else {
                let default_config_path = kubeconfig::get_default_kubeconfig_path()?;
                crate::ioutil::read_yaml(default_config_path)?
            };
            println!("{}", conf.current_context.as_deref().unwrap_or(""));
        }
        KubieInfoKind::Namespace => {
            let conf = if vars::is_kubie_active() {
                kubeconfig::get_current_config()?
            } else {
                let default_config_path = kubeconfig::get_default_kubeconfig_path()?;
                crate::ioutil::read_yaml(default_config_path)?
            };

            // Find the current context and get its namespace
            if let Some(current_ctx_name) = &conf.current_context {
                if let Some(current_ctx) = conf.contexts.iter().find(|ctx| &ctx.name == current_ctx_name) {
                    println!("{}", current_ctx.context.namespace.as_deref().unwrap_or("default"));
                } else {
                    println!("default");
                }
            } else {
                println!("default");
            }
        }
        KubieInfoKind::Depth => {
            if vars::is_kubie_active() {
                println!("{}", vars::get_depth());
            } else {
                println!("0");
            }
        }
    };

    Ok(())
}
