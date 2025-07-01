pub struct TreeNode {
    children: Vec<Box<TreeNode>>,
    pub character: char,
    pub frequency: u32
}

impl TreeNode {
    pub fn prune(&mut self) {
    }

    pub fn find_char(&self, character: char) -> Option<&TreeNode> {
        if let Some(index) = self.children.iter().position(|c| c.character == character) {
            return Some(self.children[index].as_ref());
        }
        return None;
    }

    pub fn find_or_insert_char(&mut self, character: char) -> &mut TreeNode {
        if let Some(index) = self.children.iter().position(|c| c.character == character) {
            return self.children[index].as_mut();
        }

        self.children.push(Box::new(TreeNode {
            children: vec![],
            character,
            frequency: 0
        }));

        return self.children.last_mut().unwrap().as_mut();
    }

    fn collect_suffixes(&self, prefix: String, out: &mut Vec<(String, u32)>) {
        if self.children.is_empty() {
            out.push((prefix.clone(), self.frequency));
            return;
        }
        for child in &self.children {
            let mut new_prefix = prefix.clone();
            new_prefix.push(child.character);
            child.collect_suffixes(new_prefix, out);
        }
    }
}

pub struct Tree {
    root: Box<TreeNode>
}

impl Tree {
    pub fn new() -> Self {
        Self {
            root: Box::new(TreeNode {
                children: vec![],
                character: '\0',
                frequency: 0
            })
        }
    }

    pub fn insert(&mut self, sequence: &str) {
        let mut node = self.root.as_mut();

        for character in sequence.chars() {
            node = node.find_or_insert_char(character);
            node.frequency += 1;
        }
    }

    pub fn query(&self, sequence: &str) -> Vec<(char, u32)> {
        let mut node = self.root.as_ref();

        for character in sequence.chars() {
            let node_res = node.find_char(character);
            if node_res.is_none() {
                return vec![];
            }
            node = node_res.unwrap();
        }

        return node.children.iter().map(|c| (c.character, c.frequency)).collect();
    }

    pub fn query_suffixes(&self, prefix: &str) -> Vec<(String, u32)> {
        let mut node = self.root.as_ref();

        for ch in prefix.chars() {
            match node.find_char(ch) {
                Some(n) => node = n,
                None => return vec![],
            }
        }

        let mut out = Vec::new();
        node.collect_suffixes(String::new(), &mut out);
        out
    }
}
