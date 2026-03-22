# WebSocket Chat Demo

A real-time chat application built with Socket.IO and Express.js featuring multiple rooms, typing indicators, and user presence.

## Features

### Core Functionality
- **Real-time messaging**: Instant message delivery using WebSockets
- **Multiple chat rooms**: Switch between different rooms dynamically
- **User presence**: See who's online in each room
- **Typing indicators**: Know when someone is typing
- **Message history**: Persistent chat history per room
- **System notifications**: Join/leave announcements

### Technical Features
- Socket.IO for WebSocket communication
- In-memory storage (easily replaceable with database)
- RESTful API endpoints for stats
- Responsive web interface
- Clean, modern UI

## Installation

```bash
npm install
```

## Usage

### Start Server

```bash
# Development
npm run dev

# Production
npm start
```

The server will start on http://localhost:3001

### Access Chat

Open your browser and navigate to:
```
http://localhost:3001
```

## API Endpoints

### GET /api/stats

Get server statistics:

```bash
curl http://localhost:3001/api/stats
```

Response:
```json
{
  "totalUsers": 5,
  "totalRooms": 4,
  "rooms": [
    {
      "name": "General",
      "userCount": 3,
      "users": ["Alice", "Bob", "Charlie"],
      "createdAt": "2025-11-21T10:00:00.000Z"
    }
  ],
  "uptime": 3600
}
```

### GET /api/rooms

Get list of all rooms:

```bash
curl http://localhost:3001/api/rooms
```

## Socket.IO Events

### Client → Server

| Event | Data | Description |
|-------|------|-------------|
| `join` | `{username, room}` | Join a chat room |
| `send-message` | `{text}` | Send a message |
| `typing` | - | User is typing |
| `stop-typing` | - | User stopped typing |
| `switch-room` | `{room}` | Switch to different room |
| `private-message` | `{to, toId, text}` | Send private message |

### Server → Client

| Event | Data | Description |
|-------|------|-------------|
| `message` | `{user, text, timestamp, type}` | New message |
| `room-history` | `Array<Message>` | Room message history |
| `users-update` | `{users, count}` | Updated user list |
| `rooms-list` | `Array<Room>` | Available rooms |
| `user-typing` | `{username, userId}` | User typing notification |
| `user-stop-typing` | `{userId}` | User stopped typing |
| `private-message` | `{from, to, text, timestamp}` | Private message |

## Architecture

### Server Components

```
server.js
├── Express HTTP server
├── Socket.IO WebSocket server
├── In-memory data stores
│   ├── users (Map)
│   ├── rooms (Map)
│   └── messageHistory (Array)
└── Event handlers
    ├── connection
    ├── join
    ├── send-message
    ├── typing
    ├── switch-room
    └── disconnect
```

### Data Structures

**User**:
```javascript
{
  id: socket.id,
  username: string,
  room: string,
  joinedAt: Date
}
```

**Room**:
```javascript
{
  name: string,
  users: Set<userId>,
  messages: Array<Message>,
  createdAt: Date
}
```

**Message**:
```javascript
{
  id: number,
  user: string,
  userId: string,
  text: string,
  timestamp: Date,
  type: 'user' | 'system' | 'private'
}
```

## Features in Detail

### Multiple Rooms

- Default rooms: General, Random, Tech, Gaming
- Users can switch rooms dynamically
- Each room maintains independent message history
- User count per room

### Typing Indicators

- Real-time typing notifications
- Automatic timeout after 1 second of inactivity
- Shows individual or multiple users typing

### Message History

- Last 100 messages per room stored
- New users receive history on join
- Persists during server uptime

### User Presence

- Online users list per room
- Join/leave notifications
- User count tracking

## Client Features

### UI Components

- **Login Screen**: Username and room selection
- **Chat Header**: Room name and user count
- **Sidebar**: Rooms and users list
- **Messages Area**: Chat messages with timestamps
- **Input Area**: Message input with send button
- **Typing Indicator**: Shows who's typing

### User Experience

- Clean, modern interface
- Color-coded messages (own vs others)
- System messages for events
- Timestamps for all messages
- Smooth scrolling

## Extending the Application

### Add Database Persistence

Replace in-memory storage with MongoDB:

```javascript
const mongoose = require('mongoose');

const MessageSchema = new mongoose.Schema({
  room: String,
  user: String,
  text: String,
  timestamp: Date
});

const Message = mongoose.model('Message', MessageSchema);

// Save message
socket.on('send-message', async (data) => {
  const message = new Message({
    room: user.room,
    user: user.username,
    text: data.text,
    timestamp: new Date()
  });
  await message.save();
});
```

### Add Authentication

Implement JWT authentication:

```javascript
const jwt = require('jsonwebtoken');

io.use((socket, next) => {
  const token = socket.handshake.auth.token;
  try {
    const decoded = jwt.verify(token, SECRET);
    socket.userId = decoded.userId;
    next();
  } catch (err) {
    next(new Error('Authentication error'));
  }
});
```

### Add Private Messaging

Already implemented! Use:

```javascript
socket.emit('private-message', {
  to: 'username',
  toId: 'socket-id',
  text: 'Hello!'
});
```

### Add File Sharing

```javascript
socket.on('send-file', (data) => {
  // Handle file upload
  const { filename, content, mimeType } = data;
  // Store and broadcast file
});
```

### Add Message Reactions

```javascript
socket.on('add-reaction', (data) => {
  const { messageId, emoji } = data;
  io.to(room).emit('message-reaction', {
    messageId,
    emoji,
    user: username
  });
});
```

## Security Considerations

1. **Input Validation**: Validate all user inputs
2. **Rate Limiting**: Prevent message spam
3. **Authentication**: Add user authentication
4. **XSS Protection**: Sanitize message content
5. **CORS**: Configure appropriate CORS settings

```javascript
// Rate limiting example
const rateLimit = require('express-rate-limit');

const limiter = rateLimit({
  windowMs: 15 * 60 * 1000,
  max: 100
});

app.use('/api/', limiter);
```

## Testing

Test Socket.IO events:

```javascript
const io = require('socket.io-client');
const socket = io('http://localhost:3001');

socket.on('connect', () => {
  socket.emit('join', { username: 'TestUser', room: 'general' });
});

socket.on('message', (data) => {
  console.log('Received:', data);
});
```

## Performance Optimization

1. **Redis for Scaling**: Use Redis adapter for multiple servers
2. **Message Pagination**: Load messages in chunks
3. **Compression**: Enable Socket.IO compression
4. **Connection Pooling**: Limit concurrent connections

```javascript
// Redis adapter
const redisAdapter = require('socket.io-redis');
io.adapter(redisAdapter({ host: 'localhost', port: 6379 }));
```

## Deployment

### Environment Variables

```env
PORT=3001
NODE_ENV=production
```

### Production Setup

```bash
# Use process manager
pm2 start server.js -i max

# Or Docker
docker build -t chat-app .
docker run -p 3001:3001 chat-app
```

## Troubleshooting

### Connection Issues

- Check firewall settings
- Verify WebSocket support
- Check CORS configuration

### Message Delays

- Monitor server resources
- Check network latency
- Consider Redis for scaling

## License

MIT License
