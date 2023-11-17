use core::num;

use crate::{makepad_draw::*, makepad_widgets::*};

live_design! {
    import makepad_draw::shader::std::*;

    DrawLineSegment = {{DrawLineSegment}} {
       
        fn stroke(self, side:float, progress: float) -> vec4
        {
            return self.color;
        }

        fn pixel(self) -> vec4 
        {
            
            let p = self.pos * self.rect_size;
            let b = self.line_end;
            let a = self.line_start;
          
            let ba = b-a;
            let pa = p-a;
            let h = clamp( dot(pa,ba)/dot(ba,ba), 0.0, 1.0 );
            let dist= length(pa-h*ba)
         
            let linemult = smoothstep(self.width-1., self.width, dist);
            let C = self.stroke(dist, h);
            return vec4(C.xyz*(1.-linemult),(1.0-linemult)*C.a);

 //           return vec4(self.color.xyz*abs(smoothstep(-0.1,0.1,sin(h*60.283/(self.width/4.))*self.width))*(1.-linemult),1.0-linemult);
        }
    }

    DrawArc = {{DrawArc}} {
       
        fn stroke(self, side:float, progress: float) -> vec4
        {
            return self.color;
        }

        fn doarc (self, p:vec2, a0:float, a1:float, r:float ) -> float
        {
            let a = mod(atan(p.y, p.x), 6.283);

            let  ap = a - a0;
            if (ap < 0.)  
            {
               ap+=6.283;
            }
            let  a1p = a1 - a0;
            if (a1p < 0.) 
            {
                a1p += 6.283;
            }

            if (ap >= a1p)  
            {
                let q0 = vec2(r * cos(a0), r * sin(a0));
                let q1 = vec2(r * cos(a1), r * sin(a1));
                return min(length(p - q0), length(p - q1));
            }

            return abs(length(p) - r);
        }

        fn pixel(self) -> vec4 
        {
            let pixelpos = self.pos * self.rect_size;
            let dist= self.doarc(pixelpos-self.arc_center, self.arc_a0,self.arc_a1, self.arc_radius);
            let linemult = smoothstep(self.width-1., self.width, dist);
            let C = self.stroke(dist, 0);
            return self.color * (1. - linemult);
        //    return vec4(C.xyz*(1.-linemult),(1.0-linemult)*C.a);

 //           return vec4(self.color.xyz*abs(smoothstep(-0.1,0.1,sin(h*60.283/(self.width/4.))*self.width))*(1.-linemult),1.0-linemult);
        }
    }


    VectorLine = {{VectorLine}} {
        width: Fill,
        height: Fill
    }

    VectorArc = {{VectorArc}} {
        width: Fill,
        height: Fill
    }
}

#[derive(Live, LiveHook)]
#[repr(C)]
struct DrawArc {
    #[deref]
    draw_super: DrawQuad,
    #[calc] arc_start: Vec2,
    #[calc] arc_end: Vec2,
    #[calc] arc_center: Vec2,
    #[calc] width: f32,
    #[calc] arc_a0: f32,
    #[calc] arc_a1: f32,
    #[calc] arc_radius: f32,
    #[calc] color: Vec4,
}

#[derive(Live, LiveHook)]
#[repr(C)]
struct DrawLineSegment {
    #[deref]   draw_super: DrawQuad,
    #[calc]    line_start: Vec2,
    #[calc]    line_end: Vec2,
    #[calc]    width: f32,
    #[calc]    color: Vec4,
}

#[derive(Copy, Clone, Debug, Live, LiveHook)]
#[live_ignore]
pub enum LineAlign
{
    Free,
    Left,
    #[pick] Top,
    DiagonalBottomLeftTopRight,
    DiagonalTopLeftBottomRight,
    Right,
    Bottom,
    VerticalCenter,
    HorizontalCenter
}

#[derive(Live)]
pub struct VectorLine{
    #[walk] walk: Walk,
    #[live] draw_ls: DrawLineSegment,
    #[rust] area: Area,
    #[live(15.0)] line_width: f64,
    #[live] color: Vec4,
    #[live(LineAlign::Top)] line_align: LineAlign,
    #[rust(dvec2(350., 10.))] line_start: DVec2,
    #[rust(dvec2(1000., 1440.))] line_end: DVec2,
}

#[derive(Copy, Clone, Debug, Live, LiveHook)]
#[live_ignore]
pub enum QuadCorner
{
    TopLeft,
    TopRight,
    #[pick] BottomRight,
    BottomLeft,
    UnspecifiedCorner
}


#[derive(Copy, Clone, Debug, Live, LiveHook)]
#[live_ignore]
pub enum Winding
{
    #[pick]ClockWise,
    CounterClockWise
}

