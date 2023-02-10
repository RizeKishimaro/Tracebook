const sign_in_btn = document.querySelector("#sign-in-btn");
const sign_up_btn = document.querySelector("#sign-up-btn");
const container = document.querySelector(".container");

sign_up_btn.addEventListener("click", () => {
  container.classList.add("sign-up-mode");
});

sign_in_btn.addEventListener("click", () => {
  container.classList.remove("sign-up-mode");
});

let Day = [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31];
let Month = ["January","February","March","April","May","June","July","Augest","Semptmber","October","Novenber","December"];
let Year = [1999,2000,2001,2002,2003,2004,2005,2006,2007,2008,2009,2010];


const day = document.querySelector("#day");
const month = document.querySelector("#month");
const year = document.querySelector("#year");

Day.forEach( d=> day.append(new Option(d)) );
Month.forEach( m=> month.append(new Option(m)) );
Year.forEach( y=> year.append(new Option(y)) );


