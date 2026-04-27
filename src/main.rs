extern crate malbox_plugin_sdk as malbox;

use malbox::prelude::*;

#[malbox::guest_plugin]
struct MyPlugin;

#[derive(Serialize)]
struct Output {
    message: String,
    task_id: i32,
}

#[malbox::handlers]
impl MyPlugin {
    #[malbox::on_start]
    fn start(&self) -> Result<()> {
        info!("plugin started");
        Ok(())
    }

    #[malbox::on_task]
    fn process(&self, task: Task, ctx: &Context) -> Result<()> {
        ctx.emit_progress(0.5, "processing")?;

        let output = Output {
            message: "hello from my plugin".to_string(),
            task_id: task.id(),
        };

        ctx.push_result(PluginResult::json("output", &output)?)?;
        Ok(())
    }

    #[malbox::on_stop]
    fn stop(&self) -> Result<()> {
        info!("plugin stopped");
        Ok(())
    }
}
