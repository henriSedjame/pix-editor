use wasm_bindgen::prelude::*;
use im::Vector;
use std::iter::FromIterator;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RGB {
    r: u8,
    g: u8,
    b: u8
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Image {
    width: usize,
    height: usize,
    cells: Vector<RGB>
}

#[wasm_bindgen]
impl Image {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize) -> Self {

        let mut cells = Vector::from_iter(
            (0..(width * height)).map(|_| RGB{r: 31, g: 95, b: 111}));

        Self {width, height, cells}
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn cells(&self) -> Vec<u8> {
        self.cells
            .iter()
            .map(|rgb| vec![rgb.r, rgb.g, rgb.b])
            .collect::<Vec<Vec<u8>>>()
            .concat()
    }

    pub fn brush(&self, x: usize, y: usize, color: Vec<u8>) -> Option<Image> {
        let index = (y * self.width) + x;
        let color = RGB { r: color[0], g: color[1], b: color[2] };

        if self.cells[index] == color {
            None
        } else {
            let new_cells = self.cells.update(index, color);
            Some(Image{
                width : self.width,
                height: self.height,
                cells: new_cells
            })
        }
    }
}

enum Mode {
    Normal,
    StartBlock,
    InBlock
}
pub struct UndoQueue<T: Clone> {
    queue: Vec<T>,
    index: usize,
    mode: Mode
}

impl<T: Clone> UndoQueue<T> {

    pub fn new(t: T) -> UndoQueue<T> {
        Self {
            queue: vec![t],
            index: 0,
            mode: Mode::Normal
        }
    }

    pub fn current(&self) -> T {
        self.queue[self.index].clone()
    }



    pub fn push(&mut self, t: T){

        match self.mode {
            Mode::Normal => {
                self.queue.truncate(self.index + 1);
                self.queue.push(t);
                self.index += 1;
            }
            Mode::StartBlock => {
                self.queue.truncate(self.index + 1);
                self.queue.push(t);
                self.index += 1;
                self.mode = Mode::InBlock
            }
            Mode::InBlock => {
                self.queue[self.index] = t;
            }
        }
    }

    pub fn undo(&mut self) {
        if self.index >= 1 {
            self.index -= 1
        }
    }

    pub fn redo(&mut self) {
        if self.index < (self.queue.len() - 1) {
            self.index += 1
        }
    }

    pub fn start_undo_block(&mut self) {
        self.mode = Mode::StartBlock;
    }

    pub fn close_undo_block(&mut self) {
        self.mode = Mode::Normal;
    }
}

#[wasm_bindgen]
pub struct InternalState {
    undo_queue: UndoQueue<Image>
}

#[wasm_bindgen]
impl InternalState {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height:usize) -> Self {
        Self {
            undo_queue: UndoQueue::new(Image::new(width, height))
        }
    }

    pub fn image(&self) -> Image {
        self.undo_queue.current()
    }

    pub fn brush(&mut self, x: usize, y: usize, color: Vec<u8>) {
        let image = self.image();
        if let Some(new_image) = image.brush(x, y, color) {
            self.undo_queue.push(new_image);
        }
    }

    pub fn undo(&mut self) {
        self.undo_queue.undo()
    }

    pub fn redo(&mut self) {
        self.undo_queue.redo()
    }

    pub fn start_undo_block(&mut self) {
        self.undo_queue.start_undo_block();
    }

    pub fn close_undo_block(&mut self) {
        self.undo_queue.close_undo_block();
    }
}
