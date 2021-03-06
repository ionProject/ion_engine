/*===============================================================================================*/
// Copyright 2016 Kyle Finlay
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
/*===============================================================================================*/

/*===============================================================================================*/
//! This crate is the core of ionEngine.
//!
//! It provides an easy to use framework for creating 2D / 3D games and multimedia applications.
/*===============================================================================================*/

// Crate attributes
#![deny (missing_copy_implementations)]
#![deny (missing_docs)]

#![feature (custom_derive)]
#![feature (plugin)]

#![plugin (serde_macros)]

#[macro_use]
extern crate log;

// Modules
pub mod engine;
pub mod resource;
pub mod util;
//pub mod window;
pub mod renderer;
