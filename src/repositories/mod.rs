pub mod user;           // UserRepository implementation
pub mod account;        // AccountRepository implementation
pub mod transaction;    // TransactionRepository implementation

// Re-export for cleaner imports
pub use user::UserRepository;
pub use account::AccountRepository;
pub use transaction::TransactionRepository;