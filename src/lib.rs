use inquire::Text;

fn render_config<'a>() -> inquire::ui::RenderConfig<'a> {
    inquire::ui::RenderConfig {
        prompt_prefix: "[ird] >".into(),
        answered_prompt_prefix: ">".into(),
        ..Default::default()
    }
}

pub fn tui() -> anyhow::Result<()> {
    let command = Text::new("").with_render_config(render_config()).prompt()?;

    Ok(())
}
