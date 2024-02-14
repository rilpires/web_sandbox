#![allow(clippy::absurd_extreme_comparisons)]

use std::cmp::{max, min};

use gloo::console::log;
use web_sys::{wasm_bindgen::{closure::Closure, JsCast, JsValue}, window, CanvasRenderingContext2d, HtmlCanvasElement, ImageBitmap};
use yew::prelude::*;
use world_grid::{World};
use datatype::*;

mod datatype;
mod world_grid;

const pointsize : usize = 4;

#[derive(Properties)]
struct SandboxCanvas {
    canvas: NodeRef,
    width: usize,
    height: usize,
    bg_color: (u8,u8,u8),
    mouse_pos: (u32,u32),
    render_cb: Closure<dyn FnMut()>,
    world: World,
    world_bitmap: Option<ImageBitmap>,
    emitting: bool,
    pointsize: usize,
    tickcount: usize,
}

impl PartialEq for SandboxCanvas {
    fn eq(&self, other: &Self) -> bool {
        self.canvas == other.canvas
    }
}


enum SandboxMsg {
    Initialize(),
    ProcessFrame(),
    MouseClickDown(),
    MouseClickUp(),
    FitCanvas(),
    MouseMovement(MouseEvent),
}


// Macro that creates a closure from a dynamic function
macro_rules! create_closure {
    ($func:expr) => {
        Closure::wrap(
            Box::new($func) as Box<dyn FnMut()>
        )
    };
}
trait DrawableSurface {
    fn draw_bitmap(&self, x: f64, y: f64, bitmap: &ImageBitmap);
    fn update_color(&self, color: (u8, u8, u8));
}

impl DrawableSurface for CanvasRenderingContext2d {

    fn update_color( &self, color:(u8,u8,u8)) {
        self.set_fill_style(
            &JsValue::from_str(
                format!("rgb({}, {}, {}", color.0 , color.1, color.2).as_str()
            )
        );
    }

    fn draw_bitmap(&self, x: f64, y: f64, bitmap: &ImageBitmap) {
        self.draw_image_with_image_bitmap(
            bitmap,
            x, y
        ).unwrap();
    }
}

impl SandboxCanvas {
    
    fn resize(&mut self, new_width: usize, new_height: usize) {
        self.width = new_width;
        self.height = new_height;
        self.world = World::new(
            new_width/self.pointsize, new_height/self.pointsize
        );
    }
    
    fn render (&mut self, new_points: Vec<Vector2<usize>>) {
        let mut canvas = self.canvas.cast::<HtmlCanvasElement>().unwrap();
        let context : CanvasRenderingContext2d = canvas.get_context("2d").unwrap().unwrap().unchecked_into();
        
        // self.width = canvas.width() as usize;
        // self.height = canvas.height() as usize;
        if canvas.width() != self.width as u32 {
            canvas.set_width(self.width as u32);
        }
        if canvas.height() != self.height as u32 {
            canvas.set_height(self.height as u32);
        }
        
        for point in new_points.iter() {
            context.begin_path();
            context.update_color (
                match self.world.get(point.x, point.y) {
                    world_grid::CellType::Empty =>
                        (255,255,255),
                    world_grid::CellType::Sand(particle_data) =>
                        particle_data.color,
                    _ =>
                        (0,0,0)
                }
            );
            context.fill_rect(
                pointsize as f64 * point.x as f64,
                pointsize as f64 * point.y as f64,
                pointsize as f64,
                pointsize as f64
            );
            context.fill();
        }

        // context.update_color((0,244,120));
        // context.arc(
        //     self.mouse_pos.0 as f64, self.mouse_pos.1 as f64,
        //     4.0,
        //     0.0,
        //     std::f64::consts::PI*2.0,
        // ).unwrap();
        // context.fill();

        window()
            .unwrap()
            .request_animation_frame(
                self.render_cb.as_ref().unchecked_ref()
            )
            .unwrap();
    }
}



impl yew::Component for SandboxCanvas {
    type Message = SandboxMsg;

    type Properties = ();

