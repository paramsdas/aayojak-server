use aayojak_server::structures::todo::Todo;

#[test]
fn test_new_todo() {
    let todo = Todo::new("Test Todo", Some(100));
    assert_eq!(todo.title(), "Test Todo");
    assert_eq!(*todo.id(), Some(100));
    assert_eq!(todo.completion_status(), false);
    assert_eq!(todo.description().is_none(), true);
}

#[test]
fn test_status_toggling() {
    let mut todo = Todo::new("Test Todo", Some(100));
    assert_eq!(todo.completion_status(), false);
    assert_eq!(todo.date_completed().is_none(), true);

    // test with update_date_completed false
    todo.toggle_completion_status(true);
    assert_eq!(todo.completion_status(), true);
    assert_eq!(todo.date_completed().is_some(), true);

    todo.toggle_completion_status(true);
    assert_eq!(todo.completion_status(), false);
    assert_eq!(todo.date_completed().is_none(), true);

    // test with update_date_completed false
    todo.toggle_completion_status(false);
    assert_eq!(todo.completion_status(), true);
    assert_eq!(todo.date_completed().is_none(), true);

    todo.toggle_completion_status(false);
    assert_eq!(todo.completion_status(), false);
    assert_eq!(todo.date_completed().is_none(), true);
}
