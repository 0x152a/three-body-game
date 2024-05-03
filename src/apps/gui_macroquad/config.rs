use macroquad::color::Color;
use serde::{Deserialize, Serialize};
use crate::num::{Num, num, Int};
use crate::vector::Vector;
use std::sync::Mutex;

#[derive(PartialEq, Debug, Clone, Deserialize, Serialize)]
pub enum AttrValue {
    Num(Num),
    Vector(Vector),
    Int(Int),
    USize(usize),
    ISize(isize),
    Bool(bool),
    String(String),
    ColorWrap(ColorWrap),
}

macro_rules! make_default {
    ($name: ident, $value: expr, $typ: ident) => {
        fn $name() -> $typ {
            $value
        }
    };
}


#[derive(Clone, Copy, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ColorWrap {
    #[serde(default = "Default::default")]
    pub r: f32,
    #[serde(default = "Default::default")]
    pub g: f32,
    #[serde(default = "Default::default")]
    pub b: f32,
    #[serde(default = "Default::default")]
    pub a: f32,
}

impl Into<Color> for ColorWrap {
    fn into(self) -> Color {
        if [self.r, self.g, self.b, self.a].iter().any(|&x| x > 1.0) {
            let max = 255.0;
            Color::new(self.r / max, self.g / max, self.b / max, self.a / max)
        } else {
            Color::new(self.r, self.g, self.b, self.a)
        }
    }
}


mod body {
    use std::collections::HashMap;
    use serde::{Deserializer, Serializer};
    use crate::body::{BodyId, BodyLike};
    use super::*;
    
