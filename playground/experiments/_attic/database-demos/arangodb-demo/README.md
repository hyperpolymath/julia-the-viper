# ArangoDB Multi-Model Database Demo

Comprehensive demonstration of ArangoDB's multi-model capabilities using AQL (ArangoDB Query Language).

## Why ArangoDB?

- **Multi-Model**: Document, Graph, and Key-Value in one database
- **AQL**: Powerful query language combining SQL-like and graph traversal
- **Performance**: Native graph traversal, no joins needed
- **ACID**: Full ACID transactions
- **Scalability**: Horizontal scaling with sharding
- **Flexible Schema**: JSON documents with optional validation

## Models Supported

### 1. Document Model
Like MongoDB - store JSON documents in collections.

### 2. Graph Model
Native graph database with vertices and edges for relationships.

### 3. Key-Value Model
Simple key-value storage for caching and sessions.

## Installation

### Using Podman

```bash
podman run -d \
  --name arangodb \
  -p 8529:8529 \
  -e ARANGO_ROOT_PASSWORD=rootpassword \
  docker.io/arangodb/arangodb:latest
```

### Using ArangoDB Directly

```bash
# macOS
brew install arangodb

# Linux (Debian/Ubuntu)
curl -OL https://download.arangodb.com/arangodb39/DEBIAN/Release.key
sudo apt-key add - < Release.key
echo 'deb https://download.arangodb.com/arangodb39/DEBIAN/ /' | sudo tee /etc/apt/sources.list.d/arangodb.list
sudo apt update
sudo apt install arangodb3
```

## Schema Design

### Collections

**Document Collections:**
- `users` - User profiles
- `posts` - Blog posts
- `comments` - Post comments
- `tags` - Content tags

**Edge Collections:**
- `follows` - User → User relationships
- `likes` - User → Post relationships
- `tagged_with` - Post → Tag relationships

### Graph

```
social graph:
  - users → follows → users
  - users → likes → posts
  - posts → tagged_with → tags
```

## Running the Demo

```bash
# Start ArangoDB
podman run -d -p 8529:8529 -e ARANGO_ROOT_PASSWORD=rootpassword arangodb

# Run demo with Deno
deno run --allow-net queries.js
```

## AQL Query Examples

### Basic Queries

#### Get all users

```aql
FOR user IN users
    SORT user.name
    RETURN user
```

#### Filter by condition

```aql
FOR user IN users
    FILTER user.age > 30
    RETURN { name: user.name, age: user.age }
```

#### Pagination

```aql
FOR user IN users
    SORT user.name
    LIMIT 10, 20  // Skip 10, return 20
    RETURN user
```

### Aggregations

#### Count by group

```aql
FOR user IN users
    COLLECT city = user.city WITH COUNT INTO count
    RETURN { city, userCount: count }
```

#### Complex aggregation

```aql
FOR post IN posts
    COLLECT author = post.author
    AGGREGATE
        totalViews = SUM(post.views),
        avgViews = AVG(post.views),
        postCount = LENGTH(post)
    RETURN {
        author: DOCUMENT(author).name,
        posts: postCount,
        totalViews,
        avgViews
    }
```

### Joins (Document Model)

#### Join with DOCUMENT()

```aql
FOR post IN posts
    RETURN {
        title: post.title,
        author: DOCUMENT(post.author).name,
        views: post.views
    }
```

#### Subquery joins

```aql
FOR user IN users
    LET userPosts = (
        FOR post IN posts
            FILTER post.author == user._id
            RETURN post
    )
    RETURN {
        user: user.name,
        postCount: LENGTH(userPosts),
        posts: userPosts[*].title
    }
```

### Graph Traversal

#### Simple traversal

```aql
FOR user IN users
    FILTER user._key == 'alice'
    FOR follower IN 1..1 INBOUND user follows
        RETURN follower.name
```

#### Multi-level traversal

```aql
// Followers of followers (2 hops)
FOR user IN users
    FILTER user._key == 'alice'
    FOR v IN 2..2 INBOUND user follows
        RETURN DISTINCT v.name
```

