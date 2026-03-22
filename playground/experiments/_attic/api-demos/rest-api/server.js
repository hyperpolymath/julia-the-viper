const express = require('express');
const cors = require('cors');
const helmet = require('helmet');
const morgan = require('morgan');
require('dotenv').config();

const app = express();
const PORT = process.env.PORT || 3000;

// Middleware
app.use(helmet());
app.use(cors());
app.use(morgan('dev'));
app.use(express.json());
app.use(express.urlencoded({ extended: true }));

// In-memory database (for demo purposes)
let users = [
  { id: 1, name: 'Alice Johnson', email: 'alice@example.com', role: 'admin' },
  { id: 2, name: 'Bob Smith', email: 'bob@example.com', role: 'user' },
  { id: 3, name: 'Charlie Davis', email: 'charlie@example.com', role: 'user' }
];

let posts = [
  { id: 1, userId: 1, title: 'First Post', content: 'This is the first post', createdAt: new Date() },
  { id: 2, userId: 2, title: 'Second Post', content: 'This is the second post', createdAt: new Date() }
];

let nextUserId = 4;
let nextPostId = 3;

// Middleware for logging requests
const requestLogger = (req, res, next) => {
  console.log(`[${new Date().toISOString()}] ${req.method} ${req.path}`);
  next();
};

app.use(requestLogger);

// Error handling middleware
const errorHandler = (err, req, res, next) => {
  console.error(err.stack);
  res.status(err.status || 500).json({
    error: {
      message: err.message || 'Internal Server Error',
      status: err.status || 500
    }
  });
};

// Routes

// Health check
app.get('/health', (req, res) => {
  res.json({ status: 'healthy', timestamp: new Date().toISOString() });
});

// API Info
app.get('/api', (req, res) => {
  res.json({
    name: 'REST API Demo',
    version: '1.0.0',
    endpoints: {
      users: '/api/users',
      posts: '/api/posts',
      health: '/health'
    }
  });
});

// USER ROUTES

// Get all users
app.get('/api/users', (req, res) => {
  const { role, search } = req.query;

  let filteredUsers = users;

  if (role) {
    filteredUsers = filteredUsers.filter(u => u.role === role);
  }

  if (search) {
    filteredUsers = filteredUsers.filter(u =>
      u.name.toLowerCase().includes(search.toLowerCase()) ||
      u.email.toLowerCase().includes(search.toLowerCase())
    );
  }

  res.json({
    count: filteredUsers.length,
    data: filteredUsers
  });
});

// Get user by ID
app.get('/api/users/:id', (req, res) => {
  const user = users.find(u => u.id === parseInt(req.params.id));

  if (!user) {
    return res.status(404).json({ error: 'User not found' });
  }

  res.json({ data: user });
});

// Create new user
app.post('/api/users', (req, res) => {
  const { name, email, role } = req.body;

  if (!name || !email) {
    return res.status(400).json({ error: 'Name and email are required' });
  }

  const newUser = {
    id: nextUserId++,
    name,
    email,
    role: role || 'user'
  };

  users.push(newUser);
  res.status(201).json({ data: newUser });
});

// Update user
app.put('/api/users/:id', (req, res) => {
  const userId = parseInt(req.params.id);
  const userIndex = users.findIndex(u => u.id === userId);

  if (userIndex === -1) {
    return res.status(404).json({ error: 'User not found' });
  }

  const { name, email, role } = req.body;

  users[userIndex] = {
    ...users[userIndex],
    ...(name && { name }),
    ...(email && { email }),
    ...(role && { role })
  };

  res.json({ data: users[userIndex] });
});

// Delete user
app.delete('/api/users/:id', (req, res) => {
  const userId = parseInt(req.params.id);
  const userIndex = users.findIndex(u => u.id === userId);

  if (userIndex === -1) {
    return res.status(404).json({ error: 'User not found' });
  }

  users.splice(userIndex, 1);
  res.status(204).send();
});

// POST ROUTES

// Get all posts
app.get('/api/posts', (req, res) => {
  const { userId } = req.query;

  let filteredPosts = posts;

  if (userId) {
    filteredPosts = filteredPosts.filter(p => p.userId === parseInt(userId));
  }

  res.json({
    count: filteredPosts.length,
    data: filteredPosts
  });
});

// Get post by ID
app.get('/api/posts/:id', (req, res) => {
  const post = posts.find(p => p.id === parseInt(req.params.id));

  if (!post) {
    return res.status(404).json({ error: 'Post not found' });
  }

  res.json({ data: post });
});

// Create new post
app.post('/api/posts', (req, res) => {
  const { userId, title, content } = req.body;

  if (!userId || !title || !content) {
    return res.status(400).json({ error: 'userId, title, and content are required' });
  }

  const user = users.find(u => u.id === parseInt(userId));
  if (!user) {
    return res.status(400).json({ error: 'Invalid userId' });
  }

  const newPost = {
    id: nextPostId++,
    userId: parseInt(userId),
    title,
    content,
    createdAt: new Date()
  };

  posts.push(newPost);
  res.status(201).json({ data: newPost });
});

// Update post
app.put('/api/posts/:id', (req, res) => {
  const postId = parseInt(req.params.id);
  const postIndex = posts.findIndex(p => p.id === postId);

  if (postIndex === -1) {
    return res.status(404).json({ error: 'Post not found' });
  }

  const { title, content } = req.body;

  posts[postIndex] = {
    ...posts[postIndex],
    ...(title && { title }),
    ...(content && { content }),
    updatedAt: new Date()
  };

  res.json({ data: posts[postIndex] });
});

// Delete post
app.delete('/api/posts/:id', (req, res) => {
  const postId = parseInt(req.params.id);
  const postIndex = posts.findIndex(p => p.id === postId);

  if (postIndex === -1) {
    return res.status(404).json({ error: 'Post not found' });
  }

  posts.splice(postIndex, 1);
  res.status(204).send();
});

// Get user with their posts
app.get('/api/users/:id/posts', (req, res) => {
  const userId = parseInt(req.params.id);
  const user = users.find(u => u.id === userId);

  if (!user) {
    return res.status(404).json({ error: 'User not found' });
  }

  const userPosts = posts.filter(p => p.userId === userId);

  res.json({
    user,
    posts: userPosts,
    postCount: userPosts.length
  });
});

// 404 handler
app.use((req, res) => {
  res.status(404).json({ error: 'Route not found' });
});

// Error handling
app.use(errorHandler);

// Start server
if (require.main === module) {
  app.listen(PORT, () => {
    console.log(`Server running on http://localhost:${PORT}`);
    console.log(`API documentation: http://localhost:${PORT}/api`);
  });
}

module.exports = app;
