use crate::*;

#[test]
fn dynamic_memory() {
    let locals = [<*const i32>::get_type(), <i32>::get_type()];
    let n = const_int::<usize>(4);
    let b0 = block!(storage_live(0), storage_live(1), allocate(n, n, local(0), 1)); // alloc ptr
    let b1 = block!(
        assign( // write to ptr
            deref(load(local(0)), <i32>::get_type()),
            const_int::<i32>(42),
        ),
        assign( // read from ptr
            local(1),
            load(deref(load(local(0)), <i32>::get_type())),
        ),
        deallocate(load(local(0)), n, n, 2)
    );
    let b2 = block!(exit());
    let f = function(Ret::No, 0, &locals, &[b0, b1, b2]);
    let p = program(&[f]);
    assert_stop(p);
}

#[test]
fn alloc_argcount() {
    let locals = [ <*const i32>::get_type() ];

    let b0 = block!(
        storage_live(0),
        Terminator::CallIntrinsic {
            intrinsic: Intrinsic::Allocate,
            arguments: list![],
            ret: local(0),
            next_block: None,
        },
    );

    let f = function(Ret::No, 0, &locals, &[b0]);
    let p = program(&[f]);
    dump_program(p);
    assert_ub(p, "invalid number of arguments for `Intrinsic::Allocate`");
}

#[test]
fn alloc_align_err() {
    let locals = [ <*const i32>::get_type() ];

    let b0 = block!(
        storage_live(0),
        Terminator::CallIntrinsic {
            intrinsic: Intrinsic::Allocate,
            arguments: list![const_int::<usize>(4), const_int::<usize>(13)], // 13 is no power of two! hence error!
            ret: local(0),
            next_block: Some(BbName(Name::from_internal(1))),
        },
    );
    let b1 = block!(exit());

    let f = function(Ret::No, 0, &locals, &[b0, b1]);
    let p = program(&[f]);
    dump_program(p);
    assert_ub(p, "invalid alignment for `Intrinsic::Allocate`: not a power of 2");
}

#[test]
fn alloc_size_err() {
    let locals = [ <*const i32>::get_type() ];

    let b0 = block!(
        storage_live(0),
        Terminator::CallIntrinsic {
            intrinsic: Intrinsic::Allocate,
            arguments: list![const_int::<isize>(-1), const_int::<usize>(4)], // -1 is not a valid size!
            ret: local(0),
            next_block: Some(BbName(Name::from_internal(1))),
        },
    );
    let b1 = block!(exit());

    let f = function(Ret::No, 0, &locals, &[b0, b1]);
    let p = program(&[f]);
    dump_program(p);
    assert_ub(p, "invalid size for `Intrinsic::Allocate`: negative size");
}

#[test]
fn alloc_wrongarg1() {
    let locals = [ <*const i32>::get_type() ];

    let b0 = block!(
        storage_live(0),
        Terminator::CallIntrinsic {
            intrinsic: Intrinsic::Allocate,
            // First argument should be an int, so bool is unexpected here!
            arguments: list![const_bool(true), const_int::<usize>(4)],
            ret: local(0),
            next_block: Some(BbName(Name::from_internal(1))),
        },
    );
    let b1 = block!(exit());

    let f = function(Ret::No, 0, &locals, &[b0, b1]);
    let p = program(&[f]);
    dump_program(p);
    assert_ub(p, "invalid first argument to `Intrinsic::Allocate`, not an integer");
}

#[test]
fn alloc_wrongarg2() {
    let locals = [ <*const i32>::get_type() ];

    let b0 = block!(
        storage_live(0),
        Terminator::CallIntrinsic {
            intrinsic: Intrinsic::Allocate,
            // Second argument should be an int, so bool is unexpected here!
            arguments: list![const_int::<usize>(4), const_bool(true)],
            ret: local(0),
            next_block: Some(BbName(Name::from_internal(1))),
        },
    );
    let b1 = block!(exit());

    let f = function(Ret::No, 0, &locals, &[b0, b1]);
    let p = program(&[f]);
    dump_program(p);
    assert_ub(p, "invalid second argument to `Intrinsic::Allocate`, not an integer");
}

