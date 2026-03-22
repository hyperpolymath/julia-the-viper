#!/usr/bin/env python3
"""
SQLite Database Demo with Migrations, Models, and Queries

Demonstrates:
- Database initialization
- Schema migrations
- CRUD operations
- Relationships (one-to-many, many-to-many)
- Transactions
- Indexes
- Aggregations
"""

import sqlite3
from typing import List, Dict, Any, Optional
from datetime import datetime
from contextlib import contextmanager
import json


class Database:
    """SQLite database manager with migrations and utilities."""

    def __init__(self, db_path: str = 'app.db'):
        self.db_path = db_path
        self.connection = None
        self.migrations_run = False

    @contextmanager
    def get_connection(self):
        """Context manager for database connections."""
        conn = sqlite3.connect(self.db_path)
        conn.row_factory = sqlite3.Row  # Return rows as dictionaries
        try:
            yield conn
            conn.commit()
        except Exception:
            conn.rollback()
            raise
        finally:
            conn.close()

    def initialize(self):
        """Initialize database with schema."""
        if not self.migrations_run:
            self.run_migrations()
            self.migrations_run = True

    def run_migrations(self):
        """Run database migrations."""
        with self.get_connection() as conn:
            cursor = conn.cursor()

            # Create migrations table
            cursor.execute('''
                CREATE TABLE IF NOT EXISTS migrations (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT NOT NULL UNIQUE,
                    executed_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
                )
            ''')

            # Migration 1: Create users table
            if not self._migration_executed('create_users_table', cursor):
                cursor.execute('''
                    CREATE TABLE users (
                        id INTEGER PRIMARY KEY AUTOINCREMENT,
                        username TEXT NOT NULL UNIQUE,
                        email TEXT NOT NULL UNIQUE,
                        password_hash TEXT NOT NULL,
                        full_name TEXT,
                        is_active BOOLEAN DEFAULT 1,
                        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
                    )
                ''')

                # Create indexes
                cursor.execute('CREATE INDEX idx_users_email ON users(email)')
                cursor.execute('CREATE INDEX idx_users_username ON users(username)')

                self._record_migration('create_users_table', cursor)

            # Migration 2: Create posts table
            if not self._migration_executed('create_posts_table', cursor):
                cursor.execute('''
                    CREATE TABLE posts (
                        id INTEGER PRIMARY KEY AUTOINCREMENT,
                        user_id INTEGER NOT NULL,
                        title TEXT NOT NULL,
                        content TEXT,
                        status TEXT DEFAULT 'draft',
                        view_count INTEGER DEFAULT 0,
                        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                        FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
                    )
                ''')

                cursor.execute('CREATE INDEX idx_posts_user_id ON posts(user_id)')
                cursor.execute('CREATE INDEX idx_posts_status ON posts(status)')

                self._record_migration('create_posts_table', cursor)

            # Migration 3: Create tags table
            if not self._migration_executed('create_tags_table', cursor):
                cursor.execute('''
                    CREATE TABLE tags (
                        id INTEGER PRIMARY KEY AUTOINCREMENT,
                        name TEXT NOT NULL UNIQUE,
                        slug TEXT NOT NULL UNIQUE,
                        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
                    )
                ''')

                cursor.execute('CREATE INDEX idx_tags_slug ON tags(slug)')

                self._record_migration('create_tags_table', cursor)

            # Migration 4: Create post_tags junction table
            if not self._migration_executed('create_post_tags_table', cursor):
                cursor.execute('''
                    CREATE TABLE post_tags (
                        post_id INTEGER NOT NULL,
                        tag_id INTEGER NOT NULL,
                        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                        PRIMARY KEY (post_id, tag_id),
                        FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE CASCADE,
                        FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
                    )
                ''')

                self._record_migration('create_post_tags_table', cursor)

            # Migration 5: Create comments table
            if not self._migration_executed('create_comments_table', cursor):
                cursor.execute('''
                    CREATE TABLE comments (
                        id INTEGER PRIMARY KEY AUTOINCREMENT,
                        post_id INTEGER NOT NULL,
                        user_id INTEGER NOT NULL,
                        content TEXT NOT NULL,
                        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                        FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE CASCADE,
                        FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
                    )
                ''')

                cursor.execute('CREATE INDEX idx_comments_post_id ON comments(post_id)')

                self._record_migration('create_comments_table', cursor)

            conn.commit()

    def _migration_executed(self, name: str, cursor) -> bool:
        """Check if migration has been executed."""
        cursor.execute('SELECT COUNT(*) FROM migrations WHERE name = ?', (name,))
        return cursor.fetchone()[0] > 0

    def _record_migration(self, name: str, cursor):
        """Record migration as executed."""
        cursor.execute('INSERT INTO migrations (name) VALUES (?)', (name,))


