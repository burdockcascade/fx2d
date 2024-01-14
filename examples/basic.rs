use fx2d::run;
use fx2d::scenegraph::group::GroupNode;
use fx2d::scenegraph::Parent;
use fx2d::scenegraph::scene::Scene;
use fx2d::scenegraph::sprite::Sprite;

fn main() {

    // test sprite
    let mysprite1 = Sprite::new(r#"D:\Projects\fx2d\examples\happy-tree-cartoon.png"#);
    let mysprite2 = Sprite::new(r#"D:\Projects\fx2d\examples\sample.png"#);

    // world
    let mut world = Scene::default();
    let group = GroupNode::with_children(vec![Box::new(mysprite2), Box::new(mysprite1)]);
    world.add_child(Box::new(group));


    pollster::block_on(run(world));
}