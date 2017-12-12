pub struct Tree<'a> {
    node: Link<'a>
}

type Link<'a> = Option<Box<Node<'a>>>;

struct Node<'a> {
    value: &'a str,
    left: Link<'a>,
    right: Link<'a>,
}

impl<'a> Tree<'a> {
    pub fn new() -> Self {
        Tree { node : None }
    }

    pub fn add(&mut self, value: &'a str) {
        let new_node = Box::new(Node {
            value: value,
            left: None,
            right: None,
        });

        match self.node {
            Some(ref mut node) => Tree::insert(node, new_node),
            None => self.node = Some(new_node)
        }
    }

    pub fn display(&self) {
        self.node.as_ref().map(|node| Tree::display_node(node, 0));
    }

    fn insert(head: &mut Node<'a>, new_node : Box<Node<'a>>) {
        if head.value > new_node.value {
            match head.left {
                Some(ref mut node) => Tree::insert(&mut *node, new_node),
                None => head.left = Some(new_node),
            };
        } else {
            match head.right {
                Some(ref mut node) => Tree::insert(&mut *node, new_node),
                None => head.right = Some(new_node),
            };
        }
    }

    fn display_node(node : &Node, depth: i32) {
        for _ in 0..depth {
            print!(" ");
        }

        println!("{}", node.value);

        node.left.as_ref().map(|n| Tree::display_node(n, depth + 2));
        node.right.as_ref().map(|n| Tree::display_node(n, depth + 2));
    }
}


#[cfg(test)]
mod test {
    use super::Tree;

    #[test]
    fn treeee_test() {
        let mut t = Tree::new();

        t.add("b");
        t.add("a");
        t.add("c");
        t.add("d");

        t.display();
    }

}
