# REST API Demo

A comprehensive REST API built with Express.js demonstrating best practices.

## Features

- RESTful endpoints for users and posts
- CRUD operations
- Query parameters for filtering
- Error handling
- Request logging
- Security middleware (Helmet)
- CORS support
- In-memory data storage

## Installation

```bash
npm install
```

## Usage

### Development
```bash
npm run dev
```

### Production
```bash
npm start
```

### Testing
```bash
npm test
```

## API Endpoints

### Health Check
- `GET /health` - Server health status

### API Info
- `GET /api` - API documentation

### Users
- `GET /api/users` - Get all users
  - Query params: `?role=admin`, `?search=alice`
- `GET /api/users/:id` - Get user by ID
- `POST /api/users` - Create new user
- `PUT /api/users/:id` - Update user
- `DELETE /api/users/:id` - Delete user
- `GET /api/users/:id/posts` - Get user with their posts

### Posts
- `GET /api/posts` - Get all posts
  - Query params: `?userId=1`
- `GET /api/posts/:id` - Get post by ID
- `POST /api/posts` - Create new post
- `PUT /api/posts/:id` - Update post
- `DELETE /api/posts/:id` - Delete post

## Example Requests

### Create User
```bash
curl -X POST http://localhost:3000/api/users \
  -H "Content-Type: application/json" \
  -d '{"name":"John Doe","email":"john@example.com","role":"user"}'
```

### Get All Users
```bash
curl http://localhost:3000/api/users
```

### Filter Users by Role
```bash
curl http://localhost:3000/api/users?role=admin
```

### Create Post
```bash
curl -X POST http://localhost:3000/api/posts \
  -H "Content-Type: application/json" \
  -d '{"userId":1,"title":"My Post","content":"Post content here"}'
```

### Update Post
```bash
curl -X PUT http://localhost:3000/api/posts/1 \
  -H "Content-Type: application/json" \
  -d '{"title":"Updated Title"}'
```

### Delete Post
```bash
curl -X DELETE http://localhost:3000/api/posts/1
```

## Environment Variables

Create a `.env` file:

```
PORT=3000
NODE_ENV=development
```

## Architecture

- **Middleware**: Helmet (security), CORS, Morgan (logging)
- **Error Handling**: Centralized error handler
- **Data Storage**: In-memory (can be replaced with database)
- **Validation**: Request body validation
- **Logging**: Request/response logging

## Next Steps

- Add database integration (PostgreSQL, MongoDB)
- Implement authentication (JWT)
- Add input validation library (Joi, express-validator)
- Add API documentation (Swagger)
- Implement rate limiting
- Add caching layer
