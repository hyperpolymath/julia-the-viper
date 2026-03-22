# SQLite Database Demo

Comprehensive demonstration of SQLite database operations with migrations, repositories, and relationships.

## Features

- **Schema Migrations**: Versioned database migrations
- **Repository Pattern**: Clean data access layer
- **Relationships**: One-to-many and many-to-many
- **Transactions**: ACID-compliant operations
- **Indexes**: Optimized queries
- **Aggregations**: Complex queries and statistics

## Installation

No external dependencies required (uses Python standard library):

```bash
python database.py
```

## Database Schema

### Tables

**users**
- `id`: Primary key
- `username`: Unique username
- `email`: Unique email
- `password_hash`: Hashed password
- `full_name`: Full name (optional)
- `is_active`: Account status
- `created_at`, `updated_at`: Timestamps

**posts**
- `id`: Primary key
- `user_id`: Foreign key to users
- `title`: Post title
- `content`: Post content
- `status`: draft/published
- `view_count`: Number of views
- `created_at`, `updated_at`: Timestamps

**tags**
- `id`: Primary key
- `name`: Tag name
- `slug`: URL-friendly slug
- `created_at`: Timestamp

**post_tags** (Junction table)
- `post_id`: Foreign key to posts
- `tag_id`: Foreign key to tags
- `created_at`: Timestamp

**comments**
- `id`: Primary key
- `post_id`: Foreign key to posts
- `user_id`: Foreign key to users
- `content`: Comment text
- `created_at`: Timestamp

### Relationships

- Users → Posts: One-to-many
- Posts → Tags: Many-to-many (via post_tags)
- Posts → Comments: One-to-many
- Users → Comments: One-to-many

## Usage

### Initialize Database

```python
from database import Database

db = Database('app.db')
db.initialize()
```

### User Operations

```python
from database import UserRepository

users = UserRepository(db)

# Create user
user_id = users.create(
    username='john',
    email='john@example.com',
    password_hash='hashed_password',
    full_name='John Doe'
)

# Find user
user = users.find_by_id(user_id)
user = users.find_by_email('john@example.com')

# Get all users
all_users = users.find_all(limit=10, offset=0)

# Update user
users.update(user_id, full_name='John Smith', is_active=True)

# Delete user
users.delete(user_id)

# Get user stats
stats = users.get_user_stats(user_id)
print(f"Posts: {stats['post_count']}")
print(f"Comments: {stats['comment_count']}")
print(f"Total views: {stats['total_views']}")
```

### Post Operations

```python
from database import PostRepository

posts = PostRepository(db)

# Create post
post_id = posts.create(
    user_id=1,
    title='My First Post',
    content='Hello World!',
    status='published'
)

# Find post
post = posts.find_by_id(post_id)

# Get all posts
all_posts = posts.find_all(limit=10, offset=0)

# Get published posts
published = posts.find_all(status='published')

# Get user's posts
user_posts = posts.find_by_user(user_id)

# Increment views
posts.increment_views(post_id)

# Add tags
posts.add_tags(post_id, [tag1_id, tag2_id])

# Get post tags
tags = posts.get_post_tags(post_id)
```

### Tag Operations

```python
from database import TagRepository

tags = TagRepository(db)

# Create tag
tag_id = tags.create(name='Python', slug='python')

# Get all tags
all_tags = tags.find_all()

# Find by slug
tag = tags.find_by_slug('python')

# Get popular tags
popular = tags.get_popular_tags(limit=10)
for tag in popular:
    print(f"{tag['name']}: {tag['post_count']} posts")
```

## Migration System

Migrations are tracked in the `migrations` table:

```python
class Database:
    def run_migrations(self):
        # Migration 1: Users table
        if not self._migration_executed('create_users_table'):
            cursor.execute('CREATE TABLE users ...')
            self._record_migration('create_users_table')

        # Migration 2: Posts table
        if not self._migration_executed('create_posts_table'):
            cursor.execute('CREATE TABLE posts ...')
            self._record_migration('create_posts_table')
```

### Adding New Migrations

1. Check if migration executed
2. Run SQL commands
3. Record migration

```python
if not self._migration_executed('add_user_avatar'):
    cursor.execute('ALTER TABLE users ADD COLUMN avatar TEXT')
    self._record_migration('add_user_avatar')
```

## Indexes

Optimized queries with indexes:

```sql
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_posts_user_id ON posts(user_id);
CREATE INDEX idx_posts_status ON posts(status);
CREATE INDEX idx_tags_slug ON tags(slug);
CREATE INDEX idx_comments_post_id ON comments(post_id);
```

## Transaction Management

Using context manager for automatic commit/rollback:

