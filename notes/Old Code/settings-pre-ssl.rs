


#[cfg(production)]
pub const PRODUCTION: bool = true;
#[cfg(not(production))]
pub const PRODUCTION: bool = false;










// DEVELOPMENT SETTINGS

#[cfg(not(production))]
pub const BLOG_URL: &'static str = "http://localhost:8000/";
#[cfg(not(production))]
pub const USER_LOGIN_URL: &'static str = "http://localhost:8000/user";
#[cfg(not(production))]
pub const ADMIN_LOGIN_URL: &'static str = "http://localhost:8000/admin";
#[cfg(not(production))]
pub const TEST_LOGIN_URL: &'static str = "http://localhost:8000/login";
#[cfg(not(production))]
pub const CREATE_FORM_URL: &'static str = "http://localhost:8000/create";
#[cfg(not(production))]
pub const EDIT_FORM_URL: &'static str = "http://localhost:8000/edit";
#[cfg(not(production))]
pub const MANAGE_URL: &'static str = "http://localhost:8000/manage";
#[cfg(not(production))]
pub const MAX_CREATE_TITLE: usize = 120;
#[cfg(not(production))]
pub const MAX_CREATE_DESCRIPTION: usize = 400;
#[cfg(not(production))]
pub const MAX_CREATE_TAGS: usize = 250;
#[cfg(not(production))]
pub const DATABASE_URL: &'static str = "postgres://postgres:andrew@localhost/blog";
#[cfg(not(production))]
const MAX_ATTEMPTS: i16 = 8; // 8
#[cfg(not(production))]
const LOCKOUT_DURATION: u32 = 8; // 6 seconds // 900 seconds = 15 minutes
#[cfg(not(production))]
const DB_BACKUP_SCRIPT: &'static str = r"scripts\db_backup-dev.bat";
// After the specified number of attempts, the next account lock will permanently lock the account
#[cfg(not(production))]
const ADMIN_LOCK: i16 = 20;
// After the specified number of attempts, the next account lock will permanently lock the account
#[cfg(not(production))]
const USER_LOCK: i16 = 40;
// Path to the article header images internally
#[cfg(not(production))]
const INTERNAL_IMGS: &'static str = r"c:\code\lang\rust\proj\blogr\static\imgs";
#[cfg(not(production))]
pub const HITS_SAVE_INTERVAL: usize = 20;
// Default http caching value (max age value)
#[cfg(not(production))]
const DEFAULT_TTL: isize = 3600;  // 1*60*60 = 1 hour, 43200=1/2 day, 86400=1 day
// If no description is found take a specified amount of characters from the article body
#[cfg(not(production))]
pub const DESC_LIMIT: usize = 300;

// Comrak Markdown rendering default settings
#[cfg(not(production))]
pub const COMRAK_OPTIONS: ComrakOptions = ComrakOptions {
    hardbreaks: true,            // \n => <br>\n
    width: 120usize,             
    github_pre_lang: false,      
    ext_strikethrough: true,     // hello ~world~ person.
    ext_tagfilter: true,         // filters out certain html tags
    ext_table: true,             // | a | b |\n|---|---|\n| c | d |
    ext_autolink: true,          
    ext_tasklist: true,          // * [x] Done\n* [ ] Not Done
    ext_superscript: true,       // e = mc^2^
    ext_header_ids: None,        // None / Some("some-id-prefix-".to_string())
    ext_footnotes: true,         // Hi[^x]\n\n[^x]: A footnote here\n
};









// PRODUCTION SETTINGS

#[cfg(production)]
pub const BLOG_URL: &'static str = "http://vishus.net/";
#[cfg(production)]
pub const USER_LOGIN_URL: &'static str = "http://vishus.net/user";
#[cfg(production)]
pub const ADMIN_LOGIN_URL: &'static str = "http://vishus.net/admin";
#[cfg(production)]
pub const TEST_LOGIN_URL: &'static str = "http://vishus.net/login";
#[cfg(production)]
pub const CREATE_FORM_URL: &'static str = "http://vishus.net/create";
#[cfg(production)]
pub const EDIT_FORM_URL: &'static str = "http://vishus.net/edit";
#[cfg(production)]
pub const MANAGE_URL: &'static str = "http://vishus.net/manage";
#[cfg(production)]
pub const MAX_CREATE_TITLE: usize = 50;
#[cfg(production)]
pub const MAX_CREATE_DESCRIPTION: usize = 500;
#[cfg(production)]
pub const MAX_CREATE_TAGS: usize = 250;
#[cfg(production)]
pub const DATABASE_URL: &'static str = "postgres://vishus:Mutex7892@localhost/blog";
#[cfg(production)]
const MAX_ATTEMPTS: i16 = 5; // 8
#[cfg(production)]
const LOCKOUT_DURATION: u32 = 900; // 6 seconds // 900 seconds = 15 minutes
#[cfg(production)]
const DB_BACKUP_SCRIPT: &'static str = r"bash";
#[cfg(production)]
const DB_BACKUP_ARG: &'static str = r"scripts/db_backup-prod.sh";
// After the specified number of attempts, the next account lock will permanently lock the account
#[cfg(production)]
const ADMIN_LOCK: i16 = 15;
// After the specified number of attempts, the next account lock will permanently lock the account
#[cfg(production)]
const USER_LOCK: i16 = 40;
// Path to the article header images internally
#[cfg(production)]
const INTERNAL_IMGS: &'static str = r"/var/www/html/imgs";
#[cfg(production)]
pub const HITS_SAVE_INTERVAL: usize = 5;
// Default http caching value (max age value)
#[cfg(production)]
const DEFAULT_TTL: isize = 3600;  // 1*60*60 = 1 hour, 43200=1/2 day, 86400=1 day
// If no description is found take a specified amount of characters from the article body
#[cfg(production)]
pub const DESC_LIMIT: usize = 300;

// Comrak Markdown rendering default settings
#[cfg(production)]
pub const COMRAK_OPTIONS: ComrakOptions = ComrakOptions {
    hardbreaks: true,            // \n => <br>\n
    width: 120usize,             
    github_pre_lang: false,      
    ext_strikethrough: true,     // hello ~world~ person.
    ext_tagfilter: true,         // filters out certain html tags
    ext_table: true,             // | a | b |\n|---|---|\n| c | d |
    ext_autolink: true,          
    ext_tasklist: true,          // * [x] Done\n* [ ] Not Done
    ext_superscript: true,       // e = mc^2^
    ext_header_ids: None,        // None / Some("some-id-prefix-".to_string())
    ext_footnotes: true,         // Hi[^x]\n\n[^x]: A footnote here\n
};




