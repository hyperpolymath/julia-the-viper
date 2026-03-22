/**
 * ArangoDB Multi-Model Database Demo with AQL
 *
 * Demonstrates:
 * - Document model (like MongoDB)
 * - Graph model (relationships and traversals)
 * - Key-value model
 * - AQL (ArangoDB Query Language)
 * - Complex joins and aggregations
 * - Graph traversal patterns
 */

import { Database } from "https://deno.land/x/arangojs@8.0.0/mod.ts";

class ArangoDBDemo {
    constructor(url = "http://localhost:8529") {
        this.db = new Database({
            url,
            databaseName: "playground",
            auth: { username: "root", password: "rootpassword" }
        });
    }

    async initialize() {
        console.log("🔧 Initializing ArangoDB...");

        try {
            // Create database if doesn't exist
            const databases = await this.db.listDatabases();
            if (!databases.includes("playground")) {
                await this.db.createDatabase("playground");
                this.db.useDatabase("playground");
            }

            // Create collections
            await this.createCollections();

            // Create graph
            await this.createGraph();

            // Insert sample data
            await this.insertSampleData();

            console.log("✅ Database initialized");
        } catch (error) {
            console.error("❌ Initialization error:", error);
            throw error;
        }
    }

    async createCollections() {
        const collections = [
            { name: "users", type: "document" },
            { name: "posts", type: "document" },
            { name: "comments", type: "document" },
            { name: "tags", type: "document" },
            { name: "follows", type: "edge" },
            { name: "likes", type: "edge" },
            { name: "tagged_with", type: "edge" }
        ];

        for (const { name, type } of collections) {
            const existing = await this.db.listCollections();
            if (!existing.find(c => c.name === name)) {
                if (type === "edge") {
                    await this.db.createEdgeCollection(name);
                } else {
                    await this.db.createCollection(name);
                }
                console.log(`Created ${type} collection: ${name}`);
            }
        }
    }

    async createGraph() {
        const graphs = await this.db.listGraphs();

        if (!graphs.find(g => g._key === "social")) {
            await this.db.createGraph("social", [
                {
                    collection: "follows",
                    from: ["users"],
                    to: ["users"]
                },
                {
                    collection: "likes",
                    from: ["users"],
                    to: ["posts"]
                },
                {
                    collection: "tagged_with",
                    from: ["posts"],
                    to: ["tags"]
                }
            ]);
            console.log("Created social graph");
        }
    }

    async insertSampleData() {
        const users = this.db.collection("users");
        const posts = this.db.collection("posts");
        const tags = this.db.collection("tags");
        const follows = this.db.collection("follows");
        const likes = this.db.collection("likes");
        const tagged_with = this.db.collection("tagged_with");

        // Insert users
        const userData = [
            { _key: "alice", name: "Alice Johnson", email: "alice@example.com", age: 28, city: "New York" },
            { _key: "bob", name: "Bob Smith", email: "bob@example.com", age: 35, city: "San Francisco" },
            { _key: "charlie", name: "Charlie Davis", email: "charlie@example.com", age: 42, city: "Austin" },
            { _key: "diana", name: "Diana Miller", email: "diana@example.com", age: 31, city: "Seattle" }
        ];

        for (const user of userData) {
            await users.save(user, { overwriteMode: "ignore" });
        }

        // Insert tags
        const tagData = [
            { _key: "javascript", name: "JavaScript", category: "programming" },
            { _key: "arangodb", name: "ArangoDB", category: "database" },
            { _key: "webdev", name: "Web Development", category: "general" },
            { _key: "performance", name: "Performance", category: "optimization" }
        ];

        for (const tag of tagData) {
            await tags.save(tag, { overwriteMode: "ignore" });
        }

        // Insert posts
        const postData = [
            {
                _key: "post1",
                author: "users/alice",
                title: "Getting Started with ArangoDB",
                content: "ArangoDB is a multi-model database...",
                views: 150,
                created_at: new Date("2025-01-15")
            },
            {
                _key: "post2",
                author: "users/bob",
                title: "JavaScript Best Practices",
                content: "Here are some best practices...",
                views: 320,
                created_at: new Date("2025-01-18")
            },
            {
                _key: "post3",
                author: "users/alice",
                title: "Graph Databases Explained",
                content: "Graph databases model data...",
                views: 280,
                created_at: new Date("2025-01-20")
            }
        ];

        for (const post of postData) {
            await posts.save(post, { overwriteMode: "ignore" });
        }

        // Create follows relationships
        const followsData = [
            { _from: "users/bob", _to: "users/alice" },
            { _from: "users/charlie", _to: "users/alice" },
            { _from: "users/diana", _to: "users/alice" },
            { _from: "users/alice", _to: "users/bob" },
            { _from: "users/charlie", _to: "users/bob" }
        ];

        for (const follow of followsData) {
            await follows.save(follow, { overwriteMode: "ignore" });
        }

        // Create likes relationships
        const likesData = [
            { _from: "users/bob", _to: "posts/post1", liked_at: new Date() },
            { _from: "users/charlie", _to: "posts/post1", liked_at: new Date() },
            { _from: "users/diana", _to: "posts/post1", liked_at: new Date() },
            { _from: "users/alice", _to: "posts/post2", liked_at: new Date() }
        ];

        for (const like of likesData) {
            await likes.save(like, { overwriteMode: "ignore" });
        }

        // Create post-tag relationships
        const taggedData = [
            { _from: "posts/post1", _to: "tags/arangodb" },
            { _from: "posts/post1", _to: "tags/webdev" },
            { _from: "posts/post2", _to: "tags/javascript" },
            { _from: "posts/post2", _to: "tags/webdev" },
            { _from: "posts/post3", _to: "tags/arangodb" }
        ];

        for (const tagged of taggedData) {
            await tagged_with.save(tagged, { overwriteMode: "ignore" });
        }

        console.log("✅ Sample data inserted");
    }