```python
@contextmanager
def get_connection(self):
    conn = sqlite3.connect(self.db_path)
    try:
        yield conn
        conn.commit()  # Automatically commit
    except Exception:
        conn.rollback()  # Rollback on error
        raise
    finally:
        conn.close()
```

## Complex Queries

### Get Post with Author

```python
cursor.execute('''
    SELECT p.*, u.username, u.full_name
    FROM posts p
    JOIN users u ON p.user_id = u.id
    WHERE p.id = ?
''', (post_id,))
```

### Get Popular Tags

```python
cursor.execute('''
    SELECT t.*, COUNT(pt.post_id) as post_count
    FROM tags t
    LEFT JOIN post_tags pt ON t.id = pt.tag_id
    GROUP BY t.id
    ORDER BY post_count DESC
    LIMIT ?
''', (limit,))
```

### Get User Statistics

```python
# Post count
SELECT COUNT(*) FROM posts WHERE user_id = ?

# Comment count
SELECT COUNT(*) FROM comments WHERE user_id = ?

# Total views
SELECT SUM(view_count) FROM posts WHERE user_id = ?
```

## Repository Pattern

Clean separation of concerns:

```
Database
├── UserRepository
│   ├── create()
│   ├── find_by_id()
│   ├── find_by_email()
│   ├── find_all()
│   ├── update()
│   ├── delete()
│   └── get_user_stats()
├── PostRepository
│   ├── create()
│   ├── find_by_id()
│   ├── find_all()
│   ├── find_by_user()
│   ├── increment_views()
│   ├── add_tags()
│   └── get_post_tags()
└── TagRepository
    ├── create()
    ├── find_all()
    ├── find_by_slug()
    └── get_popular_tags()
```

## Best Practices

### 1. Use Parameterized Queries

✅ **Good**:
```python
cursor.execute('SELECT * FROM users WHERE id = ?', (user_id,))
```

❌ **Bad** (SQL injection risk):
```python
cursor.execute(f'SELECT * FROM users WHERE id = {user_id}')
```

### 2. Use Transactions

```python
with db.get_connection() as conn:
    cursor = conn.cursor()
    cursor.execute('INSERT INTO users ...')
    cursor.execute('INSERT INTO posts ...')
    # Automatically commits
```

### 3. Use Indexes

```python
CREATE INDEX idx_users_email ON users(email);
```

### 4. Foreign Keys with Cascade

```python
FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
```

## Testing

```python
import unittest

class TestUserRepository(unittest.TestCase):
    def setUp(self):
        self.db = Database(':memory:')  # In-memory DB for tests
        self.db.initialize()
        self.users = UserRepository(self.db)

    def test_create_user(self):
        user_id = self.users.create('john', 'john@example.com', 'hash')
        self.assertIsNotNone(user_id)

    def test_find_user(self):
        user_id = self.users.create('john', 'john@example.com', 'hash')
        user = self.users.find_by_id(user_id)
        self.assertEqual(user['username'], 'john')
```

## Performance Tips

1. **Use Indexes**: On frequently queried columns
2. **Limit Results**: Use LIMIT and OFFSET for pagination
3. **Analyze Queries**: Use EXPLAIN QUERY PLAN
4. **Batch Operations**: Use executemany() for multiple inserts
5. **Connection Pooling**: Reuse connections when possible

```python
# Batch insert
cursor.executemany('INSERT INTO users (username, email) VALUES (?, ?)', users_data)
```

## Common Patterns

### Pagination

```python
def find_all(self, page: int = 1, per_page: int = 10):
    offset = (page - 1) * per_page
    cursor.execute('SELECT * FROM users LIMIT ? OFFSET ?', (per_page, offset))
```

### Soft Delete

```python
# Add deleted_at column
cursor.execute('ALTER TABLE users ADD COLUMN deleted_at TIMESTAMP')

# Soft delete
cursor.execute('UPDATE users SET deleted_at = ? WHERE id = ?', (datetime.now(), user_id))

# Query non-deleted
cursor.execute('SELECT * FROM users WHERE deleted_at IS NULL')
```

### Full-Text Search

```python
# Create virtual table
cursor.execute('''
    CREATE VIRTUAL TABLE posts_fts USING fts5(title, content)
''')

# Search
cursor.execute('SELECT * FROM posts_fts WHERE posts_fts MATCH ?', ('python',))
```

## Extensions

### Add Timestamps Automatically

```python
CREATE TRIGGER update_user_timestamp
AFTER UPDATE ON users
BEGIN
    UPDATE users SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;
```

### Add Audit Trail

```python
CREATE TABLE audit_log (
    id INTEGER PRIMARY KEY,
    table_name TEXT,
    record_id INTEGER,
    action TEXT,
    old_values TEXT,
    new_values TEXT,
    user_id INTEGER,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

## License

MIT License
