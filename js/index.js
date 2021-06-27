
const cell_size = 50;
const canvas = document.getElementById("pix-editor");
let ctx = canvas.getContext("2d");


function draw(state) {

    ctx.strokeStyle = "black";
    ctx.lineWidth = 1;

    const nb_cells_in_width = state.image.width();
    const nb_cells_in_height = state.image.height();
    let cells = state.image.cells();

    for (let x = 0; x < nb_cells_in_width; x++) {
        for (let y = 0; y < nb_cells_in_height; y++) {
            const index = ((y * nb_cells_in_width) + x) * 3;
            ctx.fillStyle = `rgb(${cells[index]}, ${cells[index + 1]}, ${cells[index + 2]})`;
            ctx.fillRect(x * cell_size,y * cell_size,cell_size,cell_size);
        }
    }

    for (let i = 0; i <= nb_cells_in_width; i++) {
        ctx.beginPath();

        const x_start = i * cell_size;
        const x_end = i * cell_size;
        const y_start = 0;
        const y_end = nb_cells_in_height * cell_size;

        ctx.moveTo(x_start, y_start);
        ctx.lineTo(x_end, y_end);
        ctx.stroke();
    }

    for (let i = 0; i <= nb_cells_in_height; i++) {
        ctx.beginPath();

        const x_start = 0;
        const x_end = nb_cells_in_width * cell_size;
        const y_start = i * cell_size;
        const y_end = i * cell_size;

        ctx.moveTo(x_start, y_start);
        ctx.lineTo(x_end, y_end);
        ctx.stroke();
    }

}

function handleEnvent(state, condition) {

    return (event) => {

        if (condition && !state.dragging) return;
        const rect = canvas.getBoundingClientRect();

        let x = Math.floor((event.clientX - rect.left) / cell_size);
        let y = Math.floor((event.clientY - rect.top) / cell_size);

        state.image = state.image.brush(x, y, [200, 200, 200]);

        draw(state);
    };
}

function setUpCanvas(state) {



    canvas.addEventListener('click', handleEnvent(state, false));
    canvas.addEventListener('mousemove', handleEnvent(state, true));
    canvas.addEventListener('mousedown', (e) => {state.dragging = true;});
    canvas.addEventListener('mouseup', (e) => {state.dragging = false;});
}


async function main() {
    const lib = await import("../pkg/index.js").catch(console.error);
    let image = new lib.Image(10, 10);

    let state = {
        image,
        currentColor: [200, 250, 200],
        dragging: false
    }

    setUpCanvas(state);
    draw(state);

}

main();