#[derive(Live)]
pub struct VectorArc{
    #[walk] walk: Walk,
    #[live] draw_arc: DrawArc,
    #[rust] area: Area,
   
    #[live(15.0)] line_width: f64,
    #[live] color: Vec4,
   
   #[live(QuadCorner::UnspecifiedCorner)] arc_start_corner: QuadCorner,
   #[live(QuadCorner::UnspecifiedCorner)] arc_end_corner: QuadCorner,
   #[live(Winding::ClockWise)] arc_winding: Winding,
   
    #[rust(dvec2(350., 10.))] arc_start: DVec2,
    #[rust(dvec2(1000., 1440.))] arc_end: DVec2,
    #[rust(dvec2(1000., 1440.))] arc_center: DVec2,
   
}


impl Widget for VectorLine {
    fn handle_widget_event_with(
        &mut self,
        _cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem),
    ) {
        let uid = self.widget_uid();
       
    }

    fn walk(&mut self, _cx: &mut Cx) -> Walk {
        self.walk
    }

    fn redraw(&mut self, cx: &mut Cx) {
        self.area.redraw(cx)
    }

    fn draw_walk_widget(&mut self, cx: &mut Cx2d, walk: Walk) -> WidgetDraw {
        let _ = self.draw_walk(cx, walk);
        WidgetDraw::done()
    }
}

#[derive(Clone, WidgetAction)]
pub enum LineAction {
    None,
}

impl LiveHook for VectorLine {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, VectorLine)
    }

    fn after_new_from_doc(&mut self, _cx: &mut Cx) {}
}

