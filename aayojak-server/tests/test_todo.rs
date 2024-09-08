use aayojak_server::structures::todos::todo::Todo;
use chrono::Utc;

#[test]
fn test_new_todo() {
    let todo_1 = Todo::new("Test Todo", Some(100), None, None);
    assert_eq!(todo_1.title(), "Test Todo");
    assert_eq!(todo_1.id(), &Some(100));
    assert_eq!(todo_1.completion_status(), false);
    assert_eq!(todo_1.description(), None);
    assert_eq!(todo_1.date_deadline(), None);

    let todo_2 = Todo::new("Test Todo", Some(100), Some("Hello"), None);
    assert_eq!(todo_2.title(), "Test Todo");
    assert_eq!(todo_2.id(), &Some(100));
    assert_eq!(todo_2.completion_status(), false);
    assert_eq!(todo_2.description().unwrap(), "Hello");
    assert_eq!(todo_2.date_deadline(), None);

    let todo_3 = Todo::new("Test Todo", Some(100), None, Some(Utc::now().naive_utc()));
    assert_eq!(todo_3.title(), "Test Todo");
    assert_eq!(todo_3.id(), &Some(100));
    assert_eq!(todo_3.completion_status(), false);
    assert_eq!(todo_3.description(), None);
    assert_ne!(todo_3.date_deadline(), None);

    let todo_4 = Todo::new(
        "Test Todo",
        Some(100),
        Some("Hello"),
        Some(Utc::now().naive_utc()),
    );
    assert_eq!(todo_4.title(), "Test Todo");
    assert_eq!(todo_4.id(), &Some(100));
    assert_eq!(todo_4.completion_status(), false);
    assert_eq!(todo_4.description().unwrap(), "Hello");
    assert_ne!(todo_4.date_deadline(), None);
}

#[test]
fn test_status_toggling() {
    let mut todo = Todo::new("Test Todo", Some(100), None, None);
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
