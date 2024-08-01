// Copyright 2024 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use risc0_zkvm::{
    ApiClient, Asset, AssetRequest, ProverOpts, ReceiptClaim, Segment, SuccinctReceipt,
};

use crate::task::{Job, JobKind};

#[derive(Default)]
pub struct Worker;

impl workerpool::Worker for Worker {
    type Input = Job;
    type Output = Job;

    fn execute(&mut self, job: Job) -> Job {
        println!("{:?}", job.task);
        let receipt = match job.kind {
            JobKind::Segment(segment) => self.prove_and_lift(segment),
            JobKind::Join(left, right) => self.join(left, right),
            JobKind::Receipt(receipt) => receipt,
        };
        Job {
            task: job.task,
            kind: JobKind::Receipt(receipt),
        }
    }
}

impl Worker {
    fn prove_and_lift(&self, segment: Segment) -> SuccinctReceipt<ReceiptClaim> {
        let opts = ProverOpts::default();
        let client = ApiClient::new().unwrap();
        let segment = bincode::serialize(&segment).unwrap();

        let segment_asset = Asset::Inline(segment.into());
        let segment_receipt = client
            .prove_segment(&opts, segment_asset, AssetRequest::Inline)
            .unwrap();

        let segment_receipt_asset = segment_receipt.try_into().unwrap();
        client
            .lift(&opts, segment_receipt_asset, AssetRequest::Inline)
            .unwrap()
    }

    fn join(
        &self,
        left: SuccinctReceipt<ReceiptClaim>,
        right: SuccinctReceipt<ReceiptClaim>,
    ) -> SuccinctReceipt<ReceiptClaim> {
        let opts = ProverOpts::default();
        let client = ApiClient::new().unwrap();
        let left_asset = left.try_into().unwrap();
        let right_asset = right.try_into().unwrap();
        client
            .join(&opts, left_asset, right_asset, AssetRequest::Inline)
            .unwrap()
    }
}