#[test]
fn alloc_wrongreturn() {
    let locals = [ <()>::get_type() ];

    let b0 = block!(
        storage_live(0),
        Terminator::CallIntrinsic {
            intrinsic: Intrinsic::Allocate,
            arguments: list![const_int::<usize>(4), const_int::<usize>(4)],
            ret: local(0),
            next_block: Some(BbName(Name::from_internal(1))),
        },
    );
    let b1 = block!(exit());

    let f = function(Ret::No, 0, &locals, &[b0, b1]);
    let p = program(&[f]);
    dump_program(p);
    assert_ub(p, "invalid return type for `Intrinsic::Allocate`");
}

#[test]
fn dealloc_success() {
    let locals = [ <*const i32>::get_type() ];

    let b0 = block!(
        storage_live(0),
        allocate(const_int::<usize>(4), const_int::<usize>(4), local(0), 1)
    );
    let b1 = block!(
        Terminator::CallIntrinsic {
            intrinsic: Intrinsic::Deallocate,
            arguments: list![load(local(0)), const_int::<usize>(4), const_int::<usize>(4)],
            ret: zst_place(),
            next_block: Some(BbName(Name::from_internal(2))),
        },
    );
    let b2 = block!(exit());

    let f = function(Ret::No, 0, &locals, &[b0, b1, b2]);
    let p = program(&[f]);
    dump_program(p);
    assert_stop(p);
}

#[test]
fn dealloc_argcount() {
    let locals = [];

    let b0 = block!(
        Terminator::CallIntrinsic {
            intrinsic: Intrinsic::Deallocate,
            arguments: list![],
            ret: zst_place(),
            next_block: Some(BbName(Name::from_internal(1))),
        },
    );
    let b1 = block!(exit());

    let f = function(Ret::No, 0, &locals, &[b0, b1]);
    let p = program(&[f]);
    dump_program(p);
    assert_ub(p, "invalid number of arguments for `Intrinsic::Deallocate`");
}

#[test]
fn dealloc_align_err() {
    let locals = [ <*const i32>::get_type() ];

    let b0 = block!(
        storage_live(0),
        allocate(const_int::<usize>(4), const_int::<usize>(4), local(0), 1)
    );
    let b1 = block!(
        Terminator::CallIntrinsic {
            intrinsic: Intrinsic::Deallocate,
            arguments: list![load(local(0)), const_int::<usize>(4), const_int::<usize>(13)], // 13 is not a power of two!
            ret: zst_place(),
            next_block: Some(BbName(Name::from_internal(2))),
        },
    );
    let b2 = block!(exit());

    let f = function(Ret::No, 0, &locals, &[b0, b1, b2]);
    let p = program(&[f]);
    dump_program(p);
    assert_ub(p, "invalid alignment for `Intrinsic::Deallocate`: not a power of 2");
}

#[test]
fn dealloc_size_err() {
    let locals = [ <*const i32>::get_type() ];

    let b0 = block!(
        storage_live(0),
        allocate(const_int::<usize>(4), const_int::<usize>(4), local(0), 1)
    );
    let b1 = block!(
        Terminator::CallIntrinsic {
            intrinsic: Intrinsic::Deallocate,
            arguments: list![load(local(0)), const_int::<isize>(-1), const_int::<usize>(4)], // -1 is not a valid size!
            ret: zst_place(),
            next_block: Some(BbName(Name::from_internal(2))),
        },
    );
    let b2 = block!(exit());

    let f = function(Ret::No, 0, &locals, &[b0, b1, b2]);
    let p = program(&[f]);
    dump_program(p);
    assert_ub(p, "invalid size for `Intrinsic::Deallocate`: negative size");
}

#[test]
fn dealloc_wrongarg1() {
    let locals = [ <*const i32>::get_type() ];

    let b0 = block!(
        storage_live(0),
        allocate(const_int::<usize>(4), const_int::<usize>(4), local(0), 1)
    );
    let b1 = block!(
        Terminator::CallIntrinsic {
            intrinsic: Intrinsic::Deallocate,
            arguments: list![const_bool(true), const_int::<usize>(4), const_int::<usize>(4)], // bool unexpected here
            ret: zst_place(),
            next_block: Some(BbName(Name::from_internal(2))),
        },
    );
    let b2 = block!(exit());

    let f = function(Ret::No, 0, &locals, &[b0, b1, b2]);
    let p = program(&[f]);
    dump_program(p);
    assert_ub(p, "invalid first argument to `Intrinsic::Deallocate`, not a pointer");
}

