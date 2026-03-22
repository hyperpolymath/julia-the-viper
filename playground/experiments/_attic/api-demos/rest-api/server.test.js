const request = require('supertest');
const app = require('./server');

describe('REST API Tests', () => {
  describe('Health Check', () => {
    it('should return healthy status', async () => {
      const res = await request(app).get('/health');
      expect(res.status).toBe(200);
      expect(res.body).toHaveProperty('status', 'healthy');
      expect(res.body).toHaveProperty('timestamp');
    });
  });

  describe('API Info', () => {
    it('should return API information', async () => {
      const res = await request(app).get('/api');
      expect(res.status).toBe(200);
      expect(res.body).toHaveProperty('name');
      expect(res.body).toHaveProperty('endpoints');
    });
  });

  describe('User Endpoints', () => {
    it('should get all users', async () => {
      const res = await request(app).get('/api/users');
      expect(res.status).toBe(200);
      expect(res.body).toHaveProperty('count');
      expect(res.body).toHaveProperty('data');
      expect(Array.isArray(res.body.data)).toBe(true);
    });

    it('should get user by id', async () => {
      const res = await request(app).get('/api/users/1');
      expect(res.status).toBe(200);
      expect(res.body.data).toHaveProperty('id', 1);
      expect(res.body.data).toHaveProperty('name');
      expect(res.body.data).toHaveProperty('email');
    });

    it('should return 404 for non-existent user', async () => {
      const res = await request(app).get('/api/users/9999');
      expect(res.status).toBe(404);
      expect(res.body).toHaveProperty('error');
    });

    it('should create a new user', async () => {
      const newUser = {
        name: 'Test User',
        email: 'test@example.com',
        role: 'user'
      };

      const res = await request(app)
        .post('/api/users')
        .send(newUser);

      expect(res.status).toBe(201);
      expect(res.body.data).toHaveProperty('id');
      expect(res.body.data.name).toBe(newUser.name);
      expect(res.body.data.email).toBe(newUser.email);
    });

    it('should fail to create user without required fields', async () => {
      const res = await request(app)
        .post('/api/users')
        .send({ name: 'Test' });

      expect(res.status).toBe(400);
      expect(res.body).toHaveProperty('error');
    });

    it('should filter users by role', async () => {
      const res = await request(app).get('/api/users?role=admin');
      expect(res.status).toBe(200);
      expect(res.body.data.every(u => u.role === 'admin')).toBe(true);
    });

    it('should search users by name', async () => {
      const res = await request(app).get('/api/users?search=alice');
      expect(res.status).toBe(200);
      expect(res.body.data.length).toBeGreaterThan(0);
    });
  });

  describe('Post Endpoints', () => {
    it('should get all posts', async () => {
      const res = await request(app).get('/api/posts');
      expect(res.status).toBe(200);
      expect(res.body).toHaveProperty('count');
      expect(res.body).toHaveProperty('data');
      expect(Array.isArray(res.body.data)).toBe(true);
    });

    it('should get post by id', async () => {
      const res = await request(app).get('/api/posts/1');
      expect(res.status).toBe(200);
      expect(res.body.data).toHaveProperty('id', 1);
      expect(res.body.data).toHaveProperty('title');
      expect(res.body.data).toHaveProperty('content');
    });

    it('should create a new post', async () => {
      const newPost = {
        userId: 1,
        title: 'Test Post',
        content: 'This is a test post'
      };

      const res = await request(app)
        .post('/api/posts')
        .send(newPost);

      expect(res.status).toBe(201);
      expect(res.body.data).toHaveProperty('id');
      expect(res.body.data.title).toBe(newPost.title);
    });

    it('should fail to create post with invalid userId', async () => {
      const newPost = {
        userId: 9999,
        title: 'Test Post',
        content: 'This is a test post'
      };

      const res = await request(app)
        .post('/api/posts')
        .send(newPost);

      expect(res.status).toBe(400);
      expect(res.body).toHaveProperty('error');
    });

    it('should filter posts by userId', async () => {
      const res = await request(app).get('/api/posts?userId=1');
      expect(res.status).toBe(200);
      expect(res.body.data.every(p => p.userId === 1)).toBe(true);
    });

    it('should get user with their posts', async () => {
      const res = await request(app).get('/api/users/1/posts');
      expect(res.status).toBe(200);
      expect(res.body).toHaveProperty('user');
      expect(res.body).toHaveProperty('posts');
      expect(res.body).toHaveProperty('postCount');
      expect(Array.isArray(res.body.posts)).toBe(true);
    });
  });

  describe('Error Handling', () => {
    it('should return 404 for unknown routes', async () => {
      const res = await request(app).get('/api/unknown');
      expect(res.status).toBe(404);
      expect(res.body).toHaveProperty('error');
    });
  });
});
