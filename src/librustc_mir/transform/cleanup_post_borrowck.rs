//! This module provides a pass to replacing the following statements with
//! [`Nop`]s
//!
//!   - [`AscribeUserType`]
//!   - [`FakeRead`]
//!   - [`Assign`] statements with a [`Shallow`] borrow
//!
//! The `CleanFakeReadsAndBorrows` "pass" is actually implemented as two
//! traversals (aka visits) of the input MIR. The first traversal,
//! [`DeleteAndRecordFakeReads`], deletes the fake reads and finds the
//! temporaries read by [`ForMatchGuard`] reads, and [`DeleteFakeBorrows`]
//! deletes the initialization of those temporaries.
//!
//! [`AscribeUserType`]: rustc::mir::StatementKind::AscribeUserType
//! [`Shallow`]: rustc::mir::BorrowKind::Shallow
//! [`FakeRead`]: rustc::mir::StatementKind::FakeRead
//! [`Nop`]: rustc::mir::StatementKind::Nop

use rustc::mir::{BasicBlock, BorrowKind, Rvalue, Location, Mir};
use rustc::mir::{Statement, StatementKind};
use rustc::mir::visit::MutVisitor;
use rustc::ty::TyCtxt;
use crate::transform::{MirPass, MirSource};

pub struct CleanupNonCodegenStatements;

pub struct DeleteNonCodegenStatements;

impl MirPass for CleanupNonCodegenStatements {
    fn run_pass<'a, 'tcx>(&self,
                          _tcx: TyCtxt<'a, 'tcx, 'tcx>,
                          _source: MirSource<'tcx>,
                          mir: &mut Mir<'tcx>) {
        let mut delete = DeleteNonCodegenStatements;
        delete.visit_mir(mir);
    }
}

impl<'tcx> MutVisitor<'tcx> for DeleteNonCodegenStatements {
    fn visit_statement(&mut self,
                       block: BasicBlock,
                       statement: &mut Statement<'tcx>,
                       location: Location) {
        match statement.kind {
            StatementKind::AscribeUserType(..)
            | StatementKind::Assign(_, box Rvalue::Ref(_, BorrowKind::Shallow, _))
            | StatementKind::FakeRead(..) => statement.make_nop(),
            _ => (),
        }
        self.super_statement(block, statement, location);
    }
}
