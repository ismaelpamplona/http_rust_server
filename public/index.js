let counter = document.getElementById("counter");

let count = 1;

if (counter) {
    let count = 1;
    setInterval(() => {
        counter.innerHTML = count;
        count++;
    }, 1000);
}
