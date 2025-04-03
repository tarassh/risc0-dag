use dag_core::{Inputs, Outputs};
use dag_methods::{IPFS_DAG_ELF, IPFS_DAG_ID};
use risc0_zkvm::{ExecutorEnv, ExecutorImpl, NullSegmentRef, Receipt, InnerReceipt};

mod plan;
mod task;
mod worker;

use plan::Planner;
use task::TaskManager;

fn main() {
    let data = include_str!("../res/dag.json");
    let inputs = Inputs {
        data: data.to_string(),
        root_cid: "bafybeib5ouhpwnzbzdtgfobnyv4kuj4moofw3kngqu2svgdm4ttpe56lpe".to_string(),
    };
    let outputs = custom_exec(&inputs);
    println!();
    println!("  {:?}", outputs.hash);
}

fn custom_exec(inputs: &Inputs) -> Outputs {
    let mut planner = Planner::default();
    let mut task_manager = TaskManager::new();

    let env = ExecutorEnv::builder()
        .write(&inputs)
        .unwrap()
        .segment_limit_po2(16)
        .build()
        .unwrap();

    let mut exec = ExecutorImpl::from_elf(env, IPFS_DAG_ELF).unwrap();
    let session = exec
        .run_with_callback(|segment| {
            planner.enqueue_segment(segment.index).unwrap();
            task_manager.add_segment(segment);
            while let Some(task) = planner.next_task() {
                task_manager.add_task(task.clone());
            }
            Ok(Box::new(NullSegmentRef))
        })
        .unwrap();

    planner.finish().unwrap();

    println!("Plan:");
    println!("{planner:?}");

    while let Some(task) = planner.next_task() {
        task_manager.add_task(task.clone());
    }

    let root_receipt = task_manager.run();
    let receipt = Receipt::new(
        InnerReceipt::Succinct(root_receipt),
        session.journal.unwrap().bytes.clone(),
    );
    receipt.verify(IPFS_DAG_ID).unwrap();
    println!("Receipt verified!");

    receipt.journal.decode().unwrap()
}

#[cfg(test)]
mod tests {
    use hex::FromHex;
    use risc0_zkvm::sha::Digest;

    #[test]
    fn main() {
        let data = include_str!("../res/dag.json");
        let inputs = dag_core::Inputs {
            data: data.to_string(),
            root_cid: "bafybeib5ouhpwnzbzdtgfobnyv4kuj4moofw3kngqu2svgdm4ttpe56lpe".to_string(),
        };
        let outputs = super::custom_exec(&inputs);
        assert_eq!(
            outputs.hash,
            Digest::from_hex("3d750efb3721c8e662b82dc578aa278c738b6da9a685352a986ce4e6f277cb79")
                .unwrap(),
            "Did not produce the expected hash."
        );
    }
}
