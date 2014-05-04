extern crate collections;
use collections::hashmap::HashMap;
use std::num::Zero;
use std::vec::Vec;
use std::cmp::max;

/**
Common data-type used for cells.

Common Brainfuck implementations only provide 8-bit unsigned
integers with wrap-around overflowing. Note that using this
data-type precludes representing `EOF` as `-1`.
*/
pub type Unit = u8;


/**
Linear, contiguous, sequential-access storage.

This trait models a tape made of a contiguous sequence of cells.
Access is strictly sequential, so the tape must be "wound" until
the desired cell sits under the tape head.

Only one cell can be accessed at a time. Its type should be int-
ish, and can be signed or unsigned.

It is not mandatory for a tape to be infinite, but compliant
implementations must provide at least 30.000 cells from
beginning to end. It is also not necessary for a tape to support
negative indexing.

If the tape is being wound over its bounds, it should stop at
the edge and silently ignore subsequent seeks in that direction.
*/
pub trait Tape<T:Int + Clone> {

	/**
	Winds the tape `offset` cells to the left (-) or to the
	right (+). The tape won't wind over its bounds, if any.
	*/
	fn wind(&mut self, offset: int);

	/**
	Returns a mutable reference to the contents of the
	current cell. Its uninitialized value is zero.
	*/
	fn cell<'a>(&'a mut self) -> &'a mut T;

	/**
	A shortcut for manipulating the cell using a function.
	*/
	fn mutate<'a>(&'a mut self, f: |&'a mut T|) {
		f(self.cell())
	}

}


/**
A straight-forward tape implementation using a vector.

This implementation uses a Vector to provide actual contiguous
indexing and preallocated cells.

The tape is bounded on the left, but unbounded on the right as
it has the ability to grow on necessity. It comes pre-grown with
30.000 cells, initialized to zero.
*/
pub struct VectorTape<T> {
	/// The actual underlying vector.
	storage: Vec<T>,
	/// Keeps track of the reading head.
	cur: int,
}

impl<T:Int> VectorTape<T> {
	/**
	Produces a new, empty tape.
	Comes pre-grown with 30.000 zero-ed cells.
	*/
	pub fn new() -> VectorTape<T> {
		VectorTape {
			// Make space for at least 30k zero cells.
			storage: Vec::from_elem(30000, Zero::zero()),
			cur: 0,
		}
	}
}

impl<T:Int> Tape<T> for VectorTape<T> {
	/**
	Seeks the tape `offset` cells to the left (-) or to the
	right (+). Tape is bounded on the left, so it's not possible
	to seek before that.
	*/
	fn wind(&mut self, offset: int) {
		self.cur = max(0, self.cur + offset)
	}
	/**
	Returns a mutable reference to the contents of a cell.
	*/
	fn cell<'a>(&'a mut self) -> &'a mut T {
		self.storage.get_mut( self.cur.to_uint().unwrap_or( Zero::zero() ))
	}
}

#[test]
/// Cells should initialize at zero.
fn test_vector_tape_empty_cell_is_zero() {
	let mut t : VectorTape<Unit> = VectorTape::new();
	assert!(*t.cell() == 0);
}

#[test]
/// The tape should keep values as written, on separate cells.
fn test_vector_tape_keeps_values_correctly() {
	let mut t : VectorTape<Unit> = VectorTape::new();
	// Mutate this cell and read it back.
	t.mutate( |v|{ *v = 1; } );
	assert!(*t.cell() == 1);
	// Check another cell, ensure it's different.
	t.wind(1);
	assert!(*t.cell() != 1);
	// Mutate that other cell.
	t.mutate( |v|{ *v = 2; } );
	assert!(*t.cell() == 2);
	// Do a last check to ensure more the other cell is still okay.
	t.wind(-1);
	assert!(*t.cell() == 1);
}

#[test]
/// The tape should have at least 30k cells.
fn test_vector_tape_has_at_least_30k_cells() {
	let mut t : VectorTape<Unit> = VectorTape::new();
	for _ in range(0, 30000) {
		// If we find a cell that is not zero, we weren't able
		// to advance the tape in the last iteration.
		assert!(*t.cell() == 0);
		// Write and advance.
		t.mutate( |v|{ *v += 1; } );
		t.wind(1);
	}
}


/**
A tape which provides sparse allocation.

This implementation uses a HashMap underneath to provide sparse
allocation. Cells are created on first access.

The tape is unbounded on both directions, and can be safely
considered infinite, though it will actually contain at most
`MAX_INT` cells, centered around zero.
*/
pub struct SparseTape<T> {
	/// The actual underlying hashmap.
	storage: HashMap<int, T>,
	/// Keeps track of the reading head.
	cur: int,
}

impl<T> SparseTape<T> {
	/**
	Produces a new, empty tape.
	Cells will be lazily initialized to zero on access.
	*/
	pub fn new() -> SparseTape<T> {
		SparseTape {
			storage: HashMap::new(),
			cur: 0,
		}
	}
}

impl<T:Int> Tape<T> for SparseTape<T> {
	/**
	Seeks the tape `offset` cells to the left (-) or to the
	right (+). The tape is unbounded in both direction, so
	winding never "fails".
	*/
	fn wind(&mut self, offset: int) {
		self.cur += offset;
	}
	/**
	Returns a mutable reference to the contents of a cell. The cell is
	created on first access, and its uninitialized value is zero.
	*/
	fn cell<'a>(&'a mut self) -> &'a mut T {
		self.storage.find_or_insert(self.cur, Zero::zero())
	}
}

#[test]
/// Cells should initialize at zero.
fn test_sparse_tape_empty_cell_is_zero() {
	let mut t : SparseTape<Unit> = SparseTape::new();
	assert!(*t.cell() == 0);
}

#[test]
/// The tape should keep values as written, on separate cells.
fn test_sparse_tape_keeps_values_correctly() {
	let mut t : SparseTape<Unit> = SparseTape::new();
	// Mutate this cell and read it back.
	t.mutate( |v|{ *v = 1; } );
	assert!(*t.cell() == 1);
	// Check another cell, ensure it's different.
	t.wind(1);
	assert!(*t.cell() != 1);
	// Mutate that other cell.
	t.mutate( |v|{ *v = 2; } );
	assert!(*t.cell() == 2);
	// Do a last check to ensure more the other cell is still okay.
	t.wind(-1);
	assert!(*t.cell() == 1);
}

#[test]
/// The tape should have at least 30k cells.
fn test_sparse_tape_has_at_least_30k_cells() {
	let mut t : SparseTape<Unit> = SparseTape::new();
	for _ in range(0, 30000) {
		// If we find a cell that is not zero, we weren't able
		// to advance the tape in the last iteration.
		assert!(*t.cell() == 0);
		// Write and advance.
		t.mutate( |v|{ *v += 1; } );
		t.wind(1);
	}
}
