use iocraft::prelude::*;

pub struct Template {
    pub name: String,
    pub description: String,
}

#[derive(Default, Props)]
pub struct ListProps<'a> {
    pub templates: Vec<Template>,
    pub output: Option<&'a mut String>,
}

#[component]
pub fn List<'a>(mut hooks: Hooks, props: &mut ListProps<'a>) -> impl Into<AnyElement<'static>> {
    let mut system = hooks.use_context_mut::<SystemContext>();
    let max_index = props.templates.len() - 1;
    let max_name_length = props
        .templates
        .iter()
        .map(|t| t.name.len())
        .max()
        .unwrap_or(5);
    let mut index = hooks.use_state(|| 0);
    let mut should_exit = hooks.use_state(|| false);
    let mut has_selected = hooks.use_state(|| false);

    hooks.use_terminal_events({
        move |event| match event {
            TerminalEvent::Key(KeyEvent { code, kind, .. }) if kind != KeyEventKind::Release => {
                match code {
                    KeyCode::Esc | KeyCode::Char('q') => should_exit.set(true),
                    KeyCode::Up | KeyCode::Char('k') => {
                        if index.get() > 0 {
                            index.set(index.get() - 1)
                        }
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        index.set((index.get() + 1).min(max_index))
                    }
                    KeyCode::Enter => {
                        has_selected.set(true);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    });

    if has_selected.get() {
        if let Some(output) = props.output.as_mut() {
            **output = props.templates[index.get()].name.clone();
        }
        system.exit();
        return element! {View(){}};
    }

    if should_exit.get() {
        system.exit();
        return element! {View(){}};
    }

    element! {
        View(
            flex_direction: FlexDirection::Column,
            padding: 1,
            align_items: AlignItems::Center
        ) {
            Text(content: "Choose template:")
            View(
            ) {
                #(if should_exit.get() {
                    element! {
                        View() {}
                    }
                } else {
                    element!{
                        View(
                        flex_direction: FlexDirection::Column,
                        ){
                            #(props.templates.iter().enumerate().map(|(i, template)|
                            {
                                let is_selected = index.get() == i;
                                let cursor = if is_selected {"> "} else {"  "};
                                let content = format!("{}Name: {: <3$} Description: {}", cursor, template.name, template.description, max_name_length+1);
                                element!{
                                    Text(
                                        content: content,
                                        decoration: if index.get() == i {TextDecoration::Underline} else{TextDecoration::None})
                                }
                            }
                            ))
                        }
                    }
                })
            }
        }
    }
}
