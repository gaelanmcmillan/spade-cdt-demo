use macroquad::prelude as mq;
use spade::{ConstrainedDelaunayTriangulation, Triangulation};

fn mq_vec2_to_spade_point2<T: From<f32>>(vec: mq::Vec2) -> spade::Point2<T> {
    spade::Point2 {
        x: vec.x.into(),
        y: vec.y.into(),
    }
}

fn position_local_to_pixels(ploc: mq::Vec2) -> mq::Vec2 {
    mq::Vec2 {
        x: ((ploc.x + 1.) / 2.) * mq::screen_width(),
        y: ((ploc.y + 1.) / 2.) * mq::screen_height(),
    }
}

#[macroquad::main("Learning Spade")]
async fn main() {
    let mut pts: Vec<mq::Vec2> = vec![];
    let mut cdt = ConstrainedDelaunayTriangulation::<spade::Point2<f32>>::new();

    loop {
        mq::clear_background(mq::WHITE);

        if mq::is_mouse_button_pressed(mq::MouseButton::Left) {
            // add single points when 'S' is pressed
            if mq::is_key_down(mq::KeyCode::S) {
                let pt = mq::mouse_position_local();
                let sc_pt = position_local_to_pixels(pt);
                pts.push(pt);
                cdt.insert(mq_vec2_to_spade_point2(sc_pt))
                    .expect("Could not add point");

            // add boxes when 'B' is pressed
            } else if mq::is_key_down(mq::KeyCode::B) {
                let box_size_loc = 0.05;
                let center = mq::mouse_position_local();
                // clockwise from tl
                let corners = vec![
                    /* tl */ mq::Vec2::new(center.x - box_size_loc, center.y - box_size_loc),
                    /* tr */ mq::Vec2::new(center.x + box_size_loc, center.y - box_size_loc),
                    /* br */ mq::Vec2::new(center.x + box_size_loc, center.y + box_size_loc),
                    /* bl */ mq::Vec2::new(center.x - box_size_loc, center.y + box_size_loc),
                ];

                for corner in corners.iter() {
                    pts.push(*corner);
                }

                // insert each vertex into the triangulation, getting its handle.
                let handles: Vec<_> = corners
                    .iter()
                    .map(|corner| {
                        let local_pixels = position_local_to_pixels(*corner);
                        let handle = cdt.insert(mq_vec2_to_spade_point2(local_pixels)).unwrap();
                        handle
                    })
                    .collect();

                for pair in handles.windows(2) {
                    if cdt.can_add_constraint(pair[0], pair[1]) {
                        cdt.add_constraint(pair[0], pair[1]);
                    }
                }
            }
        }

        pts.iter().for_each(|pt| {
            let mq::Vec2 { x, y } = position_local_to_pixels(*pt);
            mq::draw_circle(x, y, 3., mq::BLACK);
        });

        cdt.undirected_edges().for_each(|edge| {
            let [begin, end] = edge.positions();
            mq::draw_line(begin.x, begin.y, end.x, end.y, 1., mq::DARKGREEN);
        });

        mq::next_frame().await;
    }
}
