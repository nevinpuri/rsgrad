use layout::backends::svg::SVGWriter;
use layout::core::geometry::Point;
use layout::core::style::StyleAttr;
use layout::core::utils::save_to_file;
use layout::std_shapes::shapes::{Element, ShapeKind};
use layout::topo::layout::VisualGraph;

// TODO: implement visualize functions to help with visualizations of values
// https://docs.rs/layout-rs/0.1.1/layout/ for drawing the graphs
// https://github.com/RazrFalcon/resvg for rendering the graphs
// or get some window open library which can read svgs and do that on there, also add visualize as a feature
use layout::core::base::Orientation;
fn simple_graph() {
    let mut vg = VisualGraph::new(Orientation::LeftToRight);
    let sp0 = ShapeKind::new_box("hi");
    let look0 = StyleAttr::simple();
    let sz = Point::new(100., 100.);

    let node0 = Element::create(sp0, look0, Orientation::LeftToRight, sz);

    let handle0 = vg.add_node(node0);

    let mut svg = SVGWriter::new();
    vg.do_it(false, false, false, &mut svg);

    let _ = save_to_file("/home/nevin/Desktop/rsgrad/graph.svg", &svg.finalize());
}

mod test {
    use super::simple_graph;

    #[test]
    fn test_graph() {
        simple_graph();
    }
}
