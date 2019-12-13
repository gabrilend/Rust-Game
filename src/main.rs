use ggez::*;

// State is data in RAM. It's the game loop, and all data associated with what you need at the moment.
// Context is hardware like mouse / keyboard input, but also graphics and sound. And timers?
// GameResult is error checking

struct State{
    dt: std::time::Duration,
}

impl ggez::event::EventHandler for State{
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.dt = timer::delta(ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        println!("Hello ggez! dt = {}ns", self.dt.subsec_nanos());
        Ok(())
    }
}


fn main() {
    let state = &mut State{ dt: std::time::Duration::new(0, 0) };
    let c = conf::Conf::new();
    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("hello_ggez", "awesome_person")
         .conf(c)
         .build()
         .unwrap();
    event::run(ctx, event_loop, state).unwrap();
}
