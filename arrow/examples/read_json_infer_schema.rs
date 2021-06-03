// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

extern crate arrow;

#[cfg(feature = "csv")]
use arrow::json;
#[cfg(feature = "prettyprint")]
use arrow::util::pretty::print_batches;
use std::fs::File;
use std::io::{BufReader, SeekFrom, Seek};
use arrow::json::reader::infer_json_schema;
use std::sync::Arc;

fn main() {
    #[cfg(feature = "csv")]
    {
        let mut file = File::open("test/data/basic.json").unwrap();
        let mut reader = BufReader::new(BufReader::new(&file));
        let inferred_schema = infer_json_schema(&mut reader, None).unwrap();
        file.seek(SeekFrom::Start(0)).unwrap();

        let mut json = json::Reader::new(BufReader::new(file), Arc::new(inferred_schema), 1024, None);
        let _batch = json.next().unwrap().unwrap();
        #[cfg(feature = "prettyprint")]
        {
            print_batches(&[_batch]).unwrap();
        }
    }
}