    // AQL QUERIES

    async getAllUsers() {
        console.log("\n📋 Query: Get all users");

        const query = `
            FOR user IN users
                SORT user.name
                RETURN user
        `;

        const cursor = await this.db.query(query);
        const results = await cursor.all();
        console.log(results);
        return results;
    }

    async getUsersWithFollowerCount() {
        console.log("\n📊 Query: Users with follower count");

        const query = `
            FOR user IN users
                LET followerCount = LENGTH(
                    FOR follow IN follows
                        FILTER follow._to == user._id
                        RETURN 1
                )
                RETURN {
                    name: user.name,
                    email: user.email,
                    followers: followerCount
                }
        `;

        const cursor = await this.db.query(query);
        const results = await cursor.all();
        console.log(results);
        return results;
    }

    async getPopularPosts(minLikes = 2) {
        console.log(`\n🔥 Query: Popular posts (min ${minLikes} likes)`);

        const query = `
            FOR post IN posts
                LET likeCount = LENGTH(
                    FOR like IN likes
                        FILTER like._to == post._id
                        RETURN 1
                )
                FILTER likeCount >= @minLikes
                SORT likeCount DESC
                RETURN {
                    title: post.title,
                    author: DOCUMENT(post.author).name,
                    likes: likeCount,
                    views: post.views
                }
        `;

        const cursor = await this.db.query(query, { minLikes });
        const results = await cursor.all();
        console.log(results);
        return results;
    }

    async getPostsWithTags() {
        console.log("\n🏷️  Query: Posts with their tags");

        const query = `
            FOR post IN posts
                LET postTags = (
                    FOR v IN 1..1 OUTBOUND post tagged_with
                        RETURN v.name
                )
                RETURN {
                    title: post.title,
                    author: DOCUMENT(post.author).name,
                    tags: postTags,
                    views: post.views
                }
        `;

        const cursor = await this.db.query(query);
        const results = await cursor.all();
        console.log(results);
        return results;
    }

    async graphTraversal_FollowersOfFollowers(username) {
        console.log(`\n🔗 Graph Traversal: Followers of ${username}'s followers`);

        const query = `
            FOR user IN users
                FILTER user._key == @username
                FOR v, e, p IN 2..2 INBOUND user follows
                    FILTER v._key != @username
                    RETURN DISTINCT {
                        name: v.name,
                        distance: LENGTH(p.edges)
                    }
        `;

        const cursor = await this.db.query(query, { username });
        const results = await cursor.all();
        console.log(results);
        return results;
    }