impl VectorLine {
    pub fn draw_walk(&mut self, cx: &mut Cx2d, walk: Walk) {
        // lets draw a bunch of quads
        let rect = cx.walk_turtle_with_area(&mut self.area, walk);

       // self.line_width = 10.5;
        let maxpixels = 300. as f64;
        let mut line_start = self.line_start;
        let mut line_end = self.line_end;
        let hw = self.line_width / 2.;
       
        //println!("layout called!");
        match self.line_align 
        {
            LineAlign::Top =>{
                 line_start = dvec2(rect.pos.x+hw, rect.pos.y+hw); 
                 line_end = dvec2(rect.pos.x + rect.size.x - hw, rect.pos.y +hw);
            }
            LineAlign::Bottom =>{
                line_start = dvec2(rect.pos.x+hw, rect.pos.y+rect.size.y - hw); 
                line_end = dvec2(rect.pos.x + rect.size.x - hw, rect.pos.y + rect.size.y - hw);
            }
            LineAlign::Right =>{
                line_start = dvec2(rect.pos.x+rect.size.x-hw, rect.pos.y + hw); 
                line_end = dvec2(rect.pos.x +rect.size.x -hw, rect.pos.y + rect.size.y - hw);
            }
            LineAlign::Left =>{
                line_start = dvec2(rect.pos.x+hw, rect.pos.y + hw); 
                line_end = dvec2(rect.pos.x +hw, rect.pos.y + rect.size.y - hw);
            }
            LineAlign::HorizontalCenter =>{
                line_start = dvec2(rect.pos.x+hw, rect.pos.y + rect.size.y/2.); 
                line_end = dvec2(rect.pos.x +rect.size.x - hw, rect.pos.y + rect.size.y/2.);
            } 
            LineAlign::VerticalCenter =>{
                line_start = dvec2(rect.pos.x+rect.size.x/2., rect.pos.y + hw); 
                line_end = dvec2(rect.pos.x +rect.size.x/2., rect.pos.y + rect.size.y - hw);
            }
            LineAlign::DiagonalTopLeftBottomRight=>{
                line_start = dvec2(rect.pos.x+hw, rect.pos.y + hw); 
                line_end = dvec2(rect.pos.x +rect.size.x -hw, rect.pos.y + rect.size.y - hw);
            }
            LineAlign::DiagonalBottomLeftTopRight=>{
                line_start = dvec2(rect.pos.x+hw, rect.pos.y + rect.size.y- hw); 
                line_end = dvec2(rect.pos.x +rect.size.x-hw, rect.pos.y +  hw);
            }
            _ => {}
        }

        self.draw_ls.width = hw as f32;
        self.draw_ls.color = self.color;

        let linerect = line_end - line_start;
        if (line_start.y - line_end.y).abs().floor() == 0.0
            || (line_start.x - line_end.x).abs().floor() == 0.0
        {
            let r = Rect {
                pos: dvec2(
                    min(line_start.x, line_end.x) - hw,
                    min(line_start.y, line_end.y) - hw,
                ),
                size: dvec2(
                    linerect.x.abs() + self.line_width,
                    linerect.y.abs() + self.line_width,
                ),
            };

            self.draw_ls.line_start = (line_start - r.pos).into_vec2();
            self.draw_ls.line_end = (line_end - r.pos).into_vec2();

            self.draw_ls.draw_abs(cx, r);

            return;
        }

        if linerect.x.abs() > linerect.y.abs()
        // more horizontal than vertical
        {
            let mut actualstart = line_start;
            let mut actualend = line_end;

            if actualend.x < actualstart.x {
                std::mem::swap(&mut actualstart, &mut actualend);
            }

            let delta = actualend - actualstart;
            let normalizedelta = delta.normalize();
            let xnormalizedelta = delta.normalize_to_x();
            let normalizedarea = (xnormalizedelta.x * xnormalizedelta.y).abs();
            let scaledup = (maxpixels / normalizedarea).sqrt();

            let angle = delta.angle_in_radians();
            let tanangle = angle.tan();

            let clocktang = normalizedelta.clockwise_tangent();

            let circlepoint = clocktang * hw;
            let overside = hw - circlepoint.y;
            let aanliggend = overside / tanangle;
            let backoffset = circlepoint.x.abs() - aanliggend.abs();


            let rectstart = Rect {
                pos: actualstart - dvec2(hw, hw),
                size: dvec2(hw - backoffset, self.line_width),
            };

            let rectend = Rect {
                pos: actualend - dvec2(-backoffset, hw),
                size: dvec2(hw - backoffset, self.line_width),
            };
            
            let miny = min(rectstart.pos.y, rectend.pos.y);
            let maxy = max(
                rectend.pos.y + rectend.size.y,
                rectstart.pos.y + rectstart.size.y,
            );

            let innerwidth = rectend.pos.x - (rectstart.pos.x + rectstart.size.x);
            let numblocks = (innerwidth / scaledup).ceil();
            let blockwidth = innerwidth / (numblocks as f64);

            let step = dvec2(blockwidth, xnormalizedelta.y * blockwidth);
            let mut adjust = -backoffset * 2. * xnormalizedelta.y;
            if step.y < 0. {
                adjust = step.y;
            }
            let blockheight = self.line_width / angle.cos() + step.y.abs();

            self.draw_ls.width = hw as f32;
            let segmentstart = dvec2(rectstart.pos.x + rectstart.size.x, rectstart.pos.y + adjust);

            for i in 0..(numblocks as i32) as i32 {
                let mut r = Rect {
                    pos: segmentstart + step * (i as f64),
                    size: dvec2(blockwidth, blockheight),
                };
                r.clip_y_between(miny, maxy);

                self.draw_ls.line_start = (actualstart - r.pos).into_vec2();
                self.draw_ls.line_end = (actualend - r.pos).into_vec2();

                self.draw_ls.draw_abs(cx, r);
            }

            self.draw_ls.line_start = (actualstart - rectstart.pos).into_vec2();
            self.draw_ls.line_end = (actualend - rectstart.pos).into_vec2();

            self.draw_ls.draw_abs(cx, rectstart);

            self.draw_ls.line_start = (actualstart - rectend.pos).into_vec2();
            self.draw_ls.line_end = (actualend - rectend.pos).into_vec2();

            self.draw_ls.draw_abs(cx, rectend);


        } else {
             let mut actualstart = line_start;
            let mut actualend: DVec2 = line_end;

            if actualend.y < actualstart.y {
                std::mem::swap(&mut actualstart, &mut actualend);
            }
            let delta = actualend - actualstart;
            let normalizedelta = delta.normalize();
            let ynormalizedelta = delta.normalize_to_y();
            let normalizedarea = (ynormalizedelta.x * ynormalizedelta.y).abs();
            let scaledup = (maxpixels / normalizedarea).sqrt();
            let angle =  delta.angle_in_radians() - std::f64::consts::PI/2.;
            let tanangle = angle.tan();  
            let circlepoint = normalizedelta * hw;
            let overside = hw - circlepoint.y;
            let aanliggend = overside / tanangle;
            let backoffset = circlepoint.x.abs() - aanliggend.abs();

            let rectstart = Rect {
                pos: actualstart - dvec2(hw, hw),
                size: dvec2(self.line_width, hw - backoffset),
            };
            let rectend = Rect {
                pos: actualend - dvec2(hw, -backoffset),
                size: dvec2(self.line_width, hw - backoffset),
            };
            let minx = min(rectstart.pos.x, rectend.pos.x);
            let maxx = max(
                rectend.pos.x + rectend.size.x,
                rectstart.pos.x + rectstart.size.x,
            );

            let innerheight = rectend.pos.y - (rectstart.pos.y + rectstart.size.y);
            let numblocks = (innerheight / scaledup).ceil();
            let blockheight = innerheight / (numblocks as f64);

            let step = dvec2( ynormalizedelta.x * blockheight, blockheight);
            let mut adjust = -backoffset * 2. * ynormalizedelta.x;
            if step.x < 0. {
                adjust = step.x;
            }
            let blockwidth = self.line_width / angle.cos() + step.x.abs();

            
            self.draw_ls.width = hw as f32;
            let segmentstart = dvec2(rectstart.pos.x + adjust, rectstart.pos.y + rectstart.size.y);


            for i in 0..(numblocks as i32) as i32 {
                let mut r = Rect {
                    pos: segmentstart + step * (i as f64),
                    size: dvec2(blockwidth, blockheight),
                };
                r.clip_x_between(minx, maxx);

                self.draw_ls.line_start = (actualstart - r.pos).into_vec2();
                self.draw_ls.line_end = (actualend - r.pos).into_vec2();

                self.draw_ls.draw_abs(cx, r);
            }

            self.draw_ls.line_start = (actualstart - rectstart.pos).into_vec2();
            self.draw_ls.line_end = (actualend - rectstart.pos).into_vec2();

            self.draw_ls.draw_abs(cx, rectstart);

            self.draw_ls.line_start = (actualstart - rectend.pos).into_vec2();
            self.draw_ls.line_end = (actualend - rectend.pos).into_vec2();

            self.draw_ls.draw_abs(cx, rectend);

            
        }
    }


