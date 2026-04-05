const T: usize = 3;
#[derive(Debug)]
struct BTreeNode {
    keys: Vec<i32>,
    children: Vec<Box<BTreeNode>>,
    is_leaf: bool,
}

impl BTreeNode {
    fn search(&self, key: &i32) -> bool {
        let i = self.keys.partition_point(|&x| x < *key);

        if i < self.keys.len() && self.keys[i] == *key {
            return true;
        }

        if self.is_leaf {
            return false;
        }
        self.children[i].search(key)
    }
}

fn main() {
    let a = BTreeNode {
        keys: vec![1, 2, 3],
        children: vec![], // Empty for leaf node
        is_leaf: true,
    };

    println!("a = {:?}", a);
    println!("search 2: {}", a.search(&2));
    println!("search 5: {}", a.search(&5));
}