#[test]
fn dealloc_wrongarg2() {
    let locals = [ <*const i32>::get_type() ];

    let b0 = block!(
        storage_live(0),
        allocate(const_int::<usize>(4), const_int::<usize>(4), local(0), 1)
    );
    let b1 = block!(
        Terminator::CallIntrinsic {
            intrinsic: Intrinsic::Deallocate,
            arguments: list![load(local(0)), const_bool(true), const_int::<usize>(4)], // bool unexpected here
            ret: zst_place(),
            next_block: Some(BbName(Name::from_internal(2))),
        },
    );
    let b2 = block!(exit());

    let f = function(Ret::No, 0, &locals, &[b0, b1, b2]);
    let p = program(&[f]);
    dump_program(p);
    assert_ub(p, "invalid second argument to `Intrinsic::Deallocate`, not an integer");
}

#[test]
fn dealloc_wrongarg3() {
    let locals = [ <*const i32>::get_type() ];

    let b0 = block!(
        storage_live(0),
        allocate(const_int::<usize>(4), const_int::<usize>(4), local(0), 1)
    );
    let b1 = block!(
        Terminator::CallIntrinsic {
            intrinsic: Intrinsic::Deallocate,
            arguments: list![load(local(0)), const_int::<usize>(4), const_bool(true)], // bool unexpected here
            ret: zst_place(),
            next_block: Some(BbName(Name::from_internal(2))),
        },
    );
    let b2 = block!(exit());

    let f = function(Ret::No, 0, &locals, &[b0, b1, b2]);
    let p = program(&[f]);
    dump_program(p);
    assert_ub(p, "invalid third argument to `Intrinsic::Deallocate`, not an integer");
}

#[test]
fn dealloc_wrongreturn() {
    let locals = [ <*const i32>::get_type() ];

    let b0 = block!(
        storage_live(0),
        allocate(const_int::<usize>(4), const_int::<usize>(4), local(0), 1)
    );
    let b1 = block!(
        Terminator::CallIntrinsic {
            intrinsic: Intrinsic::Deallocate,
            arguments: list![load(local(0)), const_int::<usize>(4), const_int::<usize>(4)],
            ret: local(0),
            next_block: Some(BbName(Name::from_internal(2))),
        },
    );
    let b2 = block!(exit());

    let f = function(Ret::No, 0, &locals, &[b0, b1, b2]);
    let p = program(&[f]);
    dump_program(p);
    assert_ub(p, "invalid return type for `Intrinsic::Deallocate`");
}

#[test]
fn mem_dealloc_wrong_size() {
    let locals = [ <*const i32>::get_type() ];

    let b0 = block!(
        storage_live(0),
        allocate(const_int::<usize>(4), const_int::<usize>(4), local(0), 1)
    );
    let b1 = block!(
        Terminator::CallIntrinsic {
            intrinsic: Intrinsic::Deallocate,
            arguments: list![load(local(0)), const_int::<usize>(5), const_int::<usize>(4)],
            ret: zst_place(),
            next_block: Some(BbName(Name::from_internal(2))),
        },
    );
    let b2 = block!(exit());

    let f = function(Ret::No, 0, &locals, &[b0, b1, b2]);
    let p = program(&[f]);
    dump_program(p);
    assert_ub(p, "deallocating with incorrect size information");
}

#[test]
fn mem_dealloc_wrong_align() {
    let locals = [ <*const i32>::get_type() ];

    let b0 = block!(
        storage_live(0),
        allocate(const_int::<usize>(4), const_int::<usize>(4), local(0), 1)
    );
    let b1 = block!(
        Terminator::CallIntrinsic {
            intrinsic: Intrinsic::Deallocate,
            arguments: list![load(local(0)), const_int::<usize>(4), const_int::<usize>(8)],
            ret: zst_place(),
            next_block: Some(BbName(Name::from_internal(2))),
        },
    );
    let b2 = block!(exit());

    let f = function(Ret::No, 0, &locals, &[b0, b1, b2]);
    let p = program(&[f]);
    dump_program(p);
    assert_ub(p, "deallocating with incorrect alignment information");
}