    async graphTraversal_ShortestPath(from, to) {
        console.log(`\n🗺️  Shortest path: ${from} → ${to}`);

        const query = `
            FOR user1 IN users
                FILTER user1._key == @from
                FOR user2 IN users
                    FILTER user2._key == @to
                    LET path = (
                        FOR v, e IN OUTBOUND SHORTEST_PATH user1 TO user2 follows
                            RETURN v.name
                    )
                    RETURN path
        `;

        const cursor = await this.db.query(query, { from, to });
        const results = await cursor.all();
        console.log(results[0]);
        return results[0];
    }

    async complexAggregation() {
        console.log("\n📈 Complex Aggregation: User engagement stats");

        const query = `
            FOR user IN users
                LET userPosts = (
                    FOR post IN posts
                        FILTER post.author == user._id
                        RETURN post
                )
                LET totalViews = SUM(userPosts[*].views)
                LET totalLikes = SUM(
                    FOR post IN userPosts
                        LET likes = LENGTH(
                            FOR like IN likes
                                FILTER like._to == post._id
                                RETURN 1
                        )
                        RETURN likes
                )
                LET followerCount = LENGTH(
                    FOR follow IN follows
                        FILTER follow._to == user._id
                        RETURN 1
                )
                RETURN {
                    user: user.name,
                    posts: LENGTH(userPosts),
                    totalViews: totalViews,
                    totalLikes: totalLikes,
                    followers: followerCount,
                    avgViewsPerPost: totalViews / (LENGTH(userPosts) || 1)
                }
        `;

        const cursor = await this.db.query(query);
        const results = await cursor.all();
        console.log(results);
        return results;
    }

    async fullTextSearch(searchTerm) {
        console.log(`\n🔍 Full-text search: "${searchTerm}"`);

        const query = `
            FOR post IN posts
                FILTER CONTAINS(LOWER(post.title), LOWER(@searchTerm)) OR
                       CONTAINS(LOWER(post.content), LOWER(@searchTerm))
                RETURN {
                    title: post.title,
                    author: DOCUMENT(post.author).name,
                    excerpt: SUBSTRING(post.content, 0, 100)
                }
        `;

        const cursor = await this.db.query(query, { searchTerm });
        const results = await cursor.all();
        console.log(results);
        return results;
    }

    async recommendation_SimilarPosts(postKey) {
        console.log(`\n💡 Recommendation: Posts similar to ${postKey}`);

        const query = `
            FOR post IN posts
                FILTER post._key == @postKey
                LET postTags = (
                    FOR v IN 1..1 OUTBOUND post tagged_with
                        RETURN v._id
                )
                FOR otherPost IN posts
                    FILTER otherPost._key != @postKey
                    LET otherTags = (
                        FOR v IN 1..1 OUTBOUND otherPost tagged_with
                            RETURN v._id
                    )
                    LET commonTags = LENGTH(
                        INTERSECTION(postTags, otherTags)
                    )
                    FILTER commonTags > 0
                    SORT commonTags DESC
                    LIMIT 3
                    RETURN {
                        title: otherPost.title,
                        commonTags: commonTags,
                        views: otherPost.views
                    }
        `;

        const cursor = await this.db.query(query, { postKey });
        const results = await cursor.all();
        console.log(results);
        return results;
    }
}

// Demo execution
async function demo() {
    console.log("=".repeat(60));
    console.log("ArangoDB Multi-Model Database Demo");
    console.log("=".repeat(60));

    const arango = new ArangoDBDemo();

    try {
        await arango.initialize();

        // Run queries
        await arango.getAllUsers();
        await arango.getUsersWithFollowerCount();
        await arango.getPopularPosts(2);
        await arango.getPostsWithTags();
        await arango.graphTraversal_FollowersOfFollowers("alice");
        await arango.graphTraversal_ShortestPath("alice", "diana");
        await arango.complexAggregation();
        await arango.fullTextSearch("database");
        await arango.recommendation_SimilarPosts("post1");

    } catch (error) {
        console.error("Demo error:", error);
    }
}

// Run if main module
if (import.meta.main) {
    await demo();
}

export { ArangoDBDemo };
