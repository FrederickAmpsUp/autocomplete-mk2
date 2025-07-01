mod tree;
mod ui;

fn train_on_text(tree: &mut tree::Tree, text: &str, max_len: usize) {
    let chars: Vec<char> = text.chars().collect();
    for i in 0..chars.len() {
        for len in 1..=max_len {
            if i + len < chars.len() {
                let seq: String = chars[i..i+len].iter().collect();
                tree.insert(&seq);
            }
        }
    }
}


fn main() {
    let mut tree = tree::Tree::new();
    let text = std::fs::read_to_string("2600-0.txt").unwrap();
    train_on_text(&mut tree, &text, 12);

    ui::run_ui(&tree).unwrap();
}
