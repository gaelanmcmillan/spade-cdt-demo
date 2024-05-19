use macroquad::prelude as mq;
use spade::{ConstrainedDelaunayTriangulation, Triangulation};

#[macroquad::main("Learning Spade")]
async fn main() {
    let mut cdt = ConstrainedDelaunayTriangulation::<spade::Point2<f32>>::new();

    loop {
        mq::clear_background(mq::WHITE);

        if mq::is_mouse_button_pressed(mq::MouseButton::Left) {
            // add single points when 'S' is pressed
            if mq::is_key_down(mq::KeyCode::S) {
                let (x, y) = mq::mouse_position();
                cdt.insert(spade::Point2::new(x, y))
                    .expect("Could not add point");

            // add boxes when 'B' is pressed
            } else if mq::is_key_down(mq::KeyCode::B) {
                let box_size_pixels = 10.;
                let (cx, cy) = mq::mouse_position();
                // clockwise from tl
                let corners = vec![
                    /* tl */ spade::Point2::new(cx - box_size_pixels, cy - box_size_pixels),
                    /* tr */ spade::Point2::new(cx + box_size_pixels, cy - box_size_pixels),
                    /* br */ spade::Point2::new(cx + box_size_pixels, cy + box_size_pixels),
                    /* bl */ spade::Point2::new(cx - box_size_pixels, cy + box_size_pixels),
                ];

                // insert each vertex into the triangulation, getting its handle.
                let handles: Vec<_> = corners
                    .iter()
                    .map(|corner| {
                        let handle = cdt.insert(*corner).unwrap();
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

        cdt.undirected_edges().for_each(|edge| {
            let [begin, end] = edge.positions();
            mq::draw_line(begin.x, begin.y, end.x, end.y, 1., mq::DARKGREEN);
        });

        mq::next_frame().await;
    }
}
