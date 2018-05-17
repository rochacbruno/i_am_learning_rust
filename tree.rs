#[derive(Default, Debug)]
struct Tree {
    root: i64,
    left: Option<Box<Tree>>,
    right: Option<Box<Tree>>,
}

impl From<i64> for Tree {
    fn from(val: i64) -> Self {
        Tree::new(val)
    }
}

impl Tree {
    fn new(root: i64) -> Tree {
        Tree {
            root: root,
            ..Default::default()
        }
    }

    fn left<T>(mut self, leaf: T) -> Self
    where
        T: Into<Tree>,
    {
        self.left = Some(Box::new(leaf.into()));
        self
    }

    fn right<T>(mut self, leaf: T) -> Self
    where
        T: Into<Tree>,
    {
        self.right = Some(Box::new(leaf.into()));
        self
    }
}

fn root(val: i64) -> Tree {
    Tree::new(val)
}

fn main() {
    let t = root(15)
                .left(root(12).right(13))
                .right(root(22).left(18).right(100));
    println!("{:?}", t);
}