class UserRepository:
    """Repository for User operations."""

    def __init__(self, db: Database):
        self.db = db

    def create(self, username: str, email: str, password_hash: str,
               full_name: Optional[str] = None) -> int:
        """Create a new user."""
        with self.db.get_connection() as conn:
            cursor = conn.cursor()
            cursor.execute('''
                INSERT INTO users (username, email, password_hash, full_name)
                VALUES (?, ?, ?, ?)
            ''', (username, email, password_hash, full_name))
            return cursor.lastrowid

    def find_by_id(self, user_id: int) -> Optional[Dict[str, Any]]:
        """Find user by ID."""
        with self.db.get_connection() as conn:
            cursor = conn.cursor()
            cursor.execute('SELECT * FROM users WHERE id = ?', (user_id,))
            row = cursor.fetchone()
            return dict(row) if row else None

    def find_by_email(self, email: str) -> Optional[Dict[str, Any]]:
        """Find user by email."""
        with self.db.get_connection() as conn:
            cursor = conn.cursor()
            cursor.execute('SELECT * FROM users WHERE email = ?', (email,))
            row = cursor.fetchone()
            return dict(row) if row else None

    def find_all(self, limit: int = 100, offset: int = 0) -> List[Dict[str, Any]]:
        """Get all users with pagination."""
        with self.db.get_connection() as conn:
            cursor = conn.cursor()
            cursor.execute(
                'SELECT * FROM users ORDER BY created_at DESC LIMIT ? OFFSET ?',
                (limit, offset)
            )
            return [dict(row) for row in cursor.fetchall()]

    def update(self, user_id: int, **kwargs) -> bool:
        """Update user."""
        if not kwargs:
            return False

        kwargs['updated_at'] = datetime.now()

        fields = ', '.join(f'{k} = ?' for k in kwargs.keys())
        values = list(kwargs.values()) + [user_id]

        with self.db.get_connection() as conn:
            cursor = conn.cursor()
            cursor.execute(f'UPDATE users SET {fields} WHERE id = ?', values)
            return cursor.rowcount > 0

    def delete(self, user_id: int) -> bool:
        """Delete user."""
        with self.db.get_connection() as conn:
            cursor = conn.cursor()
            cursor.execute('DELETE FROM users WHERE id = ?', (user_id,))
            return cursor.rowcount > 0

    def get_user_stats(self, user_id: int) -> Dict[str, Any]:
        """Get user statistics."""
        with self.db.get_connection() as conn:
            cursor = conn.cursor()

            # Get post count
            cursor.execute(
                'SELECT COUNT(*) as count FROM posts WHERE user_id = ?',
                (user_id,)
            )
            post_count = cursor.fetchone()['count']

            # Get comment count
            cursor.execute(
                'SELECT COUNT(*) as count FROM comments WHERE user_id = ?',
                (user_id,)
            )
            comment_count = cursor.fetchone()['count']

            # Get total views
            cursor.execute(
                'SELECT SUM(view_count) as total FROM posts WHERE user_id = ?',
                (user_id,)
            )
            total_views = cursor.fetchone()['total'] or 0

            return {
                'post_count': post_count,
                'comment_count': comment_count,
                'total_views': total_views
            }


