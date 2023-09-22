use render_mate::Camera;
use render_mate::Color;
use render_mate::Extent;
use render_mate::Framebuffer;
use render_mate::Mesh;
use render_mate::PointLight;
use render_mate::Scene;
use render_mate::Sphere;
use render_mate::Texture;
use render_mate::Vec3;

fn main() {
    let mut scene = Scene::new();

    let white_texture = Texture::from("white_texture.bmp");
    let red_texture = Texture::from("red_texture.png");
    let green_texture = Texture::from("green_texture.png");
    let blue_texture = Texture::from("blue_texture.png");
    let yellow_texture = Texture::from("yellow_texture.png");
    let null_texture = Texture::from("null_texture.bmp");

    let sky_box = Mesh::new_cube(
        &white_texture,
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(3.0, 3.0, 3.0),
        0.0
    );

    scene.add_node(&sky_box);

    let light0 = Mesh::new_cube(
        &blue_texture,
        Vec3::new(0.0, 1.74, 1.0),
        Vec3::new(0.5, 0.5, 0.5),
        5.0,
    );

    scene.add_node(&light0);

    let light1 = Mesh::new_cube(
        &red_texture,
        Vec3::new(-1.74, 0.0, 1.0),
        Vec3::new(0.5, 0.5, 0.5),
        5.0,
    );

    scene.add_node(&light1);

    let light2 = Mesh::new_cube(
        &green_texture,
        Vec3::new(1.74, 0.0, 1.0),
        Vec3::new(0.5, 0.5, 0.5),
        5.0,
    );

    scene.add_node(&light2);

    //let light3 = Mesh::new_cube(
    //    &yellow_texture,
    //    Vec3::new(0.0, -1.0, 1.0),
    //    Vec3::new(0.25, 0.25, 0.25),
    //    5.0,
    //);

    //scene.add_node(&light3);


    let light4 = Sphere::new(
        Vec3::new(0.0, -1.0, 1.0),
        0.25,
        Color::new(1.0, 1.0, 1.0, 1.0),
        3.0,
    );

    scene.add_node(&light4);


    let box0 = Mesh::new_cube(
        &null_texture,
        Vec3::new(-0.75, -1.0, 1.0),
        Vec3::new(0.5, 1.0, 0.5),
        0.0
    );
    scene.add_node(&box0);

    //let box1 = Mesh::new_cube(
    //    &null_texture,
    //    Vec3::new(0.75, 0.0, 1.0),
    //    Vec3::new(0.5, 3.0, 1.0),
    //    0.0
    //);
    //scene.add_node(&box1);

    let box1 = Mesh::new_cube(
        &null_texture,
        Vec3::new(0.75, -1.0, 1.0),
        Vec3::new(0.5, 1.0, 0.5),
        0.0
    );
    scene.add_node(&box1);
    

    scene.add_point_light(
        PointLight::new(
            Vec3::new(0.0, 0.0, 1.0),
            Color::new(1.0, 1.0, 1.0, 1.0),
        ),
    );



    let framebuffer_extent = Extent::new(512, 512);

    let camera = Camera::new(framebuffer_extent);

    let mut fb0 = Framebuffer::new(framebuffer_extent);
    let mut fb1 = Framebuffer::new(framebuffer_extent);
    let mut fb2 = Framebuffer::new(framebuffer_extent);
    let mut fb3 = Framebuffer::new(framebuffer_extent);
    let mut fb4 = Framebuffer::new(framebuffer_extent);
    let mut fb5 = Framebuffer::new(framebuffer_extent);
    let mut fb6 = Framebuffer::new(framebuffer_extent);
    let mut fb7 = Framebuffer::new(framebuffer_extent);

    std::thread::scope(|scope| {
        scope.spawn(|| {
            scene.render(&camera, &mut fb0);
        });
        scope.spawn(|| {
            scene.render(&camera, &mut fb1);
        });
        scope.spawn(|| {
            scene.render(&camera, &mut fb2);
        });
        scope.spawn(|| {
            scene.render(&camera, &mut fb3);
        });
        scope.spawn(|| {
            scene.render(&camera, &mut fb4);
        });
        scope.spawn(|| {
            scene.render(&camera, &mut fb5);
        });
        scope.spawn(|| {
            scene.render(&camera, &mut fb6);
        });
        scope.spawn(|| {
            scene.render(&camera, &mut fb7);
        });
    });

    fb0.merge(&fb1);
    fb2.merge(&fb3);
    fb4.merge(&fb5);
    fb6.merge(&fb7);

    fb0.merge(&fb2);
    fb4.merge(&fb6);

    fb0.merge(&fb4);

    fb0.save("res.png");
}