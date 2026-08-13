pub fn build_tree<'cfg, 'res>(templates: &'cfg [crate::domain::Template]) -> TreeNode<'res>
where
    'cfg: 'res
{
    let mut root = TreeNode {
        name:     "root".into(),
        template: None,
        children: Vec::new(),
    };

    for template in templates {
        let path = template.get_groups();
        let mut current = &mut root;

        for segment in path {
            let pos = current.children.iter().position(|c| c.template.is_none() && c.name == *segment);
            match pos {
                Some(idx) => {
                    current = &mut current.children[idx];
                },
                None => {
                    let new_group = TreeNode {
                        name: segment.clone(),
                        template: None,
                        children: Vec::new(),
                    };
                    current.children.push(new_group);
                    let last = current.children.len() - 1;
                    current = &mut current.children[last];
                },
            }
        }

        let leaf = TreeNode {
            name: template.get_name().into(),
            template: Some(template),
            children: Vec::new(),
        };
        current.children.push(leaf);
    }

    root
}

pub struct TreeNode<'cfg> {
    name:     String,
    template: Option<&'cfg crate::domain::Template>,
    children: Vec<TreeNode<'cfg>>,
}

impl<'cfg> TreeNode<'cfg> {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_template(&self) -> Option<&'cfg crate::domain::Template> {
        self.template.clone()
    }

    pub fn get_children(&self) -> &[TreeNode<'cfg>] {
        &self.children
    }
}
