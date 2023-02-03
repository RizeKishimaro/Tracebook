from flask_socketio import SocketIO
from flask import Flask

app = Flask(__name__)
io  = SocketIO(app)

@io.on('test')
def text(message):
    print(message)


if __name__=='__main__':
    io.run(app, debug=True)