    static DEFAULT_TRAIL_COLOR: ColorWrap = ColorWrap {
        r: 0.419608,
        g: 0.419608,
        b: 0.419608,
        a: 1.0,
    };
    static DEFAULT_COLORS: &'static [[f32; 4]] = &[
        [  // Horizon Blue
            0.184313725490196,
            0.490196078431373,
            0.56078431372549,
            1.,
        ],
        [  // Fiesta
            0.603921568627451,
            0.172549019607843,
            0.12156862745098,
            1.,
        ],
        [  // Mint
            0.0627450980392157,
            0.486274509803922,
            0.341176470588235,
            1.,
        ],
        [  // Schönbrunn Yellow
            0.458823529411765,
            0.4,
            0.0627450980392157,
            1.,
        ],
        [  // Strong Blue
            0.125490196078431,
            0.282352941176471,
            0.450980392156863,
            1.,
        ],
        [  // Sun Orange
            0.623529411764706,
            0.294117647058824,
            0.0745098039215686,
            1.,
        ],
    ];
    
    make_default!(mass, num(1), Num);
    make_default!(trail_color, DEFAULT_TRAIL_COLOR, ColorWrap);
    
    static mut CURRENT_COLOR_INDEX: usize = 0;
    
    fn color() -> ColorWrap {
        let idx = unsafe { CURRENT_COLOR_INDEX };
        let color = DEFAULT_COLORS[idx];
        unsafe { CURRENT_COLOR_INDEX += 1; }
        ColorWrap {
            r: color[0],
            g: color[1],
            b: color[2],
            a: color[3],
        }
    }
    
    
    static mut CURRENT_ID: BodyId = 0;
    
    fn id() -> BodyId {
        let id = unsafe { CURRENT_ID };
        unsafe { CURRENT_ID += 1; }
        id
    }
    
    
    // static mut BODY_INFO: Option<HashMap<BodyId, HashMap<String, AttrValue>>> = None;
    //
    // fn init_body_info() {
    //     let info = HashMap::new();
    //     unsafe { BODY_INFO = Some(info); }
    // }
    //
    //
    // fn get_body_info<T, Func>(func: Func) -> T
    //     where Func: FnOnce(&'static HashMap<BodyId, HashMap<String, AttrValue>>) -> T
    // {
    //     if let Some(info) = unsafe { &BODY_INFO } {
    //         func(info)
    //     } else {
    //         init_body_info();
    //         crate::apps::app::config::body::get_body_info(func)
    //     }
    // }
    //
    // fn get_body_info_mut<T, Func>(func: Func) -> T
    //     where Func: FnOnce(&mut HashMap<BodyId, HashMap<String, AttrValue>>) -> T
    // {
    //     if let Some(info) = unsafe { &mut BODY_INFO } {
    //         func(info)
    //     } else {
    //         init_body_info();
    //         crate::apps::app::config::body::get_body_info_mut(func)
    //     }
    // }
    //
    
    lazy_static! {
        static ref BODY_INFO: Mutex<HashMap<BodyId, HashMap<String, AttrValue>>> =  Mutex::new(HashMap::new());
    }
    
    
    fn get_body_info<T, Func>(func: Func) -> T
        where Func: FnOnce(&'static Mutex<HashMap<BodyId, HashMap<String, AttrValue>>>) -> T
    {
        func(&BODY_INFO)
    }
    
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct Body {
        id: BodyId,
        pos: Vector,
        speed: Vector,
        mass: Num,
        color: ColorWrap,
        trail_color: ColorWrap,
    }
    
    
    impl<'de> Deserialize<'de> for Body {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
        {
            #[derive(Deserialize)]
            struct BodyHelper {
                id: Option<BodyId>,
                pos: Option<Vector>,
                speed: Option<Vector>,
                mass: Option<Num>,
                color: Option<ColorWrap>,
                trail_color: Option<ColorWrap>,
                #[serde(flatten)]
                attrs: HashMap<String, AttrValue>,
            }
            
            let body_helper = BodyHelper::deserialize(deserializer)?;
            let id = body_helper.id.unwrap_or_else(id);
            let pos = body_helper.pos.unwrap_or_else(Vector::origin);
            let speed = body_helper.speed.unwrap_or_else(Vector::origin);
            let mass = body_helper.mass.unwrap_or_else(mass);
            let color = body_helper.color.unwrap_or_else(color);
            let trail_color = body_helper.trail_color.unwrap_or_else(trail_color);
            
            get_body_info(
                |info| info
                    .lock().unwrap()
                    .entry(id)
                    .or_insert_with(HashMap::new)
                    .extend(body_helper.attrs)
            );
            
            Ok(Body { id, pos, speed, mass, color, trail_color })
        }
    }
    
    impl Serialize for Body {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
            #[derive(Serialize)]
            struct BodyHelper<'a> {
                id: BodyId,
                pos: Vector,
                speed: Vector,
                mass: Num,
                color: ColorWrap,
                trail_color: ColorWrap,
                #[serde(flatten)]
                attrs: &'a HashMap<String, AttrValue>,
            }
            
            let helper = BodyHelper {
                color: self.color,
                trail_color: self.trail_color,
                id: self.id,
                pos: self.pos,
                speed: self.speed,
                mass: self.mass,
                attrs: &get_body_info(
                    |info| info.lock().unwrap().get(&self.id).unwrap().clone()
                )
                ,
            };
            
            helper.serialize(serializer)
        }
    }
    
    #[allow(dead_code)]
    impl BodyLike for Body {
        type Value = AttrValue;
        fn id(&self) -> &BodyId { &self.id }
        fn pos(&self) -> &Vector { &self.pos }
        fn speed(&self) -> &Vector { &self.speed }
        fn mass(&self) -> &Num { &self.mass }
        fn pos_mut(&mut self) -> &mut Vector { &mut self.pos }
        fn speed_mut(&mut self) -> &mut Vector { &mut self.speed }
        fn mass_mut(&mut self) -> &mut Num { &mut self.mass }
        fn get_attr(&self, name: &String) -> Option<AttrValue> {
            get_body_info(
                |info| {
                    if let Some(x) = info.lock().unwrap().get(&self.id) {
                        x.get(name).cloned()
                    } else {
                        None
                    }
                }
            )
        }
    }
    
    impl Body {
        pub fn color(&self) -> Color {
            self.color.into()
        }
        pub fn trail_color(&self) -> Color {
            self.trail_color.into()
        }
    }
}

mod config {
    use std::collections::HashMap;
    use super::*;
    use super::body::Body;
    
