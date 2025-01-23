use super::*;

qc!(children, _children);
fn _children((map, start): (PrefixMap<TestPrefix, i32>, TestPrefix)) -> bool {
    let want = select_ref(&map, |p, _| start.contains(p));
    map.children(start).eq(want)
}

qc!(children_trieview, _children_trieview);
fn _children_trieview((map, start): (PrefixMap<TestPrefix, i32>, TestPrefix)) -> bool {
    let want = select_ref(&map, |p, _| start.contains(p));
    if let Some(view) = map.view_at(start) {
        view.iter().eq(want)
    } else {
        want.is_empty()
    }
}

qc!(children_trieview_mut, _children_trieview_mut);
fn _children_trieview_mut((mut map, start): (PrefixMap<TestPrefix, i32>, TestPrefix)) -> bool {
    let want = select(&map, |p, _| start.contains(p));
    if let Some(view) = map.view_mut_at(start) {
        view.into_iter().map(|(p, t)| (*p, *t)).eq(want)
    } else {
        want.is_empty()
    }
}

qc!(children_trieview_mut_remove, _children_trieview_mut_remove);
fn _children_trieview_mut_remove(mut map: PrefixMap<TestPrefix, i32>) -> bool {
    let start = TestPrefix(0, 0).left();
    let want = select(&map, |p, _| start.contains(p) && *p != start);
    if let Ok(mut view) = map.view_mut().left() {
        view.remove();
        view.into_iter().map(|(p, t)| (*p, *t)).eq(want)
    } else {
        want.is_empty()
    }
}

qc!(children_trieview_mut_set, _children_trieview_mut_set);
fn _children_trieview_mut_set(mut map: PrefixMap<TestPrefix, i32>) -> bool {
    let start = TestPrefix(0, 0).left();
    let mut want = select(&map, |p, _| start.contains(p) && *p != start);
    if let Ok(mut view) = map.view_mut().left() {
        want.push((start, 10));
        want.sort();
        view.set(10).unwrap();
        view.into_iter().map(|(p, t)| (*p, *t)).eq(want)
    } else {
        want.is_empty()
    }
}

qc!(children_keys_trieview, _children_keys_trieview);
fn _children_keys_trieview((map, start): (PrefixMap<TestPrefix, i32>, TestPrefix)) -> bool {
    let want = select_keys(&map, |p, _| start.contains(p));
    if let Some(view) = map.view_at(start) {
        view.keys().eq(want)
    } else {
        want.is_empty()
    }
}

qc!(children_values_trieview, _children_values_trieview);
fn _children_values_trieview((map, start): (PrefixMap<TestPrefix, i32>, TestPrefix)) -> bool {
    let want = select_values(&map, |p, _| start.contains(p));
    if let Some(view) = map.view_at(start) {
        view.values().eq(want)
    } else {
        want.is_empty()
    }
}
