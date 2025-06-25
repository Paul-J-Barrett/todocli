use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Todo {
    pub id: String,
    pub subject: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
    pub last_modified_at: DateTime<Utc>,
}

impl Todo {
    pub fn new(subject: String, description: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            subject,
            description,
            created_at: now,
            closed_at: None,
            last_modified_at: now,
        }
    }

    pub fn is_completed(&self) -> bool {
        self.closed_at.is_some()
    }

    pub fn toggle_completion(&mut self) {
        let now = Utc::now();
        if self.is_completed() {
            self.closed_at = None;
        } else {
            self.closed_at = Some(now);
        }
        self.last_modified_at = now;
    }

    pub fn update(&mut self, subject: String, description: String) {
        self.subject = subject;
        self.description = description;
        self.last_modified_at = Utc::now();
    }

    pub fn status_icon(&self) -> &'static str {
        if self.is_completed() {
            "✅"
        } else {
            "📝"
        }
    }

    pub fn display_title(&self) -> String {
        format!("{} {}", self.status_icon(), self.subject)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_todo_creation() {
        let subject = "Test Todo".to_string();
        let description = "Test Description".to_string();
        
        let todo = Todo::new(subject.clone(), description.clone());
        
        assert_eq!(todo.subject, subject);
        assert_eq!(todo.description, description);
        assert!(!todo.id.is_empty());
        assert!(!todo.is_completed());
        assert_eq!(todo.closed_at, None);
        assert_eq!(todo.created_at, todo.last_modified_at);
    }

    #[test]
    fn test_todo_completion_toggle() {
        let mut todo = Todo::new("Test".to_string(), "Description".to_string());
        let original_created_at = todo.created_at;
        let original_last_modified = todo.last_modified_at;
        
        // Wait a bit to ensure timestamp difference
        std::thread::sleep(std::time::Duration::from_millis(1));
        
        // Mark as completed
        todo.toggle_completion();
        
        assert!(todo.is_completed());
        assert!(todo.closed_at.is_some());
        assert!(todo.last_modified_at > original_last_modified);
        assert_eq!(todo.created_at, original_created_at);
        
        // Mark as incomplete
        let completed_last_modified = todo.last_modified_at;
        std::thread::sleep(std::time::Duration::from_millis(1));
        todo.toggle_completion();
        
        assert!(!todo.is_completed());
        assert_eq!(todo.closed_at, None);
        assert!(todo.last_modified_at > completed_last_modified);
    }

    #[test]
    fn test_todo_update() {
        let mut todo = Todo::new("Original".to_string(), "Original Description".to_string());
        let original_created_at = todo.created_at;
        let original_last_modified = todo.last_modified_at;
        
        std::thread::sleep(std::time::Duration::from_millis(1));
        
        let new_subject = "Updated Subject".to_string();
        let new_description = "Updated Description".to_string();
        
        todo.update(new_subject.clone(), new_description.clone());
        
        assert_eq!(todo.subject, new_subject);
        assert_eq!(todo.description, new_description);
        assert_eq!(todo.created_at, original_created_at);
        assert!(todo.last_modified_at > original_last_modified);
    }

    #[test]
    fn test_status_icon() {
        let mut todo = Todo::new("Test".to_string(), "Description".to_string());
        
        // Test incomplete status icon (using string comparison to avoid encoding issues)
        let incomplete_icon = todo.status_icon();
        assert!(!incomplete_icon.is_empty());
        
        todo.toggle_completion();
        let completed_icon = todo.status_icon();
        assert!(!completed_icon.is_empty());
        assert_ne!(incomplete_icon, completed_icon);
        
        todo.toggle_completion();
        assert_eq!(todo.status_icon(), incomplete_icon);
    }

    #[test]
    fn test_display_title() {
        let mut todo = Todo::new("Test Todo".to_string(), "Description".to_string());
        
        let incomplete_title = todo.display_title();
        assert!(incomplete_title.contains("Test Todo"));
        
        todo.toggle_completion();
        let completed_title = todo.display_title();
        assert!(completed_title.contains("Test Todo"));
        assert_ne!(incomplete_title, completed_title);
    }

    #[test]
    fn test_is_completed() {
        let mut todo = Todo::new("Test".to_string(), "Description".to_string());
        
        assert!(!todo.is_completed());
        
        todo.toggle_completion();
        assert!(todo.is_completed());
        
        todo.toggle_completion();
        assert!(!todo.is_completed());
    }
}