    make_default!(delta, num(0.001), Num);
    make_default!(steps, 20, isize);
    make_default!(step_per_trail, 5, isize);
    make_default!(trail_alpha_loss_rate, 0.98, f32);
    make_default!(trail_alpha_min, 0.05, f32);
    make_default!(trail_alpha_begin, 0.5, f32);
    make_default!(message_alpha_min, 0.02, f32);
    make_default!(message_alpha_loss_rate, 0.98, f32);
    make_default!(shine_alpha_min, 0.02, f32);
    make_default!(shine_alpha_loss_rate, 0.98, f32);
    make_default!(radius_factor, 6., Num);
    make_default!(depth_factor, 0.3, Num);
    make_default!(depth_max, 30., Num);
    make_default!(title_font_size, 40, u16);
    make_default!(trail_radius, 3., Num);
    make_default!(tooltip_font_size, 14, u16);
    make_default!(tooltip_font_color, ColorWrap{r: 1.0, g: 1.0, b: 1.0, a: 0.8 }, ColorWrap);
    make_default!(instruction_font_size, 15, u16);
    make_default!(instruction_font_color, ColorWrap{r: 1.0, g: 1.0, b: 1.0, a: 1.0 }, ColorWrap);
    make_default!(title_font_color, ColorWrap{r: 1.0, g: 1.0, b: 1.0, a: 1.0 }, ColorWrap);
    make_default!(enable_high_dpi, true, bool);
    make_default!(window_title, "Three Body".to_string(), String);
    make_default!(window_width, 800, i32);
    make_default!(window_height, 600, i32);
    make_default!(arrow_size, 10., f32);
    
    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct Config {
        pub bodies: Vec<Body>,
        #[serde(default = "delta")]
        pub delta: Num,
        #[serde(default = "steps")]
        pub steps: isize,
        #[serde(default = "step_per_trail")]
        pub step_per_trail: isize,
        #[serde(default = "trail_alpha_loss_rate")]
        pub trail_alpha_loss_rate: f32,
        #[serde(default = "trail_alpha_min")]
        pub trail_alpha_min: f32,
        #[serde(default = "trail_alpha_begin")]
        pub trail_alpha_begin: f32,
        #[serde(default = "message_alpha_loss_rate")]
        pub message_alpha_loss_rate: f32,
        #[serde(default = "message_alpha_min")]
        pub message_alpha_min: f32,
        #[serde(default = "shine_alpha_min")]
        pub shine_alpha_min: f32,
        #[serde(default = "shine_alpha_loss_rate")]
        pub shine_alpha_loss_rate: f32,
        #[serde(default = "radius_factor")]
        pub radius_factor: Num,
        #[serde(default = "depth_factor")]
        pub depth_factor: Num,
        #[serde(default = "depth_max")]
        pub depth_max: Num,
        #[serde(default = "trail_radius")]
        pub trail_radius: Num,
        
        #[serde(default = "title_font_size")]
        pub title_font_size: u16,
        #[serde(default = "title_font_color")]
        pub title_font_color: ColorWrap,
        
        pub tooltip_font: Option<String>,
        #[serde(default = "tooltip_font_size")]
        pub tooltip_font_size: u16,
        #[serde(default = "tooltip_font_color")]
        pub tooltip_font_color: ColorWrap,
        
        pub instruction_font: Option<String>,
        #[serde(default = "instruction_font_size")]
        pub instruction_font_size: u16,
        #[serde(default = "instruction_font_color")]
        pub instruction_font_color: ColorWrap,
        
        #[serde(default = "enable_high_dpi")]
        pub enable_high_dpi: bool,
        #[serde(default = "window_title")]
        pub window_title: String,
        #[serde(default = "window_width")]
        pub window_width: i32,
        #[serde(default = "window_height")]
        pub window_height: i32,
        #[serde(default = "arrow_size")]
        pub arrow_size: f32,
        
        #[serde(flatten)]
        pub fields: HashMap<String, AttrValue>,
    }
}

pub use self::body::Body;
pub use self::config::Config;