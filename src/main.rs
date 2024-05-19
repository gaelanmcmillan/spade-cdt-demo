use macroquad::prelude as mq;
use spade::{ConstrainedDelaunayTriangulation, Triangulation};

#[macroquad::main("Learning Spade")]
async fn main() {
    let mut cdt = ConstrainedDelaunayTriangulation::<spade::Point2<f32>>::new();

    loop {
        mq::clear_background(mq::WHITE);

        if mq::is_key_down(mq::KeyCode::B) {
            let box_size_pixels = 25.;
            let (cx, cy) = mq::mouse_position();

            // box corners, clockwise from tl
            let corners = vec![
                /* tl */ spade::Point2::new(cx - box_size_pixels, cy - box_size_pixels),
                /* tr */ spade::Point2::new(cx + box_size_pixels, cy - box_size_pixels),
                /* br */ spade::Point2::new(cx + box_size_pixels, cy + box_size_pixels),
                /* bl */ spade::Point2::new(cx - box_size_pixels, cy + box_size_pixels),
            ];

            // show preview
            for i in 0..corners.len() {
                let a = corners[i];
                let b = corners[(i + 1) % corners.len()];
                mq::draw_line(a.x, a.y, b.x, b.y, 1., mq::GRAY);
            }

            // add the box to the triangulation on click
            if mq::is_mouse_button_pressed(mq::MouseButton::Left) {
                // insert each vertex into the triangulation, getting its handle.
                let handles: Vec<_> = corners
                    .iter()
                    .map(|corner| {
                        let handle = cdt.insert(*corner).unwrap();
                        handle
                    })
                    .collect();

                // add constraint edges between each adjacent pair of box corners
                for i in 0..handles.len() {
                    let a = handles[i];
                    let b = handles[(i + 1) % handles.len()];

                    if cdt.can_add_constraint(a, b) {
                        cdt.add_constraint(a, b);
                    }
                }
            }
        }

        cdt.undirected_edges().for_each(|edge| {
            let (thickness, color) = if edge.is_constraint_edge() {
                (2., mq::PURPLE)
            } else {
                (1., mq::GREEN)
            };
            let [begin, end] = edge.positions();
            mq::draw_line(begin.x, begin.y, end.x, end.y, thickness, color);
        });

        let (mx, my) = mq::mouse_position();
        let info = cdt.locate(spade::Point2::new(mx, my));

        match info {
            spade::PositionInTriangulation::OnVertex(_) => {}
            spade::PositionInTriangulation::OnEdge(_) => {}
            spade::PositionInTriangulation::OnFace(face_handle) => {
                let face = cdt.face(face_handle);
                let is_constraint_face = face
                    .adjacent_edges()
                    .iter()
                    .filter(|edge_handle| cdt.directed_edge(edge_handle.fix()).is_constraint_edge())
                    .count()
                    > 1;

                if is_constraint_face {
                    let [a, b, c] = face.positions();
                    let mut col = mq::PURPLE;
                    col.a -= 0.5;
                    mq::draw_triangle(
                        mq::Vec2::new(a.x, a.y),
                        mq::Vec2::new(b.x, b.y),
                        mq::Vec2::new(c.x, c.y),
                        col,
                    );
                }
            }
            spade::PositionInTriangulation::OutsideOfConvexHull(_) => {}
            spade::PositionInTriangulation::NoTriangulation => {}
        }

        mq::draw_text(format!("{:?}", info).as_str(), 10., 20., 12., mq::BLACK);

        mq::next_frame().await;
    }
}