#### Shortest path

```aql
FOR user1 IN users
    FILTER user1._key == 'alice'
    FOR user2 IN users
        FILTER user2._key == 'bob'
        FOR v IN OUTBOUND SHORTEST_PATH user1 TO user2 follows
            RETURN v.name
```

#### All paths

```aql
FOR user1 IN users
    FILTER user1._key == 'alice'
    FOR user2 IN users
        FILTER user2._key == 'bob'
        FOR path IN OUTBOUND K_SHORTEST_PATHS user1 TO user2 follows
            LIMIT 5
            RETURN {
                path: path.vertices[*].name,
                length: LENGTH(path.edges)
            }
```

#### Pattern matching

```aql
FOR user IN users
    FOR v, e, p IN 1..3 OUTBOUND user follows
        FILTER v.city == 'New York'
        RETURN {
            user: user.name,
            connection: v.name,
            distance: LENGTH(p.edges)
        }
```

### Advanced Queries

#### Full-text search

```aql
FOR post IN posts
    FILTER CONTAINS(LOWER(post.title), LOWER('database')) OR
           CONTAINS(LOWER(post.content), LOWER('database'))
    RETURN post
```

#### Recommendation engine

```aql
// Posts similar to a given post (by tags)
FOR post IN posts
    FILTER post._key == 'post1'
    LET postTags = (
        FOR v IN 1..1 OUTBOUND post tagged_with
            RETURN v._id
    )
    FOR otherPost IN posts
        FILTER otherPost._key != 'post1'
        LET otherTags = (
            FOR v IN 1..1 OUTBOUND otherPost tagged_with
                RETURN v._id
        )
        LET similarity = LENGTH(INTERSECTION(postTags, otherTags))
        FILTER similarity > 0
        SORT similarity DESC
        LIMIT 5
        RETURN {
            title: otherPost.title,
            similarityScore: similarity
        }
```

#### Time-series aggregation

```aql
FOR post IN posts
    COLLECT
        month = DATE_FORMAT(post.created_at, '%yyyy-%mm')
    AGGREGATE
        count = LENGTH(post),
        totalViews = SUM(post.views)
    SORT month
    RETURN { month, posts: count, views: totalViews }
```

#### Geospatial queries

```aql
// Assuming users have lat/lng
FOR user IN users
    LET distance = DISTANCE(
        user.latitude, user.longitude,
        40.7128, -74.0060  // New York coordinates
    )
    FILTER distance < 50000  // Within 50km
    SORT distance
    RETURN {
        name: user.name,
        distanceKm: distance / 1000
    }
```

### Transactions

```javascript
const trx = await db.beginTransaction({
    write: ["users", "posts", "follows"]
});

try {
    // Create user
    await trx.step(() => users.save({
        _key: "newuser",
        name: "New User"
    }));

    // Create post
    await trx.step(() => posts.save({
        author: "users/newuser",
        title: "First Post"
    }));

    // Commit
    await trx.commit();
} catch (error) {
    await trx.abort();
    throw error;
}
```

## Query Optimization

### Use Indexes

```javascript
// Hash index
await collection.ensureIndex({
    type: "hash",
    fields: ["email"],
    unique: true
});

// Skiplist index (for range queries)
await collection.ensureIndex({
    type: "skiplist",
    fields: ["age"]
});

// Fulltext index
await collection.ensureIndex({
    type: "fulltext",
    fields: ["content"],
    minLength: 3
});

// Geo index
await collection.ensureIndex({
    type: "geo",
    fields: ["location"]
});
```

### Explain Query

```aql
EXPLAIN
FOR user IN users
    FILTER user.age > 30
    RETURN user
```

### Query Profiling

```javascript
const cursor = await db.query(aql`
    FOR user IN users RETURN user
`, { profile: true });

const profile = cursor.getExtra().profile;
console.log(profile);
```

## Best Practices

### 1. Index Strategy