class PostRepository:
    """Repository for Post operations."""

    def __init__(self, db: Database):
        self.db = db

    def create(self, user_id: int, title: str, content: str,
               status: str = 'draft') -> int:
        """Create a new post."""
        with self.db.get_connection() as conn:
            cursor = conn.cursor()
            cursor.execute('''
                INSERT INTO posts (user_id, title, content, status)
                VALUES (?, ?, ?, ?)
            ''', (user_id, title, content, status))
            return cursor.lastrowid

    def find_by_id(self, post_id: int) -> Optional[Dict[str, Any]]:
        """Find post by ID with user information."""
        with self.db.get_connection() as conn:
            cursor = conn.cursor()
            cursor.execute('''
                SELECT p.*, u.username, u.full_name
                FROM posts p
                JOIN users u ON p.user_id = u.id
                WHERE p.id = ?
            ''', (post_id,))
            row = cursor.fetchone()
            return dict(row) if row else None

    def find_all(self, status: Optional[str] = None,
                 limit: int = 10, offset: int = 0) -> List[Dict[str, Any]]:
        """Get posts with optional status filter."""
        with self.db.get_connection() as conn:
            cursor = conn.cursor()

            if status:
                cursor.execute('''
                    SELECT p.*, u.username
                    FROM posts p
                    JOIN users u ON p.user_id = u.id
                    WHERE p.status = ?
                    ORDER BY p.created_at DESC
                    LIMIT ? OFFSET ?
                ''', (status, limit, offset))
            else:
                cursor.execute('''
                    SELECT p.*, u.username
                    FROM posts p
                    JOIN users u ON p.user_id = u.id
                    ORDER BY p.created_at DESC
                    LIMIT ? OFFSET ?
                ''', (limit, offset))

            return [dict(row) for row in cursor.fetchall()]

    def find_by_user(self, user_id: int) -> List[Dict[str, Any]]:
        """Get all posts by a user."""
        with self.db.get_connection() as conn:
            cursor = conn.cursor()
            cursor.execute('''
                SELECT * FROM posts
                WHERE user_id = ?
                ORDER BY created_at DESC
            ''', (user_id,))
            return [dict(row) for row in cursor.fetchall()]

    def increment_views(self, post_id: int):
        """Increment post view count."""
        with self.db.get_connection() as conn:
            cursor = conn.cursor()
            cursor.execute(
                'UPDATE posts SET view_count = view_count + 1 WHERE id = ?',
                (post_id,)
            )

    def add_tags(self, post_id: int, tag_ids: List[int]):
        """Add tags to post."""
        with self.db.get_connection() as conn:
            cursor = conn.cursor()
            for tag_id in tag_ids:
                cursor.execute('''
                    INSERT OR IGNORE INTO post_tags (post_id, tag_id)
                    VALUES (?, ?)
                ''', (post_id, tag_id))

    def get_post_tags(self, post_id: int) -> List[Dict[str, Any]]:
        """Get tags for a post."""
        with self.db.get_connection() as conn:
            cursor = conn.cursor()
            cursor.execute('''
                SELECT t.*
                FROM tags t
                JOIN post_tags pt ON t.id = pt.tag_id
                WHERE pt.post_id = ?
            ''', (post_id,))
            return [dict(row) for row in cursor.fetchall()]


