use server_rust::socket::game_7_socket::card_playable;

#[test]
fn sanity_check() {
    assert!(true);
}

#[test]
fn func_card_playable_empty() {
let board: Vec<Vec<i32>> = vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0]];
    assert!(card_playable(&19, &board));
    assert!(!card_playable(&18, &board));
    assert!(!card_playable(&20, &board));
    assert!(!card_playable(&0, &board));
    assert!(!card_playable(&7, &board));
    assert!(!card_playable(&8, &board));
    assert!(!card_playable(&9, &board));
    assert!(!card_playable(&10, &board));
    assert!(!card_playable(&6, &board));
}

#[test]
fn func_card_playable_19_played() {
    let board: Vec<Vec<i32>> = vec![vec![0, 0, 0, 0], vec![0, 7, 0, 0], vec![0, 0, 0, 0]];
    assert!(card_playable(&6, &board));
    assert!(card_playable(&32, &board));
    assert!(card_playable(&45, &board));
    assert!(card_playable(&18, &board));
    assert!(card_playable(&20, &board));
    assert!(!card_playable(&0, &board));
    assert!(!card_playable(&7, &board));
    assert!(!card_playable(&8, &board));
    assert!(!card_playable(&9, &board));
    assert!(!card_playable(&10, &board));
}


#[test]
fn func_card_playable_all_7_played() {
    let board: Vec<Vec<i32>> = vec![vec![0, 0, 0, 0], vec![7, 7, 7, 7], vec![0, 0, 0, 0]];
    assert!(card_playable(&5, &board));
    assert!(card_playable(&7, &board));
    assert!(card_playable(&18, &board));
    assert!(card_playable(&20, &board));
    assert!(card_playable(&31, &board));
    assert!(card_playable(&33, &board));
    assert!(card_playable(&44, &board));
    assert!(card_playable(&46, &board));
    assert!(!card_playable(&4, &board));
    assert!(!card_playable(&8, &board));
    assert!(!card_playable(&17, &board));
    assert!(!card_playable(&21, &board));
    assert!(!card_playable(&30, &board));
    assert!(!card_playable(&34, &board));
    assert!(!card_playable(&43, &board));
    assert!(!card_playable(&47, &board));
}

