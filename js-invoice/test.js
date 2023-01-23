let items = [{name: "item1", value: "A"}, {name: "item2", value: "B"}, {name: "item3", value: "C"}, {name: "item4", value: "D"}];

function search() {
    // Get the input value
    let input = "item1";

    // Filter the items
    let filteredItems = items.filter(function(item) {
        return item.name.toLowerCase().indexOf(input.toLowerCase()) > -1;
    });
    console.log(filteredItems[0].name);
    // Display the filtered items
    // ...
}
search();