class TagRepository:
    """Repository for Tag operations."""

    def __init__(self, db: Database):
        self.db = db

    def create(self, name: str, slug: str) -> int:
        """Create a new tag."""
        with self.db.get_connection() as conn:
            cursor = conn.cursor()
            cursor.execute(
                'INSERT INTO tags (name, slug) VALUES (?, ?)',
                (name, slug)
            )
            return cursor.lastrowid

    def find_all(self) -> List[Dict[str, Any]]:
        """Get all tags."""
        with self.db.get_connection() as conn:
            cursor = conn.cursor()
            cursor.execute('SELECT * FROM tags ORDER BY name')
            return [dict(row) for row in cursor.fetchall()]

    def find_by_slug(self, slug: str) -> Optional[Dict[str, Any]]:
        """Find tag by slug."""
        with self.db.get_connection() as conn:
            cursor = conn.cursor()
            cursor.execute('SELECT * FROM tags WHERE slug = ?', (slug,))
            row = cursor.fetchone()
            return dict(row) if row else None

    def get_popular_tags(self, limit: int = 10) -> List[Dict[str, Any]]:
        """Get most popular tags by post count."""
        with self.db.get_connection() as conn:
            cursor = conn.cursor()
            cursor.execute('''
                SELECT t.*, COUNT(pt.post_id) as post_count
                FROM tags t
                LEFT JOIN post_tags pt ON t.id = pt.tag_id
                GROUP BY t.id
                ORDER BY post_count DESC
                LIMIT ?
            ''', (limit,))
            return [dict(row) for row in cursor.fetchall()]


def demo():
    """Demonstrate database operations."""
    import os

    # Clean up old database
    if os.path.exists('demo.db'):
        os.remove('demo.db')

    # Initialize database
    db = Database('demo.db')
    db.initialize()

    users_repo = UserRepository(db)
    posts_repo = PostRepository(db)
    tags_repo = TagRepository(db)

    print("=" * 60)
    print("SQLite Database Demo")
    print("=" * 60)

    # Create users
    print("\n--- Creating Users ---")
    alice_id = users_repo.create(
        'alice',
        'alice@example.com',
        'hashed_password_1',
        'Alice Johnson'
    )
    bob_id = users_repo.create(
        'bob',
        'bob@example.com',
        'hashed_password_2',
        'Bob Smith'
    )
    print(f"Created users: Alice (ID: {alice_id}), Bob (ID: {bob_id})")

    # Create posts
    print("\n--- Creating Posts ---")
    post1_id = posts_repo.create(
        alice_id,
        'First Post',
        'This is my first post!',
        'published'
    )
    post2_id = posts_repo.create(
        alice_id,
        'Second Post',
        'Another great post',
        'published'
    )
    post3_id = posts_repo.create(
        bob_id,
        'Bob\'s Post',
        'Hello from Bob',
        'draft'
    )
    print(f"Created {3} posts")

    # Create tags
    print("\n--- Creating Tags ---")
    python_tag = tags_repo.create('Python', 'python')
    js_tag = tags_repo.create('JavaScript', 'javascript')
    web_tag = tags_repo.create('Web Development', 'web-dev')
    print(f"Created {3} tags")

    # Add tags to posts
    posts_repo.add_tags(post1_id, [python_tag, web_tag])
    posts_repo.add_tags(post2_id, [js_tag, web_tag])

    # Query operations
    print("\n--- Querying Data ---")

    # Get user
    alice = users_repo.find_by_email('alice@example.com')
    print(f"Found user: {alice['username']} - {alice['email']}")

    # Get published posts
    published = posts_repo.find_all(status='published')
    print(f"\nPublished posts: {len(published)}")
    for post in published:
        print(f"  - {post['title']} by {post['username']}")

    # Get post with tags
    post = posts_repo.find_by_id(post1_id)
    tags = posts_repo.get_post_tags(post1_id)
    print(f"\nPost: {post['title']}")
    print(f"Tags: {', '.join(t['name'] for t in tags)}")

    # Get user stats
    stats = users_repo.get_user_stats(alice_id)
    print(f"\nAlice's stats:")
    print(f"  Posts: {stats['post_count']}")
    print(f"  Comments: {stats['comment_count']}")
    print(f"  Total views: {stats['total_views']}")

    # Get popular tags
    popular = tags_repo.get_popular_tags(5)
    print(f"\nPopular tags:")
    for tag in popular:
        print(f"  - {tag['name']}: {tag['post_count']} posts")

    print("\n" + "=" * 60)
    print("Demo completed!")
    print("=" * 60)


if __name__ == '__main__':
    demo()
