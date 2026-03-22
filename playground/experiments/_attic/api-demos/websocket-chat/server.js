const express = require('express');
const http = require('http');
const socketIO = require('socket.io');
const path = require('path');

const app = express();
const server = http.createServer(app);
const io = socketIO(server, {
  cors: {
    origin: "*",
    methods: ["GET", "POST"]
  }
});

const PORT = process.env.PORT || 3001;

// Serve static files
app.use(express.static('public'));

// In-memory storage
const users = new Map();
const rooms = new Map();
const messageHistory = [];
const MAX_HISTORY = 100;

// Room class
class Room {
  constructor(name) {
    this.name = name;
    this.users = new Set();
    this.messages = [];
    this.createdAt = new Date();
  }

  addUser(userId) {
    this.users.add(userId);
  }

  removeUser(userId) {
    this.users.delete(userId);
  }

  addMessage(message) {
    this.messages.push(message);
    if (this.messages.length > MAX_HISTORY) {
      this.messages.shift();
    }
  }

  getUserCount() {
    return this.users.size;
  }

  getInfo() {
    return {
      name: this.name,
      userCount: this.getUserCount(),
      users: Array.from(this.users).map(id => users.get(id)?.username),
      createdAt: this.createdAt
    };
  }
}

// Create default rooms
const defaultRooms = ['General', 'Random', 'Tech', 'Gaming'];
defaultRooms.forEach(name => {
  rooms.set(name.toLowerCase(), new Room(name));
});

// Socket.IO connection handling
io.on('connection', (socket) => {
  console.log(`New connection: ${socket.id}`);

  // Handle user join
  socket.on('join', (data) => {
    const { username, room = 'general' } = data;
    const roomKey = room.toLowerCase();

    // Store user info
    users.set(socket.id, {
      id: socket.id,
      username,
      room: roomKey,
      joinedAt: new Date()
    });

    // Join room
    socket.join(roomKey);

    // Add user to room
    let roomObj = rooms.get(roomKey);
    if (!roomObj) {
      roomObj = new Room(room);
      rooms.set(roomKey, roomObj);
    }
    roomObj.addUser(socket.id);

    // Send welcome message
    socket.emit('message', {
      user: 'System',
      text: `Welcome to ${room}, ${username}!`,
      timestamp: new Date(),
      type: 'system'
    });

    // Send room history
    socket.emit('room-history', roomObj.messages);

    // Notify room about new user
    socket.to(roomKey).emit('message', {
      user: 'System',
      text: `${username} has joined the room`,
      timestamp: new Date(),
      type: 'system'
    });

    // Send updated user list to room
    io.to(roomKey).emit('users-update', {
      users: Array.from(roomObj.users).map(id => users.get(id)),
      count: roomObj.getUserCount()
    });

    // Send available rooms
    socket.emit('rooms-list', Array.from(rooms.values()).map(r => r.getInfo()));

    console.log(`${username} joined room: ${room}`);
  });

  // Handle chat messages
  socket.on('send-message', (data) => {
    const user = users.get(socket.id);
    if (!user) return;

    const message = {
      id: Date.now(),
      user: user.username,
      userId: socket.id,
      text: data.text,
      timestamp: new Date(),
      type: 'user'
    };

    // Add to room history
    const room = rooms.get(user.room);
    if (room) {
      room.addMessage(message);
    }

    // Broadcast to room
    io.to(user.room).emit('message', message);

    console.log(`Message from ${user.username} in ${user.room}: ${data.text}`);
  });

  // Handle typing indicator
  socket.on('typing', () => {
    const user = users.get(socket.id);
    if (!user) return;

    socket.to(user.room).emit('user-typing', {
      username: user.username,
      userId: socket.id
    });
  });

  socket.on('stop-typing', () => {
    const user = users.get(socket.id);
    if (!user) return;

    socket.to(user.room).emit('user-stop-typing', {
      userId: socket.id
    });
  });

  // Handle room switching
  socket.on('switch-room', (data) => {
    const user = users.get(socket.id);
    if (!user) return;

    const newRoomKey = data.room.toLowerCase();
    const oldRoom = user.room;

    // Leave old room
    socket.leave(oldRoom);
    const oldRoomObj = rooms.get(oldRoom);
    if (oldRoomObj) {
      oldRoomObj.removeUser(socket.id);

      // Notify old room
      socket.to(oldRoom).emit('message', {
        user: 'System',
        text: `${user.username} has left the room`,
        timestamp: new Date(),
        type: 'system'
      });

      // Update old room user list
      io.to(oldRoom).emit('users-update', {
        users: Array.from(oldRoomObj.users).map(id => users.get(id)),
        count: oldRoomObj.getUserCount()
      });
    }

    // Join new room
    socket.join(newRoomKey);
    user.room = newRoomKey;

    let newRoomObj = rooms.get(newRoomKey);
    if (!newRoomObj) {
      newRoomObj = new Room(data.room);
      rooms.set(newRoomKey, newRoomObj);
    }
    newRoomObj.addUser(socket.id);

    // Send room history
    socket.emit('room-history', newRoomObj.messages);

    // Notify new room
    socket.to(newRoomKey).emit('message', {
      user: 'System',
      text: `${user.username} has joined the room`,
      timestamp: new Date(),
      type: 'system'
    });

    // Update new room user list
    io.to(newRoomKey).emit('users-update', {
      users: Array.from(newRoomObj.users).map(id => users.get(id)),
      count: newRoomObj.getUserCount()
    });

    console.log(`${user.username} switched from ${oldRoom} to ${newRoomKey}`);
  });

  // Handle private messages
  socket.on('private-message', (data) => {
    const sender = users.get(socket.id);
    if (!sender) return;

    const message = {
      id: Date.now(),
      from: sender.username,
      fromId: socket.id,
      to: data.to,
      text: data.text,
      timestamp: new Date(),
      type: 'private'
    };

    // Send to recipient
    socket.to(data.toId).emit('private-message', message);

    // Send back to sender (for confirmation)
    socket.emit('private-message', message);

    console.log(`Private message from ${sender.username} to ${data.to}`);
  });

  // Handle disconnect
  socket.on('disconnect', () => {
    const user = users.get(socket.id);
    if (!user) return;

    // Remove from room
    const room = rooms.get(user.room);
    if (room) {
      room.removeUser(socket.id);

      // Notify room
      socket.to(user.room).emit('message', {
        user: 'System',
        text: `${user.username} has left the room`,
        timestamp: new Date(),
        type: 'system'
      });

      // Update user list
      io.to(user.room).emit('users-update', {
        users: Array.from(room.users).map(id => users.get(id)),
        count: room.getUserCount()
      });
    }

    // Remove user
    users.delete(socket.id);

    console.log(`${user.username} disconnected`);
  });
});

// API endpoints
app.get('/api/stats', (req, res) => {
  res.json({
    totalUsers: users.size,
    totalRooms: rooms.size,
    rooms: Array.from(rooms.values()).map(r => r.getInfo()),
    uptime: process.uptime()
  });
});

app.get('/api/rooms', (req, res) => {
  res.json(Array.from(rooms.values()).map(r => r.getInfo()));
});

// Start server
server.listen(PORT, () => {
  console.log(`WebSocket Chat Server running on port ${PORT}`);
  console.log(`Stats available at: http://localhost:${PORT}/api/stats`);
});

module.exports = { app, server, io };
