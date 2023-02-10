// BST application  

// Project 8., Feature 1
// BST and vec : Retrieving the oder of the elemnets in a BST 



// code for the library 

use std::rc::Rc;
use std::cell::RefCell;
#[derive(Debug, Default, PartialEq, Eq,Clone)]
pub struct Node {
  pub val: String,
  pub left: Link,
  pub right: Link
}

type Link = Option<Rc<RefCell<Node>>>;
impl Node {
  pub fn new(val: String) ->Self { 
    Node {
      val,
      left: None,
      right: None
    }
  }
  pub fn insert(&mut self, val: String) {
		if val > self.val {
			match &self.right {
				None => self.right = Some(Rc::new(RefCell::new(Node {val:val.to_string(), left:None, right:None}))),   // if right is null then we just insert the value
				Some(node) => node.borrow_mut().insert(val.to_string())                         // if right not null then we recursively call the function again and again until we insert
			}
		}
		else {
			match &self.left {
				None => self.left = Some(Rc::new(RefCell::new(Node {val:val.to_string(), left:None, right:None}))),
				Some(node) => node.borrow_mut().insert(val.to_string())
			}
		}
	}
}
#[derive(Debug,Default, PartialEq, Eq)]
pub struct BinarySearchTree{
     pub root: Node
}
impl  BinarySearchTree{
  pub fn new(val: String) ->Self { 
    BinarySearchTree {
       root: Node::new(val.to_string())
    }
  }
  pub fn insert(&mut self, val: String) {
    self.root.insert(val.to_string());
	}
}

pub struct DisplayLobby {
    stack: Vec<Rc<RefCell<Node>>>,
}

impl DisplayLobby {
    pub fn new(root: Option<Rc<RefCell<Node>>>) -> Self {
        let mut stack = Vec::new();
        Self::push_all(root.clone(), &mut stack); // we do not want to consume the root so therefore call clone on
        DisplayLobby { stack }
    }

    fn push_all(mut p: Option<Rc<RefCell<Node>>>, stack: &mut Vec<Rc<RefCell<Node>>>) {
        while let Some(link) = p.clone() {
            stack.push(p.clone().unwrap());
            p = link.borrow().left.clone();
        }
    }

    pub fn next(&mut self) -> String {
        let node = self.stack.pop().unwrap();
        let res = &node.borrow().val; // we do not want to move the string outside so therefore & is added before the node which changes the return type to &String 
        let mut next = node.borrow().right.clone(); 

        // if there is something on the right then we will to the right and then grab all the left elements from there and they will be pushed onto the stack
        while let Some(inner) = next.clone() { 
            self.stack.push(inner.clone());
            next = next.unwrap().borrow().left.clone();
        }
        res.to_string()
    }

    pub fn next_page(&mut self)-> Vec<String>{
            let mut res: Vec<String> = Vec::new();
            for i in 0..10{
                if self.has_next(){
                    res.push(self.next());
                }
                else
                    {break;}
            }
            res
        }

    pub fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }
}


fn main() {
    // Driver code
    let mut bst = BinarySearchTree::new("Jeanette".to_string());
    let names: Vec<String> = vec!["Latasha", "Elvira", "Caryl", "Antoinette", "Cassie", "Charity", "Lyn", "Elia", "Anya", "Albert", "Cherlyn", "Lala", "Kandice", "Iliana"].into_iter().map(String::from).collect();
    for name in names.into_iter(){
        bst.insert(name.to_string());
    }

    let mut res: Vec<String> = Vec::new();

    let mut display:  DisplayLobby =  DisplayLobby::new(Some(Rc::new(RefCell::new(bst.root))));
    res = display.next_page();
    println!("{:?}",res);

}


