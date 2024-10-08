use chrono::{NaiveDateTime, Utc};
use diesel::{
    prelude::{AsChangeset, Identifiable, Insertable, Queryable},
    Selectable,
};
use serde::{Deserialize, Serialize};

/// Todo is a structure which wraps all relevant information about a todo-task.
///
/// All time related structs/instances work with UTC.
#[derive(
    Debug, Queryable, Selectable, Identifiable, Insertable, AsChangeset, Serialize, Deserialize,
)]
#[diesel(table_name = crate::schema::todos)]
#[diesel(treat_none_as_null = true)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Todo {
    // compulsary attributes
    #[diesel(deserialize_as = i64)]
    id: Option<i64>,
    title: String,
    completion_status: bool,
    date_created: NaiveDateTime,
    date_modified: NaiveDateTime,
    // optional attributes
    description: Option<String>,
    date_completed: Option<NaiveDateTime>,
    date_deadline: Option<NaiveDateTime>,
}

impl Todo {
    // getters
    /// get todo title
    pub fn title(&self) -> &str {
        &self.title
    }
    /// get todo id
    pub fn id(&self) -> &Option<i64> {
        return &self.id;
    }
    /// get todo description
    ///
    /// # returns:
    ///  - ```true```, if completed
    ///  - ```false```, otherwise
    pub fn completion_status(&self) -> bool {
        self.completion_status
    }
    /// get the creation date
    pub fn date_created(&self) -> &NaiveDateTime {
        &self.date_created
    }
    /// get the last modified date
    pub fn date_modified(&self) -> &NaiveDateTime {
        &self.date_modified
    }
    /// get todo description, if set
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
    /// get the completion date, if set
    pub fn date_completed(&self) -> Option<&NaiveDateTime> {
        self.date_completed.as_ref()
    }
    /// get the deadline date, if set
    pub fn date_deadline(&self) -> Option<&NaiveDateTime> {
        self.date_deadline.as_ref()
    }

    // setters
    /// set todo title
    pub fn set_title(&mut self, title: String) {
        self.title = title;
        self.update_date_modified();
    }
    /// set todo description
    pub fn set_description(&mut self, description: Option<&str>) {
        let mut description_string_option = None;
        if let Some(desc) = description {
            description_string_option = Some(String::from(desc));
        }
        self.description = description_string_option;
        self.update_date_modified();
    }
    /// set the completion date
    pub fn set_date_completed(&mut self, date_completed: Option<NaiveDateTime>) {
        self.date_completed = date_completed;
    }
    /// set the deadline date
    pub fn set_date_deadline(&mut self, date_deadline: Option<NaiveDateTime>) {
        self.date_deadline = date_deadline;
    }
    /// set the completion status
    pub fn set_completion_status(&mut self, is_completed: bool, update_date_completed: bool) {
        self.completion_status = is_completed;
        if update_date_completed {
            match self.completion_status {
                true => self.set_date_completed(Some(Utc::now().naive_utc())),
                false => self.set_date_completed(None),
            }
        }
        self.update_date_modified();
    }

    // additional functions
    /// Create a new Todo instance. Requires a title and an id.
    /// # Examples
    ///
    /// ```
    /// use aayojak_server::structures::todos::todo::Todo;
    /// let todo = Todo::new("Test", Some(1), Some("test_description"), None);
    ///
    /// assert_eq!(todo.title(), "Test");
    /// assert_eq!(todo.id(), &Some(1));
    /// assert_eq!(todo.description().unwrap(), "test_description")
    /// ```
    ///
    pub fn new(
        title: &str,
        id: Option<i64>,
        description: Option<&str>,
        date_deadline: Option<NaiveDateTime>,
    ) -> Self {
        let date_time = Utc::now().naive_utc();
        let mut description_string_option = None;
        if let Some(desc) = description {
            description_string_option = Some(String::from(desc));
        }
        Todo {
            title: String::from(title),
            id,
            description: description_string_option,
            date_created: date_time,
            date_modified: date_time,
            date_deadline,
            date_completed: None,
            completion_status: false,
        }
    }
    /// toggle the completion status completion date depending on current status
    ///
    /// ```update_date_completed```, if true, will update the completion date
    /// if completion status will be toggled to true and will delete the
    /// completion date if completion status will be toggled to false
    pub fn toggle_completion_status(&mut self, update_date_completed: bool) {
        self.set_completion_status(!self.completion_status(), update_date_completed)
    }

    // private functions
    /// function which updates the modification date to current time (UTC)
    fn update_date_modified(&mut self) {
        self.date_modified = Utc::now().naive_utc();
    }
}

impl ToString for Todo {
    fn to_string(&self) -> String {
        let formatted_string = format!(
            "\ttitle: {},\n\tdate_created: {},\n\tcompletion_status:{}",
            self.title(),
            self.date_created(),
            self.completion_status()
        );
        String::from(formatted_string)
    }
}
