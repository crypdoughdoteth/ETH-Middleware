use std::{collections::HashMap, io::{BufWriter, Write, BufReader, Read}, fs::{File, self}};
use serde::{Serialize, Deserialize};
use tiny_keccak::{Keccak, Hasher};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tree{
    root: Node,
    neighbors: HashMap<[u8;32], [u8;32]>,
    leaf_nodes: Vec<Leaf>,
    depth: u32,
}

impl Tree{
    pub(crate) fn new(mut self, leafs: Vec<Leaf>) -> Tree{
        //returns new merkle tree constructed from leafs: Vec<Leaf>
        // ---------------------------------------
        //creates initial nodes for merkle tree
        //iterate through leafs
        //on each iteration, hash the data contained in the leaf with Keccak256
        //append new Node to children_nodes  
        self.leaf_nodes = leafs.clone();
        let mut nodes: Vec<Node> = Vec::new();
        let mut parent_nodes: Vec<Node> = Vec::new();
        let mut children_nodes: Vec<Node> = leafs.iter().map(|x|{
           let x_hashed = Leaf::hash_leaf(x.data.as_bytes());
           println!("child hash: {:?}", &x_hashed);
            let y = Node::new(x_hashed);
            nodes.push(y.clone());
            y
        }).collect();
        //check length of leaf nodes, if there is an odd number of leaf nodes, copy the odd element and push it to the array
        let len = &children_nodes.len();
        if len % 2 == 1 { 
            children_nodes.push(children_nodes[len -1].clone()); 
            println!("inserted child hash: {:?}", &children_nodes[len-1].hash);
        }

        let mut n = children_nodes.len();
        self.depth = 1; 
        //main loop, make sure there is at least two elemnts left, otherwise we've hit the root
        while n > 1{
            let mut i = 0;
            //iterates over children_nodes to create new parent nodes
            while i < n - 1 {
                //link neighbors using hashmap, allows us to reconstruct the tree later with K:V pairs
                let node_one = children_nodes[i].hash.clone();
                let node_two = children_nodes[i+ 1].hash.clone();
                self.neighbors.insert(node_one, node_two);
                self.neighbors.insert(node_two, node_one);
                //hash nodes together
                let new_node_hash = Node::hash_nodes([&node_one, &node_two]);

                println!("node hash: {:?}", &new_node_hash);
                let new_node =                    
                    Node{ 

                        hash: new_node_hash,
                    };
                parent_nodes.push(new_node.clone());
                nodes.push(new_node);
                


                //println!("node child left: {:#?}", &new_node.left_child);
                //println!("node child right: {:#?}", &new_node.right_child);
                
                //increment loop by 2
                i += 2;
            }
            //change level
            self.depth += 1;
            children_nodes = parent_nodes;
            parent_nodes = Vec::new();
            //divide # of starting nodes by 2, represents remaining population of nodes that can be hashed together
            n = n / 2;
            //if there is an odd number of nodes excluding the root, insert copy of the [len-1] node
            let length : usize = nodes.len();
            if n > 1 && n % 2 == 1{
                children_nodes.push(children_nodes[children_nodes.len() -1].clone());
                nodes.push(children_nodes[children_nodes.len() -1].clone());
                self.neighbors.insert(children_nodes[children_nodes.len() -1].hash, children_nodes[children_nodes.len() - 2].hash);
                self.neighbors.insert(children_nodes[children_nodes.len() -2].hash, children_nodes[children_nodes.len() - 1].hash);
                println!("inserted node hash: {:#?}", nodes[length-1].hash);
                n += 1;
            }
            if n == 1{
                self.root = Node{                        

                    hash: nodes[length - 1].hash,
                };
                println!("root: {:#?}", self.root.hash);
            }
        }
        self
    }
    pub(crate) fn spawn() -> Tree{
        Tree{
            root: Node::new(Leaf::hash_leaf("0x00000000000000000000000000000000".as_bytes())),
            neighbors: HashMap::new(),
            leaf_nodes: Vec::new(),
            depth: 0,
        }
    }

