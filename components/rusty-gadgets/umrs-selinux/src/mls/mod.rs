// =============================================================================
// UMRS `SELinux`  Modeling Library
// =============================================================================
//
// Module: mls
//
// Author: Jamie Adams
// License: MIT
//
// Description:
//   Multi-Level Security (MLS) composition structures for `SELinux` 
//   label modeling.
//
// =============================================================================

/*!
===============================================================================
MLS Namespace — Architectural Overview
===============================================================================

This module contains composite structures modeling `SELinux`  Multi-Level
Security (MLS) labels and clearance ranges.

MLS extends the `SELinux`  security context by introducing hierarchical
classification and compartmentalization semantics.

-------------------------------------------------------------------------------
Security Label Composition
-------------------------------------------------------------------------------

An MLS level is composed of:

  `SensitivityLevel` : `CategorySet`

Example:

  s3:c0,c2,c9

Where:

  • `SensitivityLevel` defines hierarchical classification.
  • `CategorySet` defines compartment membership.

-------------------------------------------------------------------------------
Module Responsibilities
-------------------------------------------------------------------------------

This namespace defines:

  • MLS Level structures
  • Clearance ranges
  • Dominance mathematics
  • Range containment logic

Primitive label components such as `SensitivityLevel` and `CategorySet` are
defined at the crate root and consumed by this module.

-------------------------------------------------------------------------------
Implementation Lineage
-------------------------------------------------------------------------------

Conceptual lineage was studied from:

  • `SELinux`  kernel MLS subsystem
  • policydb sensitivity handling
  • mls.c dominance logic

No kernel or `SELinux`  userland source code has been copied or translated.

All implementations are original Rust constructions aligned at the
semantic level only.

===============================================================================
*/

pub mod level;
pub mod range;

