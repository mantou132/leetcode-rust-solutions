#[macro_use]
extern crate porus;

use std::mem::size_of;

use porus::traits::*;
use porus::storage::Pool;


#[test]
#[should_panic(expect="full")]
fn test_pool_full() {
    let pool = &mut Pool::<usize>::with_capacity(1);
    pool.add(1);
    pool.add(1);
}

#[test]
fn test_add_remove() {
    let pool = &mut Pool::<usize>::with_capacity(3);

    let item1 = pool.add(1);
    assert!(1 == unsafe {*item1});
    pool.remove(item1);
    let item2 = pool.add(2);
    assert!(2 == unsafe {*item2});

    assert!(item1 == item2);
    pool.remove(item2);


    let item1 = pool.add(1);
    let item2 = pool.add(2);
    let item3 = pool.add(3);
    assert!(1 == unsafe {*item1});
    assert!(2 == unsafe {*item2});
    assert!(3 == unsafe {*item3});
    pool.remove(item1);
    pool.remove(item2);
    pool.remove(item3);
    assert!(item3 == pool.add(3));
    assert!(item2 == pool.add(2));
    assert!(item1 == pool.add(1));
    assert!(1 == unsafe {*item1});
    assert!(2 == unsafe {*item2});
    assert!(3 == unsafe {*item3});
}


#[test]
fn test_add_small() {
    let pool = &mut Pool::<u8>::with_capacity(3);

    let item1 = pool.add(1);
    assert!(1 == unsafe {*item1});
    pool.remove(item1);
    let item2 = pool.add(2);
    assert!(2 == unsafe {*item2});
    assert!(item1 == item2);
    pool.remove(item2);


    let item1 = pool.add(1);
    let item2 = pool.add(2);
    let item3 = pool.add(3);

    assert!(unsafe { item1.offset(size_of::<usize>() as isize) } == item2);
    assert!(unsafe { item2.offset(size_of::<usize>() as isize) } == item3);

    assert!(1 == unsafe {*item1});
    assert!(2 == unsafe {*item2});
    assert!(3 == unsafe {*item3});
    pool.remove(item1);
    pool.remove(item2);
    pool.remove(item3);
    assert!(item3 == pool.add(3));
    assert!(item2 == pool.add(2));
    assert!(item1 == pool.add(1));
    assert!(1 == unsafe {*item1});
    assert!(2 == unsafe {*item2});
    assert!(3 == unsafe {*item3});
}
