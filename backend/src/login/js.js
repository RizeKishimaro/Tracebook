async function test() {
  var name = document.getElementById('name');
  var pass = document.getElementById('pass');

  var xhr = new XMLHttpRequest();
  xhr.open("POST", "http://localhost:8090/user/encode-token", true);
  xhr.setRequestHeader('Content-Type', 'application/json');
  xhr.send(JSON.stringify({ username: name.value, password: pass.value }));

}
