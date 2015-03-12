
use std::collections::HashMap;
use std::rc::Rc;
use RenderContext;
use View;

pub struct Router {
    stack: Vec<Rc<View>>,
    views: HashMap<String, Rc<View>>,
}

impl Router {

    pub fn renderViews<C>(&self, ctx: &mut C)
        where C: RenderContext
    {

        for v in &self.stack {
            v.render(ctx);
        }
    }
}