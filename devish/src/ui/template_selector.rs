use iocraft::prelude::*;

pub struct Repo {
    pub expr: String,
    pub templates: Vec<Template>,
}

#[derive(Clone)]
pub struct Template {
    pub name: String,
    pub description: String,
}

pub struct Selected {
    pub expr: String,
    pub name: String,
}

impl Selected {
    pub fn new(expr: String, name: String) -> Self {
        Self { expr, name }
    }
}

impl Default for Selected {
    fn default() -> Self {
        Self {
            expr: Default::default(),
            name: Default::default(),
        }
    }
}

#[derive(Default, Props)]
pub struct ListProps<'a> {
    pub repositories: Vec<Repo>,
    pub output: Option<&'a mut Selected>,
}

#[component]
pub fn List<'a>(mut hooks: Hooks, props: &mut ListProps<'a>) -> impl Into<AnyElement<'static>> {
    let mut system = hooks.use_context_mut::<SystemContext>();
    let max_index = props.repositories.iter().flat_map(|r| &r.templates).count() - 1;
    let max_name_length = props
        .repositories
        .iter()
        .flat_map(|r| &r.templates)
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
            let mut acc = 0;
            for repo in &props.repositories {
                for (i, template) in repo.templates.iter().enumerate() {
                    if index.get() == i + acc {
                        **output = Selected::new(repo.expr.clone(), template.name.clone());
                    }
                }
                acc += &props.repositories.len();
            }
        }
        system.exit();
        return element! {View(){}};
    }

    if should_exit.get() {
        system.exit();
        return element! {View(){}};
    }

    let mut acc = 0;
    let mut repos = Vec::<Element<'static, View>>::new();
    for repo in &props.repositories {
        let content = format!("\x1b[93mRepository:\x1b[0m \x1b[1m{}\x1b[0m", repo.expr);
        let rendered_repo = element! {
            View(
            flex_direction: FlexDirection::Column,
            padding_bottom: 1,
            ){
                Text(content: content)
                View(
                flex_direction: FlexDirection::Column,
                ){
                    TemplateList(
                        templates: repo.templates.clone(),
                        index_offset: acc,
                        selected_index: index.get(),
                        max_name_length: max_name_length,
                    )
                }
            }
        };
        repos.push(rendered_repo);
        acc += &repo.templates.len();
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
                            #(repos)
                        }
                    }
                })
            }
        }
    }
}

#[derive(Default, Props)]
struct TemplateListProps {
    templates: Vec<Template>,
    index_offset: usize,
    selected_index: usize,
    max_name_length: usize,
}

#[component]
fn TemplateList(props: &TemplateListProps) -> impl Into<AnyElement<'static>> {
    element! {
        View(
        flex_direction: FlexDirection::Column,
        ){
            #(props.templates.iter().enumerate().map(|(i, template)|
            {
                let is_selected = props.selected_index == (i + props.index_offset);
                let cursor = if is_selected {"> "} else {"  "};
                let content = format!("{}Name: {: <3$} Description: {}", cursor, template.name, template.description, props.max_name_length+1);
                element!{
                    Text(
                        content: content,
                        decoration: if is_selected {TextDecoration::Underline} else{TextDecoration::None})
                }
            }
            ))
        }
    }
}
