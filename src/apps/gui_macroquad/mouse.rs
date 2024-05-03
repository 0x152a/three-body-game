use std::marker::PhantomData;
use std::time::{Duration, Instant};
use macroquad::input::*;
use serde::{Deserialize, Serialize};

type MousePos = (f32, f32);

#[allow(dead_code)]
const WHEEL_SCROLL_STEP: f32 = 120.;

#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq)]
pub enum MouseEvent {
    Click(MousePos),
    DoubleClick(MousePos),
    TripleClick(MousePos),
    // MultiClick(MousePos, usize),
    DragDone { from: MousePos, to: MousePos, diff: (f32, f32) },
    DragBegin { from: MousePos, now: MousePos, diff: (f32, f32) },
    Dragging { last: MousePos, now: MousePos, diff: (f32, f32) },
    HoldDone { pos: MousePos, time: Duration },
    HoldBegin(MousePos),
    ScrollDown(MousePos),
    ScrollUp(MousePos),
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub enum SpreadStatus<Result> {
    Route(Option<Result>),
    Stop(Result),
}


pub trait MouseListener {
    type Msg;
    type Result;
    
    #[allow(unused_variables)]
    fn event(&self, event: MouseEvent, msg: Self::Msg) -> SpreadStatus<Self::Result> {
        SpreadStatus::Route(None)
    }
}

pub struct BuiltMouseListener<F, Msg, Result>
    where F: FnMut(MouseEvent, Msg) -> SpreadStatus<Result>,
          Msg: Copy,
{
    func: F,
    _msg: PhantomData<Msg>,
    _result: PhantomData<Result>,
}

impl<F, Msg, Result> MouseListener for BuiltMouseListener<F, Msg, Result>
    where F: Fn(MouseEvent, Msg) -> SpreadStatus<Result>,
          Msg: Copy,
{
    type Msg = Msg;
    type Result = Result;
    
    fn event(&self, event: MouseEvent, msg: Msg) -> SpreadStatus<Result> {
        let func = &self.func;
        (*func)(event, msg)
    }
}

pub fn make_mouse_listener<Msg, Result, F>(listener: F) -> BuiltMouseListener<F, Msg, Result>//Box<dyn MouseListener<Msg = Msg, Result = Result>>
    where F: Fn(MouseEvent, Msg) -> SpreadStatus<Result>,
          Msg: Copy,
{
    BuiltMouseListener::<F, Msg, Result> {
        func: listener,
        _msg: PhantomData::default(),
        _result: PhantomData::default(),
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum MouseEventKind {
    Drag,
    Hold,
    Click(usize),
    Wheel,  //Not used
}


pub struct MouseTracker<Msg, Result>
{
    button: MouseButton,
    is_down: bool,
    testifying: Option<MouseEventKind>,
    last_down: MousePos,
    last_down_time: Instant,
    last_event: MousePos,
    current: MousePos,
    pub listeners: Vec<Box<dyn MouseListener<Msg = Msg, Result = Result>>>,
}

const MOVE_IS_DRAG_THRESHOLD: f32 = 6.;
const HOLD_TIME_MS_THRESHOLD: u64 = 110;
const CLICK_CHECK_MAX_TIME_MS: u64 = 130;
const MAX_CLICK_COUNT: usize = 3;

fn make_position_diff(from: MousePos, to: MousePos) -> (f32, f32) {
    (from.0 - to.0, from.1 - to.1)
}

pub fn apply_event<Msg, Result>(
    event: MouseEvent, msg: Msg,
    listeners: &Vec<Box<dyn MouseListener<Msg = Msg, Result = Result>>>,
) -> Option<Result>
    where Msg: Copy,
{
    for listener in listeners {
        if let SpreadStatus::Stop(x) = listener.event(event, msg) {
            return Some(x);
        }
    }
    
    None
}

impl<Msg, Result> MouseTracker<Msg, Result>
    where Msg: Copy,
{
    #[allow(dead_code)]
    pub fn new(button: MouseButton) -> Self {
        Self {
            button,
            testifying: None,
            is_down: false,
            last_down: (0., 0.),
            last_down_time: Instant::now(),
            last_event: (0., 0.),
            current: (0., 0.),
            listeners: Vec::new(),
        }
    }
    pub(crate) fn from_vec(button: MouseButton, vec: Vec<Box<dyn MouseListener<Msg = Msg, Result = Result>>>) -> Self {
        Self {
            button,
            testifying: None,
            is_down: false,
            last_down: (0., 0.),
            last_down_time: Instant::now(),
            last_event: (0., 0.),
            current: (0., 0.),
            listeners: vec,
        }
    }
    
    async fn apply(&self, event: MouseEvent, msg: Msg) -> Option<Result> {
        // eprintln!("Mouse event: {:?}", event);
        
        apply_event(event, msg, &self.listeners)
    }
    
    async fn check_click(&mut self, msg: Msg) -> Option<Result> {
        if let Some(MouseEventKind::Click(x)) = self.testifying {
            if x >= MAX_CLICK_COUNT ||
                Instant::now()
                    .duration_since(self.last_down_time)
                    > Duration::from_millis(CLICK_CHECK_MAX_TIME_MS)
            {
                self.testifying = None;
                return match x {
                    0 => None,
                    1 => self.apply(MouseEvent::Click(self.last_down), msg).await,
                    2 => self.apply(MouseEvent::DoubleClick(self.last_down), msg).await,
                    3 => self.apply(MouseEvent::TripleClick(self.last_down), msg).await,
                    _ => {
                        for _ in 0..x {
                            let res = self.apply(MouseEvent::Click(self.last_down), msg).await;
                            if res.is_some() {
                                return res;
                            }
                        }
                        None
                    }
                };
            }
        }
        None
    }
    
    fn is_testing_drag(&self) -> bool {
        if let Some(MouseEventKind::Drag) = self.testifying {
            true
        } else {
            false
        }
    }
    
    fn is_testing_hold(&self) -> bool {
        if let Some(MouseEventKind::Hold) = self.testifying {
            true
        } else {
            false
        }
    }
    
    
    async fn check(&mut self, released: bool, msg: Msg) -> Option<Result> {
        let diff = make_position_diff(self.last_down, self.current);
        if self.is_testing_drag()
            || diff.0.abs() > MOVE_IS_DRAG_THRESHOLD
            || diff.1.abs() > MOVE_IS_DRAG_THRESHOLD {
            // Drag
            let event =
                if !self.is_testing_drag() {
                    self.testifying = Some(MouseEventKind::Drag);
                    self.last_event = self.current;
                    MouseEvent::DragBegin {
                        from: self.last_down,
                        now: self.current,
                        diff,
                    }
                } else if released {
                    self.testifying = None;
                    MouseEvent::DragDone {
                        from: self.last_down,
                        to: self.current,
                        diff,
                    }
                } else {
                    let last = self.last_event;
                    self.last_event = self.current;
                    MouseEvent::Dragging {
                        last,
                        now: self.current,
                        diff: make_position_diff(last, self.current),
                    }
                };
            return self.apply(event, msg).await;
        } else {
            let now = Instant::now();
            let duration = now.duration_since(self.last_down_time);
            if self.is_testing_hold()
                || duration > Duration::from_millis(HOLD_TIME_MS_THRESHOLD) {
                // Hold
                if !self.is_testing_hold() {
                    self.testifying = Some(MouseEventKind::Hold);
                    return self.apply(MouseEvent::HoldBegin(self.current), msg).await;
                } else if released {
                    self.testifying = None;
                    return self.apply(MouseEvent::HoldDone {
                        pos: self.current,
                        time: duration,
                    }, msg).await;
                } else {
                    // Ignore
                };
            } else {
                // Click
                if self.testifying.is_none() {
                    self.testifying = Some(MouseEventKind::Click(0));
                } else if released {
                    self.testifying = Some(
                        if let Some(MouseEventKind::Click(x)) = self.testifying {
                            MouseEventKind::Click(x + 1)
                        } else {
                            MouseEventKind::Click(1)
                        }
                    );
                } else {
                    // Ignore
                };
            }
        }
        
        None
    }
    
    pub(crate) async fn update(&mut self, msg: Msg) -> Option<Result> {
        self.current = mouse_position();
        let (_, wheel_y) = mouse_wheel();
        if wheel_y > 0. {
            return self.apply(MouseEvent::ScrollUp(self.current), msg).await;
        } else if wheel_y < 0. {
            return self.apply(MouseEvent::ScrollDown(self.current), msg).await;
        }
        
        if self.is_down {
            return if is_mouse_button_released(self.button) {
                self.is_down = false;
                // dbg!("@MouseUp: {:?}", &self.current);
                self.last_event = self.current;
                self.check(true, msg).await
                // dbg!("Now testing: {:?}", &self.testifying);
            } else {
                self.check(false, msg).await
                // dbg!("Now testing: {:?}", &self.testifying);
            };
        } else if is_mouse_button_down(self.button) {
            self.is_down = true;
            self.last_down = self.current;
            self.last_down_time = Instant::now();
            // dbg!("@MouseDown: {:?}", &self.current);
        } else {
            return self.check_click(msg).await;
        }
        
        None
    }
}

pub struct MouseArea<Msg, Result> {
    pub listeners: Vec<Box<dyn MouseListener<Msg = Msg, Result = Result>>>,
}

impl<Msg, Result> MouseArea<Msg, Result>
    where Msg: Copy
{
    fn apply(&self, event: MouseEvent, msg: Msg) -> Option<Result> {
        // eprintln!("Mouse event: {:?}", event);
        
        apply_event(event, msg, &self.listeners)
    }
}

impl<Msg, Result> MouseListener for MouseArea<Msg, Result>
    where Msg: Copy
{
    type Msg = Msg;
    type Result = Result;
    
    fn event(&self, event: MouseEvent, msg: Self::Msg) -> SpreadStatus<Self::Result> {
        if let Some(x) = self.apply(event, msg) {
            SpreadStatus::Stop(x)
        } else {
            SpreadStatus::Route(None)
        }
    }
}