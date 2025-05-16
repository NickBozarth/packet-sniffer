use std::io::Write;

use crate::packet::{data_link::{DataLinkLayer, ETHII}, network::NetworkLayer, transport::TransportLayer, *};
use anyhow::Result;
use crossterm::{cursor, event::{Event, KeyCode}, execute, terminal};


macro_rules! move_cursor_to {
    ($column:expr, $row:expr) => {
        print!("\x1B[{};{}H", $column, $row)
    };
}



#[derive(Debug, Default)]
struct DisplayRow {
    name: String,
    offx: usize,
    offy: usize
}

macro_rules! display_row {
    ($name:expr) => {
        DisplayRow {
            name: $name,
            ..Default::default()
        }
    };
    ($name:expr, $offx:expr, $offy:expr) => {
        DisplayRow {
            name: $name,
            offx: $offx,
            offy: $offy,
            ..Default::default()
        }
    };
}


#[derive(Default, Debug)]
enum DisplayScope {
    #[default]
    Default,
    LayerDisplay,
}

// #[derive(Debug)]
struct Display<'a> {
    // cur_x: usize,
    // cur_y: usize,
    display_scope: DisplayScope,

    show: &'a dyn Fn(&'a mut Self),

    layers: Vec<Layer<'a>>,

    display_rows: Vec<DisplayRow>
}

impl<'a> Display<'a> {
    fn change_scope(&'a mut self, display_scope: DisplayScope) {
        match display_scope {
            DisplayScope::Default => {
                self.show = &|display: &mut Self| {
                    println!("{:#?}", display.layers);
                }
            },
            DisplayScope::LayerDisplay => {
                self.show = &|display: &mut Self| {
                    if display.layers.len() == 0 {
                        println!("No layers");
                    } else {
                        display.display_rows = display.layers
                            .iter()
                            .map(|layer| {
                                match layer {
                                    Layer::NoLayer => display_row!("NoLayer".to_owned()),
                                    Layer::Data(bytes) => display_row!(format!("{bytes:x?}")),
                                    Layer::DataLinkLayer(data_link_layer) => display_row!(data_link_layer.get_class_name().to_owned()),
                                    Layer::NetworkLayer(network_layer) => todo!(),
                                    Layer::TransportLayer(transport_layer) => todo!(),
                                }
                            })
                            .collect();
                        println!("{:?}", display.display_rows);
                    }
                }
            },
        }

        self.display_scope = display_scope;
    }


    fn show(&'a mut self) {
        (self.show)(self);
    }

    fn cursor_down() {

    }

    fn cursor_up() {

    }
}

impl<'a> Default for Display<'a> {
    fn default() -> Self {
        let mut display = Self { 
            // cur_x: Default::default(), 
            // cur_y: Default::default(), 
            display_scope: Default::default(), 
            show: &|_: &mut Self| {}, 
            layers: Default::default() ,
            display_rows: Default::default()
        };

        display.change_scope(DisplayScope::default());

        return display;
    }
}




pub fn build_packet<'a>() -> Result<Packet<'a>> {
    print!("\x1B[2J\x1B[H");

    #[allow(unused)]
    let mut input = String::new();
    let mut input_trimmed = "";

    #[allow(unused)]
    let mut text_cur_x = 0usize;
    #[allow(unused)]
    let mut text_cur_y = 0usize;
    #[allow(unused)]
    let mut prev_char = ' ';

    let mut stdout = std::io::stdout();

    let mut display = Display::default();

    display.layers.push(Layer::DataLinkLayer(DataLinkLayer::ETHII(ETHII::default())));


    loop {
        match input_trimmed {
            r"\q" => break,
            r"\l" => display.change_scope(DisplayScope::LayerDisplay),
            r"\d" => display.change_scope(DisplayScope::Default),
            _ => {}
        }

        


        input = String::new();
        text_cur_x = 0;
        text_cur_y = 0;
        let _ = execute!(stdout, cursor::MoveTo(0, 0));
        print!(" $ ");
        let _ = execute!(stdout, cursor::MoveDown(1));
        display.show();
        let _ = execute!(stdout, cursor::MoveTo(3, 0));
        // let _ = std::io::stdout().flush();

        loop {
            if let Event::Key(key_event) = crossterm::event::read().unwrap() {
                if key_event.is_press() {
                    match key_event.code {
                        KeyCode::Enter => {
                            let _ = execute!(stdout, terminal::Clear(terminal::ClearType::All), cursor::MoveTo(0, 0));
                            print!(" $ ");
                            let _ = std::io::stdout().flush(); 
                            break
                        }
                        KeyCode::Backspace => { 
                            if text_cur_x == 0 {
                                continue;
                            }

                            input.pop(); 
                            text_cur_x -= 1;

                            print!("\x1b[2J\x1b[H $ {input}");
                            let _ = std::io::stdout().flush(); 
                        }
                        KeyCode::Char(c) => { 
                            input.push(c); 
                            prev_char = c; 
                            text_cur_x += 1; 

                            print!("\x1B[{text_cur_y};0H\x1b[{}C{prev_char}", text_cur_x + 2);
                            let _ = std::io::stdout().flush();
                        }
                        KeyCode::Down => display.cursor_down(),
                        KeyCode::Up => display.cursor_up(),
                        _ => println!("what the")
                    }
                }
            }
        }
        input_trimmed = &input;
    }

    

    todo!()
}