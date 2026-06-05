//! # read
//!
//! This module provides functions for writing networks to files for storage
//!
//! ## Invariants
//!
//! - Files must follow this format: (metadata,[in_entity_id_0,...,],[out_entity_id_0,...,],[[entity_0_sink_0,...,],...,],[(relation data,in_entity_a,in_entity_b,out_entity,),...,],)
//!
//! Author: Cole Francis
//! Last Updated: 06/02/2026