    fn walk(&mut self, _cx:&mut Cx) -> Walk {self.walk}
    
    fn draw_walk_widget(&mut self, cx: &mut Cx2d, walk: Walk) -> WidgetDraw {
        self.draw_walk(cx, walk);
        WidgetDraw::done()
    }

}


impl Widget for VectorArc {
    fn handle_widget_event_with(
        &mut self,
        _cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem),
    ) {
        let uid = self.widget_uid();
       
    }

    fn walk(&mut self, _cx: &mut Cx) -> Walk {
        self.walk
    }

    fn redraw(&mut self, cx: &mut Cx) {
        self.area.redraw(cx)
    }

    fn draw_walk_widget(&mut self, cx: &mut Cx2d, walk: Walk) -> WidgetDraw {
        let _ = self.draw_walk(cx, walk);
        WidgetDraw::done()
    }
}

#[derive(Clone, WidgetAction)]
pub enum ArcAction {
    None,
}

impl LiveHook for VectorArc {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, VectorArc)
    }

    fn after_new_from_doc(&mut self, _cx: &mut Cx) {}
}

impl VectorArc {
    pub fn draw_walk(&mut self, cx: &mut Cx2d, walk: Walk) {
        // lets draw a bunch of quads
        let rect = cx.walk_turtle_with_area(&mut self.area, walk);

      
        let maxpixels = 300. as f64;
        let mut arc_start = self.arc_start;
        let mut arc_end = self.arc_end;
        let hw = self.line_width / 2.;
       
        match self.arc_start_corner
        {
            QuadCorner::TopLeft => {arc_start = dvec2(rect.pos.x, rect.pos.y )}
            QuadCorner::TopRight => {arc_start = dvec2(rect.pos.x + rect.size.x, rect.pos.y )}
            QuadCorner::BottomRight => {arc_start = dvec2(rect.pos.x + rect.size.x, rect.pos.y +rect.size.y)}
            QuadCorner::BottomLeft => {arc_start = dvec2(rect.pos.x, rect.pos.y +rect.size.y)}
         
            _ => {}
        }
        
        match self.arc_end_corner
        {
            QuadCorner::TopLeft => {arc_end = dvec2(rect.pos.x, rect.pos.y )}
            QuadCorner::TopRight => {arc_end = dvec2(rect.pos.x + rect.size.x, rect.pos.y )}
            QuadCorner::BottomRight => {arc_end = dvec2(rect.pos.x + rect.size.x, rect.pos.y +rect.size.y)}
            QuadCorner::BottomLeft => {arc_end = dvec2(rect.pos.x, rect.pos.y +rect.size.y)}
         
            _ => {}
        }

    

        let mut arc_center = self.arc_center;


        self.draw_arc.arc_radius = (arc_start - arc_center).length() as f32;
        self.draw_arc.arc_a0 = (arc_start - arc_center).angle_in_radians() as f32;
        self.draw_arc.arc_a1 = (arc_end - arc_center).angle_in_radians() as f32;
        self.draw_arc.arc_center = (arc_center - rect.pos).into_vec2();        
        
        self.draw_arc.color = self.color;
        self.draw_arc.width = self.line_width as f32;
        self.draw_arc.draw_abs(cx, rect);
            
        
    }


    fn walk(&mut self, _cx:&mut Cx) -> Walk {self.walk}
    
    fn draw_walk_widget(&mut self, cx: &mut Cx2d, walk: Walk) -> WidgetDraw {
        self.draw_walk(cx, walk);
        WidgetDraw::done()
    }

}

