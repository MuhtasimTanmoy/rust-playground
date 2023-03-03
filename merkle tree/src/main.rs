use merkle_cbt::{merkle_tree::Merge, CBMT as ExCBMT};
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

pub struct DefaultHasherU64;

impl Merge for DefaultHasherU64 {
    type Item = u64;
    fn merge(left: &Self::Item, right: &Self::Item) -> Self::Item {
        let mut hasher = DefaultHasher::new();
        hasher.write_u64(*left);
        hasher.write_u64(*right);
        hasher.finish()
    }
}

type CBMT = ExCBMT<u64, DefaultHasherU64>;

fn main() {
    
    let leaves = vec![
        3584654056691428718,
        42,
        11643453954163878810,
        11177097603989645559,
        20191116,
        10289152030157698709,
    ];

    let root = CBMT::build_merkle_root(&leaves);
    println!("merkle root is {}", root);

    // build merkle proof for 42 (its index is 1);
    let proof = CBMT::build_merkle_proof(&leaves, &[1]).unwrap();
    println!(
        "merkle proof lemmas are {:?}, indices are {:?}",
        proof.lemmas(),
        proof.indices()
    );

    // verify merkle proof
    let verify_result = proof.verify(&root, &[42]);
    println!("merkle proof verify result is {}", verify_result);

    // build merkle proof for 42 and 20191116 (indices are 1 and 4);
    let proof = CBMT::build_merkle_proof(&leaves, &[1, 4]).unwrap();
    println!(
        "merkle proof lemmas are {:?}, indices are {:?}",
        proof.lemmas(),
        proof.indices()
    );

    // retrieve leaves
    let retrieved_leaves = CBMT::retrieve_leaves(&leaves, &proof);
    println!("retrieved leaves are {:?}", retrieved_leaves);
    println!(
        "calculated root of proof is {:?}",
        proof.root(&retrieved_leaves.unwrap())
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

// merkle root is 517427232545752892
// merkle proof lemmas are [3584654056691428718, 6308131355062673692], indices are [6]
// merkle proof verify result is true
// merkle proof lemmas are [10289152030157698709, 3584654056691428718, 12435641680370208774], indices are [6, 9]
// retrieved leaves are Some([42, 20191116])
// calculated root of proof is Some(517427232545752892)