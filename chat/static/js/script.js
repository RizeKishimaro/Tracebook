var socket = io.connect('http://127.0.0.1:5000');
let send = document.getElementById('send');
// let message = document.getElementById('message');
send.addEventListener('click', ()=>
{
    let data = 
    {
        message: document.getElementById('message').value
    }
    socket.emit('message', data);//send
    document.getElementById('message').value = '';
});
      //receive
socket.on('message', (data)=>{
    document.getElementById('live-msg').innerHTML += data.message + '</br>';
});

