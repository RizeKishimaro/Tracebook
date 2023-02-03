from flask import Flask, request, redirect, render_template, url_for
from flask_socketio import SocketIO


app = Flask(__name__)
socket = SocketIO(app)

messages = []

@app.route('/chat')
def chat():
    return render_template('index.html', messages = messages)

@socket.on('message')
def handle_message(data):
    messages.append(data)
    socket.emit('message', data)

if __name__ == '__main__':
    socket.run(app, debug=True)