    pub(crate) fn get_element_count(self) -> usize{
        let base: usize = 2;
        base.pow(self.depth)
    }
    pub(crate) fn get_root(self) -> [u8;32]{
        self.root.hash
    }

    //use hashmap to locate neighbored elements and generate parent node hashes
    pub(crate) fn generate_proof(self, leaf: &[u8;32], index: usize) -> Option<Vec<[u8;32]>> {

        //strat 2.0: the leaf hash we receive MUST be located in the HashMap, else throw error, invalid leaf (or end of set).
        //retrieve the corresponding VALUE inside of the HashMap for leaf's hash
        //hash together the resulting hashes
        //lookup result and repeat process
        let mut idx = index; 
        let mut proof_hashes: Vec<[u8;32]> = vec![];
        let mut current_hash = leaf;
        let mut parent_node_hash:[u8;32];
        loop{
            let neighbor_hash = self.neighbors.get(current_hash);
            match neighbor_hash{
                Some(hash) => {
                    if idx % 2 == 0 {
                        parent_node_hash = Node::hash_nodes([current_hash, hash]);
                    }
                    else{
                        parent_node_hash = Node::hash_nodes([hash, current_hash]);
                    }
                    proof_hashes.push(*hash);
                    current_hash = &parent_node_hash;
                    idx /= 2;
                },
                //if we cannot locate a value in the mapping given the key, break the loop and return the vector of hashes
                None => break,
            }           
        }
       
        if proof_hashes.len() == 0 {
           return None;
        }
        else{
            Some(proof_hashes)
        }

    }

    pub(crate) fn write_tree(self: Self, path: &str) -> Result<(), std::io::Error>{
        
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        let json = serde_json::to_string(&self)?;
        serde_json::to_writer(&mut writer, &json)?;
        writer.flush()?;
        Ok(())
    }
    pub(crate) fn overwrite_tree(self: Self, path: &str) -> Result<(), std::io::Error>{
        let file = File::open(path)?;
        file.set_len(0)?;
        let mut writer = BufWriter::new(file);
        let json = serde_json::to_string(&self)?;
        serde_json::to_writer(&mut writer, &json)?;
        writer.flush()?;
        Ok(())
    }
    pub(crate) fn read_tree(path : &str) -> Result<Tree, std::io::Error>{
        let file = File::open(path)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        let _ = buf_reader.read_to_string(&mut contents);
        let res: Tree = serde_json::from_str(&contents)?;
        Ok(res)
    }

}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Node{

    hash: [u8;32],
}

impl Node{
    fn new(h: [u8;32]) -> Node{
        Self{hash: h}
    }

    fn hash_nodes(input: [&[u8;32];2]) -> [u8;32] {
        let mut hasher = Keccak::v256();
        let mut output = [0u8; 32];
        hasher.update(input[0]);
        hasher.update(input[1]);
        hasher.finalize(&mut output);
        output
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]

pub struct Leaf{
    data: String,
}
impl Leaf{
    pub(crate) fn new(leaf_data: String) -> Self{
        let x = leaf_data.as_bytes();
        Leaf::hash_leaf(x);
        Self{data: leaf_data}
    }
    
    pub(crate) fn hash_leaf(input: &[u8]) -> [u8;32] {
        let mut hasher = Keccak::v256();
        let mut output = [0u8; 32];
        hasher.update(input);
        hasher.finalize(&mut output);
        output
    }
}   

fn verify_proof(merkle_root: [u8;32], leaf: Leaf, hashes: Vec<[u8;32]>, index: usize) -> bool{
    let mut counter = 0;
    let mut idx = index;
    let mut hash = Leaf::hash_leaf(leaf.data.as_bytes());
    let mut verified = false;
    loop{
        let proof_element = hashes[counter];
        
        if idx % 2 == 0 {
            hash = Node::hash_nodes([&hash, &proof_element]);
        }
        else{
            hash = Node::hash_nodes([&proof_element, &hash]);
        }
        println!("Verifying...{:?}", &hash);

        if hash == merkle_root{
           verified = true;
           break
        }
        if counter == hashes.len() - 1{
            break;
        }
        counter += 1;
        idx /= 2;
    }

    verified

}