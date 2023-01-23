//define the variables
const formParent = document.querySelector("#formParent");
const productName = document.querySelector("#productName");
const amountInput = document.querySelector("#amountInput");
const submitBtn = document.querySelector("#submitBtn");
const tableBody = document.querySelector("#productPriceTable");

const api = "server.php";

//modal for table
const modalTable = "";
// define the products

// const products = [
//   {
//     id: 1,
//     price: 70,
//     name: "Megumi Doll",
//   },
//   {
//     id: 2,
//     price: 30,
//     name: "Magic Broom",
//   },
//   {
//     id: 3,
//     price: 130,
//     name: "Hunter X Hunter Manga",
//   },
//   {
//     id: 4,
//     price: 130,
//     name: "Tanjiro Doll",
//   },
//   {
//     id: 5,
//     price: 130,
//     name: "HatsuneMiku DVD",
//   },
//   {
//     id: 6,
//     price: 130,
//     name: "Mobile Legends Movies",
//   },
// ];
function createTr(id, name, price) {
  const tr = document.createElement("tr");
  tr.innerHTML = `<td>${id}</td>
  <td>${name}</td>
  <td>${price}</td>`;
  return tr;
}

// console.log(JSON.stringify(post_data));
//spelling is wrong
//retrieving form data from php server
function post_data() {
  let idNum = tableBody.childElementCount+1;
  const formPost = {
    id: idNum,
    name: productName.value,
    price: amountInput.valueAsNumber,
  };
  const header = new Headers();
  header.append("Content-Type", "application/json");
  fetch("server.php", {
    method: "POST",
    headers: header,
    body: JSON.stringify(formPost),
  })
    .then(function (response) {
      return response.text();
    })
    .then(function (text) {
      console.log(text);
    })
    .catch(function (error) {
      console.error(error);
    });
  console.log(formPost);
}
function apiData(api) {
  fetch(api)
    .then((response) => response.json())
    .then((responses) => {
      for (obj in responses) {
        tableBody.append(
          createTr(responses[obj].id, responses[obj].name, responses[obj].price)
        );
      }
    })
    .catch((error) => {
      console.table(error);
    });
}
apiData(api);
function delChild() {
  const delChild = document.querySelector("tbody");
  delChild.innerHTML = "";
  // tableBody.innerHTML = "";
}
formParent.addEventListener("submit", (e) => {
  e.preventDefault();
  // console.log(formPost);
  post_data();
  apiData(api);
  setInterval(delChild(), 2000);
  formParent.reset();
});
// post_data();
