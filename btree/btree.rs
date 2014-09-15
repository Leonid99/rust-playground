extern crate debug;

struct Node {
    data: int,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(data: int) -> Node {
        Node {data: data, left: None, right: None}
    }

    fn _add(&self, data: int) {

    	let subtree = if data > self.data {
    		self.right
    	} else {
    		self.left
    	};

    }
}

struct BTree {
    root:Node,
}

impl BTree {
    fn new(data: int) -> BTree {
        BTree { root: Node::new(data) }
    }

    fn add(&self, data:int) {

    	self.root._add(data);

        println!("{:?}", self);
    }
}

fn main()
{
    let b:BTree = BTree::new(3);

    b.add(4)
}
