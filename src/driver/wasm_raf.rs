//! Web browser requestAnimationFrame frame driver.

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::motion::MotionInner;

/// Driver implementation using the browser's `requestAnimationFrame`.
pub struct WebRafDriver {
    running: Rc<RefCell<bool>>,
    raf_id: Rc<RefCell<Option<i32>>>,
}

impl Default for WebRafDriver {
    fn default() -> Self {
        Self::new()
    }
}

impl WebRafDriver {
    /// Creates a new `WebRafDriver`.
    pub fn new() -> Self {
        Self {
            running: Rc::new(RefCell::new(false)),
            raf_id: Rc::new(RefCell::new(None)),
        }
    }

    /// Ensures that the requestAnimationFrame loop is actively ticking.
    pub fn ensure_running(&self, inner: &Rc<RefCell<MotionInner>>) {
        let mut running = self.running.borrow_mut();
        if *running {
            return;
        }
        *running = true;

        let weak_inner = Rc::downgrade(inner);
        let running_flag = self.running.clone();
        let raf_id_cell = self.raf_id.clone();

        let window = match web_sys::window() {
            Some(w) => w,
            None => {
                *running = false;
                return;
            }
        };

        let performance = window.performance();
        let start_time = performance.as_ref().map(|p| p.now()).unwrap_or(0.0);
        let last_time = Rc::new(RefCell::new(start_time));

        type RafClosure = Closure<dyn FnMut(f64)>;
        let f: Rc<RefCell<Option<RafClosure>>> = Rc::new(RefCell::new(None));
        let g = f.clone();

        *g.borrow_mut() = Some(Closure::wrap(Box::new(move |timestamp: f64| {
            let Some(inner) = weak_inner.upgrade() else {
                *running_flag.borrow_mut() = false;
                *raf_id_cell.borrow_mut() = None;
                let _ = f.borrow_mut().take();
                return;
            };

            let prev = *last_time.borrow();
            let dt_ms = if prev > 0.0 { (timestamp - prev).max(0.0) } else { 0.0 };
            *last_time.borrow_mut() = timestamp;
            let dt = Duration::from_secs_f64(dt_ms / 1000.0);

            let has_more = inner.borrow_mut().tick(dt);

            if has_more && *running_flag.borrow() {
                if let Some(w) = web_sys::window() {
                    if let Some(ref cb) = *f.borrow() {
                        if let Ok(id) = w.request_animation_frame(cb.as_ref().unchecked_ref()) {
                            *raf_id_cell.borrow_mut() = Some(id);
                            return;
                        }
                    }
                }
            }

            // No active animations left or driver stopped
            *running_flag.borrow_mut() = false;
            *raf_id_cell.borrow_mut() = None;
            let _ = f.borrow_mut().take();
        }) as Box<dyn FnMut(f64)>));

        if let Some(ref cb) = *g.borrow() {
            if let Ok(id) = window.request_animation_frame(cb.as_ref().unchecked_ref()) {
                *self.raf_id.borrow_mut() = Some(id);
            } else {
                *self.running.borrow_mut() = false;
            }
        }
    }

    /// Stops the requestAnimationFrame loop immediately.
    pub fn stop(&self) {
        *self.running.borrow_mut() = false;
        if let Some(id) = self.raf_id.borrow_mut().take() {
            if let Some(w) = web_sys::window() {
                let _ = w.cancel_animation_frame(id);
            }
        }
    }
}