    fn create(ctx: & Context<Self>) -> Self {
        let ctx2 = ctx.link().clone();
        let render_cb = create_closure!(
            move || {
                ctx2.send_message(SandboxMsg::ProcessFrame())
            }
        );

        ctx.link().clone().send_message(SandboxMsg::Initialize());
        
        let width : usize = 100;
        let height : usize = 100;
        
        let ctx3 = ctx.link().clone();
        let resize_closure = create_closure!(
            move || ctx3.send_message(SandboxMsg::FitCanvas())
        );
        window().unwrap().add_event_listener_with_callback(
            "resize",
            resize_closure.as_ref().unchecked_ref()
        );

        resize_closure.forget();

        Self {
            canvas: NodeRef::default(),
            width,
            height,
            bg_color: (0,0,0),
            mouse_pos: (120,120),
            render_cb,
            world: World::new(
                width/pointsize, height/pointsize
            ),
            world_bitmap: None,
            emitting: false,
            pointsize,
            tickcount: 0,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <canvas
            style=" border-bottom: 2px solid grey;\
                    border-left: 2px solid grey;\
                    border-right: 2px solid grey;\
                    overflow: hidden;\
            "
            id="mycanvas"
                ref={self.canvas.clone()}
                onmousedown={ctx.link().callback(|_| {SandboxMsg::MouseClickDown()})}
                onmouseup={ctx.link().callback(|_| {SandboxMsg::MouseClickUp()})}
                onmousemove={ctx.link().callback(|event: MouseEvent| {SandboxMsg::MouseMovement(event)})}
            >
            </canvas>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        
        match msg {
            SandboxMsg::Initialize() => {
                ctx.link().send_future( async move {
                    SandboxMsg::ProcessFrame()
                });
                ctx.link().send_message(
                    SandboxMsg::FitCanvas(),
                );                
                true
            },
            SandboxMsg::ProcessFrame() => {
                let canvas = self.canvas.cast::<HtmlCanvasElement>().unwrap();
                let canvas_quad = canvas.get_bounding_client_rect();
                let mut ratiox = (self.mouse_pos.0 as f64 - canvas_quad.x() ) / (canvas_quad.width() as f64);
                let mut ratioy = (self.mouse_pos.1 as f64 - canvas_quad.y() ) / (canvas_quad.height() as f64);
                ratiox = ratiox.min(1.0).max(0.0);
                ratioy = ratioy.min(1.0).max(0.0);
                let world_x = (ratiox * self.world.width() as f64) as usize;
                let world_y = (ratioy * self.world.height() as f64) as usize;
                if (self.emitting) {
                    self.tickcount += 1;
                    self.world.add_sand(
                        world_x,
                        world_y,
                        if (self.tickcount%512) <= 256 {
                            ((self.tickcount%256) as u8, 0 , 0 )
                        } else {
                            ( 255 - (self.tickcount%256) as u8, 0 , 0 )
                        },
                        8
                    );
                }
                let result = self.world.process_frame();
                self.render( result );
                false
            },
            SandboxMsg::MouseClickDown() => {
                self.emitting = true;
                false
            },
            SandboxMsg::MouseClickUp() => {
                self.emitting = false;
                false
            },
            SandboxMsg::FitCanvas() => {
                let window_width = window().unwrap().inner_width().unwrap().as_f64().unwrap()*0.99;
                let window_height = window().unwrap().inner_height().unwrap().as_f64().unwrap()*0.98;
                let w = window_width as u32;
                let h = window_height as u32;
                self.resize(w as usize, h as usize);
                true
            },
            SandboxMsg::MouseMovement(event) => {
                self.mouse_pos = (event.client_x() as u32, event.client_y() as u32);
                false
            }
        }
    }

}

#[function_component]
fn App() -> Html {
    html!(
        <>
            <div style="width: 100%;\
                        height: 100%;\
                        overflow: hidden;\
                        display: flex;\
                        justify-content: center"
            >
                < SandboxCanvas />
            </div>
        </>
    )
}

fn main() {
    yew::Renderer::<App>::new().render();
}