```aql
// ✅ Good: Indexed field first
FOR user IN users
    FILTER user.email == @email
    FILTER user.age > 30
    RETURN user

// ❌ Bad: Non-indexed field first
FOR user IN users
    FILTER user.someField == @value
    FILTER user.email == @email
    RETURN user
```

### 2. Use LIMIT

```aql
// ✅ Good: Limit early
FOR user IN users
    SORT user.created_at DESC
    LIMIT 10
    RETURN user

// ❌ Bad: No limit
FOR user IN users
    SORT user.created_at DESC
    RETURN user
```

### 3. Avoid Unnecessary COLLECT

```aql
// ✅ Good: Use LENGTH()
FOR user IN users
    LET postCount = LENGTH(
        FOR post IN posts
            FILTER post.author == user._id
            RETURN 1
    )
    RETURN { user: user.name, posts: postCount }

// ❌ Bad: Unnecessary COLLECT
FOR user IN users
    LET posts = (
        FOR post IN posts
            FILTER post.author == user._id
            RETURN post
    )
    COLLECT u = user WITH COUNT INTO count
    RETURN { user: u.name, posts: count }
```

### 4. Use Graph Traversal

```aql
// ✅ Good: Native graph traversal
FOR v IN 1..3 OUTBOUND @startVertex follows
    RETURN v

// ❌ Bad: Manual joins
FOR user1 IN users
    FOR follow1 IN follows
        FILTER follow1._from == user1._id
        FOR user2 IN users
            FILTER user2._id == follow1._to
            RETURN user2
```

## Common Patterns

### Activity Feed

```aql
FOR user IN users
    FILTER user._key == @currentUser
    // Get followed users
    FOR followed IN 1..1 OUTBOUND user follows
        // Get their posts
        FOR post IN posts
            FILTER post.author == followed._id
            SORT post.created_at DESC
            LIMIT 20
            RETURN {
                post: post,
                author: followed.name
            }
```

### Friend Recommendations

```aql
FOR user IN users
    FILTER user._key == @currentUser
    // Friends of friends, excluding current follows
    FOR v IN 2..2 OUTBOUND user follows
        FILTER v._id != user._id
        LET alreadyFollowing = (
            FOR f IN 1..1 OUTBOUND user follows
                FILTER f._id == v._id
                RETURN 1
        )
        FILTER LENGTH(alreadyFollowing) == 0
        COLLECT suggestion = v WITH COUNT INTO mutualFriends
        SORT mutualFriends DESC
        LIMIT 10
        RETURN {
            user: suggestion.name,
            mutualFriends
        }
```

### Trending Content

```aql
LET timeWindow = DATE_SUBTRACT(DATE_NOW(), 7, 'days')

FOR post IN posts
    FILTER post.created_at >= timeWindow
    LET likes = LENGTH(
        FOR like IN likes
            FILTER like._to == post._id
            RETURN 1
    )
    LET comments = LENGTH(
        FOR comment IN comments
            FILTER comment.post_id == post._id
            RETURN 1
    )
    LET score = (likes * 2) + (comments * 3) + post.views
    SORT score DESC
    LIMIT 10
    RETURN {
        title: post.title,
        score,
        likes,
        comments,
        views: post.views
    }
```

## Web UI

Access ArangoDB web interface:
```
http://localhost:8529
```

Credentials:
- Username: `root`
- Password: `rootpassword`

## CLI Tools

```bash
# ArangoDB shell
arangosh --server.endpoint tcp://127.0.0.1:8529

# Import data
arangoimport --file data.json --collection users --type json

# Export data
arangoexport --collection users --output-directory ./export

# Backup
arangodump --output-directory ./backup

# Restore
arangorestore --input-directory ./backup
```

## Performance Tips

1. **Use appropriate indexes**
2. **Limit result sets**
3. **Avoid deep traversals when possible**
4. **Use COLLECT wisely**
5. **Profile queries in development**
6. **Batch operations when possible**
7. **Use bind parameters** (@variable)
8. **Cache frequently accessed data**

## License

MIT License