#[test]
// this tests generates a dangling ptr by casting the int 42 to a pointer.
// then we deallocate the ptr, to obtain UB.
fn mem_dealloc_inv_ptr() {
    let union_ty = union_ty(&[
            (size(0), <usize>::get_type()),
            (size(0), <*const i32>::get_type()),
        ], size(8), align(8));

    let locals = [ union_ty ];

    let b0 = block!(
        storage_live(0),
        assign(
            field(local(0), 0),
            const_int::<usize>(42)
        ),
        deallocate(
            load(field(local(0), 1)),
            const_int::<usize>(4), // size
            const_int::<usize>(4), // align
            1,
        )
    );
    let b1 = block!(exit());

    let f = function(Ret::No, 0, &locals, &[b0, b1]);
    let p = program(&[f]);
    dump_program(p);
    assert_ub(p, "deallocating invalid pointer");
}


#[test]
fn mem_dealloc_not_beginning() {
    let locals = [ <*const i32>::get_type() ];

    let b0 = block!(
        storage_live(0),
        allocate(const_int::<usize>(4), const_int::<usize>(4), local(0), 1)
    );
    let b1 = block!(
        assign(
            local(0),
            ptr_offset(
                load(local(0)),
                const_int::<usize>(1),
                InBounds::Yes
            )
        ),
        deallocate(
            load(local(0)),
            const_int::<usize>(4),
            const_int::<usize>(4),
            2
        )
    );
    let b2 = block!(exit());

    let f = function(Ret::No, 0, &locals, &[b0, b1, b2]);
    let p = program(&[f]);
    dump_program(p);
    assert_ub(p, "deallocating with pointer not to the beginning of its allocation");
}

#[test]
fn double_free() {
    let locals = vec![<*const i32>::get_type()];
    let n = const_int::<usize>(4);
    let b0 = block!(storage_live(0), allocate(n, n, local(0), 1));
    let b1 = block!(deallocate(load(local(0)), n, n, 2));
    let b2 = block!(deallocate(load(local(0)), n, n, 3));
    let b3 = block!(exit());
    let f = function(Ret::No, 0, &locals, &[b0, b1, b2, b3]);
    let p = program(&[f]);
    assert_ub(p, "double-free");
}

#[test]
fn use_after_free() {
    let locals = vec![<*const i32>::get_type()];
    let n = const_int::<usize>(4);
    let b0 = block!(storage_live(0), allocate(n, n, local(0), 1));
    let b1 = block!(deallocate(load(local(0)), n, n, 2));
    let b2 = block!(
        assign( // write to ptr after dealloc!
            deref(load(local(0)), <i32>::get_type()),
            const_int::<i32>(42),
        ),
        exit()
    );
    let f = function(Ret::No, 0, &locals, &[b0, b1, b2]);
    let p = program(&[f]);
    assert_ub(p, "memory accessed after deallocation");
}

#[test]
fn memory_leak() {
    let locals = [<*mut i32>::get_type()];

    let b0 = block!(
        storage_live(0),
        allocate(
            const_int::<usize>(1), // size
            const_int::<usize>(1), // align
            local(0),
            1,
        )
    );
    let b1 = block!(exit());
    let main = function(Ret::No, 0, &locals, &[b0, b1]);

    let p = program(&[main]);
    assert_memory_leak(p);
}

#[test]
fn mem_dealloc_stack() {
    let n = const_int::<usize>(4); // size and align of i32
    let locals = vec![<i32>::get_type()];
    let b0 = block!(
        storage_live(0),
        deallocate(
            addr_of(local(0), <*const i32>::get_type()),
            n,
            n,
            1,
        )
    );
    let b1 = block!(exit());
    let f = function(Ret::No, 0, &locals, &[b0, b1]);
    let p = program(&[f]);
    assert_ub(p, "deallocating Stack memory with Heap deallocation operation");
}
